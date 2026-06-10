#![allow(dead_code)]
#![allow(unused_imports)]


use std::net;
// use std::f64::consts::E;
use std::time::SystemTime;

use std::{thread, time::Duration};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;

use serde::{Deserialize, Serialize, de};
use protobuf::Enum;

use crate::commands::poll;
use crate::proto;
use crate::core::db;
use crate::core::net::{MessageType, ProtoMessage, NetworkClient};


const MAX_POLLING_DURATION_MS: u128 = 15 * 1000;


#[derive(Serialize, Deserialize, Debug)]
struct PollInterval {
    refresh_interval_ms: u64,
    last_refresh_time: Option<SystemTime>,
}
type MessageIntervals = HashMap<MessageType, PollInterval>;

#[derive(Serialize, Deserialize, Debug)]
struct PollConfig {
    ip_address: String,
    message_intervals: MessageIntervals,
}


pub fn test() {

    let poll_config = PollConfig {
        ip_address: "".to_string(),
        message_intervals: MessageIntervals::from([
            (MessageType::Clock, PollInterval {
                refresh_interval_ms: 5_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Configuration, PollInterval {
                refresh_interval_ms: 10_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Error, PollInterval {
                refresh_interval_ms: 15_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Filter, PollInterval {
                refresh_interval_ms: 20_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Information, PollInterval {
                refresh_interval_ms: 25_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Live, PollInterval {
                refresh_interval_ms: 30_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::OnzenLive, PollInterval {
                refresh_interval_ms: 35_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::OnzenSettings, PollInterval {
                refresh_interval_ms: 40_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Peak, PollInterval {
                refresh_interval_ms: 45_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Peripheral, PollInterval {
                refresh_interval_ms: 50_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Router, PollInterval {
                refresh_interval_ms: 55_000,
                last_refresh_time: Some(SystemTime::now())
            }),
            (MessageType::Settings, PollInterval {
                refresh_interval_ms: 60_000,
                last_refresh_time: Some(SystemTime::now())
            }),
        ]),
    };
    println!("poll_config: {:#?}", poll_config);

    println!("test {:?}", poll_config.message_intervals[&MessageType::Live]);
}


pub fn poll_device(ip_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("starting polling: ip_address={:}", ip_address);

    // ---------------------------------------------

    let running = Arc::new(AtomicBool::new(true));
    let shutdown_flag = Arc::clone(&running);

    if let Err(e) = ctrlc::set_handler(move || {
        shutdown_flag.store(false, Ordering::SeqCst);
    }) {
        log::error!("failed to set Ctrl+C handler: {}", e);
        return Err(Box::new(e));
    }

    log::info!("press Ctrl+C to stop polling and exit gracefully");

    // ---------------------------------------------

    log::debug!("connecting to device at {:}", ip_address);
    let mut network_client = NetworkClient::connect(ip_address)?;

    log::debug!("initializing database");
    let mut db_client = db::DatabaseClient::open(None, false)?;
    db_client.create_connection_session(ip_address)?;


    // ---------------------------------------------

    let start_time = SystemTime::now();

    log::debug!("requesting messages for initial data sync");
    network_client.request_message(MessageType::Clock)?;
    network_client.request_message(MessageType::Configuration)?;
    network_client.request_message(MessageType::Error)?;
    network_client.request_message(MessageType::Filter)?;
    network_client.request_message(MessageType::Information)?;
    network_client.request_message(MessageType::Live)?;
    network_client.request_message(MessageType::OnzenLive)?;
    network_client.request_message(MessageType::OnzenSettings)?;
    network_client.request_message(MessageType::Peak)?;
    network_client.request_message(MessageType::Peripheral)?;
    network_client.request_message(MessageType::Router)?;
    network_client.request_message(MessageType::Settings)?;

    log::debug!("starting polling loop");

    // let mut iteration = 0;

    while running.load(Ordering::SeqCst) {

        let mut message_count = 0;

        log::debug!("polling messages...");
        match network_client.read_messages() {
            Ok(messages) => {
                message_count = messages.len();

                if message_count > 0 {
                    log::debug!("received {} messages", message_count);
                }
                for message in messages {
                    log::debug!("received message {:?} at {:?}", message.message_type(), message.received_at());
                    if let Err(e) = db_client.insert_message(&message) {
                        log::error!("database io error: {}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("network io error: {}", e);
                log::debug!("sleeping for 2000ms before retrying...");
                thread::sleep(Duration::from_millis(2_000));
            }
        }

        if message_count == 0 {
            log::trace!("no messages received, sleeping for 1000ms...");
            thread::sleep(Duration::from_millis(1_000));
        }

        if start_time.elapsed()?.as_millis() > MAX_POLLING_DURATION_MS {
            log::info!("reached max polling duration of {} milliseconds, exiting polling loop", MAX_POLLING_DURATION_MS);
            break;
        }

        // iteration += 1;
        // if iteration == 5 {
        //     log::debug!("iteration {}: sending message request to device to ensure connection is alive", iteration);
        //     if let Err(e) = network_client.request_message(MessageType::Live) {
        //         log::error!("network io error during message request: {}", e);
        //     }

        //     let mut command = proto::Command::Command::new();
        //     command.set_set_lights(true);
        //     network_client.send_command(command)?;

        //     let mut command2 = proto::Command::Command::new();
        //     command2.set_set_temperature_setpoint_fahrenheit(104);
        //     network_client.send_command(command2)?;
        // }

    }

    log::info!("received Ctrl+C, exiting polling loop gracefully");

    Ok(())
}

