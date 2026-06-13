#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::net::IpAddr;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::commands::mock_server;
use crate::core::logging;
use crate::core::net::MessageType;
use crate::core::utils::{default_config_path, default_database_path, initialize_path};

#[derive(Serialize, Deserialize, Debug)]
pub struct MockServerConfig {
    #[serde(default = "MockServerConfig::cfg_default_mock_server_ip_address")]
    pub ip_address: String,
    #[serde(default)]
    pub enabled: bool,
    pub logging: LoggingConfig,
}

impl MockServerConfig {
    fn cfg_default_mock_server_ip_address() -> String {
        String::from(mock_server::DEFAULT_HOST)
    }

    pub fn default() -> Self {
        Self {
            ip_address: Self::cfg_default_mock_server_ip_address(),
            enabled: false,
            logging: LoggingConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePollingConfig {
    pub refresh_interval_ms: u64,
    pub once_per_session: bool,
}

pub type MessagePollingConfigs = HashMap<MessageType, MessagePollingConfig>;

#[derive(Serialize, Deserialize, Debug)]
pub struct PollingConfig {
    pub max_duration_ms: u64,
    pub database_path: Option<PathBuf>,
    pub messages: MessagePollingConfigs,
}

impl PollingConfig {
    fn cfg_default_database_path() -> Option<PathBuf> {
        Some(default_database_path())
    }

    pub fn default() -> Self {
        let messages = MessagePollingConfigs::from([
            (
                MessageType::Clock,
                MessagePollingConfig {
                    refresh_interval_ms: 59_000,
                    once_per_session: false,
                },
            ),
            (
                MessageType::Configuration,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Error,
                MessagePollingConfig {
                    refresh_interval_ms: 15_000,
                    once_per_session: false,
                },
            ),
            (
                MessageType::Filter,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Information,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Live,
                MessagePollingConfig {
                    refresh_interval_ms: 2_500,
                    once_per_session: false,
                },
            ),
            (
                MessageType::OnzenLive,
                MessagePollingConfig {
                    refresh_interval_ms: 2_500,
                    once_per_session: false,
                },
            ),
            (
                MessageType::OnzenSettings,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Peak,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Peripheral,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Router,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
            (
                MessageType::Settings,
                MessagePollingConfig {
                    refresh_interval_ms: 0,
                    once_per_session: true,
                },
            ),
        ]);

        Self {
            max_duration_ms: 0,
            database_path: Self::cfg_default_database_path(),
            messages,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingConfig {
    #[serde(default = "LoggingConfig::cfg_default_level")]
    pub level: logging::LogLevel,
    #[serde(default)]
    pub path: Option<PathBuf>,
}

impl LoggingConfig {
    fn cfg_default_level() -> logging::LogLevel {
        logging::DEFAULT_LOGGING_LEVEL
    }

    pub fn default() -> Self {
        Self {
            level: Self::cfg_default_level(),
            path: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    #[serde(default)]
    pub ip_address: String,
    pub logging: LoggingConfig,
    pub mock_server: MockServerConfig,
    pub polling: PollingConfig,
}

impl AppConfig {
    fn cfg_default_ip_address() -> String {
        String::from("")
    }

    pub fn default() -> Self {
        Self {
            ip_address: Self::cfg_default_ip_address(),
            logging: LoggingConfig::default(),
            polling: PollingConfig::default(),
            mock_server: MockServerConfig::default(),
        }
    }
}

pub struct AppConfigManager {
    data: AppConfig,
    path: PathBuf,
}

impl AppConfigManager {
    fn value_as_u64(path: &str, value: &Value) -> Result<u64, Box<dyn std::error::Error>> {
        value.as_u64().ok_or_else(|| {
            format!(
                "config value for '{}' must be an unsigned integer. Use 0 for infinite duration",
                path
            )
            .into()
        })
    }

    fn validate_path_value(path: &str, value: &Value) -> Result<(), Box<dyn std::error::Error>> {
        match path {
            "logging.level" | "mock_server.logging.level" => {
                let level = value.as_str().ok_or_else(|| "invalid log level".to_string())?;
                level.parse::<logging::LogLevel>().map_err(|_| {
                    format!(
                        "invalid log level '{}'; valid values: off, error, warn, info, debug, trace",
                        level
                    )
                })?;
            }
            "logging.path" | "mock_server.logging.path" => {
                if !value.is_null() {
                    let path = value
                        .as_str()
                        .ok_or_else(|| "invalid logging output file path".to_string())?;

                    if path.trim().is_empty() {
                        return Err("invalid logging output file path".into());
                    }
                }
            }
            "ip_address" | "mock_server.ip_address" => {
                let ip_value = value.as_str().ok_or_else(|| value.to_string())?;

                if !ip_value.trim().is_empty() {
                    ip_value.parse::<IpAddr>().map_err(|_| {
                        format!("invalid ip address '{}'; expected a valid IPv4/IPv6 address", ip_value)
                    })?;
                }
            }
            "polling.max_duration_ms" => {
                let duration_ms = Self::value_as_u64(path, value)?;
                if duration_ms > 86_400_000 {
                    return Err(
                        "polling.max_duration_ms must be between 0 (infinite) and 86400000ms (24 hours)".into(),
                    );
                }
            }
            _ if path.starts_with("polling.messages.") && path.ends_with(".refresh_interval_ms") => {
                let refresh_interval_ms = Self::value_as_u64(path, value)?;
                if refresh_interval_ms != 0 && !(250..=86_400_000).contains(&refresh_interval_ms) {
                    return Err(
                        "polling message refresh_interval_ms must be 0 (disabled) or between 250ms and 86400000ms (24 hours)".into(),
                    );
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn get_path_value(&self, path: &str) -> Option<serde_json::Value> {
        let json = serde_json::to_value(&self.data).ok()?;
        let value = path.split('.').fold(Some(json), |node, key| node?.get(key).cloned());
        log::trace!("get_path_value: path={:}, value={:#?}", path, value);
        value
    }

    pub fn set_path_value(&mut self, path: &str, value: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        let mut json = serde_json::to_value(&self.data)?;
        let default_json = serde_json::to_value(AppConfig::default())?;
        // let keys: Vec<&str> = path.split('.').collect();

        let pointer_path = format!("/{}", path.replace('.', "/"));
        // println!("pointer_path: {:#?}", pointer_path);

        let current_value = json
            .pointer(&pointer_path)
            .ok_or_else(|| format!("invalid config path: {}", path))?;

        let template_value = if current_value.is_null() {
            default_json.pointer(&pointer_path).unwrap_or(current_value)
        } else {
            current_value
        };

        let coerced_value = if template_value.is_number() {
            let number = match value {
                Value::Number(number) => number,
                Value::String(string_value) => match serde_json::from_str::<Value>(&string_value) {
                    Ok(Value::Number(number)) => number,
                    _ => return Err("invalid number value".into()),
                },
                _ => return Err("invalid number value".into()),
            };

            Value::Number(number)
        } else if template_value.is_boolean() {
            let boolean = match value {
                Value::Bool(boolean) => boolean,
                Value::String(string_value) => match serde_json::from_str::<Value>(&string_value) {
                    Ok(Value::Bool(boolean)) => boolean,
                    _ => return Err("invalid boolean value".into()),
                },
                _ => return Err("invalid boolean value".into()),
            };

            Value::Bool(boolean)
        } else {
            value
        };

        Self::validate_path_value(path, &coerced_value)?;

        let old_value = json
            .pointer_mut(&pointer_path)
            .ok_or_else(|| format!("invalid config path: {}", path))?;

        log::trace!(
            "set_path_value: path={:}, old_value={:#?}, new_value={:#?}",
            path,
            old_value,
            coerced_value
        );

        *old_value = coerced_value;

        self.data = serde_json::from_value(json)?;
        self.save()?;

        Ok(())
    }

    pub fn load_or_create() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = default_config_path();
        let is_new_file = initialize_path(&config_path)?;

        // if config file doesn't exist, create it from template
        if is_new_file {
            log::debug!(
                "config file {:#?} not found, setting default values",
                config_path.display()
            );

            let default_config = AppConfig::default();
            let file = std::fs::File::create(&config_path)?;
            serde_json::to_writer_pretty(file, &default_config)?;

            log::info!("created new config file {:#?}", config_path.display());
            return Ok(Self {
                data: default_config,
                path: config_path,
            });
        }

        // load existing config file
        log::debug!("loading config from {:#?}", config_path.display());

        let config_content = fs::read_to_string(&config_path)?;
        let data: AppConfig = serde_json::from_str(&config_content).unwrap_or_else(|_| {
            log::warn!("failed to parse config file, overwriting with default values");
            AppConfig::default()
        });

        let config_manager = Self {
            data,
            path: config_path.to_path_buf(),
        };

        config_manager.save()?;

        log::info!("config loaded successfully from {:#?}", config_path.display());

        Ok(config_manager)
    }

    /// load config from a custom file path
    /// if the file doesn't exist or is invalid, it will be created with default values
    pub fn load_from_path(config_path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        log::debug!("loading config from custom location {:#?}", config_path.display());

        let is_new_file = initialize_path(config_path)?;

        let data: AppConfig = if is_new_file {
            AppConfig::default()
        } else {
            let config_content = fs::read_to_string(&config_path)?;
            match serde_json::from_str(&config_content) {
                Ok(data) => {
                    log::debug!("config file parsed successfully");
                    data
                }
                Err(err) => {
                    log::error!("failed to parse config file: {}", err);
                    return Err(err.into());
                }
            }
        };

        let config_manager = Self {
            data,
            path: config_path.to_path_buf(),
        };

        config_manager.save()?;

        log::info!("config loaded successfully from {:#?}", config_path.display());

        Ok(config_manager)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(&self.path)?;
        serde_json::to_writer_pretty(file, &self.data)?;
        log::trace!("config saved to {:#?}", self.path.display());
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
