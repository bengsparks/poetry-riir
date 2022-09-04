use chrono::Datelike;
use std::fs::{File, self};
use std::path::PathBuf;

use std::io::{Error, ErrorKind, Write};

pub mod license;

#[derive(Default, Debug)]
pub struct Options {
    pub path: PathBuf,

    pub name: Option<String>,

    pub description: Option<String>,

    pub author: Option<String>,

    pub license: Option<license::Kind>,
}

pub fn climain(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(license) = &options.license {
        let license = create_license(*license, &options)?;

        let mut license_path = options.path.clone();
        license_path.push("LICENSE.md");

        fs::create_dir_all(&options.path)?;
        let mut file = File::create(&license_path)?;
        file.write_all(license.as_bytes())?;
    }

    return Ok(());
}

fn create_license(
    kind: license::Kind,
    options: &Options,
) -> Result<String, Box<dyn std::error::Error>> {
    let name = match &options.name {
        Some(name) => name,
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "Name is required to create license",
            )))
        }
    };

    let license = license::License {
        unfilled: kind,
        fmt_options: license::LicenseFormatOptions {
            name: name.to_string(),
            year: chrono::Utc::now().year(),
        },
    };

    return Ok(license.to_string());
}
