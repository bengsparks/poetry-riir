mod package_format;
mod provider;
mod pypi;

use reqwest::Client;

use crate::document;
use crate::error::PoetryError;

use self::pypi::ProjectResp;

pub struct Options {
    pub deps: Vec<String>,
    pub group: String,
    pub dev: bool,
    pub extras: Vec<String>,
    pub dry_run: bool,
}

pub async fn climain(options: Options) -> Result<(), PoetryError> {
    let client = Client::new();

    let projects = pypi_project(&client, &options.deps).await?;
    for project in projects {}

    let cwd = std::env::current_dir()?;
    let config = document::load_pyproject(&cwd)?;

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