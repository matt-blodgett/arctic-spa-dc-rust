use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

fn test_db_path(suffix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX_EPOCH")
        .as_nanos();
    env::temp_dir().join(format!("asdc-tests-{}-{}.sqlite", suffix, nanos))
}

fn remove_db_file(path: &PathBuf) {
    let _ = fs::remove_file(path);
}

fn count_rows(client: &DatabaseClient, table: &str) -> i64 {
    let sql = format!("SELECT COUNT(*) FROM {}", table);
    client
        .conn
        .query_row(&sql, [], |row| row.get::<usize, i64>(0))
        .expect("count query should succeed")
}

fn default_message_clock() -> proto::Clock::Clock {
    let mut message = proto::Clock::Clock::new();
    message.set_year(2026);
    message.set_month(6);
    message.set_day(13);
    message.set_hour(8);
    message.set_minute(30);
    message.set_second(45);
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

#[test]
fn open_initializes_schema_and_process_run() {
    let path = test_db_path("open-initializes");

    let client = DatabaseClient::open(Some(&path), true).expect("database open should succeed");

    assert_eq!(count_rows(&client, "process_run"), 1);
    assert_eq!(count_rows(&client, "connection_session"), 0);

    let has_message_live: i64 = client
        .conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='message_live'",
            [],
            |row| row.get(0),
        )
        .expect("sqlite_master query should succeed");
    assert_eq!(has_message_live, 1);

    drop(client);
    remove_db_file(&path);
}

#[test]
fn insert_message_requires_connection_session() {
    let path = test_db_path("requires-session");

    let client = DatabaseClient::open(Some(&path), true).expect("database open should succeed");

    let mut clock = proto::Clock::Clock::new();
    clock.set_year(2026);

    let message = ProtoMessage::Clock {
        message: clock,
        received_at: SystemTime::now(),
    };

    let result = client.insert_message(&message);
    assert!(matches!(result, Err(rusqlite::Error::InvalidQuery)));

    drop(client);
    remove_db_file(&path);
}

#[test]
fn insert_message_clock_persists_payload() {
    let path = test_db_path("clock-insert");

    let mut client = DatabaseClient::open(Some(&path), true).expect("database open should succeed");
    client
        .create_connection_session("127.0.0.1")
        .expect("creating connection session should succeed");

    let mut clock = proto::Clock::Clock::new();
    clock.set_year(2026);
    clock.set_month(6);
    clock.set_day(13);
    clock.set_hour(8);
    clock.set_minute(30);
    clock.set_second(45);

    let message = ProtoMessage::Clock {
        message: clock,
        received_at: SystemTime::now(),
    };

    client.insert_message(&message).expect("clock insert should succeed");

    let row: (i32, i32, i32, i32, i32, i32) = client
        .conn
        .query_row(
            "SELECT year, month, day, hour, minute, second FROM message_clock ORDER BY id DESC LIMIT 1",
            [],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )
        .expect("message_clock row should exist");

    assert_eq!(row, (2026, 6, 13, 8, 30, 45));

    drop(client);
    remove_db_file(&path);
}

#[test]
fn insert_message_skips_command_and_router() {
    let path = test_db_path("unsupported-routes");

    let mut client = DatabaseClient::open(Some(&path), true).expect("database open should succeed");
    client
        .create_connection_session("127.0.0.1")
        .expect("creating connection session should succeed");

    let command_message = ProtoMessage::Command {
        message: proto::Command::Command::new(),
        received_at: SystemTime::now(),
    };
    let router_message = ProtoMessage::Router {
        message: proto::Router::Router::new(),
        received_at: SystemTime::now(),
    };

    assert!(client.insert_message(&command_message).is_ok());
    assert!(client.insert_message(&router_message).is_ok());
    assert_eq!(count_rows(&client, "message_clock"), 0);
    assert_eq!(count_rows(&client, "message_live"), 0);
    assert_eq!(count_rows(&client, "message_settings"), 0);

    drop(client);
    remove_db_file(&path);
}

