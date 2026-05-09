#![allow(dead_code)]
#![allow(unused_imports)]


use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};


use crate::config::AppConfigManager;

mod proto;
mod asdc;
mod cmds;
mod config;


const DEFAULT_LOGGING_LEVEL: u8 = 0;


#[derive(Parser)]
#[command(name = "arctic-spa-dc-rust")]
#[command(version = "0.1.0")]
#[command(about = "Interact with your Arctic Spa brand hot tub", long_about = None)]
struct Cli {
    /// Load settings from a specific config file
    #[arg(short, long, value_name = "FILE_PATH")]
    config_path: Option<PathBuf>,

    /// Hot tub IP Address
    #[arg(short, long, value_name = "IP_ADDRESS")]
    ip_address: Option<String>,

    /// Logging level: 0=OFF, 1=ERROR, 2=WARN, 3=INFO, 4=DEBUG, 5=ALL
    #[arg(short, long, value_name = "LOGGING_LEVEL")]
    verbosity: Option<u8>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Request information from the device
    Get {
        /// Message type to query
        #[arg(value_enum)]
        message_name: MessageName,

        /// Optional output file path to write the message data to; if not specified, will print to stdout
        #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
        output_path: Option<PathBuf>,
    },
    /// Send a property update command to the device
    Set {
        /// Hot tub property to set
        #[arg(value_enum)]
        property_name: CommandPropertyName,

        /// Value to set the property to
        value: String,
    },
    /// Store and retrieve application settings
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    }
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
    Dump { }
}

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[derive(Copy, Clone, ValueEnum, Debug)]
enum ConfigPropertyName {
    /// IP Address
    IpAddress,
    /// Verbosity
    Verbosity
}

impl ConfigPropertyName {
    fn as_str(&self) -> &str {
        match self {
            ConfigPropertyName::IpAddress => "ip_address",
            ConfigPropertyName::Verbosity => "verbosity",
        }
    }
}


// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[derive(Copy, Clone, ValueEnum, Debug)]
enum MessageName {
    /// Status of temperatures, pumps, blowers, lights, filters, ozone, etc
    Live,
    /// Settings for filtration, onzen, ozone, minimum and maximum values, etc
    Settings,
    /// Capabilities of the hot tub such as pump layouts and installed features
    Configuration,
    /// Settings for power draw management
    Peak,
    /// Device system clock information
    Clock,
    /// Serial numbers, firmware and hardware versions, etc
    Information,
    /// Error status indicators
    Error,
    /// Router details
    Router,
    /// Filter maintenance information
    Filter,
    /// Information about installed peripheral device
    Peripheral,
    /// Status of orp and ph levels, electrode details, etc
    OnzenLive,
    /// Definitions for minimum and maximum thresholds of OnzenLive statuses
    OnzenSettings
}

impl From<MessageName> for asdc::MessageType {
    fn from(value: MessageName) -> Self {
        match value {
            MessageName::Live => asdc::MessageType::Live,
            MessageName::Settings => asdc::MessageType::Settings,
            MessageName::Configuration => asdc::MessageType::Configuration,
            MessageName::Peak => asdc::MessageType::Peak,
            MessageName::Clock => asdc::MessageType::Clock,
            MessageName::Information => asdc::MessageType::Information,
            MessageName::Error => asdc::MessageType::Error,
            MessageName::Router => asdc::MessageType::Router,
            MessageName::Filter => asdc::MessageType::Filter,
            MessageName::Peripheral => asdc::MessageType::Peripheral,
            MessageName::OnzenLive => asdc::MessageType::OnzenLive,
            MessageName::OnzenSettings => asdc::MessageType::OnzenSettings,
        }
    }
}


