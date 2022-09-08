#[derive(Default, Debug)]
pub(crate) struct LicenseFormatOptions {
    pub(crate) name: String,
    pub(crate) year: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    MIT,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        return match *self {
            Kind::MIT => String::from("MIT"),
        }
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
            Kind::MIT => {
                format!(
                    include_str!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/resources/license/MIT.md"
                    )),
                    name = self.fmt_options.name,
                    year = self.fmt_options.year
                )
            }
        };
    }
}
