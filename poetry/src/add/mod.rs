mod package_format;
mod provider;
mod pypi;

use std::collections::HashMap;
use std::path::PathBuf;

use reqwest::Client;

use crate::document;
use crate::error::PoetryError;

use self::pypi::ProjectResp;

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
    let client = Client::new();

    let projects = pypi_project(&client, &options.deps).await?;

    let mut config = document::load_pyproject(&options.working_directory)?;

    let requested: HashMap<_, _> = projects
        .iter()
        .map(|p| (p.info.name.clone(), p.info.version.clone()))
        .collect();

    let prev_count = match config.tool.poetry.dependency {
        Some(ref s) => s.len(),
        None => 0,
    };

    let preexisting = config
        .tool
        .poetry
        .dependency
        .get_or_insert_with(HashMap::new);

    preexisting.extend(requested.clone());
    if prev_count + requested.len() != preexisting.len() {
        return Err(PoetryError::AddError {
            deps: options.deps.clone(),
        });
    }

    document::write_pyproject(&options.working_directory, &config)?;

    return Ok(());
}

async fn pypi_project(
    client: &Client,
    deps: &Vec<String>,
) -> Result<Vec<ProjectResp>, PoetryError> {
    let mut requests = Vec::with_capacity(deps.len());
    for dep in deps {
        let req = client
            .get(format!("https://pypi.org/pypi/{dep}/json"))
            .header("Host", "pypi.org")
            .header("Accept", "application/json")
            .send();
        requests.push(req);
    }

    let responses = futures::future::try_join_all(requests)
        .await
        .map_err(|e| PoetryError::ReqwestError { source: e })?;

    let mut projresps = Vec::with_capacity(deps.len());
    for response in responses {
        projresps.push(response.json::<ProjectResp>().await?);
    }
    return Ok(projresps);
}
