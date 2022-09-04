#[derive(Default, Debug)]
pub(crate) struct LicenseFormatOptions {
    pub(crate) name: String,
    pub(crate) year: i32,
}

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    MIT,
}

impl From<Kind> for &'static str {
    fn from(kind: Kind) -> Self {
        return match kind {
            Kind::MIT => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/license/MIT.md"
            )),
        };
    }
}

#[derive(Debug)]
pub(crate) struct License {
    pub(crate) fmt_options: LicenseFormatOptions,
    pub(crate) unfilled: Kind,
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
