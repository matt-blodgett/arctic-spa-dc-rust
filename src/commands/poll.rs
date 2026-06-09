#![allow(dead_code)]
#![allow(unused_imports)]


// use std::f64::consts::E;
use std::time::SystemTime;

use std::{thread, time::Duration};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};


use protobuf::Enum;

use crate::proto;
use crate::core::db;
use crate::core::net::{MessageType, ProtoMessage, NetworkClient};


const MAX_POLLING_DURATION_MS: u128 = 30 * 1000;


pub fn poll_device(ip_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("starting polling: ip_address={:}", ip_address);

    let running = Arc::new(AtomicBool::new(true));
    let shutdown_flag = Arc::clone(&running);

    if let Err(e) = ctrlc::set_handler(move || {
        shutdown_flag.store(false, Ordering::SeqCst);
    }) {
        log::error!("failed to set Ctrl+C handler: {}", e);
        return Err(Box::new(e));
    }

    log::info!("press Ctrl+C to stop polling and exit gracefully");

    log::debug!("connecting to device at {:}", ip_address);
    let mut network_client = NetworkClient::connect_to(ip_address)?;

    log::debug!("initializing database");
    let mut db_client = db::DatabaseClient::open(None)?;
    db_client.create_connection_session(ip_address)?;

    let start_time = SystemTime::now();

    log::debug!("requesting initial data sync");
    network_client.request_message(MessageType::Clock)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Configuration)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Error)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Filter)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Information)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Live)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::OnzenLive)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::OnzenSettings)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Peak)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Peripheral)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Router)?;
    thread::sleep(Duration::from_millis(500));
    network_client.request_message(MessageType::Settings)?;
    thread::sleep(Duration::from_millis(500));


    log::debug!("starting polling loop");


    while running.load(Ordering::SeqCst) {

        match network_client.read_messages() {
            Ok(messages) => {
                log::debug!("received {} messages", messages.len());
                for message in messages {
                    log::debug!("received message {:?} at {:?}", message.message_type(), message.received_at());
                    if let Err(e) = db_client.insert_message(&message) {
                        log::error!("database io error: {}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("network io error: {}", e);
                log::debug!("sleeping for 5000ms before retrying...");
                thread::sleep(Duration::from_millis(5_000));
            }
        }

        log::debug!("sleeping for 1000ms...");
        thread::sleep(Duration::from_millis(1_000));

        if start_time.elapsed()?.as_millis() > MAX_POLLING_DURATION_MS {
            log::debug!("reached max polling duration of {} milliseconds, exiting polling loop", MAX_POLLING_DURATION_MS);
            break;
        }
    }

    log::info!("received Ctrl+C, exiting polling loop gracefully");

    Ok(())
}

