#![allow(dead_code)]
#![allow(unused_imports)]


use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use serde::{Serialize, Deserialize};

use crate::core::utils::{default_config_path, initialize_path};


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub ip_address: Option<String>,
    pub log_level: Option<String>,
    pub mock_server_mode: Option<bool>,
    pub mock_server_ip_address: Option<String>,
}

impl AppConfig {
    pub fn new(ip_address: Option<String>, log_level: Option<String>, mock_server_mode: Option<bool>, mock_server_ip_address: Option<String>) -> Self {
        Self {
            ip_address,
            log_level,
            mock_server_mode,
            mock_server_ip_address,
        }
    }

    pub fn default() -> Self {
        Self {
            ip_address: Some(String::from("")),
            log_level: Some(String::from("off")),
            mock_server_mode: Some(false),
            mock_server_ip_address: Some(String::from("127.0.0.1")),
        }
    }
}


pub struct AppConfigManager {
    pub data: AppConfig,
    path: PathBuf,
}

impl AppConfigManager {

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = default_config_path();
        let is_new_file = initialize_path(&config_path)?;

        // if config file doesn't exist, create it from template
        if is_new_file {
            log::debug!("config file {:#?} not found, setting default values", config_path.display());

            // let template_config_str = r#"
            //     {
            //         "ip_address": "",
            //         "log_level": "off"
            //     }
            // "#;
            // let mut template_config: AppConfig = serde_json::from_str(template_config_str).unwrap();

            let default_config = AppConfig::default();

            let file = std::fs::File::create(&config_path)?;
            serde_json::to_writer_pretty(file, &default_config)?;

            log::info!("created config file {:#?}", config_path.display());
            return Ok(Self {
                data: default_config,
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
        log::info!("config loaded successfully from {:#?}", config_path.display());
        Ok(Self {
            data,
            path: config_path.to_path_buf(),
        })
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(&self.path)?;
        serde_json::to_writer_pretty(file, &self.data)?;
        log::trace!("config saved to {:#?}", self.path.display());
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Result<Value, serde_json::Error> {
        let app_config_json = serde_json::to_value(&self.data)?;
        let value = serde_json::to_value(app_config_json[key].clone())?;
        log::trace!("get_value: {:?}={:?}", key, value);
        Ok(value)
    }

    pub fn set_value(&mut self, key: &str, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
        if key == "ip_address" {
            self.data.ip_address = Some(value.as_str().unwrap_or_default().to_string());
            log::trace!("set_value: {:?}={:?}", key, self.data.ip_address);
        } else if key == "log_level" {
            self.data.log_level = Some(value.as_str().unwrap_or_default().to_string());
            log::trace!("set_value: {:?}={:?}", key, self.data.log_level);
        } else if key == "mock_server_mode" {
            if value.as_str().is_some() {
                let mock_server_mode_str = value.as_str().unwrap_or_default().to_lowercase();
                self.data.mock_server_mode = Some(
                    mock_server_mode_str == "1"
                    || mock_server_mode_str == "true"
                    || mock_server_mode_str == "on"
                    || mock_server_mode_str == "yes"
                    || mock_server_mode_str == "y"
                    || mock_server_mode_str == "enable"
                    || mock_server_mode_str == "enabled"
                );
            } else if value.is_boolean() {
                self.data.mock_server_mode = Some(value.as_bool().unwrap());
            }
            log::trace!("set_value: {:?}={:?}", key, self.data.mock_server_mode);
        } else if key == "mock_server_ip_address" {
            self.data.mock_server_ip_address = Some(value.as_str().unwrap_or_default().to_string());
            log::trace!("set_value: {:?}={:?}", key, self.data.mock_server_ip_address);
        } else {
            log::warn!("unknown config key: {:?}", key);
            return Ok(());
        }

        self.save()?;

        Ok(())
    }

    pub fn set_value2(&mut self, key: &str, value: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let mut app_config_json = serde_json::to_value(&self.data)?;

        if value.is_string() {
            app_config_json[key] = serde_json::Value::String(value.as_str().unwrap().to_string());
        } else if value.is_number() {
            app_config_json[key] = serde_json::Value::Number(value.as_i64().unwrap().into());
        } else if value.is_boolean() {
            app_config_json[key] = serde_json::Value::Bool(value.as_bool().unwrap());
        }

        self.data = serde_json::from_value(app_config_json)?;
        self.save()?;

        log::trace!("set_value: {:?}={:?}", key, value);

        Ok(())
    }

    pub fn to_string_pretty(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string_pretty(&self.data)?)
    }

    pub fn reset_to_defaults(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.data = AppConfig::default();
        self.save()?;
        log::trace!("config file reset to default values");
        Ok(())
    }
}
