use clap::ValueEnum;

use crate::asdc;
use crate::proto;


#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum DevicePropertyNameGet {
    /// READONLY: Current water temperature in Fahrenheit
    #[value(name = "temperature-current")]
    TemperatureCurrent,
    /// READONLY: Current water temperature in Fahrenheit (shortcut)
    #[value(name = "temp-current")]
    TempCurrent,
    /// READONLY: Current water temperature in Fahrenheit (shortcut)
    #[value(name = "temp")]
    Temp,
    /// Temperature setpoint in Fahrenheit
    #[value(name = "temperature-setpoint")]
    TemperatureSetpoint,
    /// Temperature setpoint (shortcut)
    #[value(name = "temp-setpoint")]
    TempSetpoint,
    /// Temperature setpoint (shortcut)
    #[value(name = "temp-sp")]
    TempSp,
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
    /// READONLY: Heater 1 (IDLE, WARMUP, HEATING, COOLDOWN)
    #[value(name = "heater-1")]
    Heater1,
    /// READONLY: Heater 2 (IDLE, WARMUP, HEATING, COOLDOWN)
    #[value(name = "heater-2")]
    Heater2,
    /// Filter (IDLE, FILTERING, BOOST, PURGE, SANITIZE, SUSPENDED, RESUMING, OVER_TEMPERATURE)
    Filter,
    /// Onzen (ON, OFF)
    Onzen,
    /// Ozone (IDLE, ACTIVE, SUSPENDED)
    Ozone,
    /// Exhaust fan (ON, OFF)
    #[value(name = "exhaust-fan")]
    ExhaustFan,
    /// Sauna state (NORMAL, PRESET_A, PRESET_B, PRESET_C)
    #[value(name = "sauna-state")]
    SaunaState,
    /// Sauna time left in minutes (number: min 0, max 60)
    #[value(name = "sauna-time-left")]
    SaunaTimeLeft,
    /// EZ button - turn on / off all jets and lights at once (ON, OFF)
    #[value(name = "all-on")]
    AllOn,
    /// Fogger (ON, OFF)
    Fogger,
    /// SDS (ON, OFF)
    Sds,
    /// YESS (ON, OFF)
    Yess,
    /// READONLY: ORP (Oxidation-Reduction Potential) in mV
    Orp,
    /// READONLY: PH_100 (pH level multiplied by 100, e.g. 712 = pH 7.12)
    #[value(name = "ph-100")]
    Ph100,
    /// READONLY: ORP Color (LOW, CAUTION_LOW, OK, CAUTION_HIGH, HIGH)
    #[value(name = "orp-color")]
    OrpColor,
    /// READONLY: pH Color (LOW, CAUTION_LOW, OK, CAUTION_HIGH, HIGH)
    #[value(name = "ph-color")]
    PhColor,
}

impl DevicePropertyNameGet {
    fn as_name(&self) -> &'static str {
        match self {
            DevicePropertyNameGet::TemperatureCurrent => "temperature-current",
            DevicePropertyNameGet::TempCurrent => "temp-current",
            DevicePropertyNameGet::Temp => "temp",
            DevicePropertyNameGet::TemperatureSetpoint => "temperature-setpoint",
            DevicePropertyNameGet::TempSetpoint => "temp-setpoint",
            DevicePropertyNameGet::TempSp => "temp-sp",
            DevicePropertyNameGet::Pump1 => "pump-1",
            DevicePropertyNameGet::Pump2 => "pump-2",
            DevicePropertyNameGet::Pump3 => "pump-3",
            DevicePropertyNameGet::Pump4 => "pump-4",
            DevicePropertyNameGet::Pump5 => "pump-5",
            DevicePropertyNameGet::Blower1 => "blower-1",
            DevicePropertyNameGet::Blower2 => "blower-2",
            DevicePropertyNameGet::Lights => "lights",
            DevicePropertyNameGet::Stereo => "stereo",
            DevicePropertyNameGet::Heater1 => "heater-1",
            DevicePropertyNameGet::Heater2 => "heater-2",
            DevicePropertyNameGet::Filter => "filter",
            DevicePropertyNameGet::Onzen => "onzen",
            DevicePropertyNameGet::Ozone => "ozone",
            DevicePropertyNameGet::ExhaustFan => "exhaust-fan",
            DevicePropertyNameGet::SaunaState => "sauna-state",
            DevicePropertyNameGet::SaunaTimeLeft => "sauna-time-left",
            DevicePropertyNameGet::AllOn => "all-on",
            DevicePropertyNameGet::Fogger => "fogger",
            DevicePropertyNameGet::Sds => "sds",
            DevicePropertyNameGet::Yess => "yess",
            DevicePropertyNameGet::Orp => "orp",
            DevicePropertyNameGet::Ph100 => "ph-100",
            DevicePropertyNameGet::OrpColor => "orp-color",
            DevicePropertyNameGet::PhColor => "ph-color",
        }
    }
}


