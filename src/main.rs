#![allow(dead_code)]


use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod proto;
mod core;
mod commands;

use commands::query::QueryMessageName;
use commands::config::ConfigPropertyName;
use commands::device::{DevicePropertyNameGet, DevicePropertyNameSet};


const DEFAULT_LOGGING_LEVEL: u8 = 0;


#[derive(Parser)]
#[command(name = "asdc")]
#[command(version = "0.1.0")]
#[command(about = "Interact with your Arctic Spa brand hot tub", long_about = None)]
struct Cli {
    /// Load settings from a specific config file
    #[arg(long, value_name = "FILE_PATH", global = true)]
    config_path: Option<PathBuf>,

    /// Testing mode (do not connect over TCP, use mock data)
    #[arg(long, global = true)]
    test: bool,

    /// Logging level: 0=OFF, 1=ERROR, 2=WARN, 3=INFO, 4=DEBUG, 5=ALL
    #[arg(short = 'v', long, value_name = "LOGGING_LEVEL", global = true)]
    verbosity: Option<u8>,

    /// Hot tub IP Address
    #[arg(short, long, value_name = "IP_ADDRESS", global = true)]
    ip_address: Option<String>,

    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand, Debug)]
enum Commands {
    /// Search the local network for hot tubs and display their IP addresses
    Discover {
        /// Update config file with first discovered device's IP address (overrides any existing IP address in config)
        #[arg(long)]
        update_config: bool,
    },
    /// Request protobuf messages from the hot tub device
    Query {
        /// Message type to query
        #[arg(value_enum)]
        message_name: QueryMessageName,

        /// Optional output file path to write the message data to; if not specified, will print to stdout
        #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
        output_path: Option<PathBuf>,
    },
    /// Primary hot tub functions
    Device {
        #[command(subcommand)]
        command: DeviceCommands,
    },
    /// Store and retrieve this application's settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    }
}


#[derive(Subcommand, Debug)]
enum DeviceCommands {
    /// Display a device property
    Get {
        /// Device property to display
        #[arg(value_enum)]
        property_name: DevicePropertyNameGet,
    },
    /// Set a device property
    Set {
        /// Device property to set
        #[arg(value_enum)]
        property_name: DevicePropertyNameSet,

        /// Value to set the property to
        value: String,
    },
    /// Display all device properties
    List { }
}


#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Display a config property
    Get {
        /// Config property to display
        #[arg(value_enum)]
        property_name: ConfigPropertyName,
    },
    /// Set a config property
    Set {
        /// Config property to set
        #[arg(value_enum)]
        property_name: ConfigPropertyName,

        /// Value to set the property to
        value: String,
    },
    /// Display all config properties
    List { },
    /// Overwrite config file with default values
    Reset { },
}


fn init_logging(logging_level: u8) -> () {
    let level_filter = match logging_level {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Debug,
        5 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Off
    };

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


fn fatal_error_and_exit(message: &str) -> ! {
    log::error!("{}", message);
    eprintln!("{}", message);
    std::process::exit(1);
}


fn main () {
    let cli = Cli::parse();

    let mut logging_initialized = false;

    // check cli args for verbosity (takes precedence over config file / env vars)
    if let Some(verbosity) = cli.verbosity {
        init_logging(verbosity);
        logging_initialized = true;
        log::debug!("logging initialized: level={}", log::max_level());
    }

    log::info!("initializing config");

    // check if config file location is specified explicitly in cli args
    let mut config = if let Some(path) = cli.config_path.as_ref() {
        log::debug!("config_path: {}", path.display());
        core::config::AppConfigManager::load_from_path(path)
            .unwrap_or_else(|e| {
                fatal_error_and_exit(&format!("failed to load config from specified path: {}", e));
            })
    } else {
        core::config::AppConfigManager::load_or_create()
            .unwrap_or_else(|e| {
                fatal_error_and_exit(&format!("failed to load or create config: {}", e));
            })
    };

    // if cli arg for verbosity is not present, logging is not initialized - check config file or use default value
    if !logging_initialized {
        let verbosity = config.data.verbosity.unwrap_or(DEFAULT_LOGGING_LEVEL);
        init_logging(verbosity);
        log::debug!("logging initialized: level={}", log::max_level());
    }

    log::info!("initializing app");

    let testing_mode = cli.test;
    if testing_mode {
        log::debug!("testing mode enabled");
    }

    // get ip address first from cli, then check config file
    let ip_address = cli.ip_address.unwrap_or_else(|| {
        config.data.ip_address.to_string()
    });

    log::info!("running command: {:?}", cli.command);

    match &cli.command {
        Commands::Discover { update_config } => {
            let mut devices = commands::discover::discover_devices();

            commands::discover::display_devices(&devices);

            if *update_config {
                if devices.is_empty() {
                    if log::log_enabled!(log::Level::Warn) {
                        log::warn!("--update-config flag is set but no devices were discovered; config file will not be updated");
                    } else {
                         println!("--update-config flag is set but no devices were discovered; config file will not be updated");
                    }
                } else {
                    let first_device_ip = devices.remove(0);
                    log::info!("updating config with first discovered device's IP address: {:}", first_device_ip);
                    println!("updating config with first discovered device's IP address: {:}", first_device_ip);

                    config.set_value("ip_address", &first_device_ip)
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to set config: {:?}", e));
                        });

                    log::info!("config updated successfully");
                    println!("config updated successfully");
                }
            }
        },
        Commands::Query { message_name, output_path } => {
            if testing_mode {
                commands::query::test_display_message((*message_name).into(), output_path.as_deref());
                return;
            }

            if ip_address.is_empty() {
                fatal_error_and_exit("no ip address specified; aborting");
            }

            let message_type: core::net::MessageType = (*message_name).into();
            let proto_message = commands::query::get_message(&ip_address, message_type)
                .unwrap_or_else(|e| {
                    fatal_error_and_exit(&format!("command execution failed: {:#?}", e));
                });
            commands::query::display_message(message_type, proto_message, output_path.as_deref());
        },
        Commands::Device { command } => {
            if ip_address.is_empty() {
                fatal_error_and_exit("no ip address specified; aborting");
            }
            match command {
                DeviceCommands::Get { property_name } => {
                    let value = commands::device::get_device_property_value(&ip_address, *property_name)
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to get device property value: {:?}", e));
                        });
                    commands::device::display_device_property_value(*property_name, &value);
                },
                DeviceCommands::Set { property_name, value } => {
                    commands::device::set_device_property_value(&ip_address, *property_name, value)
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to set device property value: {:?}", e));
                        });
                },
                DeviceCommands::List {  } => {
                    commands::device::get_and_display_all_device_properties(&ip_address)
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to display all device properties: {:?}", e));
                        });
                }
            }
        },
        Commands::Config { command } => {
            match command {
                ConfigCommands::Get { property_name } => {
                    let key = property_name.as_str();
                    let value = config.get_value(key)
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to get config: {:?}", e));
                        });
                    println!("config value: {:?} = {:?}", key, value);
                },
                ConfigCommands::Set { property_name, value } => {
                    let key = property_name.as_str();
                    config.set_value(key, value)
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to set config: {:?}", e));
                        });
                    println!("config value set: {:?} = {:?}", key, value);
                },
                ConfigCommands::List {  } => {
                    let config_json_string = config.to_string_pretty()
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to display config: {:?}", e));
                        });
                    println!("displaying all config properties");
                    println!("{}", config_json_string);
                },
                ConfigCommands::Reset {  } => {
                    config.reset_to_defaults()
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to reset config: {:?}", e));
                        });
                    println!("config reset to default values");
                }
            }
        },
    }
}
