#![allow(dead_code)]


use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub ip_address: String,
    pub verbosity: Option<u8>
}


pub struct AppConfigManager {
    pub data: AppConfig,
    path: PathBuf,
}

impl AppConfigManager {
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
            // let template_config = json!({
            //     "ip_address": "",
            //     "verbosity": 5
            // });
            let template_config_str = r#"
                {
                    "ip_address": "192.168.0.1",
                    "verbosity": 5
                }
            "#;
            let mut template_config: AppConfig = serde_json::from_str(template_config_str).unwrap();

            template_config.ip_address = "192.168.0.1".to_string();

            let file = std::fs::File::create(&config_path)?;
            serde_json::to_writer_pretty(file, &template_config)?;

            // let config_content = serde_json::to_string_pretty(&template_config)?;
            // fs::write(&config_path, config_content)?;
            log::info!("created config file {:#?}", config_path.display());
            return Ok(Self {
                data: template_config,
                path: config_path,
            });
        }

        // load existing config file
        log::debug!("loading config from {:#?}", config_path.display());
        let config_content = fs::read_to_string(&config_path)?;

        let data: AppConfig = serde_json::from_str(&config_content).unwrap();

        log::info!("config loaded successfully from {:#?}", config_path.display());
        Ok(Self {
            data,
            path: config_path,
        })
    }

    /// load config from a custom file path
    pub fn load_from_path(config_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        log::debug!("loading config from custom location {:#?}", config_path.display());
        let config_content = fs::read_to_string(&config_path)?;
        let data: AppConfig = serde_json::from_str(&config_content).unwrap();
        log::info!("config loaded sucessfully from {:#?}", config_path.display());
        Ok(Self {
            data,
            path: config_path.to_path_buf(),
        })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(&self.path)?;
        // let file = std::fs::File::create("output.json")?;
        serde_json::to_writer_pretty(file, &self.data)?;
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Result<Value, serde_json::Error> {
        let app_config_json = serde_json::to_value(&self.data)?;
        let value = serde_json::to_value(app_config_json[key].clone())?;
        log::trace!("config get_value: {:?}={:?}", key, value);
        Ok(value)
    }

    pub fn set_value(&mut self, key: &str, value: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut app_config_json = serde_json::to_value(&self.data)?;

        if app_config_json[key].is_string() {
            app_config_json[key] = serde_json::Value::String(value.clone());
        } else if app_config_json[key].is_number() {
            app_config_json[key] = serde_json::Value::Number(value.parse()?);
        }

        self.data = serde_json::from_value(app_config_json)?;
        self.save()?;

        log::trace!("config set_value: {:?}={:?}", key, value);

        Ok(())
    }

    pub fn to_string_pretty(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self.data)?)
    }
}
