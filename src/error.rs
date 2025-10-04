use std::fmt;

#[derive(Debug)]
pub enum Error {
    ConfigError(String),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    TomlError(toml::de::Error),
    PathNotFound(String),
    DescriptionNotFound(String),
    EmptyDescription,
    ImportError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Error::IoError(err) => write!(f, "I/O error: {}", err),
            Error::JsonError(err) => write!(f, "JSON error: {}", err),
            Error::TomlError(err) => write!(f, "TOML error: {}", err),
            Error::PathNotFound(path) => write!(f, "Path not found: {}", path),
            Error::DescriptionNotFound(path) => write!(f, "Description not found for: {}", path),
            Error::EmptyDescription => write!(f, "Description cannot be empty"),
            Error::ImportError(msg) => write!(f, "Import error: {}", msg),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlError(err)
    }
}
