use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum PoetryError {
    #[error("`{path}` is not a directory")]
    InitIsNotDirectory { path: String },

    #[error("`{path}` is not an empty directory")]
    InitIsNotEmptyDirectory { path: String },

    #[error("Failed during serialising TOML operation: {source}")]
    TomlSerError {
        #[from]
        source: toml::ser::Error,
    },

    #[error("Failed during deserialising TOML operation: {source}")]
    TomlDeError {
        #[from]
        source: toml::de::Error,
    },

    #[error("IO Error occurred: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to make request - details:\n{source}")]
    ReqwestError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Unable to create package format from {format}")]
    UnknownPackageFormat { format: String },

    #[error("Unknown license: {license}")]
    UnknownLicense { license: String },

    #[error("Could not add {deps:?} to pyproject.toml")]
    AddError { deps: Vec<String> },
}
