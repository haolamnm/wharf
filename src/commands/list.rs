use crate::error::Error;
use crate::storage::Storage;

pub fn run(storage: &Storage) -> Result<(), Error> {
    let desc = storage.load_descriptions()?;

    if desc.descriptions.is_empty() {
        println!("No descriptions stored yet. Use: wharf add <path> \"description\"");
    } else {
        for (path, description) in &desc.descriptions {
            println!("{}: {}", path, description);
        }
    }

    Ok(())
}
