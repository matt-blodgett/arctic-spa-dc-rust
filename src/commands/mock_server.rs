use std::collections::VecDeque;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Datelike, Timelike, Utc};
use std::time::{Duration, SystemTime};

use protobuf::Message;

use crate::core::net::MessageType;
use crate::proto;

const HEADER_SIZE: usize = 20;
const HEADER_PREAMBLE: [u8; 4] = [171, 173, 29, 58];

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 65534;
pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:65534";

const READ_TIMEOUT_MS: u64 = 1_000;

fn proto_error_to_io(error: protobuf::Error) -> Error {
    Error::new(ErrorKind::InvalidData, format!("protobuf io error: {}", error))
}

fn serialize_proto_message(message: &impl Message) -> Result<Vec<u8>, Error> {
    message.write_to_bytes().map_err(proto_error_to_io)
}

fn random_delta(seed: u64, max_abs: i32) -> i32 {
    let now_nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);

    // Lightweight xorshift-style mixing from time + seed.
    let mut x = now_nanos ^ seed.wrapping_mul(0x9E3779B97F4A7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;

    let magnitude = (x % (max_abs as u64) + 1) as i32;
    if (x & 1) == 0 { magnitude } else { -magnitude }
}

struct MockState {
    live: proto::Live::Live,
    onzen_live: proto::OnzenLive::OnzenLive,
    settings: proto::Settings::Settings,
    configuration: proto::Configuration::Configuration,
    peak: proto::Peak::Peak,
    clock: proto::Clock::Clock,
    information: proto::Information::Information,
    error: proto::Error::Error,
    router: proto::Router::Router,
    filter: proto::Filter::Filter,
    peripheral: proto::Peripheral::Peripheral,
    onzen_settings: proto::OnzenSettings::OnzenSettings,
    error_sequence_step: usize,
    response_queue: VecDeque<(MessageType, Vec<u8>)>,
}

impl MockState {
    fn default_message_clock() -> proto::Clock::Clock {
        let datetime: DateTime<Utc> = SystemTime::now().into();

        let mut message = proto::Clock::Clock::new();
        message.set_year(datetime.year() as i32);
        message.set_month(datetime.month() as i32);
        message.set_day(datetime.day() as i32);
        message.set_hour(datetime.hour() as i32);
        message.set_minute(datetime.minute() as i32);
        message.set_second(datetime.second() as i32);
        message
    }

    fn default_message_configuration() -> proto::Configuration::Configuration {
        let mut message = proto::Configuration::Configuration::new();
        message.set_pump1(true);
        message.set_pump2(true);
        message.set_pump3(true);
        message.set_pump4(false);
        message.set_pump5(false);
        message.set_blower1(false);
        message.set_blower2(false);
        message.set_lights(true);
        message.set_stereo(true);
        message.set_heater1(true);
        message.set_heater2(false);
        message.set_filter(true);
        message.set_onzen(false);
        message.set_ozone_peak_1(true);
        message.set_ozone_peak_2(false);
        message.set_exhaust_fan(true);
        message.set_powerlines(proto::Configuration::configuration::Phase::PHASE_SINGLE);
        message.set_breaker_size(50);
        message.set_smart_onzen(0);
        message.set_fogger(false);
        message.set_sds(false);
        message.set_yess(false);
        message
    }

    fn default_message_error() -> proto::Error::Error {
        let mut message = proto::Error::Error::new();
        message.set_no_flow(false);
        message.set_flow_switch(false);
        message.set_heater_over_temperature(false);
        message.set_spa_over_temperature(false);
        message.set_spa_temperature_probe(false);
        message.set_spa_high_limit(false);
        message.set_eeprom(false);
        message.set_freeze_protect(false);
        message.set_ph_high(false);
        message.set_heater_probe_disconnected(false);
        message
    }