#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum DevicePropertyNameSet {
    /// Temperature setpoint in Fahrenheit (number: min 59, max 104)
    #[value(name = "temperature-setpoint")]
    TemperatureSetpoint,
    /// Temperature setpoint (shortcut)
    #[value(name = "temp-setpoint")]
    TempSetpoint,
    /// Temperature setpoint (shortcut)
    #[value(name = "temp-sp")]
    TempSp,
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
    /// Sauna state (NORMAL, PRESET_A, PRESET_B, PRESET_C, TIMER)
    #[value(name = "sauna-state")]
    SaunaState,
    /// Sauna time left in minutes (number: min 0, max 120)
    #[value(name = "sauna-time-left")]
    SaunaTimeLeft,
    /// "EZ Button" - Turn on / off all jets and lights at once (ON, OFF)
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

impl DevicePropertyNameSet {
    fn as_name(&self) -> &'static str {
        match self {
            DevicePropertyNameSet::TemperatureSetpoint => "temperature-setpoint",
            DevicePropertyNameSet::TempSetpoint => "temp-setpoint",
            DevicePropertyNameSet::TempSp => "temp-sp",
            DevicePropertyNameSet::Pump1 => "pump-1",
            DevicePropertyNameSet::Pump2 => "pump-2",
            DevicePropertyNameSet::Pump3 => "pump-3",
            DevicePropertyNameSet::Pump4 => "pump-4",
            DevicePropertyNameSet::Pump5 => "pump-5",
            DevicePropertyNameSet::Blower1 => "blower-1",
            DevicePropertyNameSet::Blower2 => "blower-2",
            DevicePropertyNameSet::Lights => "lights",
            DevicePropertyNameSet::Stereo => "stereo",
            DevicePropertyNameSet::Filter => "filter",
            DevicePropertyNameSet::Onzen => "onzen",
            DevicePropertyNameSet::Ozone => "ozone",
            DevicePropertyNameSet::ExhaustFan => "exhaust-fan",
            DevicePropertyNameSet::SaunaState => "sauna-state",
            DevicePropertyNameSet::SaunaTimeLeft => "sauna-time-left",
            DevicePropertyNameSet::AllOn => "all-on",
            DevicePropertyNameSet::Fogger => "fogger",
            DevicePropertyNameSet::SpaboyBoost => "spaboy-boost",
            DevicePropertyNameSet::PackReset => "pack-reset",
            DevicePropertyNameSet::LogDump => "log-dump",
            DevicePropertyNameSet::Sds => "sds",
            DevicePropertyNameSet::Yess => "yess",
        }
    }
}


