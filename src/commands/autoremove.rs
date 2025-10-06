use crate::error::Error;
use crate::storage::Storage;
use crate::utils;

pub fn run(storage: &Storage) -> Result<(), Error> {
    // Create backup before making changes
    storage.backup_descriptions()?;

    // Load descriptions, filter out non-existent paths, and save
    let mut desc = storage.load_descriptions()?;
    let original_count = desc.descriptions.len();
    desc.descriptions.retain(|path, _| utils::path_exists(path));
    let removed_count = original_count - desc.descriptions.len();
    storage.save_descriptions(&desc)?;
    println!(
        "Removed {} descriptions for non-existent paths.",
        removed_count
    );

    Ok(())
}
