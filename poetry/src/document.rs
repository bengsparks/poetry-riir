use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
