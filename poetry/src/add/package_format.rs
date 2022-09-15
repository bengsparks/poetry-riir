use std::path::PathBuf;

use semver::VersionReq;

use crate::error::PoetryError;

#[derive(Debug)]
pub enum PackageFormat {
    Named(Named),
    GitUrl(GitUrl),
    FilePath(FilePath),
    Folder(Folder),
}

impl std::convert::TryFrom<&String> for PackageFormat {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return if let Ok(named) = Named::try_from(value) {
            Ok(PackageFormat::Named(named))
        } else if let Ok(git) = GitUrl::try_from(value) {
            Ok(PackageFormat::GitUrl(git))
        } else if let Ok(fp) = FilePath::try_from(value) {
            Ok(PackageFormat::FilePath(fp))
        } else if let Ok(fldr) = Folder::try_from(value) {
            Ok(PackageFormat::Folder(fldr))
        } else {
            Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            })
        };
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

        // Check all characters before this position are alphabetical
        if !package_name.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            });
        }

        let req = segments
            .next()
            .map(VersionReq::parse)
            .transpose()
            .map_err(|_| PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            });

        if segments.next().is_some() {
            return Err(PoetryError::UnknownPackageFormat {
                format: value.to_owned(),
            });
        }

        return Ok(Named {
            name: package_name.to_owned(),
            constraint: req?,
        });
    }
}

#[derive(Debug)]
pub enum GitKind {
    Https,
    Ssh,
}

#[derive(Debug)]
pub struct GitUrl {
    pub url: String,
    pub revision: Option<String>,
    pub kind: GitKind,
}

impl std::convert::TryFrom<&String> for GitUrl {
    type Error = PoetryError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        return Err(PoetryError::UnknownPackageFormat {
            format: value.to_owned(),
        });
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
