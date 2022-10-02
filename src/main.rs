use std::path::PathBuf;
use std::str::FromStr;

use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{AppSettings, ArgMatches, Command, CommandFactory, Parser, ValueEnum};

use dialoguer::{theme::ColorfulTheme, Input};

use poetry::error::PoetryError;
use poetry::{add, init};

#[derive(Clone, Debug, ValueEnum)]
enum LicenseKind {
    Mit,
}

impl From<LicenseKind> for init::license::Kind {
    fn from(lk: LicenseKind) -> Self {
        return match lk {
            LicenseKind::Mit => init::license::Kind::Mit,
        };
    }
}

#[tokio::main]
async fn main() -> Result<(), PoetryError> {
    let app = Command::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .subcommand(Init::command())
        .subcommand(Add::command());

    match app.get_matches().subcommand() {
        Some(("init", m)) => init(m).await,
        Some(("add", m)) => add(m).await,
        _ => Ok(()),
    }
}

#[derive(Parser, Debug)]
#[clap(name = "init")]
struct Init {
    #[clap(short, long, arg_enum, value_parser, required = false)]
    path: Option<PathBuf>,

    #[clap(short, long, value_parser)]
    name: Option<String>,

    #[clap(short, long, value_parser)]
    description: Option<String>,

    #[clap(short, long, value_parser)]
    author: Option<String>,

    #[clap(short, long, arg_enum, value_parser)]
    license: Option<LicenseKind>,
}

async fn init(matches: &ArgMatches) -> Result<(), PoetryError> {
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

    return init::climain(options).await;
}

#[derive(Parser, Debug)]
#[clap(name = "add")]
struct Add {
    #[clap(value_parser)]
    deps: Vec<String>,

    #[clap(short, long, value_parser)]
    group: Option<String>,

    #[clap(short = 'D', long, action)]
    dev: bool,

    #[clap(short, long, value_parser)]
    extras: Option<Vec<String>>,

    #[clap(short, long, action)]
    dry_run: bool,

    #[clap(short, long, value_parser)]
    working_directory: PathBuf,
}

async fn add(matches: &ArgMatches) -> Result<(), PoetryError> {
    let options = add::Options {
        deps: match matches.get_many::<String>("deps") {
            Some(ds) => ds.into_iter().cloned().collect(),
            None => Vec::new(),
        },

        group: match matches.get_one::<String>("group") {
            Some(g) => g.to_owned(),
            None => String::from("main"),
        },

        dev: matches.get_flag("dev"),

        extras: match matches.get_many::<String>("extras") {
            Some(es) => es.into_iter().cloned().collect(),
            None => Vec::new(),
        },

        dry_run: matches.get_flag("dry-run"),

        working_directory: match matches.get_one::<PathBuf>("working_directory") {
            Some(wd) => wd.clone(),
            None => std::env::current_dir()?,
        },
    };

    return add::climain(options).await;
}

/// Prompt for user input
fn prompt<T>(prompt: &str) -> Result<T, PoetryError>
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: std::fmt::Debug + ToString,
{
    let theme = &ColorfulTheme::default();
    let mut builder = Input::<T>::with_theme(theme);
    builder.with_prompt(prompt);

    return Ok(builder.interact_text()?);
}
