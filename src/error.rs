use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse devcontainer config: {0}")]
    ConfigParse(String),

    #[error("Failed to load settings: {0}")]
    SettingsLoad(String),

    #[error("Provider error: {0}")]
    #[allow(dead_code)]
    Provider(String),
}

pub type Result<T> = std::result::Result<T, Error>;