fn pump_status_to_string(value: proto::Live::live::PumpStatus) -> String {
    match value {
        proto::Live::live::PumpStatus::PUMP_OFF => "OFF".to_string(),
        proto::Live::live::PumpStatus::PUMP_LOW => "LOW".to_string(),
        proto::Live::live::PumpStatus::PUMP_HIGH => "HIGH".to_string(),
    }
}
fn heater_status_to_string(value: proto::Live::live::HeaterStatus) -> String {
    match value {
        proto::Live::live::HeaterStatus::HEATER_IDLE => "IDLE".to_string(),
        proto::Live::live::HeaterStatus::HEATER_WARMUP => "WARMUP".to_string(),
        proto::Live::live::HeaterStatus::HEATER_HEATING => "HEATING".to_string(),
        proto::Live::live::HeaterStatus::HEATER_COOLDOWN => "COOLDOWN".to_string(),
    }
}
fn filter_status_to_string(value: proto::Live::live::FilterStatus) -> String {
    match value {
        proto::Live::live::FilterStatus::FILTER_IDLE => "IDLE".to_string(),
        proto::Live::live::FilterStatus::FILTER_FILTERING => "FILTERING".to_string(),
        proto::Live::live::FilterStatus::FILTER_BOOST => "BOOST".to_string(),
        proto::Live::live::FilterStatus::FILTER_PURGE => "PURGE".to_string(),
        proto::Live::live::FilterStatus::FILTER_SANITIZE => "SANITIZE".to_string(),
        proto::Live::live::FilterStatus::FILTER_SUSPENDED => "SUSPENDED".to_string(),
        proto::Live::live::FilterStatus::FILTER_RESUMING => "RESUMING".to_string(),
        proto::Live::live::FilterStatus::FILTER_OVER_TEMPERATURE => "OVER_TEMPERATURE".to_string(),
    }
}
fn ozone_status_to_string(value: proto::Live::live::OzoneStatus) -> String {
    match value {
        proto::Live::live::OzoneStatus::OZONE_IDLE => "IDLE".to_string(),
        proto::Live::live::OzoneStatus::OZONE_ACTIVE => "ACTIVE".to_string(),
        proto::Live::live::OzoneStatus::OZONE_SUSPENDED => "SUSPENDED".to_string(),
    }
}
fn sauna_status_to_string(value: proto::Live::live::SaunaStatus) -> String {
    match value {
        proto::Live::live::SaunaStatus::SAUNA_NORMAL => "NORMAL".to_string(),
        proto::Live::live::SaunaStatus::SAUNA_PRESET_A => "PRESET_A".to_string(),
        proto::Live::live::SaunaStatus::SAUNA_PRESET_B => "PRESET_B".to_string(),
        proto::Live::live::SaunaStatus::SAUNA_PRESET_C => "PRESET_C".to_string(),
    }
}
fn sauna_state_to_string(value: proto::Command::command::SetSaunaState) -> String {
    match value {
        proto::Command::command::SetSaunaState::SAUNA_IDLE => "IDLE".to_string(),
        proto::Command::command::SetSaunaState::SAUNA_PRESET_A => "PRESET_A".to_string(),
        proto::Command::command::SetSaunaState::SAUNA_PRESET_B => "PRESET_B".to_string(),
        proto::Command::command::SetSaunaState::SAUNA_PRESET_C => "PRESET_C".to_string(),
        proto::Command::command::SetSaunaState::SAUNA_TIMER => "TIMER".to_string(),
    }
}
fn color_status_to_string(value: proto::OnzenLive::onzen_live::Color) -> String {
    match value {
        proto::OnzenLive::onzen_live::Color::COLOR_LOW => "LOW".to_string(),
        proto::OnzenLive::onzen_live::Color::COLOR_CAUTION_LOW => "CAUTION_LOW".to_string(),
        proto::OnzenLive::onzen_live::Color::COLOR_OK => "OK".to_string(),
        proto::OnzenLive::onzen_live::Color::COLOR_CAUTION_HIGH => "CAUTION_HIGH".to_string(),
        proto::OnzenLive::onzen_live::Color::COLOR_HIGH => "HIGH".to_string(),
    }
}
fn bool_to_string(value: bool) -> String {
    let value_parsed = if value { "ON" } else { "OFF" };
    value_parsed.to_string()
}
fn i32_to_string(value: i32) -> String {
    value.to_string()
}


fn string_to_set_pump_status(value: &String) -> Result<proto::Command::command::SetPumpStatus, Box<dyn std::error::Error>> {
    let value_parsed = value.trim().to_uppercase();
    match value_parsed.as_str() {
        "OFF" => Ok(proto::Command::command::SetPumpStatus::PUMP_OFF),
        "LOW" => Ok(proto::Command::command::SetPumpStatus::PUMP_LOW),
        "HIGH" => Ok(proto::Command::command::SetPumpStatus::PUMP_HIGH),
        _ => Err(format!("invalid set pump status value: {}", value).into()),
    }
}
fn string_to_set_sauna_state(value: &String) -> Result<proto::Command::command::SetSaunaState, Box<dyn std::error::Error>> {
    let value_parsed = value.trim().to_uppercase();
    match value_parsed.as_str() {
        "IDLE" => Ok(proto::Command::command::SetSaunaState::SAUNA_IDLE),
        "PRESET_A" => Ok(proto::Command::command::SetSaunaState::SAUNA_PRESET_A),
        "PRESET_B" => Ok(proto::Command::command::SetSaunaState::SAUNA_PRESET_B),
        "PRESET_C" => Ok(proto::Command::command::SetSaunaState::SAUNA_PRESET_C),
        "TIMER" => Ok(proto::Command::command::SetSaunaState::SAUNA_TIMER),
        _ => Err(format!("invalid set sauna state value: {}", value).into()),
    }
}
fn string_to_bool(value: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let value_parsed = value.trim().to_uppercase();
    match value_parsed.as_str() {
        "ON" => Ok(true),
        "OFF" => Ok(false),
        _ => Err(format!("invalid boolean value: {}", value).into()),
    }
}
fn string_to_i32(value: &String, min: i32, max: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let value_parsed = value.parse::<i32>()?;
    if value_parsed < min || value_parsed > max {
        return Err(
            Box::new(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("value {} is out of range ({}-{})", value_parsed, min, max)
                )
            )
        );
    }
    Ok(value_parsed)
}


