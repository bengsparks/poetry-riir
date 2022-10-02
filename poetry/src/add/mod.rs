pub mod package_format;

mod git;
mod provider;
mod pypi;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::PoetryError;
use crate::pyproject::{self, PyProject};
use crate::virtualenv::Virtualenv;

use self::package_format::Packages;

use derive_builder::Builder;

#[derive(Builder, Clone, Debug)]
#[builder(derive(Debug))]
pub struct Options {
    #[builder(field(public))]
    pub deps: Vec<String>,

    #[builder(default = r#"String::from("main")"#)]
    pub group: String,

    #[builder(default = r#"false"#)]
    pub dev: bool,

    #[builder(default)]
    pub extras: Vec<String>,

    #[builder(default = r#"false"#)]
    pub dry_run: bool,

    #[builder(
        default = r#"std::env::current_dir().expect("Could not get current working directory")"#
    )]
    pub working_directory: PathBuf,
}

pub async fn climain(options: Options) -> Result<(), PoetryError> {
    let config = Config::load(&options.working_directory)?;
    let mut venv = Virtualenv::create_from_config(&config, &options.working_directory)?;
    
    let packages: Packages = (&options.deps).try_into()?;
    let metadata = packages.download(&config.virtualenvs.path).await?;

    let mut pyproject = PyProject::load(&options.working_directory)?;
    pyproject = venv.apply_changes(pyproject, &options.deps, &metadata)?;
    pyproject.write(&options.working_directory)?;

    return Ok(());
}

fn update_dependencies(
    original: Option<HashMap<String, pyproject::DependencyMetadata>>,
    update: HashMap<String, pyproject::DependencyMetadata>,
) -> Result<HashMap<String, pyproject::DependencyMetadata>, PoetryError> {
    let mut updateable = original.unwrap_or_default();

    let (prev_count, update_count) = (updateable.len(), update.len());
    updateable.extend(update.clone());
    if prev_count + update_count != updateable.len() {
        return Err(PoetryError::AddError { deps: update });
    }

    return Ok(updateable);
}
