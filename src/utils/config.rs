use configparser::ini::Ini;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ConfigError {
    IoError(io::Error),
    ParseError(String),
    HomeDirNotFound,
}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError::IoError(error)
    }
}

/// Load config (local first, then global)
pub fn get_config_value(section: &str, key: &str) -> Result<Option<String>, ConfigError> {
    // 1. Try local config
    let local_path = PathBuf::from(".hit").join("config");
    if Path::exists(&local_path) {
        let mut conf = Ini::new();
        conf.load(local_path.to_str().unwrap())
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        if let Some(val) = conf.get(section, key) {
            return Ok(Some(val));
        }
    }

    // 2. Try global config
    if let Some(home) = home::home_dir() {
        let global_path = home.join(".hitconfig");
        if global_path.exists() {
            let mut conf = Ini::new();
            conf.load(global_path.to_str().unwrap())
                .map_err(|e| ConfigError::ParseError(e.to_string()))?;

            if let Some(val) = conf.get(section, key) {
                return Ok(Some(val));
            }
        }
    } else {
        return Err(ConfigError::HomeDirNotFound);
    }

    Ok(None)
}

/// Set config value (local or global)
pub fn set_config_value(
    scope: &str,
    section: &str,
    key: &str,
    value: &str,
) -> Result<(), ConfigError> {
    let path = match scope {
        "--global" => home::home_dir()
            .ok_or(ConfigError::HomeDirNotFound)?
            .join(".hitconfig"),
        _ => PathBuf::from(".hit").join("config"),
    };

    let mut conf = if path.exists() {
        let mut conf = Ini::new();
        conf.load(path.to_str().unwrap())
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
        conf
    } else {
        Ini::new()
    };

    conf.set(section, key, Some(value.to_owned()));

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    conf.write(path.to_str().unwrap())
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;

    Ok(())
}