#[derive(Copy, Clone, ValueEnum, Debug)]
enum CommandPropertyName {
    /// Temperature setpoint in Fahrenheit (integer: min 59, max 104)
    #[value(name = "temperature-setpoint")]
    TemperatureSetpoint,
    /// Temperature setpoint shortcut
    #[value(name = "temp")]
    Temp,
    /// Pump 1 status (HIGH, LOW, OFF)
    #[value(name = "pump-1")]
    Pump1,
    /// Pump 2 status (HIGH, LOW, OFF)
    #[value(name = "pump-2")]
    Pump2,
    /// Pump 3 status (HIGH, LOW, OFF)
    #[value(name = "pump-3")]
    Pump3,
    /// Pump 4 status (HIGH, LOW, OFF)
    #[value(name = "pump-4")]
    Pump4,
    /// Pump 5 status (HIGH, LOW, OFF)
    #[value(name = "pump-5")]
    Pump5,
    /// Blower 1 status (HIGH, LOW, OFF)
    #[value(name = "blower-1")]
    Blower1,
    /// Blower 2 status (HIGH, LOW, OFF)
    #[value(name = "blower-2")]
    Blower2,
    /// Lights (ON, OFF)
    Lights,
    /// Stereo (ON, OFF)
    Stereo,
    /// Filter (ON, OFF)
    Filter,
    /// Onzen (ON, OFF)
    Onzen,
    /// Ozone (ON, OFF)
    Ozone,
    /// Exhaust fan (ON, OFF)
    #[value(name = "exhaust-fan")]
    ExhaustFan,
    /// Sauna state (accepts integer value)
    #[value(name = "sauna-state")]
    SaunaState,
    /// Sauna time left in minutes (integer: min 0, max 60)
    #[value(name = "sauna-time-left")]
    SaunaTimeLeft,
    /// EZ button - turn on / off all jets and lights at once (ON, OFF)
    #[value(name = "all-on")]
    AllOn,
    /// Fogger (ON, OFF)
    Fogger,
    /// Spaboy boost (ON, OFF)
    #[value(name = "spaboy-boost")]
    SpaboyBoost,
    /// Pack reset (ON, OFF)
    #[value(name = "pack-reset")]
    PackReset,
    /// Log dump (ON, OFF)
    #[value(name = "log-dump")]
    LogDump,
    /// SDS (ON, OFF)
    Sds,
    /// YESS (ON, OFF)
    Yess,
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


fn main () {
    let cli = Cli::parse();

    let mut logging_initialized = false;

    // check cli args for verbosity (takes precedence over config file / env vars)
    if let Some(verbosity) = cli.verbosity {
        init_logging(verbosity);
        logging_initialized = true;
        log::debug!("logging initialized to level {}", log::max_level());
    }

    log::info!("initializing config");

    // check if config file location is specified explicitly in cli args
    let mut config = if let Some(path) = cli.config_path.as_ref() {
        log::debug!("config_path: {}", path.display());
        match AppConfigManager::load_from_path(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to load config: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match AppConfigManager::load_or_create() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed to load config: {}", e);
                std::process::exit(1);
            }
        }
    };

    // if cli arg for verbosity is not present, logging is not initialized - check config file or use default value
    if !logging_initialized {
        let verbosity = config.data.verbosity.unwrap_or_else(|| DEFAULT_LOGGING_LEVEL);
        init_logging(verbosity);
        log::debug!("logging initialized to level {}", log::max_level());
    }

    log::info!("initializing app");

    // get ip address first from cli, then check config file
    let ip_address = cli.ip_address.unwrap_or_else(|| {
        config.data.ip_address.to_string()
    });

    log::info!("running command: {:?}", cli.command);

