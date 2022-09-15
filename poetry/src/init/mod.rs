use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

use chrono::Datelike;

use crate::document;
use crate::error::PoetryError;

pub mod license;

#[derive(Clone, Debug)]
pub struct Options {
    pub path: PathBuf,

    pub name: String,

    pub description: String,

    pub author: String,

    pub license: Option<license::Kind>,
}

pub async fn climain(options: Options) -> Result<(), PoetryError> {
    sanity_checks(&options)?;

    create_bare_project(&options)?;

    if let Some(license) = license_from_options(&options) {
        let mut license_path = options.path.clone();
        license_path.push("LICENSE.md");

        let mut file = fs::File::create(&license_path)?;
        io::Write::write_all(&mut file, license.as_bytes())?;
    }

    return Ok(());
}

fn sanity_checks(options: &Options) -> Result<(), PoetryError> {
    return if options.path.exists() && !options.path.is_dir() {
        Err(PoetryError::IoError {
            source: io::Error::new(
                io::ErrorKind::InvalidData,
                format!("`{path}` is not a directory", path = options.path.display()),
            ),
        })
    } else if options.path.exists() && options.path.read_dir()?.next().is_some() {
        Err(PoetryError::IoError {
            source: io::Error::new(
                io::ErrorKind::DirectoryNotEmpty,
                format!(
                    "`{path}` is not an empty directory",
                    path = options.path.display()
                ),
            ),
        })
    } else {
        Ok(())
    };
}

fn license_from_options(options: &Options) -> Option<String> {
    return match &options.license {
        Some(license) => {
            let license = license::License {
                unfilled: *license,
                fmt_options: license::LicenseFormatOptions {
                    name: options.name.to_owned(),
                    year: chrono::Utc::now().year(),
                },
            };

            Some(license.to_string())
        }
        None => None,
    };
}

fn create_bare_project(options: &Options) -> Result<(), PoetryError> {
    fs::create_dir_all(&options.path)?;
    return document::write_pyproject(
        &options.path,
        &document::PyProject {
            tool: document::Tool {
                poetry: document::Poetry {
                    name: options.name.clone(),
                    version: "0.1.0".to_string(),
                    description: options.description.clone(),
                    license: options.license.map(|l| ToString::to_string(&l)),

                    authors: None,
                    maintainers: None,
                    readme: None,
                    url: None,
                    repository: None,
                    documentation: None,

                    dependency: None,
                },
            },
        },
    );
}
