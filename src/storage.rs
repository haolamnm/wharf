use crate::config::{BackupConfig, Config};
use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct DescriptionsMap {
    pub descriptions: HashMap<String, String>,
}

impl DescriptionsMap {
    pub fn new() -> Self {
        Self {
            descriptions: HashMap::new(),
        }
    }
}

pub struct Storage {
    pub storage_path: PathBuf,
    pub backup_dir: PathBuf,
    pub backup_config: BackupConfig,
}

impl Storage {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let storage_path = config.dirs.storage_dir.join("wharf.json");
        let backup_dir = config.dirs.backup_dir.clone();

        Ok(Self {
            storage_path,
            backup_dir,
            backup_config: config.backup.clone(),
        })
    }

    pub fn ensure_storage_dir(&self) -> Result<(), Error> {
        if let Some(parent) = self.storage_path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }

    pub fn ensure_backup_dir(&self) -> Result<(), Error> {
        fs::create_dir_all(&self.backup_dir)?;
        Ok(())
    }

    pub fn load_descriptions(&self) -> Result<DescriptionsMap, Error> {
        self.ensure_storage_dir()?;
        if !self.storage_path.exists() {
            let desc = DescriptionsMap::new();
            self.save_descriptions(&desc)?;
            return Ok(desc);
        }
        let content = fs::read_to_string(&self.storage_path)?;
        if content.trim().is_empty() {
            return Ok(DescriptionsMap::new());
        }
        let desc = serde_json::from_str(&content)?;
        Ok(desc)
    }

    pub fn save_descriptions(&self, desc: &DescriptionsMap) -> Result<(), Error> {
        self.ensure_storage_dir()?;
        let content = serde_json::to_string_pretty(desc)?;
        fs::write(&self.storage_path, content)?;

        Ok(())
    }

    pub fn backup_descriptions(&self) -> Result<String, Error> {
        if !self.storage_path.exists() {
            return Err(Error::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no descriptions file to backup",
            )));
        }

        self.ensure_backup_dir()?;

        let timestamp = chrono::Local::now().format(&self.backup_config.timestamp_format);
        let backup_filename = format!("wharf.bak.{}.json", timestamp);
        let backup_path = self.backup_dir.join(backup_filename);

        fs::copy(&self.storage_path, &backup_path)?;
        Ok(backup_path.to_string_lossy().into_owned())
    }

    pub fn get_storage_path(&self) -> &PathBuf {
        &self.storage_path
    }
}
