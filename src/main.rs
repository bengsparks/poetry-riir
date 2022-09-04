use std::path::PathBuf;

use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, ValueEnum};
use clap::{Command, CommandFactory, Parser};

use poetry::init;

#[derive(Clone, Debug, ValueEnum)]
enum LicenseKind {
    MIT,
}

impl From<LicenseKind> for init::license::Kind {
    fn from(lk: LicenseKind) -> Self {
        return match lk {
            LicenseKind::MIT => init::license::Kind::MIT,
        };
    }
}

#[derive(Parser, Debug)]
#[clap(name = "init")]
struct Init {
    #[clap(short, long, value_parser)]
    name: Option<String>,

    #[clap(short, long, value_parser)]
    description: Option<String>,

    #[clap(short, long, value_parser)]
    author: Option<String>,

    #[clap(short, long, arg_enum, value_parser)]
    license: Option<LicenseKind>,

    #[clap(short, long, arg_enum, value_parser, required = false)]
    path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(Init::command());

    match app.get_matches().subcommand() {
        Some(("init", m)) => init::climain(init::Options {
            name: m.get_one::<String>("name").cloned(),
            description: m.get_one::<String>("description").cloned(),
            author: m.get_one::<String>("author").cloned(),
            license: {
                let this = m.get_one::<LicenseKind>("license").cloned();
                this.map(Into::into)
            },
            path: {
                match m.get_one::<PathBuf>("path") {
                    Some(p) => p.to_owned(),
                    None => std::env::current_dir()?,
                }
            },
        }),
        _ => Ok(()),
    }
}
