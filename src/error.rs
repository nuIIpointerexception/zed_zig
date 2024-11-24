use std::fmt;

#[derive(Debug)]
pub enum Error {
    AssetNotFound(String),
    DownloadFailed(String),
    FileSystem(String),
    InstallationFailed(String),
    LanguageServer(String),
    FetchFailed { url: String, error: String },
    Missing { field: String },
    SerializationFailed(String),
    Settings(String),
    Configuration { message: String, fix: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AssetNotFound(s) => write!(f, "Asset not found: {}", s),
            Error::DownloadFailed(s) => write!(f, "Download failed: {}", s),
            Error::FileSystem(s) => write!(f, "File system error: {}", s),
            Error::InstallationFailed(s) => write!(f, "Installation failed: {}", s),
            Error::LanguageServer(s) => write!(f, "Language server error: {}", s),
            Error::FetchFailed { url, error } => {
                write!(f, "Failed to fetch from {}: {}", url, error)
            }
            Error::Missing { field } => write!(f, "Missing field in response: {}", field),
            Error::SerializationFailed(s) => write!(f, "Failed to serialize: {}", s),
            Error::Settings(s) => write!(f, "Failed to get settings: {}", s),
            Error::Configuration { message, fix } => {
                write!(f, "Configuration error: {}. {}", message, fix)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::FileSystem(error.to_string())
    }
}

impl From<Error> for String {
    fn from(err: Error) -> String {
        err.to_string()
    }
}

pub type Result<T> = zed_extension_api::Result<T>;
