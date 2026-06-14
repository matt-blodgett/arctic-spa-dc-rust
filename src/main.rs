#![allow(dead_code)]

use std::io::{self, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod commands;
mod core;
mod proto;

use commands::device::{DevicePropertyNameGet, DevicePropertyNameSet};
use commands::query::{QueryMessageName, QueryOutputFormat};

#[derive(Parser)]
#[command(name = "asdc")]
#[command(version = "0.1.0")]
#[command(about = "Interact with your Arctic Spa brand hot tub", long_about = None)]
struct Cli {
    /// Specify a config file location
    #[arg(long, value_name = "FILE_PATH", global = true)]
    config_path: Option<PathBuf>,

    /// Hot tub IP Address
    #[arg(short, long, value_name = "IP_ADDRESS", global = true)]
    ip_address: Option<String>,

    /// Logging level filter
    #[arg(short = 'l', long, value_name = "LOG_LEVEL", global = true)]
    log_level: Option<core::logging::LogLevel>,

    /// Optional file path to append log output to; if omitted, logs are written to stderr
    #[arg(long, value_name = "FILE_PATH", global = true)]
    log_path: Option<PathBuf>,

    /// Mock server mode (connect to local running mock server for testing)
    #[arg(long, global = true)]
    mock_server: bool,

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

        /// Optional output format for displaying the message data (defaults to plain text)
        #[arg(long, value_enum)]
        output_format: Option<QueryOutputFormat>,

        /// Optional output file path to write the message data to; if not specified, will print to stdout
        #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
        output_path: Option<PathBuf>,
    },
    /// Continuously poll protobuf messages from the hot tub and write structured data to a sqlite database
    Poll {
        /// Overwrite the existing database file (deleting existing data)
        #[arg(long)]
        reset_database: bool,

        /// Write to a non-default database file instead of the default location; if the file does not exist, it will be created
        #[arg(long, value_name = "DATABASE_FILE_PATH")]
        database_path: Option<PathBuf>,
    },
    /// Store and retrieve this application's settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Run a local mock TCP server for staging (request/response emulation)
    MockServer {
        /// Bind address for the mock server
        #[arg(long, value_name = "BIND_HOST")]
        ip_address: Option<String>,
    },
    /// Reset all application data - overwrite config file with default values, delete current database files, log files, etc.
    Reset {
        /// Do not prompt for confirmation before resetting app data; use with caution
        #[arg(long)]
        force: bool,
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
    List {},
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Display a config property
    Get {
        /// Config property to display
        property_path: String,
    },
    /// Set a config property
    Set {
        /// Config property to set
        property_path: String,

        /// Value to set the property to
        value: String,
    },
    /// Display all config properties
    List {},
    /// Overwrite config file with default values
    Reset {},
}

fn fatal_error(message: &str) -> ! {
    log::error!("{}", message);
    eprintln!("{}", message);
    std::process::exit(1);
}

fn assert_ip_address(ip_address: &str) {
    if ip_address.is_empty() {
        fatal_error("no ip address specified; aborting");
    }
}

fn confirm_action(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush().unwrap_or_else(|e| {
        fatal_error(&format!("failed to flush stdout for confirmation prompt: {}", e));
    });

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        fatal_error(&format!("failed to read confirmation input: {}", e));
    });

    matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

