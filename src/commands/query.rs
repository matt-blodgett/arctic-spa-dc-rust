#![allow(dead_code)]


use clap::ValueEnum;

use std::io::Error;
use std::path::Path;

use crate::proto;
use crate::core::net::{MessageType, ProtoMessage, NetworkClient};


#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum QueryMessageName {
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

impl From<QueryMessageName> for MessageType {
    fn from(value: QueryMessageName) -> Self {
        match value {
            QueryMessageName::Live => MessageType::Live,
            QueryMessageName::Settings => MessageType::Settings,
            QueryMessageName::Configuration => MessageType::Configuration,
            QueryMessageName::Peak => MessageType::Peak,
            QueryMessageName::Clock => MessageType::Clock,
            QueryMessageName::Information => MessageType::Information,
            QueryMessageName::Error => MessageType::Error,
            QueryMessageName::Router => MessageType::Router,
            QueryMessageName::Filter => MessageType::Filter,
            QueryMessageName::Peripheral => MessageType::Peripheral,
            QueryMessageName::OnzenLive => MessageType::OnzenLive,
            QueryMessageName::OnzenSettings => MessageType::OnzenSettings,
        }
    }
}

pub fn get_message(ip_address: &str, message_type: MessageType) -> Result<ProtoMessage, Error> {
    log::info!("querying host {:?} for message type {:?}", ip_address, message_type);
    let mut network_client = NetworkClient::connect_to(ip_address)?;
    let message = network_client.request_message_and_await_response(message_type)?;
    Ok(message)
}


pub fn display_message(message_type: MessageType, message: ProtoMessage, output_path: Option<&Path>) -> () {
    log::info!("outputting message data");

    let output_string = match message {
        ProtoMessage::Live(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Command(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Settings(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Configuration(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Peak(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Clock(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Information(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Error(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Router(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Filter(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::Peripheral(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::OnzenLive(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        ProtoMessage::OnzenSettings(msg) => protobuf::text_format::print_to_string_pretty(&msg),
    };

    match output_path {
        Some(path) => {
            log::info!("writing message data to file: {:?}", path);
            match std::fs::write(path, &output_string) {
                Ok(_) => log::info!("successfully wrote message data to {:?}", path),
                Err(e) => {
                    log::error!("failed to write message data to file: {:?}", e);
                    eprintln!("failed to write to file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!("Message data for \"{:#?}\"", message_type);
            for line in output_string.split('\n') {
                if !line.is_empty() {
                    println!("{}", line);
                }
            }
        }
    }
}


pub fn test_display_message(message_type: MessageType, output_path: Option<&Path>) {
    log::info!("testing mode enabled - using mock data for message_type {:?}", message_type);

    let mut msg = proto::Live::Live::new();
    msg.set_temperature_fahrenheit(104);
    msg.set_alarm(24);
    msg.set_all_on(false);
    msg.set_blower_1(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_blower_2(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_current_adc(0);
    msg.set_economy(false);
    msg.set_error(0);
    msg.set_exhaust_fan(false);
    msg.set_filter(proto::Live::live::FilterStatus::FILTER_IDLE);
    msg.set_fogger(false);
    msg.set_heater_1(proto::Live::live::HeaterStatus::HEATER_HEATING);
    msg.set_heater_2(proto::Live::live::HeaterStatus::HEATER_IDLE);
    msg.set_heater_adc(20);
    msg.set_lights(false);
    msg.set_onzen(true);
    msg.set_orp(650);
    msg.set_ozone(proto::Live::live::OzoneStatus::OZONE_ACTIVE);
    msg.set_ph(712);
    msg.set_pump_1(proto::Live::live::PumpStatus::PUMP_LOW);
    msg.set_pump_2(proto::Live::live::PumpStatus::PUMP_HIGH);
    msg.set_pump_3(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_pump_4(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_pump_5(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_sauna(proto::Live::live::SaunaStatus::SAUNA_NORMAL);
    msg.set_sauna_time_remaining(0);
    msg.set_sds(false);
    msg.set_status(67);
    msg.set_stereo(false);
    msg.set_temperature_fahrenheit(102);
    msg.set_temperature_setpoint_fahrenheit(104);
    msg.set_yess(false);

    let msg_wrapped = ProtoMessage::Live(msg);

    display_message(MessageType::Live, msg_wrapped, output_path.as_deref());
    return;
}
