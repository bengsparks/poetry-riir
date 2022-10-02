use std::path::PathBuf;

use crate::{
    pyproject::{DependencyMetadata, Versioned},
    error::{self, PoetryError},
};

use super::{
    package_format,
    provider::{PackageProvider, ProjectResp},
};

pub struct PyPi {
    pub output: PathBuf,
}

#[async_trait::async_trait]
impl PackageProvider for PyPi {
    type Package = package_format::Named;

    async fn download(
        &self,
        packages: Vec<Self::Package>,
    ) -> Result<Vec<DependencyMetadata>, PoetryError> {
        let client = reqwest::Client::new();

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
            let resp = response.json::<ProjectResp>().await?;

            let version = DependencyMetadata::Versioned(Versioned {
                version: if let Ok(v) = semver::Version::parse(&resp.info.version) {
                    v
                } else {
                    return Err(error::PoetryError::SemverError {
                        name: resp.info.name,
                        version: resp.info.version,
                    });
                },
            });

            projresps.push(version);
        }

        return Ok(projresps);
    }
}
