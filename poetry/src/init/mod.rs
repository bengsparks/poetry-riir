use chrono::Datelike;
use std::fs::{self, File};
use std::path::PathBuf;

use std::io::{Error, ErrorKind, Write};

pub mod license;

#[derive(Debug)]
pub struct Options {
    pub path: PathBuf,

    pub name: String,

    pub description: String,

    pub author: String,

    pub license: Option<license::Kind>,
}

pub fn climain(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    sanity_checks(&options)?;

    if let Some(license) = create_license_from_options(&options)? {
        let mut license_path = options.path.clone();
        license_path.push("LICENSE.md");

        fs::create_dir_all(&options.path)?;
        let mut file = File::create(&license_path)?;
        file.write_all(license.as_bytes())?;
    }

    return Ok(());
}

fn sanity_checks(options: &Options) -> Result<(), Box<dyn std::error::Error>> {
    return if options.path.exists() {
        Err(Box::new(Error::new(
            ErrorKind::IsADirectory,
            format!(
                "The provided path {path} is a directory",
                path = options.path.display().to_string()
            ),
        )))
    } else if !options.path.is_dir() {
        Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "The provided path {path} is not a directory",
                path = options.path.display().to_string()
            ),
        )))
    } else if options.path.read_dir()?.next().is_none() {
        Err(Box::new(Error::new(
            ErrorKind::DirectoryNotEmpty,
            format!(
                "The provided path {path} is not a directory",
                path = options.path.display().to_string()
            ),
        )))
    } else {
        Ok(())
    };
}

fn create_license_from_options(
    options: &Options,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    return Ok(match &options.license {
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
    });
}

fn create_bare_pyproject() -> Result<(), Box<dyn std::error::Error>> {
    return Ok(());
}
