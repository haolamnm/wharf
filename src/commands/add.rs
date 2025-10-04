use crate::error::Error;
use crate::storage::Storage;
use crate::utils;

pub fn run(storage: &Storage, path: &str, description: &str) -> Result<(), Error> {
    if description.trim().is_empty() {
        return Err(Error::EmptyDescription);
    }
    if !utils::path_exists(path) {
        return Err(Error::PathNotFound(path.to_string()));
    }

    let relative_path = utils::get_relative_path(path)?;
    let mut desc = storage.load_descriptions()?;

    if let Some(existing) = desc.descriptions.get(&relative_path) {
        println!("Description already exists: {}", existing);
        if !utils::confirm("Replace it?") {
            println!("Cancelled");
            return Ok(());
        }
    }

    desc.descriptions
        .insert(relative_path.clone(), description.to_string());
    storage.save_descriptions(&desc)?;

    if desc.descriptions.contains_key(&relative_path) {
        println!("Description updated for: {}", relative_path);
    } else {
        println!("Description added for: {}", relative_path);
    }

    Ok(())
}
