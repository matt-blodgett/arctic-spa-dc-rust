#![allow(dead_code)]

use std::io::ErrorKind;
use std::path::PathBuf;

use crate::core::config::AppConfigManager;

fn optional_path(config: &AppConfigManager, key: &str) -> Option<PathBuf> {
    config
        .get_path_value(key)
        .and_then(|value| serde_json::from_value::<Option<PathBuf>>(value).ok())
        .flatten()
}

fn delete_file_if_present(path: &PathBuf, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    match std::fs::remove_file(path) {
        Ok(()) => {
            log::info!("deleted {} at {:?}", label, path.display());
            Ok(())
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            log::debug!("{} not found at {:?}; skipping", label, path.display());
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

pub fn reset_all(config: &mut AppConfigManager) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(log_file_base) = optional_path(config, "logging.path") {
        delete_file_if_present(&log_file_base, "log file")?;
    }

    if let Some(log_file_mock_server) = optional_path(config, "mock_server.logging.path") {
        delete_file_if_present(&log_file_mock_server, "mock server log file")?;
    }

    if let Some(database_path) = optional_path(config, "polling.database_path") {
        delete_file_if_present(&database_path, "database file")?;
    }

    config.reset_to_defaults()?;

    log::info!("reset config to default values");

    log::info!("reset complete");

    Ok(())
}