fn device_property_name_to_message_type(property_name: DevicePropertyNameGet) -> asdc::MessageType {
    match property_name {
        DevicePropertyNameGet::TemperatureCurrent
        | DevicePropertyNameGet::TempCurrent
        | DevicePropertyNameGet::Temp
        | DevicePropertyNameGet::TemperatureSetpoint
        | DevicePropertyNameGet::TempSetpoint
        | DevicePropertyNameGet::TempSp
        | DevicePropertyNameGet::Pump1
        | DevicePropertyNameGet::Pump2
        | DevicePropertyNameGet::Pump3
        | DevicePropertyNameGet::Pump4
        | DevicePropertyNameGet::Pump5
        | DevicePropertyNameGet::Blower1
        | DevicePropertyNameGet::Blower2
        | DevicePropertyNameGet::Lights
        | DevicePropertyNameGet::Stereo
        | DevicePropertyNameGet::Heater1
        | DevicePropertyNameGet::Heater2
        | DevicePropertyNameGet::Filter
        | DevicePropertyNameGet::Onzen
        | DevicePropertyNameGet::Ozone
        | DevicePropertyNameGet::ExhaustFan
        | DevicePropertyNameGet::SaunaState
        | DevicePropertyNameGet::SaunaTimeLeft
        | DevicePropertyNameGet::AllOn
        | DevicePropertyNameGet::Fogger
        | DevicePropertyNameGet::Sds
        | DevicePropertyNameGet::Yess => {
            asdc::MessageType::Live
        },
        DevicePropertyNameGet::Orp
        | DevicePropertyNameGet::Ph100
        | DevicePropertyNameGet::OrpColor
        | DevicePropertyNameGet::PhColor => {
            asdc::MessageType::OnzenLive
        }
    }
}
fn get_message_value_from_property(message_type: asdc::MessageType, message: asdc::ProtoMessage, property_name: DevicePropertyNameGet) -> Result<String, Box<dyn std::error::Error>> {
    if message_type == asdc::MessageType::Live {
        let message_live = message.as_live().ok_or("Failed to convert message to Live")?;
        match property_name {
            DevicePropertyNameGet::TemperatureCurrent
            | DevicePropertyNameGet::TempCurrent
            | DevicePropertyNameGet::Temp => {
                return Ok(i32_to_string(message_live.temperature_fahrenheit()));
            }
            DevicePropertyNameGet::TemperatureSetpoint
            | DevicePropertyNameGet::TempSetpoint
            | DevicePropertyNameGet::TempSp => {
                return Ok(i32_to_string(message_live.temperature_setpoint_fahrenheit()));
            }
            DevicePropertyNameGet::Pump1 => { return Ok(pump_status_to_string(message_live.pump_1())); }
            DevicePropertyNameGet::Pump2 => { return Ok(pump_status_to_string(message_live.pump_2())); }
            DevicePropertyNameGet::Pump3 => { return Ok(pump_status_to_string(message_live.pump_3())); }
            DevicePropertyNameGet::Pump4 => { return Ok(pump_status_to_string(message_live.pump_4())); }
            DevicePropertyNameGet::Pump5 => { return Ok(pump_status_to_string(message_live.pump_5())); }
            DevicePropertyNameGet::Blower1 => { return Ok(pump_status_to_string(message_live.blower_1())); }
            DevicePropertyNameGet::Blower2 => { return Ok(pump_status_to_string(message_live.blower_2())); }
            DevicePropertyNameGet::Lights => { return Ok(bool_to_string(message_live.lights())); }
            DevicePropertyNameGet::Stereo => { return Ok(bool_to_string(message_live.stereo())); }
            DevicePropertyNameGet::Heater1 => { return Ok(heater_status_to_string(message_live.heater_1())); }
            DevicePropertyNameGet::Heater2 => { return Ok(heater_status_to_string(message_live.heater_2())); }
            DevicePropertyNameGet::Filter => { return Ok(filter_status_to_string(message_live.filter())); }
            DevicePropertyNameGet::Onzen => { return Ok(bool_to_string(message_live.onzen())); }
            DevicePropertyNameGet::Ozone => { return Ok(ozone_status_to_string(message_live.ozone())); }
            DevicePropertyNameGet::ExhaustFan => { return Ok(bool_to_string(message_live.exhaust_fan())); }
            DevicePropertyNameGet::SaunaState => { return Ok(sauna_status_to_string(message_live.sauna())); }
            DevicePropertyNameGet::SaunaTimeLeft => { return Ok(i32_to_string(message_live.sauna_time_remaining())); }
            DevicePropertyNameGet::AllOn => { return Ok(bool_to_string(message_live.all_on())); }
            DevicePropertyNameGet::Fogger => { return Ok(bool_to_string(message_live.fogger())); }
            DevicePropertyNameGet::Sds => { return Ok(bool_to_string(message_live.sds())); }
            DevicePropertyNameGet::Yess => { return Ok(bool_to_string(message_live.yess())); }
            _ => {
                log::error!("unsupported property for Live message type: {:?}", property_name.as_name());
                return Err(format!("unsupported property for Live message type: {:?}", property_name.as_name()).into());
            }
        }
    } else if message_type == asdc::MessageType::OnzenLive {
        let message_onzen_live = message.as_onzen_live().ok_or("Failed to convert message to OnzenLive")?;
        match property_name {
            DevicePropertyNameGet::Orp => { return Ok(i32_to_string(message_onzen_live.orp())); }
            DevicePropertyNameGet::Ph100 => { return Ok(i32_to_string(message_onzen_live.ph_100())); }
            DevicePropertyNameGet::OrpColor => { return Ok(color_status_to_string(message_onzen_live.orp_color())); }
            DevicePropertyNameGet::PhColor => { return Ok(color_status_to_string(message_onzen_live.ph_color())); }
            _ => {
                log::error!("unsupported property for OnzenLive message type: {:?}", property_name.as_name());
                return Err(format!("unsupported property for OnzenLive message type: {:?}", property_name.as_name()).into());
            }
        }
    }

    Err(format!("unsupported message type: {:?}", message_type).into())
}
fn get_message_value(network_client: &mut asdc::NetworkClient, property_name: DevicePropertyNameGet) -> Result<String, Box<dyn std::error::Error>> {
    let message_type = device_property_name_to_message_type(property_name);
    let message = network_client.request_message_and_await_response(message_type)?;
    let value = get_message_value_from_property(message_type, message, property_name)?;
    Ok(value)
}

