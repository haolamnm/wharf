use crate::error::Error;
use crate::storage::Storage;
use crate::utils;

pub fn run(storage: &Storage, path: &str) -> Result<(), Error> {
    let relative_path = utils::get_relative_path(path)?;
    let desc = storage.load_descriptions()?;

    if let Some(description) = desc.descriptions.get(&relative_path) {
        println!("{}: {}", relative_path, description);
    } else {
        println!("{}: No description found", relative_path);

        // Suggest similar paths
        let similar: Vec<_> = desc
            .descriptions
            .keys()
            .filter(|k| k.to_lowercase().contains(&path.to_lowercase()))
            .take(3)
            .collect();

        if !similar.is_empty() {
            println!("Similar paths with descriptions:");
            for similar_path in similar {
                println!("  {}", similar_path);
            }
        } else {
            println!(
                "Add a description with: wharf add \"{}\" \"description\"",
                relative_path
            );
        }
    }

    Ok(())
}