fn main() {
    let cli = Cli::parse();

    // ----------------------------------------------------------------------

    let mut config = if let Some(path) = cli.config_path.as_ref() {
        log::debug!("config_path={}", path.display());
        core::config::AppConfigManager::load_from_path(path).unwrap_or_else(|e| {
            fatal_error(&format!("failed to load config from specified path: {}", e));
        })
    } else {
        core::config::AppConfigManager::load_or_create().unwrap_or_else(|e| {
            fatal_error(&format!("failed to load or create config: {}", e));
        })
    };

    // ----------------------------------------------------------------------

    let mut is_mock_server_instance = false;
    let mut config_path_logging_level = "logging.level";
    let mut config_path_logging_path = "logging.path";
    if let Commands::MockServer { .. } = &cli.command {
        is_mock_server_instance = true;
        config_path_logging_level = "mock_server.logging.level";
        config_path_logging_path = "mock_server.logging.path";
    }

    let config_log_level = config
        .get_path_value(config_path_logging_level)
        .and_then(|value| serde_json::from_value::<core::logging::LogLevel>(value).ok());
    let config_log_path = config
        .get_path_value(config_path_logging_path)
        .and_then(|value| serde_json::from_value::<Option<PathBuf>>(value).ok())
        .flatten();

    // cli args take precedence over config values
    let log_level = cli
        .log_level
        .or(config_log_level)
        .unwrap_or(core::logging::DEFAULT_LOGGING_LEVEL);
    let log_path = cli.log_path.or(config_log_path);

    core::logging::init_logging(log_level, log_path.as_deref()).unwrap_or_else(|e| {
        fatal_error(&format!("failed to initialize logging: {}", e));
    });
    log::debug!(
        "logging initialized: level={}, file_path={}",
        log::max_level(),
        log_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "stderr".to_string())
    );

    // ----------------------------------------------------------------------

    log::info!("initializing app");

    let cli_ip_address = cli.ip_address;
    let config_ip_address = config
        .get_path_value("ip_address")
        .and_then(|value| serde_json::from_value::<String>(value).ok())
        .unwrap_or_default();
    let cli_mock_server = cli.mock_server;
    let config_mock_server_enabled = config
        .get_path_value("mock_server.enabled")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let config_mock_server_ip_address = config
        .get_path_value("mock_server.ip_address")
        .and_then(|value| serde_json::from_value::<String>(value).ok())
        .unwrap_or_default();
    let mock_server_mode = cli_mock_server || config_mock_server_enabled;

    // ensure any ip provided in cli args take precendence
    let ip_address = if cli_ip_address.is_some() {
        cli_ip_address.clone().unwrap()
    } else {
        // now check if we are running against a local mock server
        if mock_server_mode {
            // use the mock server ip if set in config file
            config_mock_server_ip_address.clone()
        } else {
            // use config file ip last
            config_ip_address.clone()
        }
    };

    if is_mock_server_instance {
        log::debug!(
            "\
            ip_address={:?}, \
            cli_ip_address={:?}, \
            config_ip_address={:?}, \
            mock_server_mode={:?}, \
            config_mock_server_ip_address={:?} \
            ",
            ip_address,
            cli_ip_address,
            config_ip_address,
            mock_server_mode,
            config_mock_server_ip_address,
        );
    }

    log::debug!("using ip_address={:}", ip_address);

    // ----------------------------------------------------------------------

    log::info!("running command: {:?}", cli.command);

    match &cli.command {
        Commands::Discover { update_config } => {
            let mut devices = if mock_server_mode {
                log::info!("mock server mode discover: returning local mock device");
                vec![config_mock_server_ip_address.clone()]
            } else {
                commands::discover::discover_devices()
            };

            commands::discover::display_devices(&devices);

            if *update_config {
                if devices.is_empty() {
                    if log::log_enabled!(log::Level::Warn) {
                        log::warn!(
                            "--update-config flag is set but no devices were discovered; config file will not be updated"
                        );
                    } else {
                        println!(
                            "--update-config flag is set but no devices were discovered; config file will not be updated"
                        );
                    }
                } else {
                    let first_device_ip = devices.remove(0);
                    log::info!(
                        "updating config with first discovered device's IP address: {}",
                        first_device_ip
                    );
                    println!(
                        "updating config with first discovered device's IP address: {}",
                        first_device_ip
                    );

                    config
                        .set_path_value("ip_address", serde_json::Value::String(first_device_ip.clone()))
                        .unwrap_or_else(|e| {
                            fatal_error(&format!("failed to set config: {:?}", e));
                        });
                    log::info!("config updated successfully");
                    println!("config updated successfully");
                }
            }
        }
        Commands::Device { command } => {
            assert_ip_address(&ip_address);

            match command {
                DeviceCommands::Get { property_name } => {
                    let value =
                        commands::device::get_device_property_value(&ip_address, property_name).unwrap_or_else(|e| {
                            fatal_error(&format!("failed to get device property value: {:?}", e));
                        });
                    commands::device::display_device_property_value(property_name, &value);
                }
                DeviceCommands::Set { property_name, value } => {
                    commands::device::set_device_property_value(&ip_address, property_name, value).unwrap_or_else(
                        |e| {
                            fatal_error(&format!("failed to set device property value: {:?}", e));
                        },
                    );
                }
                DeviceCommands::List {} => {
                    commands::device::get_and_display_all_device_properties(&ip_address).unwrap_or_else(|e| {
                        fatal_error(&format!("failed to display all device properties: {:?}", e));
                    });
                }
            }
        }
        Commands::Query {
            message_name,
            output_format,
            output_path,
        } => {
            assert_ip_address(&ip_address);

            let message_type: core::net::MessageType = (*message_name).into();
            let proto_message = commands::query::get_message(&ip_address, &message_type).unwrap_or_else(|e| {
                fatal_error(&format!("command execution failed: {:#?}", e));
            });
            commands::query::display_message(
                &message_type,
                &proto_message,
                output_format.as_ref(),
                output_path.as_deref(),
            );
        }
        Commands::Poll {
            reset_database,
            database_path,
        } => {
            assert_ip_address(&ip_address);

            commands::poll::poll_device(&ip_address, &config, *reset_database, database_path.as_ref()).unwrap_or_else(
                |e| {
                    fatal_error(&format!("command execution failed: {:#?}", e));
                },
            );
        }
        Commands::Config { command } => match command {
            ConfigCommands::Get { property_path } => {
                let value = config.get_path_value(property_path).unwrap_or_else(|| {
                    fatal_error(&format!("no config value found for path {:?}", property_path));
                });
                println!("got config value -> {:?} = {:?}", property_path, value);
            }
            ConfigCommands::Set { property_path, value } => {
                config
                    .set_path_value(property_path, value.clone().into())
                    .unwrap_or_else(|e| {
                        fatal_error(&format!("failed to set config {:?}: {:?}", property_path, e));
                    });
                println!("set config value -> {:?} = {:?}", property_path, value);
            }
            ConfigCommands::List {} => {
                let config_json_string = config.to_string_pretty().unwrap_or_else(|e| {
                    fatal_error(&format!("failed to display config: {:?}", e));
                });
                println!("displaying all config properties");
                println!("{}", config_json_string);
            }
            ConfigCommands::Reset {} => {
                if !confirm_action("Are you sure you want to reset config values to defaults?") {
                    println!("cancelled config reset");
                    return;
                }

                config.reset_to_defaults().unwrap_or_else(|e| {
                    fatal_error(&format!("failed to reset config: {:?}", e));
                });
                println!("config reset to default values");
            }
        },
        Commands::MockServer { ip_address } => {
            let host = if ip_address.is_some() {
                ip_address.clone().unwrap()
            } else if !config_mock_server_ip_address.is_empty() {
                config_mock_server_ip_address.clone()
            } else {
                commands::mock_server::DEFAULT_HOST.to_string()
            };

            let bind_address = format!("{}:{}", host, commands::mock_server::DEFAULT_PORT);

            log::debug!("starting mock server: bind_address={:}", bind_address);

            commands::mock_server::run(&bind_address).unwrap_or_else(|e| {
                fatal_error(&format!("mock server failed: {:#?}", e));
            });
        }
        Commands::Reset { force } => {
            if !force && !confirm_action("Are you sure you want to reset all app data (configs, logs, databases)?") {
                println!("cancelled reset");
                return;
            }

            commands::reset::reset_all(&mut config).unwrap_or_else(|e| {
                fatal_error(&format!("failed to reset app state: {:?}", e));
            });
        }
    }
}
