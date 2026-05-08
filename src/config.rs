#![allow(dead_code)]


use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};

/// Get the path to the config directory for this application
fn get_config_dir() -> PathBuf {
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "arctic-spa-dc-rust") {
        proj_dirs.config_dir().to_path_buf()
    } else {
        // Fallback to current directory
        PathBuf::from(".")
    }
}

/// Get the path to the config file
pub fn get_config_path() -> PathBuf {
    get_config_dir().join("config.json")
}

/// Initialize and load the config file, creating from template if needed
pub fn load_or_create_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let config_dir = config_path.parent().unwrap();

    // Create config directory if it doesn't exist
    if !config_dir.exists() {
        log::debug!("Creating config directory: {}", config_dir.display());
        fs::create_dir_all(config_dir)?;
    }

    log::debug!("{:#?}", config_dir);


    // If config file doesn't exist, create it from template
    if !config_path.exists() {
        log::debug!("Config file not found at {}, creating from template", config_path.display());
        let template_config = json!({
            "ip-address": "",
            "verbosity": 5
        });
        let config_content = serde_json::to_string_pretty(&template_config)?;
        fs::write(&config_path, config_content)?;
        log::info!("Created config file at {}", config_path.display());
        return Ok(template_config);
    }

    // Load existing config file
    log::debug!("Loading config from {}", config_path.display());
    let config_content = fs::read_to_string(&config_path)?;
    let config: Value = serde_json::from_str(&config_content)?;
    log::debug!("Config loaded successfully");
    Ok(config)
}

/// Get a string value from the config
pub fn get_string(config: &Value, key: &str) -> Option<String> {
    config.get(key)?.as_str().map(|s| s.to_string())
}

/// Get an integer value from the config
pub fn get_int(config: &Value, key: &str) -> Option<i64> {
    config.get(key)?.as_i64()
}

/// Set a value in the config and save it
pub fn set_value(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_or_create_config()?;

    // Try to parse as integer, otherwise store as string
    if let Ok(int_val) = value.parse::<i64>() {
        config[key] = json!(int_val);
    } else {
        config[key] = json!(value);
    }

    let config_path = get_config_path();
    let config_content = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_content)?;
    log::info!("Config updated: {} = {}", key, value);
    Ok(())
}
