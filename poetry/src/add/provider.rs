use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{pyproject::DependencyMetadata, error::PoetryError};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResp {
    pub info: Info,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}

#[async_trait]
pub trait PackageProvider: Sized {
    type Package: Send;
    async fn download(
        &self,
        packages: Vec<Self::Package>,
    ) -> Result<Vec<DependencyMetadata>, PoetryError>;
}
