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
            Error::ConfigError(msg) => write!(f, "configuration error: {}", msg),
            Error::IoError(err) => write!(f, "io error: {}", err),
            Error::JsonError(err) => write!(f, "json error: {}", err),
            Error::TomlError(err) => write!(f, "toml error: {}", err),
            Error::PathNotFound(path) => write!(f, "path not found: {}", path),
            Error::DescriptionNotFound(path) => write!(f, "description not found for: {}", path),
            Error::EmptyDescription => write!(f, "description cannot be empty"),
            Error::ImportError(msg) => write!(f, "import error: {}", msg),
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
