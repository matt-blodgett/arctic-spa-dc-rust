#![allow(dead_code)]


use std::net;
use std::ops::Add;
use std::time::{Instant, SystemTime};

use std::{thread, time::Duration};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::collections::HashMap;

use serde::{Deserialize, Serialize, de};
use protobuf::Enum;

use crate::commands::poll;
use crate::proto;
use crate::core::db;
use crate::core::net::{MessageType, ProtoMessage, NetworkClient};
use crate::core::config::AppConfigManager;
use crate::core::config::MessagePollingConfigs;



pub fn poll_device(ip_address: &str, config: &AppConfigManager) -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------
    // polling configuration setup

    let max_polling_duration_ms = config
        .get_path_value("polling.max_duration_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or_default();

    log::info!("starting polling: ip_address={:}, max_duration={:}ms", ip_address, max_polling_duration_ms);

    let message_polling_config: MessagePollingConfigs = config
        .get_path_value("polling.messages")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // log::trace!("message polling config: {:?}", message_polling_config);

    let (once_per_session_message_types, constant_refresh_message_types):
        (Vec<MessageType>, Vec<MessageType>) = message_polling_config
        .iter()
        .fold(
            (Vec::new(), Vec::new()),
            |mut acc, (message_type, polling_config)| {
                if polling_config.once_per_session {
                    acc.0.push(*message_type);
                } else {
                    acc.1.push(*message_type);
                }
                acc
            },
        );

    // log::debug!("message types: once per session: {:?}", once_per_session_message_types);
    // log::debug!("message types: constant refresh: {:?}", constant_refresh_message_types);

    // ---------------------------------------------

    type MessageNextRefreshTimes = HashMap<MessageType, u128>;
    let mut polling_next_refresh: MessageNextRefreshTimes = HashMap::from_iter(
        constant_refresh_message_types.iter().map(|message_type| (*message_type, 0))
    );

    // ---------------------------------------------
    // network and database setup

    log::info!("connecting to device at {:}", ip_address);
    let mut network_client = NetworkClient::connect(ip_address)?;

    log::info!("initializing database");
    let mut db_client = db::DatabaseClient::open(None, false)?;
    db_client.create_connection_session(ip_address)?;

    // ---------------------------------------------
    // infinite polling loop with graceful shutdown on Ctrl+C or max elapsed time configuration

    let running = Arc::new(AtomicBool::new(true));
    let shutdown_flag = Arc::clone(&running);
    if let Err(e) = ctrlc::set_handler(move || {
        shutdown_flag.store(false, Ordering::SeqCst);
    }) {
        log::error!("failed to set Ctrl+C handler: {:#?}", e);
        return Err(Box::new(e));
    }

    log::info!("press Ctrl+C to stop polling and exit gracefully");

    // ---------------------------------------------
    // start main polling loop - request once per session messages, then continuously poll for all others

    let start_time = Instant::now();

    log::debug!("requesting {} message types once for initial data sync", once_per_session_message_types.len());
    for message_type in once_per_session_message_types {
        network_client.request_message(message_type)?;
    }

    log::debug!("starting continuous polling {} message types", constant_refresh_message_types.len());

    while running.load(Ordering::SeqCst) {
        let elapsed_time = start_time.elapsed().as_millis();

        for message_type in constant_refresh_message_types.iter() {
            let next_refresh_time = *polling_next_refresh.get(message_type).unwrap_or(&0);
            let refresh_interval_ms = message_polling_config
                .get(message_type)
                .map(|config| config.refresh_interval_ms as u128)
                .unwrap_or_default();

            if elapsed_time >= next_refresh_time {
                network_client.request_message(*message_type)?;

                let next_due_time = elapsed_time + refresh_interval_ms;
                polling_next_refresh.insert(*message_type, next_due_time);

                log::trace!(
                    "scheduled next refresh: message_type={:?}, next_refresh_time={}ms, refresh_interval_ms={}ms, elapsed_time={}ms",
                    message_type,
                    next_due_time,
                    refresh_interval_ms,
                    elapsed_time
                );
            }
        }

        let mut received_messaged_count = 0;

        log::debug!("reading messages...");

        match network_client.read_messages() {
            Ok(messages) => {
                received_messaged_count = messages.len();

                if received_messaged_count > 0 {
                    log::debug!("received {} messages", received_messaged_count);
                }
                for message in messages {
                    log::debug!("received message {:?} at {:?}", message.message_type(), message.received_at());
                    if let Err(e) = db_client.insert_message(&message) {
                        log::error!("database io error: {:#?}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("network io error: {:#?}", e);
                log::debug!("sleeping for 1000ms before retrying...");
                thread::sleep(Duration::from_millis(1_000));
            }
        }

        if received_messaged_count == 0 {
            log::trace!("no messages received, sleeping for 1000ms...");
            thread::sleep(Duration::from_millis(1_000));
        }

        let elapsed_time = start_time.elapsed().as_millis();

        if max_polling_duration_ms > 0 && elapsed_time > max_polling_duration_ms as u128 {
            log::info!("reached max polling duration of {}ms, exiting polling loop", max_polling_duration_ms);
            break;
        }
    }

    log::info!("received Ctrl+C, exiting polling loop gracefully");

    Ok(())
}
