use reqwest;

use async_trait::async_trait;

use crate::error::PoetryError;

use crate::add::package_format;
use crate::add::pypi::ProjectResp;

pub async fn fetch_responses(
    client: &reqwest::Client,
    packages: Vec<package_format::PackageFormat>,
) -> Result<Vec<ProjectResp>, PoetryError> {
    let provider: BatchProvider = Default::default();
    return provider.request(client, packages).await;
}

#[async_trait]
trait PackageProvider: Sized {
    type Package: Send;
    async fn request(
        &self,
        client: &reqwest::Client,
        packages: Vec<Self::Package>,
    ) -> Result<Vec<ProjectResp>, PoetryError> {
        return self
            ._request(client, packages)
            .await
            .map_err(|e| PoetryError::ReqwestError { source: e });
    }

    async fn _request(
        &self,
        client: &reqwest::Client,
        packages: Vec<Self::Package>,
    ) -> Result<Vec<ProjectResp>, reqwest::Error>;
}

struct BatchProvider {
    pypi: PyPi,
}

#[async_trait]
impl PackageProvider for BatchProvider {
    type Package = package_format::PackageFormat;

    async fn _request(
        &self,
        client: &reqwest::Client,
        packages: Vec<Self::Package>,
    ) -> Result<Vec<ProjectResp>, reqwest::Error> {
        let mut responses = Vec::with_capacity(packages.len());

        let mut nameds = Vec::with_capacity(packages.len());

        for package in packages {
            match package {
                package_format::PackageFormat::Named(named) => nameds.push(named),
                _ => {}
            }
        }

        responses.append(&mut self.pypi._request(client, nameds).await?);
        return Ok(responses);
    }
}

impl Default for BatchProvider {
    fn default() -> Self {
        Self { pypi: PyPi {} }
    }
}

struct PyPi {}

#[async_trait]
impl PackageProvider for PyPi {
    type Package = package_format::Named;

    async fn _request(
        &self,
        client: &reqwest::Client,
        packages: Vec<Self::Package>,
    ) -> Result<Vec<ProjectResp>, reqwest::Error> {
        let mut requests = Vec::with_capacity(packages.len());
        for dep in packages {
            let req = client
                .get(format!(
                    "https://pypi.org/pypi/{name}/json",
                    name = dep.name
                ))
                .header("Host", "pypi.org")
                .header("Accept", "application/json")
                .send();
            requests.push(req);
        }

        let responses = futures::future::try_join_all(requests).await?;

        let mut projresps = Vec::with_capacity(responses.len());
        for response in responses {
            projresps.push(response.json::<ProjectResp>().await?);
        }
        return Ok(projresps);
    }
}
