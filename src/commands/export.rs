use crate::error::Error;
use crate::storage::Storage;
use crate::utils;
use std::{fs, path::Path};

pub fn run(storage: &Storage, file: Option<&str>) -> Result<(), Error> {
    let export_path = file.unwrap_or("./wharf.json");
    let desc = storage.load_descriptions()?;

    if Path::new(export_path).exists() {
        if !utils::confirm(&format!("File '{}' exists. Overwrite?", export_path)) {
            println!("Export cancelled");
            return Ok(());
        }
    }

    let export_dir = Path::new(export_path).parent().unwrap_or(Path::new("."));
    if !export_dir.exists() {
        fs::create_dir_all(export_dir)?;
    }

    let content = serde_json::to_string_pretty(&desc)?;
    fs::write(export_path, content)?;

    println!("Exported to {}", export_path);

    Ok(())
}
