use crate::errors::Error;
use crate::storage::Storage;
use crate::utils;

pub fn run(storage: &Storage) -> Result<(), Error> {
    let mut desc = storage.load_descriptions()?;

    // Count how many entries would be removed (without modifying yet)
    let to_remove: Vec<_> = desc
        .descriptions
        .iter()
        .filter(|(path, _)| !utils::path_exists(path))
        .map(|(path, _)| path.clone())
        .collect();

    let removed_count = to_remove.len();
    if removed_count == 0 {
        println!("everything is fine");
        return Ok(());
    }

    // Only create backup if we're actually going to change something
    let backup_path = storage.backup_descriptions()?;
    println!("backup created: {}", backup_path);

    for path in to_remove {
        desc.descriptions.remove(&path);
    }
    storage.save_descriptions(&desc)?;

    println!(
        "removed {} descriptions for non-existent paths",
        removed_count
    );

    Ok(())
}