    match &cli.command {
        Commands::Get { message_name, output_path } => {
            log::debug!("message_name: {:?}", message_name);
            // TODO: actually use or remove
            log::debug!("output_path: {:?}", output_path);

            if ip_address.is_empty() {
                log::error!("no ip address specified; aborting");
                std::process::exit(1);
            }
            let message_type: asdc::MessageType = (*message_name).into();
            // let message = match cmds::get_message(&ip_address, message_type) {
            //     Ok(m) => m,
            //     Err(e) => {
            //         log::error!("command execution failed: {:#?}", e);
            //         std::process::exit(1);
            //     }
            // };
            // cmds::display_message(message_type, message);

            let mut msg = proto::Live::Live::new();
            msg.set_temperature_fahrenheit(104);
            msg.set_alarm(24);
            msg.set_all_on(false);
            msg.set_blower_1(proto::Live::live::PumpStatus::PUMP_OFF);
            msg.set_blower_2(proto::Live::live::PumpStatus::PUMP_OFF);
            // msg.set_current_adc(0);
            // msg.set_economy(false);
            // msg.set_error(0);
            // msg.set_exhaust_fan(false);
            // msg.set_filter(proto::Live::live::FilterStatus::FILTER_IDLE);
            // msg.set_fogger(false);
            // msg.set_heater_1(proto::Live::live::HeaterStatus::HEATER_HEATING);
            // msg.set_heater_2(proto::Live::live::HeaterStatus::HEATER_IDLE);
            // msg.set_heater_adc(20);
            // msg.set_lights(false);
            // msg.set_onzen(true);
            // msg.set_orp(650);
            // msg.set_ozone(proto::Live::live::OzoneStatus::OZONE_ACTIVE);
            // msg.set_ph(712);
            // msg.set_pump_1(proto::Live::live::PumpStatus::PUMP_LOW);
            // msg.set_pump_2(proto::Live::live::PumpStatus::PUMP_HIGH);
            // msg.set_pump_3(proto::Live::live::PumpStatus::PUMP_OFF);
            // msg.set_pump_4(proto::Live::live::PumpStatus::PUMP_OFF);
            // msg.set_pump_5(proto::Live::live::PumpStatus::PUMP_OFF);
            // msg.set_sauna(proto::Live::live::SaunaStatus::SAUNA_NORMAL);
            // msg.set_sauna_time_remaining(0);
            // msg.set_sds(false);
            // msg.set_status(67);
            // msg.set_stereo(false);
            // msg.set_temperature_fahrenheit(102);
            // msg.set_temperature_setpoint_fahrenheit(104);
            // msg.set_yess(false);

            let msg_wrapped = asdc::ProtoMessage::Live(msg);

            cmds::display_message(message_type, msg_wrapped);
        },
        Commands::Set { property_name, value } => {
            if ip_address.is_empty() {
                log::error!("no ip address specified; aborting");
                std::process::exit(1);
            }
            log::debug!("set property_name: {:?}, value: {:?}", property_name, value);

            // Validate value based on property type
            match property_name {
                // Pump/blower status properties: valid values are HIGH, LOW, OFF
                CommandPropertyName::Pump1
                | CommandPropertyName::Pump2
                | CommandPropertyName::Pump3
                | CommandPropertyName::Pump4
                | CommandPropertyName::Pump5
                | CommandPropertyName::Blower1
                | CommandPropertyName::Blower2 => {
                    match value.to_uppercase().as_str() {
                        "HIGH" | "LOW" | "OFF" => {
                            println!("set {:?} = {}", property_name, value);
                        }
                        _ => {
                            eprintln!("Invalid pump/blower status: {}. Valid values are: HIGH, LOW, OFF", value);
                            std::process::exit(1);
                        }
                    }
                }
                // Boolean properties: valid values are ON, OFF
                CommandPropertyName::Lights
                | CommandPropertyName::Stereo
                | CommandPropertyName::Filter
                | CommandPropertyName::Onzen
                | CommandPropertyName::Ozone
                | CommandPropertyName::ExhaustFan
                | CommandPropertyName::AllOn
                | CommandPropertyName::Fogger
                | CommandPropertyName::SpaboyBoost
                | CommandPropertyName::PackReset
                | CommandPropertyName::LogDump
                | CommandPropertyName::Sds
                | CommandPropertyName::Yess => {
                    match value.to_uppercase().as_str() {
                        "ON" | "OFF" => {
                            println!("set {:?} = {}", property_name, value);
                        }
                        _ => {
                            eprintln!("Invalid toggle value: {}. Valid values are: ON, OFF", value);
                            std::process::exit(1);
                        }
                    }
                }
                CommandPropertyName::SaunaState => {
                    match value.to_uppercase().as_str() {
                        "IDLE" | "PRESET_A" | "PRESET_B" | "PRESET_C" | "TIMER" => {
                             println!("set {:?} = {}", property_name, value);
                        }
                        _ => {
                            eprintln!("Invalid sauna state: {}. Valid values are: IDLE, PRESET_A, PRESET_B, PRESET_C, TIMER", value);
                            std::process::exit(1);
                        }
                    }
                }
                // Numeric properties: validate as integer
                CommandPropertyName::TemperatureSetpoint
                | CommandPropertyName::Temp
                | CommandPropertyName::SaunaTimeLeft => {
                    match value.parse::<i32>() {
                        Ok(_) => {
                            println!("set {:?} = {}", property_name, value);
                        }
                        Err(_) => {
                            eprintln!("Invalid numeric value: {}. Must be an integer", value);
                            std::process::exit(1);
                        }
                    }
                }
            }
        },
        Commands::Config { command } => {
            match command {
                ConfigCommands::Get { property_name } => {
                    let key = property_name.as_str();

                    match config.get_value(key) {
                        Ok(value) => println!("config value: {:?} = {:?}", key, value),
                        Err(e) => {
                            if log::log_enabled!(log::Level::Error) {
                                log::error!("failed to get config: {:?}", e);
                            } else {
                                eprintln!("failed to get config: {:?}", e);
                            }
                        }
                    }
                },
                ConfigCommands::Set { property_name, value } => {
                    let key = property_name.as_str();

                    match config.set_value(key, value) {
                        Ok(_) => println!("config value set: {:?} = {:?}", key, value),
                        Err(e) => {
                            if log::log_enabled!(log::Level::Error) {
                                log::error!("failed to set config: {:?}", e);
                            } else {
                                eprintln!("failed to set config: {:?}", e);
                            }
                            std::process::exit(1);
                        }
                    }
                },
                ConfigCommands::Dump {  } => {
                    println!("displaying all config properties");
                    println!("{}", config.to_string_pretty().unwrap());
                }
            }
        },
    }
}
