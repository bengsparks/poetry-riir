use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum PoetryError {
    #[error("`{path}` is not a directory")]
    InitIsNotDirectory { path: String },

    #[error("`{path}` is not an empty directory")]
    InitIsNotEmptyDirectory { path: String },

    #[error("Failed during TOML operatiom: {source}")]
    TomlError {
        #[from]
        source: toml::ser::Error,
    },

    #[error("IO Error occurred: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}
