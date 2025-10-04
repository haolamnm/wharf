use crate::error::Error;
use crate::storage::Storage;
use crate::utils;
use std::{fs, path::Path};

pub fn run(storage: &Storage, file: &str) -> Result<(), Error> {
    if !Path::new(file).exists() {
        return Err(Error::ImportError(format!(
            "File '{}' does not exist",
            file
        )));
    }

    // Validate JSON
    let content = fs::read_to_string(file)?;
    let _: serde_json::Value = serde_json::from_str(&content)
        .map_err(|_| Error::ImportError("File is not valid JSON".to_string()))?;

    // Create backup using Storage method
    if storage.get_storage_path().exists() {
        let backup_path = storage.backup_descriptions()?;
        println!("Backup created: {}", backup_path);
    }

    println!("Warning: This will overwrite ALL current descriptions");
    if !utils::confirm("Continue?") {
        println!("Import cancelled");
        return Ok(());
    }

    fs::copy(file, &storage.get_storage_path())?;
    println!("Imported from {}", file);

    Ok(())
}
