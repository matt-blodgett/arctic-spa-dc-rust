#![allow(dead_code)]

use std::io::Error;
use std::path::Path;

use clap::ValueEnum;

use serde_json::json;

use crate::core::net::{MessageType, NetworkClient, ProtoMessage};

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
    OnzenSettings,
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

pub fn get_message(ip_address: &str, message_type: &MessageType) -> Result<ProtoMessage, Error> {
    log::info!(
        "getting message: host={:?}, message_type={:?}",
        ip_address,
        message_type
    );
    let mut network_client = NetworkClient::connect(ip_address)?;
    let message = network_client.request_message_and_await_response(message_type)?;
    Ok(message)
}

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum QueryOutputFormat {
    /// Plain text format using protobuf's text_format printer
    PlainText,
    /// Pretty-printed JSON format with all fields and values explicitly represented
    Json,
}

fn get_message_formatted_plain_text(message: &ProtoMessage) -> String {
    match message {
        ProtoMessage::Live { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Command { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Settings { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Configuration { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Peak { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Clock { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Information { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Error { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Router { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Filter { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::Peripheral { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::OnzenLive { message, .. } => protobuf::text_format::print_to_string_pretty(message),
        ProtoMessage::OnzenSettings { message, .. } => protobuf::text_format::print_to_string_pretty(message),
    }
}

fn get_message_formatted_json(message: &ProtoMessage) -> String {
    let mut message_json = json!({
        "message_received_at": format!("{:?}", message.received_at_formatted(None)),
        "message_type": format!("{:?}", message.message_type()),
    });

    match message {
        ProtoMessage::Live { message: msg, .. } => {
            message_json["temperature_fahrenheit"] =
                serde_json::Value::Number(msg.temperature_fahrenheit.unwrap_or_default().into());
            message_json["temperature_setpoint_fahrenheit"] =
                serde_json::Value::Number(msg.temperature_setpoint_fahrenheit.unwrap_or_default().into());
            message_json["pump_1"] = serde_json::Value::String(format!("{:?}", msg.pump_1.unwrap_or_default()));
            message_json["pump_1_raw"] = serde_json::Value::Number(msg.pump_1.unwrap_or_default().value().into());
            message_json["pump_2"] = serde_json::Value::String(format!("{:?}", msg.pump_2.unwrap_or_default()));
            message_json["pump_2_raw"] = serde_json::Value::Number(msg.pump_2.unwrap_or_default().value().into());
            message_json["pump_3"] = serde_json::Value::String(format!("{:?}", msg.pump_3.unwrap_or_default()));
            message_json["pump_3_raw"] = serde_json::Value::Number(msg.pump_3.unwrap_or_default().value().into());
            message_json["pump_4"] = serde_json::Value::String(format!("{:?}", msg.pump_4.unwrap_or_default()));
            message_json["pump_4_raw"] = serde_json::Value::Number(msg.pump_4.unwrap_or_default().value().into());
            message_json["pump_5"] = serde_json::Value::String(format!("{:?}", msg.pump_5.unwrap_or_default()));
            message_json["pump_5_raw"] = serde_json::Value::Number(msg.pump_5.unwrap_or_default().value().into());
            message_json["blower_1"] = serde_json::Value::String(format!("{:?}", msg.blower_1.unwrap_or_default()));
            message_json["blower_1_raw"] = serde_json::Value::Number(msg.blower_1.unwrap_or_default().value().into());
            message_json["blower_2"] = serde_json::Value::String(format!("{:?}", msg.blower_2.unwrap_or_default()));
            message_json["blower_2_raw"] = serde_json::Value::Number(msg.blower_2.unwrap_or_default().value().into());
            message_json["lights"] = serde_json::Value::Bool(msg.lights.unwrap_or_default());
            message_json["stereo"] = serde_json::Value::Bool(msg.stereo.unwrap_or_default());
            message_json["heater_1"] = serde_json::Value::String(format!("{:?}", msg.heater_1.unwrap_or_default()));
            message_json["heater_1_raw"] = serde_json::Value::Number(msg.heater_1.unwrap_or_default().value().into());
            message_json["heater_2"] = serde_json::Value::String(format!("{:?}", msg.heater_2.unwrap_or_default()));
            message_json["heater_2_raw"] = serde_json::Value::Number(msg.heater_2.unwrap_or_default().value().into());
            message_json["filter"] = serde_json::Value::String(format!("{:?}", msg.filter.unwrap_or_default()));
            message_json["filter_raw"] = serde_json::Value::Number(msg.filter.unwrap_or_default().value().into());
            message_json["onzen"] = serde_json::Value::Bool(msg.onzen.unwrap_or_default());
            message_json["ozone"] = serde_json::Value::String(format!("{:?}", msg.ozone.unwrap_or_default()));
            message_json["ozone_raw"] = serde_json::Value::Number(msg.ozone.unwrap_or_default().value().into());
            message_json["exhaust_fan"] =
                serde_json::Value::String(format!("{:?}", msg.exhaust_fan.unwrap_or_default()));
            message_json["sauna"] = serde_json::Value::String(format!("{:?}", msg.sauna.unwrap_or_default()));
            message_json["sauna_raw"] = serde_json::Value::Number(msg.sauna.unwrap_or_default().value().into());
            message_json["heater_adc"] = serde_json::Value::Number(msg.heater_adc.unwrap_or_default().into());
            message_json["sauna_time_remaining"] =
                serde_json::Value::Number(msg.sauna_time_remaining.unwrap_or_default().into());
            message_json["economy"] = serde_json::Value::Bool(msg.economy.unwrap_or_default());
            message_json["current_adc"] = serde_json::Value::Number(msg.current_adc.unwrap_or_default().into());
            message_json["all_on"] = serde_json::Value::Bool(msg.all_on.unwrap_or_default());
            message_json["fogger"] = serde_json::Value::Bool(msg.fogger.unwrap_or_default());
            message_json["error"] = serde_json::Value::Number(msg.error.unwrap_or_default().into());
            message_json["alarm"] = serde_json::Value::Number(msg.alarm.unwrap_or_default().into());
            message_json["status"] = serde_json::Value::Number(msg.status.unwrap_or_default().into());
            message_json["ph"] = serde_json::Value::Number(msg.ph.unwrap_or_default().into());
            message_json["orp"] = serde_json::Value::Number(msg.orp.unwrap_or_default().into());
            message_json["sds"] = serde_json::Value::Bool(msg.sds.unwrap_or_default());
            message_json["yess"] = serde_json::Value::Bool(msg.yess.unwrap_or_default());
        }
        ProtoMessage::Command { message: msg, .. } => {
            message_json["set_temperature_setpoint_fahrenheit"] =
                serde_json::Value::Number(msg.set_temperature_setpoint_fahrenheit.unwrap_or_default().into());
            message_json["set_pump_1"] = serde_json::Value::String(format!("{:?}", msg.set_pump_1.unwrap_or_default()));
            message_json["set_pump_1_raw"] =
                serde_json::Value::Number(msg.set_pump_1.unwrap_or_default().value().into());
            message_json["set_pump_2"] = serde_json::Value::String(format!("{:?}", msg.set_pump_2.unwrap_or_default()));
            message_json["set_pump_2_raw"] =
                serde_json::Value::Number(msg.set_pump_2.unwrap_or_default().value().into());
            message_json["set_pump_3"] = serde_json::Value::String(format!("{:?}", msg.set_pump_3.unwrap_or_default()));
            message_json["set_pump_3_raw"] =
                serde_json::Value::Number(msg.set_pump_3.unwrap_or_default().value().into());
            message_json["set_pump_4"] = serde_json::Value::String(format!("{:?}", msg.set_pump_4.unwrap_or_default()));
            message_json["set_pump_4_raw"] =
                serde_json::Value::Number(msg.set_pump_4.unwrap_or_default().value().into());
            message_json["set_pump_5"] = serde_json::Value::String(format!("{:?}", msg.set_pump_5.unwrap_or_default()));
            message_json["set_pump_5_raw"] =
                serde_json::Value::Number(msg.set_pump_5.unwrap_or_default().value().into());
            message_json["set_blower_1"] =
                serde_json::Value::String(format!("{:?}", msg.set_blower_1.unwrap_or_default()));
            message_json["set_blower_1_raw"] =
                serde_json::Value::Number(msg.set_blower_1.unwrap_or_default().value().into());
            message_json["set_blower_2"] =
                serde_json::Value::String(format!("{:?}", msg.set_blower_2.unwrap_or_default()));
            message_json["set_blower_2_raw"] =
                serde_json::Value::Number(msg.set_blower_2.unwrap_or_default().value().into());
            message_json["set_lights"] = serde_json::Value::Bool(msg.set_lights.unwrap_or_default());
            message_json["set_stereo"] = serde_json::Value::Bool(msg.set_stereo.unwrap_or_default());
            message_json["set_filter"] = serde_json::Value::Bool(msg.set_filter.unwrap_or_default());
            message_json["set_onzen"] = serde_json::Value::Bool(msg.set_onzen.unwrap_or_default());
            message_json["set_ozone"] = serde_json::Value::Bool(msg.set_ozone.unwrap_or_default());
            message_json["set_exhaust_fan"] = serde_json::Value::Bool(msg.set_exhaust_fan.unwrap_or_default());
            message_json["set_sauna_state"] =
                serde_json::Value::String(format!("{:?}", msg.set_sauna_state.unwrap_or_default()));
            message_json["set_sauna_state_raw"] =
                serde_json::Value::Number(msg.set_sauna_state.unwrap_or_default().value().into());
            message_json["set_sauna_time_left"] =
                serde_json::Value::Number(msg.set_sauna_time_left.unwrap_or_default().into());
            message_json["set_all_on"] = serde_json::Value::Bool(msg.set_all_on.unwrap_or_default());
            message_json["set_fogger"] = serde_json::Value::Bool(msg.set_fogger.unwrap_or_default());
            message_json["set_spaboy_boost"] = serde_json::Value::Bool(msg.set_spaboy_boost.unwrap_or_default());
            message_json["set_pack_reset"] = serde_json::Value::Bool(msg.set_pack_reset.unwrap_or_default());
            message_json["set_log_dump"] = serde_json::Value::Bool(msg.set_log_dump.unwrap_or_default());
            message_json["set_sds"] = serde_json::Value::Bool(msg.set_sds.unwrap_or_default());
            message_json["set_yess"] = serde_json::Value::Bool(msg.set_yess.unwrap_or_default());
        }
        ProtoMessage::Settings { message: msg, .. } => {
            message_json["max_filtration_frequency"] =
                serde_json::Value::Number(msg.max_filtration_frequency.unwrap_or_default().into());
            message_json["min_filtration_frequency"] =
                serde_json::Value::Number(msg.min_filtration_frequency.unwrap_or_default().into());
            message_json["filtration_frequency"] =
                serde_json::Value::Number(msg.filtration_frequency.unwrap_or_default().into());
            message_json["max_filtration_duration"] =
                serde_json::Value::Number(msg.max_filtration_duration.unwrap_or_default().into());
            message_json["min_filtration_duration"] =
                serde_json::Value::Number(msg.min_filtration_duration.unwrap_or_default().into());
            message_json["filtration_duration"] =
                serde_json::Value::Number(msg.filtration_duration.unwrap_or_default().into());
            message_json["max_onzen_hours"] = serde_json::Value::Number(msg.max_onzen_hours.unwrap_or_default().into());
            message_json["min_onzen_hours"] = serde_json::Value::Number(msg.min_onzen_hours.unwrap_or_default().into());
            message_json["onzen_hours"] = serde_json::Value::Number(msg.onzen_hours.unwrap_or_default().into());
            message_json["max_onzen_cycles"] =
                serde_json::Value::Number(msg.max_onzen_cycles.unwrap_or_default().into());
            message_json["min_onzen_cycles"] =
                serde_json::Value::Number(msg.min_onzen_cycles.unwrap_or_default().into());
            message_json["onzen_cycles"] = serde_json::Value::Number(msg.onzen_cycles.unwrap_or_default().into());
            message_json["max_ozone_hours"] = serde_json::Value::Number(msg.max_ozone_hours.unwrap_or_default().into());
            message_json["min_ozone_hours"] = serde_json::Value::Number(msg.min_ozone_hours.unwrap_or_default().into());
            message_json["ozone_hours"] = serde_json::Value::Number(msg.ozone_hours.unwrap_or_default().into());
            message_json["max_ozone_cycles"] =
                serde_json::Value::Number(msg.max_ozone_cycles.unwrap_or_default().into());
            message_json["min_ozone_cycles"] =
                serde_json::Value::Number(msg.min_ozone_cycles.unwrap_or_default().into());
            message_json["ozone_cycles"] = serde_json::Value::Number(msg.ozone_cycles.unwrap_or_default().into());
            message_json["filter_suspension"] = serde_json::Value::Bool(msg.filter_suspension.unwrap_or_default());
            message_json["flash_lights_on_error"] =
                serde_json::Value::Bool(msg.flash_lights_on_error.unwrap_or_default());
            message_json["temperature_offset"] =
                serde_json::Value::Number(msg.temperature_offset.unwrap_or_default().into());
            message_json["sauna_duration"] = serde_json::Value::Number(msg.sauna_duration.unwrap_or_default().into());
            message_json["min_temperature"] = serde_json::Value::Number(msg.min_temperature.unwrap_or_default().into());
            message_json["max_temperature"] = serde_json::Value::Number(msg.max_temperature.unwrap_or_default().into());
            message_json["filtration_offset"] =
                serde_json::Value::Number(msg.filtration_offset.unwrap_or_default().into());
            message_json["spaboy_hours"] = serde_json::Value::Number(msg.spaboy_hours.unwrap_or_default().into());
        }
        ProtoMessage::Configuration { message: msg, .. } => {
            message_json["pump1"] = serde_json::Value::Bool(msg.pump1.unwrap_or_default());
            message_json["pump2"] = serde_json::Value::Bool(msg.pump2.unwrap_or_default());
            message_json["pump3"] = serde_json::Value::Bool(msg.pump3.unwrap_or_default());
            message_json["pump4"] = serde_json::Value::Bool(msg.pump4.unwrap_or_default());
            message_json["pump5"] = serde_json::Value::Bool(msg.pump5.unwrap_or_default());
            message_json["blower1"] = serde_json::Value::Bool(msg.blower1.unwrap_or_default());
            message_json["blower2"] = serde_json::Value::Bool(msg.blower2.unwrap_or_default());
            message_json["lights"] = serde_json::Value::Bool(msg.lights.unwrap_or_default());
            message_json["stereo"] = serde_json::Value::Bool(msg.stereo.unwrap_or_default());
            message_json["heater1"] = serde_json::Value::Bool(msg.heater1.unwrap_or_default());
            message_json["heater2"] = serde_json::Value::Bool(msg.heater2.unwrap_or_default());
            message_json["filter"] = serde_json::Value::Bool(msg.filter.unwrap_or_default());
            message_json["onzen"] = serde_json::Value::Bool(msg.onzen.unwrap_or_default());
            message_json["ozone_peak_1"] = serde_json::Value::Bool(msg.ozone_peak_1.unwrap_or_default());
            message_json["ozone_peak_2"] = serde_json::Value::Bool(msg.ozone_peak_2.unwrap_or_default());
            message_json["exhaust_fan"] = serde_json::Value::Bool(msg.exhaust_fan.unwrap_or_default());
            message_json["powerlines"] = serde_json::Value::String(format!("{:?}", msg.powerlines.unwrap_or_default()));
            message_json["powerlines_raw"] =
                serde_json::Value::Number(msg.powerlines.unwrap_or_default().value().into());
            message_json["breaker_size"] = serde_json::Value::Number(msg.breaker_size.unwrap_or_default().into());
            message_json["smart_onzen"] = serde_json::Value::Number(msg.smart_onzen.unwrap_or_default().into());
            message_json["fogger"] = serde_json::Value::Bool(msg.fogger.unwrap_or_default());
            message_json["sds"] = serde_json::Value::Bool(msg.sds.unwrap_or_default());
            message_json["yess"] = serde_json::Value::Bool(msg.yess.unwrap_or_default());
        }
        ProtoMessage::Peak { message: msg, .. } => {
            message_json["peaknum"] = serde_json::Value::Number(msg.peaknum.unwrap_or_default().into());
            message_json["peakstart1"] = serde_json::Value::Number(msg.peakstart1.unwrap_or_default().into());
            message_json["peakend1"] = serde_json::Value::Number(msg.peakend1.unwrap_or_default().into());
            message_json["peakstart2"] = serde_json::Value::Number(msg.peakstart2.unwrap_or_default().into());
            message_json["peakend2"] = serde_json::Value::Number(msg.peakend2.unwrap_or_default().into());
            message_json["midpeaknum"] = serde_json::Value::Number(msg.midpeaknum.unwrap_or_default().into());
            message_json["midpeakstart1"] = serde_json::Value::Number(msg.midpeakstart1.unwrap_or_default().into());
            message_json["midpeakend1"] = serde_json::Value::Number(msg.midpeakend1.unwrap_or_default().into());
            message_json["midpeakstart2"] = serde_json::Value::Number(msg.midpeakstart2.unwrap_or_default().into());
            message_json["midpeakend2"] = serde_json::Value::Number(msg.midpeakend2.unwrap_or_default().into());
            message_json["offpeakstart"] = serde_json::Value::Number(msg.offpeakstart.unwrap_or_default().into());
            message_json["offpeakend"] = serde_json::Value::Number(msg.offpeakend.unwrap_or_default().into());
            message_json["offset"] = serde_json::Value::Number(msg.offset.unwrap_or_default().into());
            message_json["peakheater"] = serde_json::Value::Bool(msg.peakheater.unwrap_or_default());
            message_json["peakfilter"] = serde_json::Value::Bool(msg.peakfilter.unwrap_or_default());
            message_json["peakozone"] = serde_json::Value::Bool(msg.peakozone.unwrap_or_default());
            message_json["midpeakheater"] = serde_json::Value::Bool(msg.midpeakheater.unwrap_or_default());
            message_json["midpeakfilter"] = serde_json::Value::Bool(msg.midpeakfilter.unwrap_or_default());
            message_json["midpeakozone"] = serde_json::Value::Bool(msg.midpeakozone.unwrap_or_default());
            message_json["sat"] = serde_json::Value::Bool(msg.sat.unwrap_or_default());
            message_json["sun"] = serde_json::Value::Bool(msg.sun.unwrap_or_default());
            message_json["mon"] = serde_json::Value::Bool(msg.mon.unwrap_or_default());
            message_json["tue"] = serde_json::Value::Bool(msg.tue.unwrap_or_default());
            message_json["wed"] = serde_json::Value::Bool(msg.wed.unwrap_or_default());
            message_json["thu"] = serde_json::Value::Bool(msg.thu.unwrap_or_default());
            message_json["fri"] = serde_json::Value::Bool(msg.fri.unwrap_or_default());
        }
        ProtoMessage::Clock { message: msg, .. } => {
            message_json["year"] = serde_json::Value::Number(msg.year.unwrap_or_default().into());
            message_json["month"] = serde_json::Value::Number(msg.month.unwrap_or_default().into());
            message_json["day"] = serde_json::Value::Number(msg.day.unwrap_or_default().into());
            message_json["hour"] = serde_json::Value::Number(msg.hour.unwrap_or_default().into());
            message_json["minute"] = serde_json::Value::Number(msg.minute.unwrap_or_default().into());
            message_json["second"] = serde_json::Value::Number(msg.second.unwrap_or_default().into());
        }
        ProtoMessage::Information { message: msg, .. } => {
            message_json["pack_serial_number"] =
                serde_json::Value::String(msg.pack_serial_number.clone().unwrap_or_default());
            message_json["pack_firmware_version"] =
                serde_json::Value::String(msg.pack_firmware_version.clone().unwrap_or_default());
            message_json["pack_hardware_version"] =
                serde_json::Value::String(msg.pack_hardware_version.clone().unwrap_or_default());
            message_json["pack_product_id"] =
                serde_json::Value::String(msg.pack_product_id.clone().unwrap_or_default());
            message_json["pack_board_id"] = serde_json::Value::String(msg.pack_board_id.clone().unwrap_or_default());
            message_json["topside_product_id"] =
                serde_json::Value::String(msg.topside_product_id.clone().unwrap_or_default());
            message_json["topside_software_version"] =
                serde_json::Value::String(msg.topside_software_version.clone().unwrap_or_default());
            message_json["guid"] = serde_json::Value::String(msg.guid.clone().unwrap_or_default());
            message_json["spa_type"] = serde_json::Value::String(format!("{:?}", msg.spa_type.unwrap_or_default()));
            message_json["spa_type_raw"] = serde_json::Value::Number(msg.spa_type.unwrap_or_default().value().into());
            message_json["website_registration"] =
                serde_json::Value::Bool(msg.website_registration.unwrap_or_default());
            message_json["website_registration_confirm"] =
                serde_json::Value::Bool(msg.website_registration_confirm.unwrap_or_default());
            message_json["mac_address"] = serde_json::Value::Array(
                msg.mac_address
                    .clone()
                    .unwrap_or_default()
                    .iter()
                    .map(|byte| serde_json::Value::Number((*byte).into()).into())
                    .collect(),
            );
            message_json["firmware_version"] =
                serde_json::Value::Number(msg.firmware_version.unwrap_or_default().into());
            message_json["product_code"] = serde_json::Value::Number(msg.product_code.unwrap_or_default().into());
            message_json["var_software_version"] =
                serde_json::Value::String(msg.var_software_version.clone().unwrap_or_default());
            message_json["spaboy_firmware_version"] =
                serde_json::Value::String(msg.spaboy_firmware_version.clone().unwrap_or_default());
            message_json["spaboy_hardware_version"] =
                serde_json::Value::String(msg.spaboy_hardware_version.clone().unwrap_or_default());
            message_json["spaboy_product_id"] =
                serde_json::Value::String(msg.spaboy_product_id.clone().unwrap_or_default());
            message_json["spaboy_serial_number"] =
                serde_json::Value::String(msg.spaboy_serial_number.clone().unwrap_or_default());
            message_json["rfid_firmware_version"] =
                serde_json::Value::String(msg.rfid_firmware_version.clone().unwrap_or_default());
            message_json["rfid_hardware_version"] =
                serde_json::Value::String(msg.rfid_hardware_version.clone().unwrap_or_default());
            message_json["rfid_product_id"] =
                serde_json::Value::String(msg.rfid_product_id.clone().unwrap_or_default());
            message_json["rfid_serial_number"] =
                serde_json::Value::String(msg.rfid_serial_number.clone().unwrap_or_default());
        }
        ProtoMessage::Error { message: msg, .. } => {
            message_json["no_flow"] = serde_json::Value::Bool(msg.no_flow.unwrap_or_default());
            message_json["flow_switch"] = serde_json::Value::Bool(msg.flow_switch.unwrap_or_default());
            message_json["heater_over_temperature"] =
                serde_json::Value::Bool(msg.heater_over_temperature.unwrap_or_default());
            message_json["spa_over_temperature"] =
                serde_json::Value::Bool(msg.spa_over_temperature.unwrap_or_default());
            message_json["spa_temperature_probe"] =
                serde_json::Value::Bool(msg.spa_temperature_probe.unwrap_or_default());
            message_json["spa_high_limit"] = serde_json::Value::Bool(msg.spa_high_limit.unwrap_or_default());
            message_json["eeprom"] = serde_json::Value::Bool(msg.eeprom.unwrap_or_default());
            message_json["freeze_protect"] = serde_json::Value::Bool(msg.freeze_protect.unwrap_or_default());
            message_json["ph_high"] = serde_json::Value::Bool(msg.ph_high.unwrap_or_default());
            message_json["heater_probe_disconnected"] =
                serde_json::Value::Bool(msg.heater_probe_disconnected.unwrap_or_default());
        }
        ProtoMessage::Router { message: msg, .. } => {
            message_json["ssid"] = serde_json::Value::String(msg.ssid.clone().unwrap_or_default());
            message_json["password"] = serde_json::Value::String(msg.password.clone().unwrap_or_default());
            message_json["encryption"] = serde_json::Value::String(format!("{:?}", msg.encryption.unwrap_or_default()));
            message_json["encryption_raw"] =
                serde_json::Value::Number(msg.encryption.unwrap_or_default().value().into());
            message_json["protocol"] = serde_json::Value::String(format!("{:?}", msg.protocol.unwrap_or_default()));
            message_json["protocol_raw"] = serde_json::Value::Number(msg.protocol.unwrap_or_default().value().into());
        }
        ProtoMessage::Filter { message: msg, .. } => {
            message_json["serial_nums"] = serde_json::Value::String(msg.serial_nums.clone().unwrap_or_default());
            message_json["filter_state"] =
                serde_json::Value::String(format!("{:?}", msg.filter_state.unwrap_or_default()));
            message_json["filter_state_raw"] =
                serde_json::Value::Number(msg.filter_state.unwrap_or_default().value().into());
            message_json["install_dates"] = serde_json::Value::String(msg.install_dates.clone().unwrap_or_default());
        }
        ProtoMessage::Peripheral { message: msg, .. } => {
            message_json["guid"] = serde_json::Value::String(msg.guid.clone().unwrap_or_default());
            message_json["hardware_version"] =
                serde_json::Value::Number(msg.hardware_version.unwrap_or_default().into());
            message_json["firmware_version"] =
                serde_json::Value::Number(msg.firmware_version.unwrap_or_default().into());
            message_json["product_code"] =
                serde_json::Value::String(format!("{:?}", msg.product_code.unwrap_or_default()));
            message_json["product_code_raw"] =
                serde_json::Value::Number(msg.product_code.unwrap_or_default().value().into());
            message_json["connected"] = serde_json::Value::Bool(msg.connected.unwrap_or_default());
        }
        ProtoMessage::OnzenLive { message: msg, .. } => {
            message_json["guid"] = serde_json::Value::String(msg.guid.clone().unwrap_or_default());
            message_json["orp"] = serde_json::Value::Number(msg.orp.unwrap_or_default().into());
            message_json["ph_100"] = serde_json::Value::Number(msg.ph_100.unwrap_or_default().into());
            message_json["current"] = serde_json::Value::Number(msg.current.unwrap_or_default().into());
            message_json["voltage"] = serde_json::Value::Number(msg.voltage.unwrap_or_default().into());
            message_json["current_setpoint"] =
                serde_json::Value::Number(msg.current_setpoint.unwrap_or_default().into());
            message_json["voltage_setpoint"] =
                serde_json::Value::Number(msg.voltage_setpoint.unwrap_or_default().into());
            message_json["pump1"] = serde_json::Value::Bool(msg.pump1.unwrap_or_default());
            message_json["pump2"] = serde_json::Value::Bool(msg.pump2.unwrap_or_default());
            message_json["orp_state_machine"] =
                serde_json::Value::Number(msg.orp_state_machine.unwrap_or_default().into());
            message_json["electrode_state_machine"] =
                serde_json::Value::Number(msg.electrode_state_machine.unwrap_or_default().into());
            message_json["electrode_id"] = serde_json::Value::Number(msg.electrode_id.unwrap_or_default().into());
            message_json["electrode_polarity"] =
                serde_json::Value::String(format!("{:?}", msg.electrode_polarity.unwrap_or_default()));
            message_json["electrode_polarity_raw"] =
                serde_json::Value::Number(msg.electrode_polarity.unwrap_or_default().value().into());
            message_json["electrode_1_resistance_1"] =
                serde_json::Value::Number(msg.electrode_1_resistance_1.unwrap_or_default().into());
            message_json["electrode_1_resistance_2"] =
                serde_json::Value::Number(msg.electrode_1_resistance_2.unwrap_or_default().into());
            message_json["electrode_2_resistance_1"] =
                serde_json::Value::Number(msg.electrode_2_resistance_1.unwrap_or_default().into());
            message_json["electrode_2_resistance_2"] =
                serde_json::Value::Number(msg.electrode_2_resistance_2.unwrap_or_default().into());
            message_json["command_mode"] = serde_json::Value::Bool(msg.command_mode.unwrap_or_default());
            message_json["electrode_mAH"] = serde_json::Value::Number(msg.electrode_mAH.unwrap_or_default().into());
            message_json["ph_color"] = serde_json::Value::String(format!("{:?}", msg.ph_color.unwrap_or_default()));
            message_json["ph_color_raw"] = serde_json::Value::Number(msg.ph_color.unwrap_or_default().value().into());
            message_json["orp_color"] = serde_json::Value::String(format!("{:?}", msg.orp_color.unwrap_or_default()));
            message_json["orp_color_raw"] = serde_json::Value::Number(msg.orp_color.unwrap_or_default().value().into());
            message_json["electrode_wear"] = serde_json::Value::Number(msg.electrode_wear.unwrap_or_default().into());
        }
        ProtoMessage::OnzenSettings { message: msg, .. } => {
            message_json["guid"] = serde_json::Value::String(msg.guid.clone().unwrap_or_default());
            message_json["over_voltage"] = serde_json::Value::Number(msg.over_voltage.unwrap_or_default().into());
            message_json["under_voltage"] = serde_json::Value::Number(msg.under_voltage.unwrap_or_default().into());
            message_json["over_current"] = serde_json::Value::Number(msg.over_current.unwrap_or_default().into());
            message_json["under_current"] = serde_json::Value::Number(msg.under_current.unwrap_or_default().into());
            message_json["orp_high"] = serde_json::Value::Number(msg.orp_high.unwrap_or_default().into());
            message_json["orp_low"] = serde_json::Value::Number(msg.orp_low.unwrap_or_default().into());
            message_json["ph_high"] = serde_json::Value::Number(msg.ph_high.unwrap_or_default().into());
            message_json["ph_low"] = serde_json::Value::Number(msg.ph_low.unwrap_or_default().into());
            message_json["pwm_pump1_time_on"] =
                serde_json::Value::Number(msg.pwm_pump1_time_on.unwrap_or_default().into());
            message_json["pwm_pump1_time_off"] =
                serde_json::Value::Number(msg.pwm_pump1_time_off.unwrap_or_default().into());
            message_json["sampling_interval"] =
                serde_json::Value::Number(msg.sampling_interval.unwrap_or_default().into());
            message_json["sampling_duration"] =
                serde_json::Value::Number(msg.sampling_duration.unwrap_or_default().into());
            message_json["pwm_pump2_time_on"] =
                serde_json::Value::Number(msg.pwm_pump2_time_on.unwrap_or_default().into());
            message_json["pwm_pump2_time_off"] =
                serde_json::Value::Number(msg.pwm_pump2_time_off.unwrap_or_default().into());
            message_json["sb_low_cl"] = serde_json::Value::Number(msg.sb_low_cl.unwrap_or_default().into());
            message_json["sb_caution_low_cl"] =
                serde_json::Value::Number(msg.sb_caution_low_cl.unwrap_or_default().into());
            message_json["sb_caution_high_cl"] =
                serde_json::Value::Number(msg.sb_caution_high_cl.unwrap_or_default().into());
            message_json["sb_high_cl"] = serde_json::Value::Number(msg.sb_high_cl.unwrap_or_default().into());
            message_json["sb_low_ph"] = serde_json::Value::Number(msg.sb_low_ph.unwrap_or_default().into());
            message_json["sb_caution_low_ph"] =
                serde_json::Value::Number(msg.sb_caution_low_ph.unwrap_or_default().into());
            message_json["sb_caution_high_ph"] =
                serde_json::Value::Number(msg.sb_caution_high_ph.unwrap_or_default().into());
            message_json["sb_high_ph"] = serde_json::Value::Number(msg.sb_high_ph.unwrap_or_default().into());
        }
    }

    serde_json::to_string_pretty(&message_json).expect("failed to serialize message to JSON")
}

pub fn display_message(
    message_type: &MessageType,
    message: &ProtoMessage,
    output_format: Option<&QueryOutputFormat>,
    output_path: Option<&Path>,
) -> () {
    log::info!("outputting message data");

    let output_string = match output_format {
        Some(QueryOutputFormat::PlainText) | None => get_message_formatted_plain_text(message),
        Some(QueryOutputFormat::Json) => get_message_formatted_json(message),
    };

    let received_at = message.received_at_formatted(None);

    match output_path {
        Some(path) => {
            log::info!("writing message data to file: {:?}", path);
            match std::fs::write(path, output_string) {
                Ok(_) => log::info!("successfully wrote message data to {:?}", path),
                Err(e) => {
                    log::error!("failed to write message data to file: {:?}", e);
                    eprintln!("failed to write to file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => match output_format {
            Some(QueryOutputFormat::PlainText) | None => {
                println!("message type: \"{:#?}\"", message_type);
                println!("received at: {:?}", received_at);
                for line in output_string.split('\n') {
                    if !line.is_empty() {
                        println!("{}", line);
                    }
                }
            }
            Some(QueryOutputFormat::Json) => {
                println!("{}", output_string);
                return;
            }
        },
    }
}
