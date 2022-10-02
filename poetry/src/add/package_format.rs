use std::path::{Path, PathBuf};

use git_url_parse::{GitUrl, Scheme};
use semver::VersionReq;

use crate::error::PoetryError;
use crate::pyproject::DependencyMetadata;

use super::provider::PackageProvider;
use super::{git::GitProvider, pypi::PyPi};

#[derive(Debug)]
pub enum PackageFormat {
    Named(Named),
    Git(Git),
    FilePath(FilePath),
    Folder(Folder),
}

pub struct Packages {
    pub packages: Vec<PackageFormat>,
}

pub enum PackageChange {
    Insert {
        name: String,
        metadata: DependencyMetadata,
    },
    Update {
        name: String,
        metadata: DependencyMetadata,
    },
    Remove {
        name: String,
        metadata: DependencyMetadata,
    },
}

pub struct PackageChanges {
    pub changes: Vec<PackageChange>,
}

impl Packages {
    pub async fn download(self, output: &Path) -> Result<Vec<DependencyMetadata>, PoetryError> {
        let mut responses = Vec::with_capacity(self.packages.len());

        let mut nameds = Vec::with_capacity(self.packages.len());
        let mut gits = Vec::with_capacity(self.packages.len());

        for package in self.packages {
            match package {
                PackageFormat::Named(named) => nameds.push(named),
                PackageFormat::Git(url) => gits.push(url),
                _ => {}
            }
        }

        let pypi = PyPi {
            output: output.to_path_buf(),
        };
        responses.append(&mut pypi.download(nameds).await?);

        let git = GitProvider {
            output: output.to_path_buf(),
        };
        responses.append(&mut git.download(gits).await?);

        return Ok(responses);
    }
}

impl std::convert::TryFrom<&String> for PackageFormat {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return if let Ok(git) = Git::try_from(value) {
            Ok(PackageFormat::Git(git))
        } else if let Ok(fp) = FilePath::try_from(value) {
            Ok(PackageFormat::FilePath(fp))
        } else if let Ok(fldr) = Folder::try_from(value) {
            Ok(PackageFormat::Folder(fldr))
        } else if let Ok(named) = Named::try_from(value) {
            Ok(PackageFormat::Named(named))
        } else {
            Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            })
        };
    }
}

impl std::convert::TryFrom<&Vec<String>> for Packages {
    type Error = PoetryError;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let multi: Result<Vec<_>, _> = value.iter().map(|v| v.try_into()).collect();

        return multi.map(|pfs| Packages { packages: pfs });
    }
}

#[derive(Debug)]
pub struct Named {
    pub name: String,
    pub constraint: Option<VersionReq>,
}

impl std::convert::TryFrom<&String> for Named {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let segments = &mut value.split('@');

        let package_name = segments
            .next()
            .ok_or_else(|| PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            })?;

        let version = match segments.next() {
            Some(c) => c,
            None => {
                return Ok(Named {
                    name: package_name.to_owned(),
                    constraint: None,
                })
            }
        };

        let constraint = VersionReq::parse(version).map_err(|_| PoetryError::SemverError {
            name: value.to_owned(),
            version: version.to_owned(),
        })?;

        if segments.next().is_some() {
            return Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            });
        }

        return Ok(Named {
            name: package_name.to_owned(),
            constraint: Some(constraint),
        });
    }
}

#[derive(Debug)]
pub struct Git {
    pub name: String,
    pub url: String,
    pub revision: Option<String>,
}

impl std::convert::TryFrom<&String> for Git {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let mut splits = value.split('#');

        let url = splits.next().map(GitUrl::parse).transpose().map_err(|_| {
            PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            }
        })?;

        let giturl = if let Some(GitUrl {
            scheme: Scheme::Http | Scheme::Https | Scheme::Git | Scheme::Ssh | Scheme::GitSsh,
            name,
            ..
        }) = url
        {
            Git {
                name,
                url: value.to_owned(),
                revision: splits.next().map(ToOwned::to_owned),
            }
        } else {
            return Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            });
        };

        if splits.next().is_some() {
            return Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            });
        }

        return Ok(giturl);
    }
}

#[derive(Debug)]
pub struct FilePath {
    pub file_path: PathBuf,
}

impl std::convert::TryFrom<&String> for FilePath {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return Err(PoetryError::UnknownPackageFormat {
            format: value.to_owned(),
        });
    }
}

#[derive(Debug)]
pub struct Folder {
    pub folder_path: PathBuf,
}

impl std::convert::TryFrom<&String> for Folder {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return Err(PoetryError::UnknownPackageFormat {
            format: value.to_owned(),
        });
    }
}
