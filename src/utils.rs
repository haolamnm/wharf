use crate::config::Config;
use crate::errors::Error;
use dirs::home_dir;
use rustyline::{Editor, error::ReadlineError};
use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
};

pub fn load_config() -> Result<Config, Error> {
    // Determine config path
    let config_path = if let Ok(env_path) = env::var("WHARF_CONFIG_PATH") {
        PathBuf::from(env_path)
    } else {
        // Default config path: ~/.config/wharf/config.toml (or platform equivalent)
        dirs::config_dir()
            .map(|dir| dir.join("wharf").join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("config.toml"))
    };

    // Load or create default config
    let config = if config_path.exists() {
        Config::load(&config_path)
            .map_err(|e| Error::ConfigError(format!("Failed to load config: {}", e)))?
    } else {
        // Use default config if file doesn't exist
        let default_config = Config::default();

        // Optionally create the config directory and save default config
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::ConfigError(format!("Failed to create config directory: {}", e))
            })?;
        }

        // Save default config for user to customize
        let config_content = toml::to_string_pretty(&default_config).map_err(|e| {
            Error::ConfigError(format!("Failed to serialize default config: {}", e))
        })?;

        std::fs::write(&config_path, config_content)
            .map_err(|e| Error::ConfigError(format!("Failed to write default config: {}", e)))?;

        println!("Created default config at: {}", config_path.display());
        default_config
    };

    Ok(config)
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists() || {
        if let Some(home) = home_dir() {
            home.join(path).exists()
        } else {
            false
        }
    }
}

pub fn get_relative_path(path: &str) -> Result<String, Error> {
    let path_buf = PathBuf::from(path);

    if path_buf.is_absolute() {
        if let Some(home) = home_dir() {
            if let Ok(relative) = path_buf.strip_prefix(&home) {
                return Ok(relative.to_string_lossy().to_string());
            }
        }
        Ok(path.to_string())
    } else {
        // For relative paths, resolve relative to current diretory
        let current_dir = env::current_dir()?;
        let absolute_path = current_dir.join(path);

        if let Some(home) = home_dir() {
            if let Ok(relative) = absolute_path.strip_prefix(&home) {
                return Ok(relative.to_string_lossy().to_string());
            }
        }
        Ok(absolute_path.to_string_lossy().to_string())
    }
}

pub fn confirm(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn read_input(prompt: &str, default: &str) -> Result<String, Error> {
    let mut rl = Editor::<(), _>::new().map_err(|e: ReadlineError| {
        Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to initialize input editor: {}", e),
        ))
    })?;

    let full_prompt = format!("{}: ", prompt);

    // Pre-fill the input buffer with the default value (user can edit it!)
    let _ = rl.add_history_entry(default);
    let initial = default.to_string();
    let readline = rl.readline_with_initial(&full_prompt, (&initial, ""));

    match readline {
        Ok(line) => {
            let trimmed = line.trim();
            // If user just pressed Enter, return default
            if trimmed.is_empty() {
                Ok(default.to_string())
            } else {
                Ok(trimmed.to_string())
            }
        }
        Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
            Ok(default.to_string()) // Ctrl-C or Ctrl-D
        }
        Err(err) => Err(Error::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("input error: {}", err),
        ))),
    }
}
