#![allow(dead_code)]


use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};


pub struct Config {
    data: Value,
    path: Option<PathBuf>,
}

impl Config {
    /// get the default config directory for this application
    fn default_config_dir() -> PathBuf {
        if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "arctic-spa-dc-rust") {
            proj_dirs.config_dir().to_path_buf()
        } else {
            // fallback to current directory
            PathBuf::from(".")
        }
    }

    /// get the default config file path
    fn default_config_path() -> PathBuf {
        Self::default_config_dir().join("config.json")
    }

    /// load or create config from the default OS location
    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::default_config_path();
        let config_dir = config_path.parent().unwrap();

        // create config directory if it doesn't exist
        if !config_dir.exists() {
            log::debug!("creating config directory: {:#?}", config_dir.display());
            fs::create_dir_all(config_dir)?;
        }

        // if config file doesn't exist, create it from template
        if !config_path.exists() {
            log::debug!("config file not found at {:#?}, creating from template", config_path.display());
            let template_config = json!({
                "ip_address": "",
                "verbosity": 5
            });
            let config_content = serde_json::to_string_pretty(&template_config)?;
            fs::write(&config_path, config_content)?;
            log::info!("created config file {:#?}", config_path.display());
            return Ok(Self {
                data: template_config,
                path: Some(config_path),
            });
        }

        // load existing config file
        log::debug!("loading config from {:#?}", config_path.display());
        let config_content = fs::read_to_string(&config_path)?;
        let data: Value = serde_json::from_str(&config_content)?;
        log::info!("config loaded successfully from {:#?}", config_path.display());
        Ok(Self {
            data,
            path: Some(config_path),
        })
    }

    /// load config from a custom file path
    pub fn load_from_path(config_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        log::debug!("loading config from custom location {:#?}", config_path.display());
        let config_content = fs::read_to_string(&config_path)?;
        let data: Value = serde_json::from_str(&config_content)?;
        log::info!("config loaded sucessfully from {:#?}", config_path.display());
        Ok(Self {
            data,
            path: Some(config_path.to_path_buf()),
        })
    }

    /// get a string value from the config
    pub fn get_string(&self, key: &str) -> Option<String> {
        let value = self.data.get(key)?.as_str().map(|s| s.to_string());
        log::trace!("config get_string -> {} = {:?}", key, value);
        return value;
    }

    /// get an integer value from the config
    pub fn get_int(&self, key: &str) -> Option<i64> {
        let value = self.data.get(key)?.as_i64();
        log::trace!("config get_int -> {} = {:?}", key, value);
        return value;
    }

    /// set a value in the config and save it to disk
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        // try to parse as integer, otherwise store as string
        if let Ok(int_val) = value.parse::<i64>() {
            self.data[key] = json!(int_val);
        } else {
            self.data[key] = json!(value);
        }

        // Save to the path where this config was loaded from (or default if created new)
        let save_path = self.path.as_ref().unwrap_or(&Self::default_config_path()).clone();
        let config_content = serde_json::to_string_pretty(&self.data)?;
        fs::write(&save_path, config_content)?;
        log::trace!("config set_value -> {} = {}", key, value);
        Ok(())
    }
}
