use thiserror::Error;

/// Application-level errors for `devcont`.
#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse devcontainer config: {0}")]
    ConfigParse(String),

    #[error("Failed to load settings: {0}")]
    SettingsLoad(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// A lifecycle hook command exited with a non-zero status.
    #[error("Lifecycle hook failed — {0}")]
    HookFailed(String),

    /// A container engine operation failed.
    #[error("Provider error: {0}")]
    ProviderError(String),
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Io(io_err) => io_err,
            other => std::io::Error::new(std::io::ErrorKind::Other, other.to_string()),
        }
    }
}

/// Convenience alias for `std::result::Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