    fn default_message_filter() -> proto::Filter::Filter {
        let mut message = proto::Filter::Filter::new();
        message.set_serial_nums(String::from("SN000"));
        message.set_filter_state(proto::Filter::filter::FilterState::FILTER_PURCHASE);
        message.set_install_dates(String::from("2020-01-01"));
        message
    }

    fn default_message_information() -> proto::Information::Information {
        let mut message = proto::Information::Information::new();
        message.set_pack_serial_number(String::from(""));
        message.set_pack_firmware_version(String::from("1.22.0033"));
        message.set_pack_hardware_version(String::from("1.12"));
        message.set_pack_product_id(String::from("ARCTIC"));
        message.set_pack_board_id(String::from("NA"));
        message.set_topside_product_id(String::from("TSC-14"));
        message.set_topside_software_version(String::from("106"));
        message.set_guid(String::from("f69a9o5g-f6a0-4v7q-71hh-2c2b46g4c098"));
        message.set_spa_type(proto::Information::information::SpaType::SPA_TYPE_HOT_TUB);
        message.set_website_registration(false);
        message.set_website_registration_confirm(false);
        message.set_mac_address(vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        message.set_firmware_version(53674022);
        message.set_product_code(65537);
        message.set_var_software_version(String::from(""));
        message.set_spaboy_firmware_version(String::from(""));
        message.set_spaboy_hardware_version(String::from(""));
        message.set_spaboy_product_id(String::from(""));
        message.set_spaboy_serial_number(String::from(""));
        message.set_rfid_firmware_version(String::from(""));
        message.set_rfid_hardware_version(String::from(""));
        message.set_rfid_product_id(String::from(""));
        message.set_rfid_serial_number(String::from(""));
        message
    }

    fn default_message_live() -> proto::Live::Live {
        let mut message = proto::Live::Live::new();
        message.set_temperature_fahrenheit(102);
        message.set_temperature_setpoint_fahrenheit(104);
        message.set_pump_1(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_pump_2(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_pump_3(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_pump_4(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_pump_5(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_blower_1(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_blower_2(proto::Live::live::PumpStatus::PUMP_OFF);
        message.set_lights(false);
        message.set_stereo(false);
        message.set_heater_1(proto::Live::live::HeaterStatus::HEATER_IDLE);
        message.set_heater_2(proto::Live::live::HeaterStatus::HEATER_IDLE);
        message.set_filter(proto::Live::live::FilterStatus::FILTER_IDLE);
        message.set_onzen(false);
        message.set_ozone(proto::Live::live::OzoneStatus::OZONE_IDLE);
        message.set_exhaust_fan(false);
        message.set_sauna(proto::Live::live::SaunaStatus::SAUNA_NORMAL);
        message.set_heater_adc(724);
        message.set_sauna_time_remaining(0);
        message.set_economy(false);
        message.set_current_adc(0);
        message.set_all_on(false);
        message.set_fogger(false);
        message.set_error(0);
        message.set_alarm(0);
        message.set_status(0);
        message.set_ph(0);
        message.set_orp(0);
        message.set_sds(false);
        message.set_yess(false);
        message
    }

    fn default_message_onzen_live() -> proto::OnzenLive::OnzenLive {
        let mut message = proto::OnzenLive::OnzenLive::new();
        message.set_guid(String::from("x67a2o5g-f6a0-5h9p-71xx-3f2b46g4c980"));
        message.set_orp(599);
        message.set_ph_100(727);
        message.set_current(4);
        message.set_voltage(8);
        message.set_current_setpoint(2400);
        message.set_voltage_setpoint(13000);
        message.set_pump1(false);
        message.set_pump2(false);
        message.set_orp_state_machine(0);
        message.set_electrode_state_machine(4);
        message.set_electrode_id(0);
        message.set_electrode_polarity(proto::OnzenLive::onzen_live::Polarity::POLARITY_NEGATIVE);
        message.set_electrode_1_resistance_1(1404);
        message.set_electrode_1_resistance_2(716);
        message.set_electrode_2_resistance_1(9999999);
        message.set_electrode_2_resistance_2(9999999);
        message.set_command_mode(false);
        message.set_electrode_mAH(9688);
        message.set_ph_color(proto::OnzenLive::onzen_live::Color::COLOR_OK);
        message.set_orp_color(proto::OnzenLive::onzen_live::Color::COLOR_OK);
        message.set_electrode_wear(85);
        message
    }

    fn default_message_onzen_settings() -> proto::OnzenSettings::OnzenSettings {
        let mut message = proto::OnzenSettings::OnzenSettings::new();
        message.set_guid(String::from("x67a2o5g-f6a0-5h9p-71xx-3f2b46g4c980"));
        message.set_over_voltage(13000);
        message.set_under_voltage(3000);
        message.set_over_current(7000);
        message.set_under_current(100);
        message.set_orp_high(555);
        message.set_orp_low(545);
        message.set_ph_high(760);
        message.set_ph_low(720);
        message.set_pwm_pump1_time_on(150);
        message.set_pwm_pump1_time_off(2500);
        message.set_sampling_interval(600);
        message.set_sampling_duration(40);
        message.set_pwm_pump2_time_on(150);
        message.set_pwm_pump2_time_off(2500);
        message.set_sb_low_cl(0);
        message.set_sb_caution_low_cl(0);
        message.set_sb_caution_high_cl(0);
        message.set_sb_high_cl(0);
        message.set_sb_low_ph(0);
        message.set_sb_caution_low_ph(0);
        message.set_sb_caution_high_ph(0);
        message.set_sb_high_ph(0);
        message
    }

    fn default_message_peak() -> proto::Peak::Peak {
        let mut message = proto::Peak::Peak::new();
        message.set_peaknum(1);
        message.set_peakstart1(11);
        message.set_peakend1(17);
        message.set_peakstart2(0);
        message.set_peakend2(0);
        message.set_midpeaknum(2);
        message.set_midpeakstart1(7);
        message.set_midpeakend1(11);
        message.set_midpeakstart2(17);
        message.set_midpeakend2(19);
        message.set_offpeakstart(19);
        message.set_offpeakend(7);
        message.set_offset(5);
        message.set_peakheater(false);
        message.set_peakfilter(false);
        message.set_peakozone(false);
        message.set_midpeakheater(false);
        message.set_midpeakfilter(true);
        message.set_midpeakozone(true);
        message.set_sat(false);
        message.set_sun(false);
        message.set_mon(false);
        message.set_tue(false);
        message.set_wed(false);
        message.set_thu(false);
        message.set_fri(false);
        message
    }

    fn default_message_peripheral() -> proto::Peripheral::Peripheral {
        let mut message = proto::Peripheral::Peripheral::new();
        message.set_guid(String::from("x67a2o5g-f6a0-5h9p-71xx-3f2b46g4c980"));
        message.set_hardware_version(50593792);
        message.set_firmware_version(51249155);
        message.set_product_code(proto::Peripheral::peripheral::PeripheralProductCode::PERIPHERAL_PRODUCT_CODE_ONZEN);
        message.set_connected(true);
        message
    }

    fn default_message_router() -> proto::Router::Router {
        let mut message = proto::Router::Router::new();
        message.set_ssid(String::from("BELL 123"));
        message.set_password(String::from("password"));
        message.set_encryption(proto::Router::router::Encryption::ENCRYPTION_WPA2);
        message.set_protocol(proto::Router::router::Protocol::PROTOCOL_AES);
        message
    }

    fn default_message_settings() -> proto::Settings::Settings {
        let mut message = proto::Settings::Settings::new();
        message.set_max_filtration_frequency(4);
        message.set_min_filtration_frequency(0);
        message.set_filtration_frequency(4);
        message.set_max_filtration_duration(8);
        message.set_min_filtration_duration(0);
        message.set_filtration_duration(1);
        message.set_max_onzen_hours(24);
        message.set_min_onzen_hours(0);
        message.set_onzen_hours(1);
        message.set_max_onzen_cycles(24);
        message.set_min_onzen_cycles(0);
        message.set_onzen_cycles(1);
        message.set_max_ozone_hours(24);
        message.set_min_ozone_hours(0);
        message.set_ozone_hours(0);
        message.set_max_ozone_cycles(24);
        message.set_min_ozone_cycles(0);
        message.set_ozone_cycles(1);
        message.set_filter_suspension(false);
        message.set_flash_lights_on_error(false);
        message.set_temperature_offset(0);
        message.set_sauna_duration(3600);
        message.set_min_temperature(59);
        message.set_max_temperature(104);
        message.set_filtration_offset(0);
        message.set_spaboy_hours(0);
        message
    }

    fn update_message_clock(&mut self) -> () {
        let datetime: DateTime<Utc> = SystemTime::now().into();

        self.clock.set_year(datetime.year() as i32);
        self.clock.set_month(datetime.month() as i32);
        self.clock.set_day(datetime.day() as i32);
        self.clock.set_hour(datetime.hour() as i32);
        self.clock.set_minute(datetime.minute() as i32);
        self.clock.set_second(datetime.second() as i32);
    }

    fn clear_all_errors(&mut self) {
        self.error.set_no_flow(false);
        self.error.set_flow_switch(false);
        self.error.set_heater_over_temperature(false);
        self.error.set_spa_over_temperature(false);
        self.error.set_spa_temperature_probe(false);
        self.error.set_spa_high_limit(false);
        self.error.set_eeprom(false);
        self.error.set_freeze_protect(false);
        self.error.set_ph_high(false);
        self.error.set_heater_probe_disconnected(false);
    }

    fn update_message_error(&mut self) {
        // Session sequence:
        // 1) first request => all errors off
        // 2) next requests => turn on one distinct error at a time
        // 3) after each error has been on once => keep all errors off
        const ERROR_COUNT: usize = 10;

        self.clear_all_errors();

        if self.error_sequence_step == 0 {
            self.error_sequence_step += 1;
            return;
        }

        let error_index = self.error_sequence_step - 1;
        match error_index {
            0 => self.error.set_no_flow(true),
            1 => self.error.set_flow_switch(true),
            2 => self.error.set_heater_over_temperature(true),
            3 => self.error.set_spa_over_temperature(true),
            4 => self.error.set_spa_temperature_probe(true),
            5 => self.error.set_spa_high_limit(true),
            6 => self.error.set_eeprom(true),
            7 => self.error.set_freeze_protect(true),
            8 => self.error.set_ph_high(true),
            9 => self.error.set_heater_probe_disconnected(true),
            _ => {}
        }

        if self.error_sequence_step <= ERROR_COUNT {
            self.error_sequence_step += 1;
        }
    }

    fn update_message_onzen_live(&mut self) {
        const MIN_ORP: i32 = 500;
        const MAX_ORP: i32 = 850;
        const MIN_PH_100: i32 = 689;
        const MAX_PH_100: i32 = 780;

        let current_orp = self.onzen_live.orp();
        let current_ph_100 = self.onzen_live.ph_100();

        let orp_delta = random_delta(current_orp as u64 ^ 0xA55A_A55A, 12);
        let ph_100_delta = random_delta(current_ph_100 as u64 ^ 0x5AA5_5AA5, 8);

        let next_orp = (current_orp + orp_delta).clamp(MIN_ORP, MAX_ORP);
        let next_ph_100 = (current_ph_100 + ph_100_delta).clamp(MIN_PH_100, MAX_PH_100);

        self.onzen_live.set_orp(next_orp);
        self.onzen_live.set_ph_100(next_ph_100);
    }

    fn new() -> Self {
        Self {
            clock: MockState::default_message_clock(),
            configuration: MockState::default_message_configuration(),
            error: MockState::default_message_error(),
            filter: MockState::default_message_filter(),
            information: MockState::default_message_information(),
            live: MockState::default_message_live(),
            onzen_live: MockState::default_message_onzen_live(),
            onzen_settings: MockState::default_message_onzen_settings(),
            peak: MockState::default_message_peak(),
            peripheral: MockState::default_message_peripheral(),
            router: MockState::default_message_router(),
            settings: MockState::default_message_settings(),
            error_sequence_step: 0,

            response_queue: VecDeque::new(),
        }
    }

    fn command_pump_to_live(value: proto::Command::command::SetPumpStatus) -> proto::Live::live::PumpStatus {
        match value {
            proto::Command::command::SetPumpStatus::PUMP_OFF => proto::Live::live::PumpStatus::PUMP_OFF,
            proto::Command::command::SetPumpStatus::PUMP_LOW => proto::Live::live::PumpStatus::PUMP_LOW,
            proto::Command::command::SetPumpStatus::PUMP_HIGH => proto::Live::live::PumpStatus::PUMP_HIGH,
        }
    }

    fn command_sauna_to_live(value: proto::Command::command::SetSaunaState) -> proto::Live::live::SaunaStatus {
        match value {
            proto::Command::command::SetSaunaState::SAUNA_IDLE => proto::Live::live::SaunaStatus::SAUNA_NORMAL,
            proto::Command::command::SetSaunaState::SAUNA_PRESET_A => proto::Live::live::SaunaStatus::SAUNA_PRESET_A,
            proto::Command::command::SetSaunaState::SAUNA_PRESET_B => proto::Live::live::SaunaStatus::SAUNA_PRESET_B,
            proto::Command::command::SetSaunaState::SAUNA_PRESET_C => proto::Live::live::SaunaStatus::SAUNA_PRESET_C,
            proto::Command::command::SetSaunaState::SAUNA_TIMER => proto::Live::live::SaunaStatus::SAUNA_NORMAL,
        }
    }

    fn apply_command(&mut self, command: &proto::Command::Command) -> Result<(), Error> {
        // log::debug!("applying command={:?}", command);
        if command.has_set_temperature_setpoint_fahrenheit() {
            self.live
                .set_temperature_setpoint_fahrenheit(command.set_temperature_setpoint_fahrenheit());
        }
        if command.has_set_pump_1() {
            self.live
                .set_pump_1(MockState::command_pump_to_live(command.set_pump_1()));
        }
        if command.has_set_pump_2() {
            self.live
                .set_pump_2(MockState::command_pump_to_live(command.set_pump_2()));
        }
        if command.has_set_pump_3() {
            self.live
                .set_pump_3(MockState::command_pump_to_live(command.set_pump_3()));
        }
        if command.has_set_pump_4() {
            self.live
                .set_pump_4(MockState::command_pump_to_live(command.set_pump_4()));
        }
        if command.has_set_pump_5() {
            self.live
                .set_pump_5(MockState::command_pump_to_live(command.set_pump_5()));
        }
        if command.has_set_blower_1() {
            self.live
                .set_blower_1(MockState::command_pump_to_live(command.set_blower_1()));
        }
        if command.has_set_blower_2() {
            self.live
                .set_blower_2(MockState::command_pump_to_live(command.set_blower_2()));
        }
        if command.has_set_lights() {
            self.live.set_lights(command.set_lights());
        }
        if command.has_set_stereo() {
            self.live.set_stereo(command.set_stereo());
        }
        if command.has_set_filter() {
            self.live.set_filter(if command.set_filter() {
                proto::Live::live::FilterStatus::FILTER_FILTERING
            } else {
                proto::Live::live::FilterStatus::FILTER_IDLE
            });
        }
        if command.has_set_onzen() {
            self.live.set_onzen(command.set_onzen());
        }
        if command.has_set_ozone() {
            self.live.set_ozone(if command.set_ozone() {
                proto::Live::live::OzoneStatus::OZONE_ACTIVE
            } else {
                proto::Live::live::OzoneStatus::OZONE_IDLE
            });
        }
        if command.has_set_exhaust_fan() {
            self.live.set_exhaust_fan(command.set_exhaust_fan());
        }
        if command.has_set_sauna_state() {
            self.live
                .set_sauna(MockState::command_sauna_to_live(command.set_sauna_state()));
        }
        if command.has_set_sauna_time_left() {
            self.live.set_sauna_time_remaining(command.set_sauna_time_left());
        }
        if command.has_set_all_on() {
            self.live.set_all_on(command.set_all_on());
        }
        if command.has_set_fogger() {
            self.live.set_fogger(command.set_fogger());
        }
        if command.has_set_sds() {
            self.live.set_sds(command.set_sds());
        }
        if command.has_set_yess() {
            self.live.set_yess(command.set_yess());
        }

        self.queue_response(MessageType::Live)?;
        Ok(())
    }

    fn queue_response(&mut self, message_type: MessageType) -> Result<(), Error> {
        let payload = self.response_payload(message_type)?;
        // queue response is there is no payload or if it's a heartbeat
        if !payload.is_empty() || message_type == MessageType::Heartbeat {
            self.response_queue.push_back((message_type, payload));
        }
        Ok(())
    }

    fn dequeue_response(&mut self) -> Option<(MessageType, Vec<u8>)> {
        self.response_queue.pop_front()
    }

    fn response_payload(&mut self, message_type: MessageType) -> Result<Vec<u8>, Error> {
        match message_type {
            MessageType::Clock => {
                self.update_message_clock();
                serialize_proto_message(&self.clock)
            }
            MessageType::Configuration => serialize_proto_message(&self.configuration),
            MessageType::Error => {
                self.update_message_error();
                serialize_proto_message(&self.error)
            }
            MessageType::Filter => serialize_proto_message(&self.filter),
            MessageType::Information => serialize_proto_message(&self.information),
            MessageType::Live => serialize_proto_message(&self.live),
            MessageType::OnzenLive => {
                self.update_message_onzen_live();
                serialize_proto_message(&self.onzen_live)
            }
            MessageType::OnzenSettings => serialize_proto_message(&self.onzen_settings),
            MessageType::Peak => serialize_proto_message(&self.peak),
            MessageType::Peripheral => serialize_proto_message(&self.peripheral),
            MessageType::Router => serialize_proto_message(&self.router),
            MessageType::Settings => serialize_proto_message(&self.settings),
            MessageType::Command | MessageType::Heartbeat => Ok(vec![]),
        }
    }
}

fn checksum_for_packet(data: &[u8]) -> [u8; 4] {
    const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    CRC32.checksum(data).to_be_bytes()
}

fn packet_bytes(message_type: MessageType, payload: Vec<u8>) -> Vec<u8> {
    let mut packet = Vec::with_capacity(HEADER_SIZE + payload.len());

    packet.extend_from_slice(&HEADER_PREAMBLE);
    packet.extend_from_slice(&0u32.to_be_bytes());
    packet.extend_from_slice(&0u32.to_be_bytes());
    packet.extend_from_slice(&0u32.to_be_bytes());
    packet.extend_from_slice(&u16::from(message_type).to_be_bytes());
    packet.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    packet.extend_from_slice(&payload);

    let checksum = checksum_for_packet(&packet);
    packet[4..8].copy_from_slice(&checksum);
    packet
}

fn read_packet(stream: &mut TcpStream) -> Result<(MessageType, Vec<u8>), Error> {
    let mut header = [0u8; HEADER_SIZE];
    stream.read_exact(&mut header)?;

    if header[0..4] != HEADER_PREAMBLE {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("invalid packet preamble: {:?}", &header[0..4]),
        ));
    }

    let message_type_value = u16::from_be_bytes([header[16], header[17]]);
    let payload_size = u16::from_be_bytes([header[18], header[19]]) as usize;

    let mut payload = vec![0u8; payload_size];
    stream.read_exact(&mut payload)?;

    let message_type = MessageType::try_from(message_type_value)?;
    log::debug!(
        "received packet: message_type_value={:?}, message_type={:?}, payload_size={}",
        message_type_value,
        message_type,
        payload_size
    );
    Ok((message_type, payload))
}

fn write_packet(stream: &mut TcpStream, message_type: MessageType, payload: Vec<u8>) -> Result<(), Error> {
    let bytes = packet_bytes(message_type, payload);
    stream.write_all(&bytes)?;
    log::debug!(
        "sent packet: message_type={:?}, payload_size={}",
        message_type,
        bytes.len() - HEADER_SIZE
    );
    Ok(())
}

fn handle_client(mut stream: TcpStream, state: Arc<Mutex<MockState>>) -> Result<(), Error> {
    stream.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT_MS)))?;

    loop {
        let (message_type, payload) = match read_packet(&mut stream) {
            Ok(packet) => packet,
            Err(err) if err.kind() == ErrorKind::TimedOut => {
                log::debug!("read timed out ({} ms), sending Heartbeat...", READ_TIMEOUT_MS);
                (MessageType::Heartbeat, vec![])
            }
            Err(err) if err.kind() == ErrorKind::UnexpectedEof => {
                log::debug!("mock client disconnected");
                return Ok(());
            }
            Err(err) => {
                log::warn!("mock server packet read failed: {}", err);
                return Err(err);
            }
        };

        if message_type == MessageType::Command && !payload.is_empty() {
            let command = proto::Command::Command::parse_from_bytes(&payload).map_err(proto_error_to_io)?;
            let mut guard = state.lock().map_err(|_| Error::other("failed to lock mock state"))?;
            guard.apply_command(&command)?;
            continue;
        }

        if payload.is_empty() {
            let responses_to_send = {
                let mut guard = state.lock().map_err(|_| Error::other("failed to lock mock state"))?;

                // dequeue all queued responses first
                let mut responses = Vec::new();
                while let Some((queued_type, queued_payload)) = guard.dequeue_response() {
                    responses.push((queued_type, queued_payload));
                }

                // always append the requested message type at the end
                let payload = guard.response_payload(message_type)?;
                if !payload.is_empty() {
                    responses.push((message_type, payload));
                } else {
                    // no payload for requested type, send as Heartbeat
                    responses.push((MessageType::Heartbeat, vec![]));
                }

                responses
            };

            log::debug!("sending {} response(s) to client...", responses_to_send.len());
            for (response_type, response_payload) in responses_to_send {
                // only send responses with data or Heartbeats
                if !response_payload.is_empty() || response_type == MessageType::Heartbeat {
                    write_packet(&mut stream, response_type, response_payload)?;
                }
            }
            continue;
        }

        log::debug!(
            "ignoring unsupported packet: message_type={:?}, payload_size={}",
            message_type,
            payload.len()
        );
    }
}

pub fn run(bind_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(bind_address)?;
    let state = Arc::new(Mutex::new(MockState::new()));

    log::info!("mock server listening on {}", bind_address);
    println!("mock server listening on {}", bind_address);

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                let peer = stream.peer_addr().ok();
                log::info!("mock client connected: {:?}", peer);
                let state_clone = Arc::clone(&state);
                std::thread::spawn(move || {
                    if let Err(error) = handle_client(stream, state_clone) {
                        log::warn!("mock client session ended with error: {}", error);
                    }
                });
            }
            Err(error) => {
                log::warn!("mock server accept error: {}", error);
            }
        }
    }

    Ok(())
}
