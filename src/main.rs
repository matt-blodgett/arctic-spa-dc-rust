use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};


mod proto;
mod asdc;
mod cmds;


const DEFAULT_LOGGING_LEVEL: u8 = 5;


#[derive(Parser)]
#[command(name = "arctic-spa-dc-rust")]
#[command(version = "0.1.0")]
#[command(about = "Interact with your Arctic Spa brand hot tub", long_about = None)]
struct Cli {
    /// Load flags from a config file
    #[arg(short, long, value_name = "FILE_PATH")]
    config_path: Option<PathBuf>,

    /// Hot tub IP Address
    #[arg(short, long, value_name = "IP_ADDRESS")]
    ip_address: Option<String>,

    /// Logging level: 0=OFF, 1=ERROR, 2=WARN, 3=INFO, 4=DEBUG, 5=ALL
    #[arg(short, long, value_name = "LOGGING_LEVEL", default_value_t = DEFAULT_LOGGING_LEVEL)]
    verbosity: u8,

    // /// Dry run
    // #[arg(short, long)]
    // dry_run: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Request information from the host
    Get {
        /// Message to query
        #[arg(value_enum)]
        message_name: MessageName,

        /// Optional output file path to write the message data to; if not specified, will print to stdout
        #[arg(short, long, value_name = "OUTPUT_FILE_PATH")]
        output_path: Option<PathBuf>,
    },
    /// Send an update command to the host
    Set {
        /// Property to set
        property_name: String,
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

    init_logging(cli.verbosity);

    log::info!("initializing app");
    log::debug!("initialized logging handler with level {}", log::max_level());

    let ip_address = cli.ip_address.unwrap_or_default();

    if let Some(path) = cli.config_path.as_deref() {
        log::debug!("config_path: {}", path.display());
        // TODO: also check config file for ip address and other settings
    }

    log::info!("running command: {:?}", cli.command);

    match &cli.command {
        Commands::Get { message_name, output_path } => {
            log::debug!("output_path: {:?}", output_path);

            if ip_address.is_empty() {
                log::error!("no ip address specified; aborting");
                std::process::exit(1);
            }
            let message_type: asdc::MessageType = (*message_name).into();
            let message = match cmds::get_message(&ip_address, message_type) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("command execution failed: {:#?}", e);
                    std::process::exit(1);
                }
            };
            cmds::display_message(message);
        },
        Commands::Set { property_name } => {
            if ip_address.is_empty() {
                log::error!("no ip address specified; aborting");
                std::process::exit(1);
            }
            println!("set property: {:?}", property_name);
        },
    }
}
