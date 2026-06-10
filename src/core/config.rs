#![allow(dead_code)]
#![allow(unused_imports)]


use std::fs;
use std::path::PathBuf;

use clap::builder::Str;
use serde_json::Value;
use serde::{Deserialize, Serialize, de};

use crate::core::utils::{default_config_path, initialize_path};

use crate::commands::config::ConfigPropertyName;


#[derive(Debug)]
pub enum ConfigValue {
    Str(String),
    Bool(bool),
}


impl ConfigValue {
    pub fn as_str(&self) -> &str {
        let value = match self {
            ConfigValue::Str(s) => s.as_str(),
            ConfigValue::Bool(b) => if *b { "true" } else { "false" },
        };
        value
    }
    pub fn as_bool(&self) -> bool {
        let value = match self {
            ConfigValue::Str(s) => match s.to_lowercase().as_str() {
                "1" | "true" | "on" | "yes" | "y" | "enable" | "enabled" => Some(true),
                "0" | "false" | "off" | "no" | "n" | "disable" | "disabled" => Some(false),
                _ => None,
            },
            ConfigValue::Bool(b) => Some(*b),
        };
        value.unwrap_or(false)
    }
}


// #[derive(Debug)]
// impl ConfigValue {
//     pub fn as_str(&self) -> Option<&str> {
//         match self {
//             ConfigValue::Str(s) => Some(s.as_str()),
//             ConfigValue::Bool(b) => Some(if *b { "true" } else { "false" }),
//         }
//     }
//     pub fn as_bool(&self) -> Option<bool> {
//         match self {
//             ConfigValue::Str(s) => match s.to_lowercase().as_str() {
//                 "1" | "true" | "on" | "yes" | "y" | "enable" | "enabled" => Some(true),
//                 "0" | "false" | "off" | "no" | "n" | "disable" | "disabled" => Some(false),
//                 _ => None,
//             },
//             ConfigValue::Bool(b) => Some(*b),
//         }
//     }
// }


#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub ip_address: String,
    #[serde(default = "AppConfig::default_log_level")]
    pub log_level: String,
    #[serde(default)]
    pub mock_server_mode: bool,
    #[serde(default = "AppConfig::default_mock_server_ip_address")]
    pub mock_server_ip_address: String,
}

impl AppConfig {
    fn default_log_level() -> String { String::from("off") }
    fn default_mock_server_ip_address() -> String { String::from("127.0.0.1") }

    pub fn default() -> Self {
        Self {
            ip_address: String::from(""),
            log_level: Self::default_log_level(),
            mock_server_mode: false,
            mock_server_ip_address: Self::default_mock_server_ip_address(),
        }
    }

    pub fn get_value(&self, property_name: &ConfigPropertyName) -> ConfigValue {
        match property_name {
            ConfigPropertyName::IpAddress           => ConfigValue::Str(self.ip_address.clone()),
            ConfigPropertyName::LogLevel            => ConfigValue::Str(self.log_level.clone()),
            ConfigPropertyName::MockServerMode      => ConfigValue::Bool(self.mock_server_mode),
            ConfigPropertyName::MockServerIpAddress => ConfigValue::Str(self.mock_server_ip_address.clone()),
        }
    }
}


pub struct AppConfigManager {
    data: AppConfig,
    path: PathBuf,
}

impl AppConfigManager {

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = default_config_path();
        let is_new_file = initialize_path(&config_path)?;

        // if config file doesn't exist, create it from template
        if is_new_file {
            log::debug!("config file {:#?} not found, setting default values", config_path.display());

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

    pub fn get_value(&self, property_name: ConfigPropertyName) -> ConfigValue {
        self.data.get_value(&property_name)
    }

    pub fn set_value(&mut self, property_name: ConfigPropertyName, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
        match property_name {
            ConfigPropertyName::IpAddress => self.data.ip_address = value.as_str().unwrap_or_default().to_string(),
            ConfigPropertyName::LogLevel => self.data.log_level = value.as_str().unwrap_or_default().to_string(),
            ConfigPropertyName::MockServerMode => self.data.mock_server_mode = value
                .as_bool()
                .unwrap_or_else(|| value
                    .as_str()
                    .map(|s|
                        matches!(s.to_lowercase().as_str(), "1" | "y" | "yes" | "true" | "on" | "enable" | "enabled")
                    ).unwrap_or(false)
                ),
            ConfigPropertyName::MockServerIpAddress => self.data.mock_server_ip_address = value.as_str().unwrap_or_default().to_string(),
        };

        self.save()?;

        log::trace!("set_value: {:?}={:?}", property_name, value);

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
