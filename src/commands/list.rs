use crate::errors::Error;
use crate::storage::Storage;

pub fn run(storage: &Storage) -> Result<(), Error> {
    let desc = storage.load_descriptions()?;

    if desc.descriptions.is_empty() {
        println!("no descriptions stored yet");
    } else {
        for (path, description) in &desc.descriptions {
            println!("{}: {}", path, description);
        }
    }

    Ok(())
}
