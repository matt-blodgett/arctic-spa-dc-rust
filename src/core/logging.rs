#![allow(dead_code)]

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn to_level_filter(self) -> log::LevelFilter {
        match self {
            LogLevel::Off => log::LevelFilter::Off,
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LogLevel::Off => "off",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };
        write!(f, "{}", text)
    }
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_lowercase().as_str() {
            "off" => Ok(LogLevel::Off),
            "error" => Ok(LogLevel::Error),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(format!("unsupported log level: {}", value)),
        }
    }
}

pub const DEFAULT_LOGGING_LEVEL: LogLevel = LogLevel::Debug;

pub fn init_logging(log_level: LogLevel, log_file_path: Option<&Path>) -> Result<(), std::io::Error> {
    let level_filter = log_level.to_level_filter();
    let mut builder = env_logger::Builder::new();

    if let Some(path) = log_file_path {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let file = OpenOptions::new().create(true).append(true).open(path)?;
        builder.target(env_logger::Target::Pipe(Box::new(file)));
    }

    builder
        .format(|buf, record| {
            // https://docs.rs/log/0.4.29/log/struct.Record.html
            // println!("{:?}", record);
            // println!("{:?}", record.metadata());
            // println!("{:?}", buf);

            // writeln!(
            //     buf,
            //     "[{} | {}] {}",
            //     buf.timestamp(),
            //     record.level(),
            //     record.args()
            // )

            // writeln!(
            //     buf,
            //     "[{} | {} | {}] {}",
            //     buf.timestamp(),
            //     record.target(),
            //     record.level(),
            //     record.args()
            // )

            // FILES AND LINES
            writeln!(
                buf,
                "[{} | {:<5} | {}:{}] {}",
                buf.timestamp(),
                record.level().to_string(),
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.args()
            )

            // MODULES
            // writeln!(
            //     buf,
            //     "[{} | {:<5} | {:}] {}",
            //     buf.timestamp(),
            //     record.level().to_string(),
            //     record.module_path().unwrap_or(""),
            //     record.args()
            // )
        })
        .filter_level(level_filter)
        .init();

    Ok(())
}