#[test]
fn workflow_creates_run_session_and_inserts_all_message_types() {
    let path = test_db_path("full-workflow");

    let mut client = DatabaseClient::open(Some(&path), true).expect("database open should succeed");
    client
        .create_connection_session("127.0.0.1")
        .expect("creating connection session should succeed");

    assert_eq!(count_rows(&client, "process_run"), 1);
    assert_eq!(count_rows(&client, "connection_session"), 1);

    let now = SystemTime::now();

    let clock_message = ProtoMessage::Clock {
        message: default_message_clock(),
        received_at: now,
    };
    let configuration_message = ProtoMessage::Configuration {
        message: default_message_configuration(),
        received_at: now,
    };
    let error_message = ProtoMessage::Error {
        message: default_message_error(),
        received_at: now,
    };
    let filter_message = ProtoMessage::Filter {
        message: default_message_filter(),
        received_at: now,
    };
    let information_message = ProtoMessage::Information {
        message: default_message_information(),
        received_at: now,
    };
    let live_message = ProtoMessage::Live {
        message: default_message_live(),
        received_at: now,
    };
    let onzen_live_message = ProtoMessage::OnzenLive {
        message: default_message_onzen_live(),
        received_at: now,
    };
    let onzen_settings_message = ProtoMessage::OnzenSettings {
        message: default_message_onzen_settings(),
        received_at: now,
    };
    let peak_message = ProtoMessage::Peak {
        message: default_message_peak(),
        received_at: now,
    };
    let peripheral_message = ProtoMessage::Peripheral {
        message: default_message_peripheral(),
        received_at: now,
    };
    let settings_message = ProtoMessage::Settings {
        message: default_message_settings(),
        received_at: now,
    };

    client
        .insert_message(&clock_message)
        .expect("clock insert should succeed");
    client
        .insert_message(&configuration_message)
        .expect("configuration insert should succeed");
    client
        .insert_message(&error_message)
        .expect("error insert should succeed");
    client
        .insert_message(&filter_message)
        .expect("filter insert should succeed");
    client
        .insert_message(&information_message)
        .expect("information insert should succeed");
    client
        .insert_message(&live_message)
        .expect("live insert should succeed");
    client
        .insert_message(&onzen_live_message)
        .expect("onzen live insert should succeed");
    client
        .insert_message(&onzen_settings_message)
        .expect("onzen settings insert should succeed");
    client
        .insert_message(&peak_message)
        .expect("peak insert should succeed");
    client
        .insert_message(&peripheral_message)
        .expect("peripheral insert should succeed");
    client
        .insert_message(&settings_message)
        .expect("settings insert should succeed");

    let command_message = ProtoMessage::Command {
        message: proto::Command::Command::new(),
        received_at: now,
    };
    let router_message = ProtoMessage::Router {
        message: proto::Router::Router::new(),
        received_at: now,
    };
    client
        .insert_message(&command_message)
        .expect("command no-op route should succeed");
    client
        .insert_message(&router_message)
        .expect("router no-op route should succeed");

    assert_eq!(count_rows(&client, "message_clock"), 1);
    assert_eq!(count_rows(&client, "message_configuration"), 1);
    assert_eq!(count_rows(&client, "message_error"), 1);
    assert_eq!(count_rows(&client, "message_filter"), 1);
    assert_eq!(count_rows(&client, "message_information"), 1);
    assert_eq!(count_rows(&client, "message_live"), 1);
    assert_eq!(count_rows(&client, "message_onzen_live"), 1);
    assert_eq!(count_rows(&client, "message_onzen_settings"), 1);
    assert_eq!(count_rows(&client, "message_peak"), 1);
    assert_eq!(count_rows(&client, "message_peripheral"), 1);
    assert_eq!(count_rows(&client, "message_settings"), 1);
    assert_eq!(count_rows(&client, "command_request"), 0);

    drop(client);
    remove_db_file(&path);
}

#[test]
fn workflow_inserts_one_of_each_supported_message() {
    let path = test_db_path("workflow-all-supported");

    let mut client = DatabaseClient::open(Some(&path), true).expect("database open should succeed");
    client
        .create_connection_session("127.0.0.1")
        .expect("creating connection session should succeed");

    let messages = vec![
        ProtoMessage::Clock {
            message: default_message_clock(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Configuration {
            message: default_message_configuration(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Error {
            message: default_message_error(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Filter {
            message: default_message_filter(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Information {
            message: default_message_information(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Live {
            message: default_message_live(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::OnzenLive {
            message: default_message_onzen_live(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::OnzenSettings {
            message: default_message_onzen_settings(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Peak {
            message: default_message_peak(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Peripheral {
            message: default_message_peripheral(),
            received_at: SystemTime::now(),
        },
        ProtoMessage::Settings {
            message: default_message_settings(),
            received_at: SystemTime::now(),
        },
    ];

    for message in messages {
        client.insert_message(&message).expect("message insert should succeed");
    }

    assert_eq!(count_rows(&client, "process_run"), 1);
    assert_eq!(count_rows(&client, "connection_session"), 1);
    assert_eq!(count_rows(&client, "message_clock"), 1);
    assert_eq!(count_rows(&client, "message_configuration"), 1);
    assert_eq!(count_rows(&client, "message_error"), 1);
    assert_eq!(count_rows(&client, "message_filter"), 1);
    assert_eq!(count_rows(&client, "message_information"), 1);
    assert_eq!(count_rows(&client, "message_live"), 1);
    assert_eq!(count_rows(&client, "message_onzen_live"), 1);
    assert_eq!(count_rows(&client, "message_onzen_settings"), 1);
    assert_eq!(count_rows(&client, "message_peak"), 1);
    assert_eq!(count_rows(&client, "message_peripheral"), 1);
    assert_eq!(count_rows(&client, "message_settings"), 1);

    drop(client);
    remove_db_file(&path);
}
