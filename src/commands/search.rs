use crate::errors::Error;
use crate::storage::Storage;

pub fn run(storage: &Storage, text: &str) -> Result<(), Error> {
    let desc = storage.load_descriptions()?;

    let results: Vec<_> = desc
        .descriptions
        .iter()
        .filter(|(path, description)| path.contains(text) || description.contains(text))
        .collect();

    if results.is_empty() {
        println!("no results found for: {}", text);
    } else {
        for (path, description) in results {
            println!("{}: {}", path, description);
        }
    }

    Ok(())
}
