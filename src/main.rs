use std::path::PathBuf;
use std::str::FromStr;

use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{AppSettings, ArgMatches, Command, CommandFactory, Parser, ValueEnum};

use dialoguer::{theme::ColorfulTheme, Input};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Command::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(Init::command());

    match app.get_matches().subcommand() {
        Some(("init", m)) => init(m),
        _ => Ok(()),
    }
}

#[derive(Parser, Debug)]
#[clap(name = "init")]
struct Init {
    #[clap(short, long, arg_enum, value_parser, required = false)]
    path: Option<PathBuf>,

    #[clap(short, long, value_parser)]
    name: Option<Vec<String>>,

    #[clap(short, long, value_parser)]
    description: Option<String>,

    #[clap(short, long, value_parser)]
    author: Option<String>,

    #[clap(short, long, arg_enum, value_parser)]
    license: Option<LicenseKind>,
}

fn init(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let options = init::Options {
        path: match matches.get_one::<PathBuf>("path") {
            Some(p) => p.to_owned(),
            None => std::env::current_dir()?,
        },

        name: match matches.get_one::<String>("name") {
            Some(n) => n.to_owned(),
            None => prompt("Name of the package")?,
        },
        description: match matches.get_one::<String>("description") {
            Some(d) => d.to_owned(),
            None => prompt("Description of the package")?,
        },
        author: match matches.get_one::<String>("author") {
            Some(a) => a.to_owned(),
            None => prompt("Author name of the package")?,
        },
        license: matches
            .get_one::<LicenseKind>("license")
            .cloned()
            .map(Into::into),
    };

    return init::climain(options);
}

/// Prompt for user input with the ability to provide a default value
fn prompt<T>(prompt: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: std::fmt::Debug + ToString,
{
    let theme = &ColorfulTheme::default();
    let mut builder = Input::<T>::with_theme(theme);
    builder.with_prompt(prompt);

    return Ok(builder.interact_text()?);
}
