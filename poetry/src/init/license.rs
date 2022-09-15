use std::convert::TryFrom;

use crate::error::PoetryError;

#[derive(Default, Debug)]
pub(crate) struct LicenseFormatOptions {
    pub(crate) name: String,
    pub(crate) year: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Mit,
    Apache2,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        return match *self {
            Kind::Mit => String::from("MIT"),
            Kind::Apache2 => String::from("Apache-2.0"),
        };
    }
}

impl TryFrom<String> for Kind {
    type Error = PoetryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return match value.to_lowercase().as_str() {
            "mit" => Ok(Self::Mit),
            "apache-2.0" => Ok(Self::Apache2),
            _ => Err(PoetryError::UnknownLicense { license: value }),
        };
    }
}

#[derive(Debug)]
pub(crate) struct License {
    pub(crate) unfilled: Kind,
    pub(crate) fmt_options: LicenseFormatOptions,
}

impl ToString for License {
    fn to_string(&self) -> String {
        return match self.unfilled {
            Kind::Mit => {
                format!(
                    include_str!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/resources/license/MIT.md"
                    )),
                    name = self.fmt_options.name,
                    year = self.fmt_options.year
                )
            }
            Kind::Apache2 => {
                format!(
                    include_str!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/resources/license/Apache2.md"
                    )),
                    name = self.fmt_options.name,
                    year = self.fmt_options.year
                )
            }
        };
    }
}
