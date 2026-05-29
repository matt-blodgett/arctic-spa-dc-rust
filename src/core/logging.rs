#![allow(dead_code)]


use std::io::Write;

use clap::ValueEnum;


#[derive(ValueEnum, Debug, Clone, Copy)]
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

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "off" => LogLevel::Off,
            "error" => LogLevel::Error,
            "warn" | "warning" => LogLevel::Warn,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => LogLevel::Off,
        }
    }
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Off => "off".to_string(),
            LogLevel::Error => "error".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
        }
    }
}


pub const DEFAULT_LOGGING_LEVEL: LogLevel = LogLevel::Off;


pub fn init_logging(log_level: LogLevel) -> () {
    let level_filter = log_level.to_level_filter();

    env_logger::Builder::new()
        .format(|buf, record| {
            // https://docs.rs/log/0.4.29/log/struct.Record.html

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

            let mut level = record.level().to_string();
            if level.len() == 4 {
                level += " ";
            }

            writeln!(
                buf,
                "[{} | {} | {}:{}] {}",
                buf.timestamp(),
                level,
                record.file().unwrap_or(""),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter_level(level_filter)
        .init();
}
