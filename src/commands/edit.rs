use crate::error::Error;
use crate::storage::Storage;
use crate::utils;

pub fn run(storage: &Storage, path: &str) -> Result<(), Error> {
    if !utils::path_exists(path) {
        return Err(Error::PathNotFound(path.to_string()));
    }

    let relative_path = utils::get_relative_path(path)?;
    let mut desc = storage.load_descriptions()?;

    let current = desc.descriptions.get(&relative_path).cloned().unwrap_or_default();
    println!(
        "Current description: {}",
        if current.is_empty() {
            "<none>"
        } else {
            &current
        }
    );

    let new_desc = utils::read_input("New description", &current)?;
    if new_desc.trim().is_empty() {
        return Err(Error::EmptyDescription);
    }

    desc.descriptions.insert(relative_path.clone(), new_desc.clone());
    storage.save_descriptions(&desc)?;

    println!("Updated: {}: {}", relative_path, new_desc);

    Ok(())
}
