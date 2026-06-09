#![allow(dead_code)]
#![allow(unused_imports)]


use std::time::SystemTime;


use crate::proto;
use crate::core::db;
use crate::core::net::{MessageType, ProtoMessage, NetworkClient};


fn test_message_live(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Live::Live::new();
    msg.set_temperature_fahrenheit(102);
    msg.set_temperature_setpoint_fahrenheit(104);
    msg.set_pump_1(proto::Live::live::PumpStatus::PUMP_LOW);
    msg.set_pump_2(proto::Live::live::PumpStatus::PUMP_HIGH);
    msg.set_pump_3(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_pump_4(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_pump_5(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_blower_1(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_blower_2(proto::Live::live::PumpStatus::PUMP_OFF);
    msg.set_lights(false);
    msg.set_stereo(false);
    msg.set_heater_1(proto::Live::live::HeaterStatus::HEATER_HEATING);
    msg.set_heater_2(proto::Live::live::HeaterStatus::HEATER_IDLE);
    msg.set_filter(proto::Live::live::FilterStatus::FILTER_IDLE);
    msg.set_onzen(true);
    msg.set_ozone(proto::Live::live::OzoneStatus::OZONE_ACTIVE);
    msg.set_exhaust_fan(false);
    msg.set_sauna(proto::Live::live::SaunaStatus::SAUNA_NORMAL);
    msg.set_heater_adc(20);
    msg.set_sauna_time_remaining(0);
    msg.set_economy(false);
    msg.set_current_adc(0);
    msg.set_all_on(false);
    msg.set_fogger(false);
    msg.set_error(0);
    msg.set_alarm(24);
    msg.set_status(67);
    msg.set_ph(712);
    msg.set_orp(650);
    msg.set_sds(false);
    msg.set_yess(false);

    let msg_wrapped = ProtoMessage::Live {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_live(&msg_wrapped) {
        log::error!("failed to insert live data: {}", e);
        return;
    }
}

fn test_message_clock(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Clock::Clock::new();
    msg.set_year(2026);
    msg.set_month(6);
    msg.set_day(9);
    msg.set_hour(12);
    msg.set_minute(0);
    msg.set_second(0);

    let msg_wrapped = ProtoMessage::Clock {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_clock(&msg_wrapped) {
        log::error!("failed to insert clock data: {}", e);
    }
}

fn test_message_configuration(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Configuration::Configuration::new();
    msg.set_pump1(true);
    msg.set_pump2(true);
    msg.set_pump3(false);
    msg.set_pump4(false);
    msg.set_pump5(false);
    msg.set_blower1(true);
    msg.set_blower2(false);
    msg.set_lights(true);
    msg.set_stereo(false);
    msg.set_heater1(true);
    msg.set_heater2(true);
    msg.set_filter(true);
    msg.set_onzen(true);
    msg.set_ozone_peak_1(false);
    msg.set_ozone_peak_2(false);
    msg.set_exhaust_fan(false);
    msg.set_powerlines(proto::Configuration::configuration::Phase::PHASE_SINGLE);
    msg.set_breaker_size(50);
    msg.set_smart_onzen(0);
    msg.set_fogger(false);
    msg.set_sds(false);
    msg.set_yess(false);

    let msg_wrapped = ProtoMessage::Configuration {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_configuration(&msg_wrapped) {
        log::error!("failed to insert configuration data: {}", e);
    }
}

fn test_message_error(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Error::Error::new();
    msg.set_no_flow(false);
    msg.set_flow_switch(false);
    msg.set_heater_over_temperature(false);
    msg.set_spa_over_temperature(false);
    msg.set_spa_temperature_probe(false);
    msg.set_spa_high_limit(false);
    msg.set_eeprom(false);
    msg.set_freeze_protect(false);
    msg.set_ph_high(false);
    msg.set_heater_probe_disconnected(false);

    let msg_wrapped = ProtoMessage::Error {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_error(&msg_wrapped) {
        log::error!("failed to insert error data: {}", e);
    }
}

fn test_message_filter(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Filter::Filter::new();
    msg.set_serial_nums("SN001".to_string());
    msg.set_filter_state(proto::Filter::filter::FilterState::FILTER_PURCHASE);
    msg.set_install_dates("2025-01-01".to_string());

    let msg_wrapped = ProtoMessage::Filter {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_filter(&msg_wrapped) {
        log::error!("failed to insert filter data: {}", e);
    }
}

fn test_message_information(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Information::Information::new();
    msg.set_pack_serial_number("PACK-001".to_string());
    msg.set_pack_firmware_version("1.0.0".to_string());
    msg.set_pack_hardware_version("1.0".to_string());
    msg.set_pack_product_id("PROD-001".to_string());
    msg.set_pack_board_id("BOARD-001".to_string());
    msg.set_topside_product_id("TOP-001".to_string());
    msg.set_topside_software_version("1.0.0".to_string());
    msg.set_guid("00000000-0000-0000-0000-000000000000".to_string());
    msg.set_spa_type(proto::Information::information::SpaType::SPA_TYPE_HOT_TUB);
    msg.set_website_registration(false);
    msg.set_website_registration_confirm(false);
    msg.set_mac_address(vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    msg.set_firmware_version(100);
    msg.set_product_code(1);
    msg.set_var_software_version("1.0.0".to_string());
    msg.set_spaboy_firmware_version("".to_string());
    msg.set_spaboy_hardware_version("".to_string());
    msg.set_spaboy_product_id("".to_string());
    msg.set_spaboy_serial_number("".to_string());
    msg.set_rfid_firmware_version("".to_string());
    msg.set_rfid_hardware_version("".to_string());
    msg.set_rfid_product_id("".to_string());
    msg.set_rfid_serial_number("".to_string());

    let msg_wrapped = ProtoMessage::Information {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_information(&msg_wrapped) {
        log::error!("failed to insert information data: {}", e);
    }
}

fn test_message_onzen_live(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::OnzenLive::OnzenLive::new();
    msg.set_guid("00000000-0000-0000-0000-000000000000".to_string());
    msg.set_orp(650);
    msg.set_ph_100(712);
    msg.set_current(5);
    msg.set_voltage(120);
    msg.set_current_setpoint(5);
    msg.set_voltage_setpoint(120);
    msg.set_pump1(true);
    msg.set_pump2(false);
    msg.set_orp_state_machine(0);
    msg.set_electrode_state_machine(0);
    msg.set_electrode_id(1);
    msg.set_electrode_polarity(proto::OnzenLive::onzen_live::Polarity::POLARITY_POSITIVE);
    msg.set_electrode_1_resistance_1(0);
    msg.set_electrode_1_resistance_2(0);
    msg.set_electrode_2_resistance_1(0);
    msg.set_electrode_2_resistance_2(0);
    msg.set_command_mode(false);
    msg.set_electrode_mAH(0);
    msg.set_ph_color(proto::OnzenLive::onzen_live::Color::COLOR_OK);
    msg.set_orp_color(proto::OnzenLive::onzen_live::Color::COLOR_OK);
    msg.set_electrode_wear(0);

    let msg_wrapped = ProtoMessage::OnzenLive {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_onzen_live(&msg_wrapped) {
        log::error!("failed to insert onzen live data: {}", e);
    }
}

fn test_message_onzen_settings(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::OnzenSettings::OnzenSettings::new();
    msg.set_guid("00000000-0000-0000-0000-000000000000".to_string());
    msg.set_over_voltage(240);
    msg.set_under_voltage(100);
    msg.set_over_current(15);
    msg.set_under_current(1);
    msg.set_orp_high(800);
    msg.set_orp_low(200);
    msg.set_ph_high(800);
    msg.set_ph_low(600);
    msg.set_pwm_pump1_time_on(30);
    msg.set_pwm_pump1_time_off(30);
    msg.set_sampling_interval(60);
    msg.set_sampling_duration(10);
    msg.set_pwm_pump2_time_on(30);
    msg.set_pwm_pump2_time_off(30);
    msg.set_sb_low_cl(100);
    msg.set_sb_caution_low_cl(200);
    msg.set_sb_caution_high_cl(600);
    msg.set_sb_high_cl(700);
    msg.set_sb_low_ph(650);
    msg.set_sb_caution_low_ph(680);
    msg.set_sb_caution_high_ph(780);
    msg.set_sb_high_ph(800);

    let msg_wrapped = ProtoMessage::OnzenSettings {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_onzen_settings(&msg_wrapped) {
        log::error!("failed to insert onzen settings data: {}", e);
    }
}

fn test_message_peak(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Peak::Peak::new();
    msg.set_peaknum(1);
    msg.set_peakstart1(7);
    msg.set_peakend1(11);
    msg.set_peakstart2(17);
    msg.set_peakend2(21);
    msg.set_midpeaknum(0);
    msg.set_midpeakstart1(0);
    msg.set_midpeakend1(0);
    msg.set_midpeakstart2(0);
    msg.set_midpeakend2(0);
    msg.set_offpeakstart(22);
    msg.set_offpeakend(6);
    msg.set_offset(0);
    msg.set_peakheater(true);
    msg.set_peakfilter(false);
    msg.set_peakozone(false);
    msg.set_midpeakheater(false);
    msg.set_midpeakfilter(false);
    msg.set_midpeakozone(false);
    msg.set_sat(true);
    msg.set_sun(true);
    msg.set_mon(true);
    msg.set_tue(true);
    msg.set_wed(true);
    msg.set_thu(true);
    msg.set_fri(true);

    let msg_wrapped = ProtoMessage::Peak {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_peak(&msg_wrapped) {
        log::error!("failed to insert peak data: {}", e);
    }
}

fn test_message_peripheral(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Peripheral::Peripheral::new();
    msg.set_guid("00000000-0000-0000-0000-000000000000".to_string());
    msg.set_hardware_version(1);
    msg.set_firmware_version(100);
    msg.set_product_code(proto::Peripheral::peripheral::PeripheralProductCode::PERIPHERAL_PRODUCT_CODE_ONZEN);
    msg.set_connected(true);

    let msg_wrapped = ProtoMessage::Peripheral {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_peripheral(&msg_wrapped) {
        log::error!("failed to insert peripheral data: {}", e);
    }
}

fn test_message_settings(db_client: &mut db::DatabaseClient) -> () {
    let mut msg = proto::Settings::Settings::new();
    msg.set_max_filtration_frequency(24);
    msg.set_min_filtration_frequency(1);
    msg.set_filtration_frequency(4);
    msg.set_max_filtration_duration(240);
    msg.set_min_filtration_duration(30);
    msg.set_filtration_duration(120);
    msg.set_max_onzen_hours(24);
    msg.set_min_onzen_hours(1);
    msg.set_onzen_hours(8);
    msg.set_max_onzen_cycles(10);
    msg.set_min_onzen_cycles(1);
    msg.set_onzen_cycles(4);
    msg.set_max_ozone_hours(24);
    msg.set_min_ozone_hours(1);
    msg.set_ozone_hours(8);
    msg.set_max_ozone_cycles(10);
    msg.set_min_ozone_cycles(1);
    msg.set_ozone_cycles(4);
    msg.set_filter_suspension(false);
    msg.set_flash_lights_on_error(true);
    msg.set_temperature_offset(0);
    msg.set_sauna_duration(30);
    msg.set_min_temperature(60);
    msg.set_max_temperature(104);
    msg.set_filtration_offset(0);
    msg.set_spaboy_hours(0);

    let msg_wrapped = ProtoMessage::Settings {
        message: msg,
        received_at: SystemTime::now(),
    };

    if let Err(e) = db_client.insert_message_settings(&msg_wrapped) {
        log::error!("failed to insert settings data: {}", e);
    }
}

pub fn poll_device(ip_address: &str) -> () {
    log::info!("starting polling loop: ip_address={:}", ip_address);

    let mut db_client = match db::DatabaseClient::open(None) {
        Ok(client) => client,
        Err(e) => {
            log::error!("failed to initialize database client: {}", e);
            return;
        }
    };

    if let Err(e) = db_client.create_connection_session(ip_address) {
        log::error!("failed to create connection session: {}", e);
        return;
    }

    test_message_clock(&mut db_client);
    test_message_configuration(&mut db_client);
    test_message_error(&mut db_client);
    test_message_filter(&mut db_client);
    test_message_information(&mut db_client);
    test_message_live(&mut db_client);
    test_message_onzen_live(&mut db_client);
    test_message_onzen_settings(&mut db_client);
    test_message_peak(&mut db_client);
    test_message_peripheral(&mut db_client);
    test_message_settings(&mut db_client);
}
