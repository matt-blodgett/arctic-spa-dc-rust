#![allow(dead_code)]


use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod proto;
mod core;
mod commands;

use commands::device::{DevicePropertyNameGet, DevicePropertyNameSet};
use commands::query::QueryMessageName;
use commands::config::ConfigPropertyName;

use crate::commands::mock_server;


#[derive(Parser)]
#[command(name = "asdc")]
#[command(version = "0.1.0")]
#[command(about = "Interact with your Arctic Spa brand hot tub", long_about = None)]
struct Cli {
    /// Specify a config file location
    #[arg(long, value_name = "FILE_PATH", global = true)]
    config_path: Option<PathBuf>,

    /// Mock server mode (connect to local running mock server for testing)
    #[arg(long, global = true)]
    mock_server_mode: bool,

    /// Logging level filter
    #[arg(short = 'l', long, value_name = "LOG_LEVEL", global = true)]
    log_level: Option<core::logging::LogLevel>,

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
        /// Update config file with first discovered hot tub's IP address (overrides any existing IP address in config)
        #[arg(long)]
        update_config: bool,
    },
    /// Primary hot tub functions
    Device {
        #[command(subcommand)]
        command: DeviceCommands,
    },
    /// Request protobuf messages from the hot tub
    Query {
        /// Message type to query
        #[arg(value_enum)]
        message_name: QueryMessageName,

        /// Optional output file path to write the message data to; if not specified, will print to stdout
        #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
        output_path: Option<PathBuf>,
    },
    /// Continuously poll protobuf messages from the hot tub and write structured data to a sqlite database
    Poll { },
    /// Store and retrieve this application's settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Run a local mock TCP server for staging (request/response emulation)
    StartMockServer {
        /// Bind address for the mock server
        #[arg(long, value_name = "IP_ADDRESS", default_value = mock_server::DEFAULT_HOST)]
        ip_address: String,
    },
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


fn fatal_error_and_exit(message: &str) -> ! {
    log::error!("{}", message);
    eprintln!("{}", message);
    std::process::exit(1);
}


fn main () {
    let cli = Cli::parse();

    // ---------------------------------------------

    let mut logging_initialized = false;

    // check cli args for log_level (takes precedence over config file / env vars)
    if let Some(log_level) = cli.log_level {
        core::logging::init_logging(log_level);
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

    // if cli arg for log_level is not present, logging is not initialized - check config file or use default value
    if !logging_initialized {
        let log_level = if config.data.log_level.is_empty() {
            core::logging::DEFAULT_LOGGING_LEVEL
        } else {
            core::logging::LogLevel::from(config.data.log_level.as_str())
        };
        core::logging::init_logging(log_level);
        log::debug!("logging initialized: level={}", log::max_level());
    }

    // ---------------------------------------------

    log::info!("initializing app");

    let mock_server_mode = cli.mock_server_mode || config.data.mock_server_mode;
    let mock_server_ip_address = if mock_server_mode {
        config.data.mock_server_ip_address.clone()
    } else {
        mock_server::DEFAULT_HOST.to_string()
    };
    if mock_server_mode {
        log::debug!(
            "mock_server_mode={:}, mock_server_ip_address={:}, cli={:?}, config={:?}",
            mock_server_mode,
            mock_server_ip_address,
            cli.mock_server_mode,
            config.data.mock_server_mode
        );
    }

    // target localhost in mock server mode
    let ip_address = if mock_server_mode {
        mock_server_ip_address.clone()
    } else {
        cli.ip_address.unwrap_or_else(|| config.data.ip_address.clone())
    };

    log::debug!("using ip_address={}", ip_address);

    // ---------------------------------------------

    log::info!("running command: {:?}", cli.command);

    match &cli.command {
        Commands::Discover { update_config } => {
            let mut devices = if mock_server_mode {
                log::info!("mock server mode discover: returning local mock device");
                vec![mock_server_ip_address]
            } else {
                commands::discover::discover_devices()
            };

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
                    log::info!("updating config with first discovered device's IP address: {}", first_device_ip);
                    println!("updating config with first discovered device's IP address: {}", first_device_ip);

                    config.set_value(ConfigPropertyName::IpAddress, &first_device_ip.clone().into())
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to set config: {:?}", e));
                        });

                    log::info!("config updated successfully");
                    println!("config updated successfully");
                }
            }
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
        Commands::Query { message_name, output_path } => {
            if ip_address.is_empty() {
                fatal_error_and_exit("no ip address specified; aborting");
            }

            let message_type: core::net::MessageType = (*message_name).into();
            let proto_message = commands::query::get_message(&ip_address, message_type)
                .unwrap_or_else(|e| {
                    fatal_error_and_exit(&format!("command execution failed: {:#?}", e));
                });
            commands::query::display_message(&message_type, &proto_message, output_path.as_deref());
        },
        Commands::Poll {  } => {
            if ip_address.is_empty() {
                fatal_error_and_exit("no ip address specified; aborting");
            }

            commands::poll::poll_device(&ip_address)
                .unwrap_or_else(|e| {
                    fatal_error_and_exit(&format!("command execution failed: {:#?}", e));
                });
        },
        Commands::Config { command } => {
            match command {
                ConfigCommands::Get { property_name } => {
                    let value = match config.get_value(*property_name) {
                        core::config::ConfigValue::Str(s) => s,
                        core::config::ConfigValue::Bool(b) => b.to_string(),
                    };
                    println!("got config value -> {:?} = {:?}", property_name, value);
                },
                ConfigCommands::Set { property_name, value } => {
                    config.set_value(*property_name, &value.clone().into())
                        .unwrap_or_else(|e| {
                            fatal_error_and_exit(&format!("failed to set config: {:?}", e));
                        });
                    println!("set config value -> {:?} = {:?}", property_name, value);
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
        Commands::StartMockServer { ip_address } => {
            let mut host = ip_address;
            if &mock_server_ip_address != ip_address {
                host = &mock_server_ip_address;
            }

            let bind_address = format!("{}:{}", host, mock_server::DEFAULT_PORT);

            commands::mock_server::run(&bind_address)
                .unwrap_or_else(|e| {
                    fatal_error_and_exit(&format!("mock server failed: {:#?}", e));
                });
        },
    }
}
