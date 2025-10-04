mod commands;
mod config;
mod error;
mod storage;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone)]
pub enum Shell {
    /// Generate bash completion script
    Bash,
    /// Generate zsh completion script
    Zsh,
    /// Generate fish completion script
    Fish,
    /// Generate PowerShell completion script
    Powershell,
}

#[derive(Parser)]
#[command(name = "wharf")]
#[command(
    about = "A simple file and directory description tool",
    version = "1.2.0"
)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show description
    Show { path: String },
    /// Add or update description
    Add { path: String, description: String },
    /// Edit description interactively
    Edit { path: String },
    /// List all descriptions
    List,
    /// Search descriptions
    Search { text: String },
    /// Remove description
    Remove { path: String },
    /// Export descriptions to file
    Export { file: Option<String> },
    /// Import descriptions from file
    Import { file: String },
    /// Generate shell completion scripts
    Generate {
        #[command(subcommand)]
        shell: Shell,
    },
}

use error::Error;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let cli = Cli::parse();

    // Load configuration
    let config = utils::load_config()?;

    // Initialize storage with config
    let storage = storage::Storage::new(&config)?;

    match cli.command {
        Commands::Show { path } => commands::show::run(&storage, &path),
        Commands::Add { path, description } => commands::add::run(&storage, &path, &description),
        Commands::Edit { path } => commands::edit::run(&storage, &path),
        Commands::List => commands::list::run(&storage),
        Commands::Search { text } => commands::search::run(&storage, &text),
        Commands::Remove { path } => commands::remove::run(&storage, &path),
        Commands::Export { file } => commands::export::run(&storage, file.as_deref()),
        Commands::Import { file } => commands::import::run(&storage, &file),
        Commands::Generate { shell } => commands::generate::run(shell),
    }
}
