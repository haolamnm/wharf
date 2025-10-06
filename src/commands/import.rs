use crate::errors::Error;
use crate::storage::Storage;
use crate::utils;
use std::{fs, path::Path};

pub fn run(storage: &Storage, file: &str) -> Result<(), Error> {
    if !Path::new(file).exists() {
        return Err(Error::ImportError(format!(
            "file '{}' does not exist",
            file
        )));
    }

    // Validate JSON
    let content = fs::read_to_string(file)?;
    let _: serde_json::Value = serde_json::from_str(&content)
        .map_err(|_| Error::ImportError("file is not valid JSON".to_string()))?;

    println!("WARNING: this will overwrite ALL current descriptions");
    if !utils::confirm("continue?") {
        println!("cancelled");
        return Ok(());
    }

    if storage.get_storage_path().exists() {
        let backup_path = storage.backup_descriptions()?;
        println!("backup created: {}", backup_path);
    }

    fs::copy(file, &storage.get_storage_path())?;
    println!("imported from {}", file);

    Ok(())
}
