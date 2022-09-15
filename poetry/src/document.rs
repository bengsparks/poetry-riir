use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::PoetryError;

pub fn load_pyproject(folder: &Path) -> Result<PyProject, PoetryError> {
    let mut pyproject_path = folder.to_path_buf();
    pyproject_path.push("pyproject.toml");

    let de = std::fs::read_to_string(pyproject_path)?;
    return match toml::from_str(de.as_str()) {
        Ok(pp) => Ok(pp),
        Err(e) => Err(PoetryError::TomlDeError { source: e }),
    };
}

pub fn write_pyproject(folder: &Path, project: &PyProject) -> Result<(), PoetryError> {
    let mut pyproject_path = folder.to_path_buf();
    pyproject_path.push("pyproject.toml");

    let ser = toml::to_string(project)?;

    let mut file = fs::File::create(&pyproject_path)?;
    io::Write::write_all(&mut file, ser.as_bytes())?;

    return Ok(());
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PyProject {
    pub tool: Tool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tool {
    pub poetry: Poetry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Poetry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub maintainers: Option<Vec<String>>,
    pub readme: Option<ReadMe>,
    pub url: Option<String>,
    pub repository: Option<String>,
    pub documentation: Option<String>,

    pub dependency: Option<HashMap<String, DependencyMetadata>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Maintainer {
    name: String,
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReadMe {
    path: Vec<PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DependencyMetadata {
    version: semver::Version,
    source: Option<String>,
}
