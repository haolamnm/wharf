use crate::errors::Error;
use dirs::data_local_dir;
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
    #[serde(default = "default_max_backups")]
    pub max_backups: usize,

    #[serde(default = "default_timestamp_format")]
    pub timestamp_format: String,
}

fn default_storage_dir() -> PathBuf {
    data_local_dir()
        .map(|dir| dir.join("wharf"))
        .unwrap_or_else(|| PathBuf::from("wharf"))
}

fn default_backup_dir() -> PathBuf {
    default_storage_dir().join("backups")
}

fn default_max_backups() -> usize {
    5
}

fn default_timestamp_format() -> String {
    "%Y%m%d-%H%M%S".to_string()
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
            max_backups: default_max_backups(),
            timestamp_format: default_timestamp_format(),
        }
    }
}