pub fn get_device_property_value(ip_address: &str, property_name: DevicePropertyNameGet) -> Result<String, Box<dyn std::error::Error>> {
    log::debug!("read device property value {:?}", property_name.as_name());
    let mut network_client = asdc::NetworkClient::connect_to(ip_address)?;
    let value = get_message_value(&mut network_client, property_name)?;
    log::info!("successfully read device property value {:?}={:?}", property_name.as_name(), value);
    Ok(value)
}
pub fn display_device_property_value(property_name: DevicePropertyNameGet, value: &String) -> () {
    println!("device value: {:?} = {:?}", property_name.as_name(), value);
}
pub fn get_and_display_all_device_properties(ip_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut network_client = asdc::NetworkClient::connect_to(ip_address)?;

    let all_get_properties = [
        DevicePropertyNameGet::TemperatureCurrent,
        DevicePropertyNameGet::TemperatureSetpoint,
        DevicePropertyNameGet::Pump1,
        DevicePropertyNameGet::Pump2,
        DevicePropertyNameGet::Pump3,
        DevicePropertyNameGet::Pump4,
        DevicePropertyNameGet::Pump5,
        DevicePropertyNameGet::Blower1,
        DevicePropertyNameGet::Blower2,
        DevicePropertyNameGet::Lights,
        DevicePropertyNameGet::Stereo,
        DevicePropertyNameGet::Heater1,
        DevicePropertyNameGet::Heater2,
        DevicePropertyNameGet::Filter,
        DevicePropertyNameGet::Onzen,
        DevicePropertyNameGet::Ozone,
        DevicePropertyNameGet::ExhaustFan,
        DevicePropertyNameGet::SaunaState,
        DevicePropertyNameGet::SaunaTimeLeft,
        DevicePropertyNameGet::AllOn,
        DevicePropertyNameGet::Fogger,
        DevicePropertyNameGet::Sds,
        DevicePropertyNameGet::Yess,
        DevicePropertyNameGet::Orp,
        DevicePropertyNameGet::Ph100,
        DevicePropertyNameGet::OrpColor,
        DevicePropertyNameGet::PhColor
    ];

    println!("displaying all device properties");
    for property in all_get_properties.iter() {
        let value = get_message_value(&mut network_client, *property)?;
        display_device_property_value(*property, &value);
    }

    Ok(())
}


