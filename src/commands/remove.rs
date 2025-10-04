use crate::error::Error;
use crate::storage::Storage;
use crate::utils;

pub fn run(storage: &Storage, path: &str) -> Result<(), Error> {
    let relative_path = utils::get_relative_path(path)?;
    let mut desc = storage.load_descriptions()?;

    if !desc.descriptions.contains_key(&relative_path) {
        return Err(Error::DescriptionNotFound(relative_path));
    }

    desc.descriptions.remove(&relative_path);
    storage.save_descriptions(&desc)?;

    println!("Removed description for: {}", relative_path);

    Ok(())
}
