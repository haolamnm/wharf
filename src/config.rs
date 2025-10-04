use crate::error::Error;
use dirs::{cache_dir, data_dir};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub dirs: DirsConfig,
    pub backup: BackupConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DirsConfig {
    #[serde(default = "default_storage_dir")]
    pub storage_dir: PathBuf,

    #[serde(default = "default_backup_dir")]
    pub backup_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackupConfig {
    #[serde(default = "default_auto_backup")]
    pub auto_backup: bool,

    #[serde(default = "default_timestamp_format")]
    pub timestamp_format: String,
}

fn default_storage_dir() -> PathBuf {
    data_dir()
        .map(|dir| dir.join("wharf"))
        .unwrap_or_else(|| PathBuf::from("wharf"))
}

fn default_backup_dir() -> PathBuf {
    cache_dir()
        .map(|dir| dir.join("wharf"))
        .unwrap_or_else(|| PathBuf::from("wharf"))
}

fn default_auto_backup() -> bool {
    false
}

fn default_timestamp_format() -> String {
    "%Y-%m-%d_%H-%M-%S".to_string()
}

impl Config {
    pub fn load(config_path: &Path) -> Result<Self, Error> {
        let config_content = fs::read_to_string(config_path)?;
        let config = toml::from_str(&config_content)?;
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dirs: DirsConfig::default(),
            backup: BackupConfig::default(),
        }
    }
}

impl Default for DirsConfig {
    fn default() -> Self {
        Self {
            storage_dir: default_storage_dir(),
            backup_dir: default_backup_dir(),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            auto_backup: default_auto_backup(),
            timestamp_format: default_timestamp_format(),
        }
    }
}