fn send_and_log(
    network_client: &mut asdc::NetworkClient,
    property_name: DevicePropertyNameSet,
    value: &String,
    command_message: proto::Command::Command,
) -> Result<(), Box<dyn std::error::Error>> {
    network_client.send_command(command_message)?;
    log::info!("successfully wrote device property value {:?}={:?}", property_name.as_name(), value);
    Ok(())
}
pub fn set_device_property_value(ip_address: &str, property_name: DevicePropertyNameSet, value: &String) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("write device property value {:?}={:?}", property_name.as_name(), value);

    let mut network_client = asdc::NetworkClient::connect_to(ip_address)?;
    let mut command_message = proto::Command::Command::new();

    match property_name {
        DevicePropertyNameSet::TemperatureSetpoint
        | DevicePropertyNameSet::TempSetpoint
        | DevicePropertyNameSet::TempSp => {
            let i32_value = string_to_i32(value, 59, 104)?;
            command_message.set_set_temperature_setpoint_fahrenheit(i32_value);
        }
        DevicePropertyNameSet::Pump1 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_pump_1(pump_status);
        }
        DevicePropertyNameSet::Pump2 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_pump_2(pump_status);
        }
        DevicePropertyNameSet::Pump3 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_pump_3(pump_status);
        }
        DevicePropertyNameSet::Pump4 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_pump_4(pump_status);
        }
        DevicePropertyNameSet::Pump5 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_pump_5(pump_status);
        }
        DevicePropertyNameSet::Blower1 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_blower_1(pump_status);
        }
        DevicePropertyNameSet::Blower2 => {
            let pump_status = string_to_set_pump_status(value)?;
            command_message.set_set_blower_2(pump_status);
        }
        DevicePropertyNameSet::Lights => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_lights(bool_value);
        }
        DevicePropertyNameSet::Stereo => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_stereo(bool_value);
        }
        DevicePropertyNameSet::Filter => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_filter(bool_value);
        }
        DevicePropertyNameSet::Onzen => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_onzen(bool_value);
        }
        DevicePropertyNameSet::Ozone => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_ozone(bool_value);
        }
        DevicePropertyNameSet::ExhaustFan => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_exhaust_fan(bool_value);
        }
        DevicePropertyNameSet::SaunaState => {
            let sauna_state = string_to_set_sauna_state(value)?;
            command_message.set_set_sauna_state(sauna_state);
        }
        DevicePropertyNameSet::SaunaTimeLeft => {
            let i32_value = string_to_i32(value, 0, 120)?;
            command_message.set_set_sauna_time_left(i32_value);
        }
        DevicePropertyNameSet::AllOn => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_all_on(bool_value);
        }
        DevicePropertyNameSet::Fogger => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_fogger(bool_value);
        }
        DevicePropertyNameSet::SpaboyBoost => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_spaboy_boost(bool_value);
        }
        DevicePropertyNameSet::PackReset => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_pack_reset(bool_value);
        }
        DevicePropertyNameSet::LogDump => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_log_dump(bool_value);
        }
        DevicePropertyNameSet::Sds => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_sds(bool_value);
        }
        DevicePropertyNameSet::Yess => {
            let bool_value = string_to_bool(value)?;
            command_message.set_set_yess(bool_value);
        }
    }

    send_and_log(&mut network_client, property_name, value, command_message)
}
