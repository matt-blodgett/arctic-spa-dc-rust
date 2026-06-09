#![allow(dead_code)]
#![allow(unused_imports)]


use std::{fs, os::windows::process};
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use protobuf::{Enum, Message, MessageDyn};
use rusqlite::{Connection, Result, ToSql, params};


use crate::proto;
use crate::core::net::{MessageType, ProtoMessage, NetworkClient};
use crate::core::utils::{default_database_path, initialize_path};


pub struct DatabaseClient {
    path: PathBuf,
    conn: Connection,
    process_run_id: Option<i64>,
    connection_session_id: Option<i64>,
}

impl DatabaseClient {

    pub fn open(path: Option<&PathBuf>, overwrite: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = path.unwrap_or(&default_database_path()).to_path_buf();
        let is_new_file = initialize_path(&db_path)?;

        if !is_new_file && overwrite {
            log::warn!("overwriting database file {:#?}", db_path.display());
            fs::remove_file(&db_path)?;
        }

        log::info!("opening database file {:#?}", db_path.display());

        let conn = Connection::open(&db_path)?;

        let mut client = Self {
            path: db_path,
            conn,
            process_run_id: None,
            connection_session_id: None,
        };

        if is_new_file || overwrite {
            log::debug!("new database file, initializing schema");
            client.create_tables()?;
        }

        client.create_process_run()?;

        Ok(client)
    }

    fn create_tables(&mut self) -> Result<(), rusqlite::Error> {
        let sql_create_table_process_run = r#"
            CREATE TABLE "process_run" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME
            )
        "#;
        let sql_create_table_connection_session = r#"
            CREATE TABLE "connection_session" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,

                "host" TEXT,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_clock = r#"
            CREATE TABLE "message_clock" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "year" INTEGER,
                "month" INTEGER,
                "day" INTEGER,
                "hour" INTEGER,
                "minute" INTEGER,
                "second" INTEGER,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_configuration = r#"
            CREATE TABLE "message_configuration" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "pump1" BOOLEAN,
                "pump2" BOOLEAN,
                "pump3" BOOLEAN,
                "pump4" BOOLEAN,
                "pump5" BOOLEAN,
                "blower1" BOOLEAN,
                "blower2" BOOLEAN,
                "lights" BOOLEAN,
                "stereo" BOOLEAN,
                "heater1" BOOLEAN,
                "heater2" BOOLEAN,
                "filter" BOOLEAN,
                "onzen" BOOLEAN,
                "ozone_peak_1" BOOLEAN,
                "ozone_peak_2" BOOLEAN,
                "exhaust_fan" BOOLEAN,
                "powerlines" INTEGER,
                "breaker_size" INTEGER,
                "smart_onzen" INTEGER,
                "fogger" BOOLEAN,
                "sds" BOOLEAN,
                "yess" BOOLEAN,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_error = r#"
            CREATE TABLE "message_error" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "no_flow" BOOLEAN,
                "flow_switch" BOOLEAN,
                "heater_over_temperature" BOOLEAN,
                "spa_over_temperature" BOOLEAN,
                "spa_temperature_probe" BOOLEAN,
                "spa_high_limit" BOOLEAN,
                "eeprom" BOOLEAN,
                "freeze_protect" BOOLEAN,
                "ph_high" BOOLEAN,
                "heater_probe_disconnected" BOOLEAN,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_filter = r#"
            CREATE TABLE "message_filter" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "serial_nums" TEXT,
                "filter_state" INTEGER,
                "install_dates" TEXT,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_information = r#"
            CREATE TABLE "message_information" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "pack_serial_number" TEXT,
                "pack_firmware_version" TEXT,
                "pack_hardware_version" TEXT,
                "pack_product_id" TEXT,
                "pack_board_id" TEXT,
                "topside_product_id" TEXT,
                "topside_software_version" TEXT,
                "guid" TEXT,
                "spa_type" INTEGER,
                "website_registration" BOOLEAN,
                "website_registration_confirm" BOOLEAN,
                "mac_address" BLOB,
                "firmware_version" INTEGER,
                "product_code" INTEGER,
                "var_software_version" TEXT,
                "spaboy_firmware_version" TEXT,
                "spaboy_hardware_version" TEXT,
                "spaboy_product_id" TEXT,
                "spaboy_serial_number" TEXT,
                "rfid_firmware_version" TEXT,
                "rfid_hardware_version" TEXT,
                "rfid_product_id" TEXT,
                "rfid_serial_number" TEXT,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_live = r#"
            CREATE TABLE "message_live" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "temperature_fahrenheit" INTEGER,
                "temperature_setpoint_fahrenheit" INTEGER,
                "pump_1" INTEGER,
                "pump_2" INTEGER,
                "pump_3" INTEGER,
                "pump_4" INTEGER,
                "pump_5" INTEGER,
                "blower_1" INTEGER,
                "blower_2" INTEGER,
                "lights" BOOLEAN,
                "stereo" BOOLEAN,
                "heater_1" INTEGER,
                "heater_2" INTEGER,
                "filter" INTEGER,
                "onzen" BOOLEAN,
                "ozone" INTEGER,
                "exhaust_fan" BOOLEAN,
                "sauna" INTEGER,
                "heater_adc" INTEGER,
                "sauna_time_remaining" INTEGER,
                "economy" BOOLEAN,
                "current_adc" INTEGER,
                "all_on" BOOLEAN,
                "fogger" BOOLEAN,
                "error" INTEGER,
                "alarm" INTEGER,
                "status" INTEGER,
                "ph" INTEGER,
                "orp" INTEGER,
                "sds" BOOLEAN,
                "yess" BOOLEAN,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_onzen_live = r#"
            CREATE TABLE "message_onzen_live" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "guid" TEXT,
                "orp" INTEGER,
                "ph_100" INTEGER,
                "current" INTEGER,
                "voltage" INTEGER,
                "current_setpoint" INTEGER,
                "voltage_setpoint" INTEGER,
                "pump1" BOOLEAN,
                "pump2" BOOLEAN,
                "orp_state_machine" INTEGER,
                "electrode_state_machine" INTEGER,
                "electrode_id" INTEGER,
                "electrode_polarity" INTEGER,
                "electrode_1_resistance_1" INTEGER,
                "electrode_1_resistance_2" INTEGER,
                "electrode_2_resistance_1" INTEGER,
                "electrode_2_resistance_2" INTEGER,
                "command_mode" BOOLEAN,
                "electrode_mah" INTEGER,
                "ph_color" INTEGER,
                "orp_color" INTEGER,
                "electrode_wear" INTEGER,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_onzen_settings = r#"
            CREATE TABLE "message_onzen_settings" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "guid" TEXT,
                "over_voltage" INTEGER,
                "under_voltage" INTEGER,
                "over_current" INTEGER,
                "under_current" INTEGER,
                "orp_high" INTEGER,
                "orp_low" INTEGER,
                "ph_high" INTEGER,
                "ph_low" INTEGER,
                "pwm_pump1_time_on" INTEGER,
                "pwm_pump1_time_off" INTEGER,
                "sampling_interval" INTEGER,
                "sampling_duration" INTEGER,
                "pwm_pump2_time_on" INTEGER,
                "pwm_pump2_time_off" INTEGER,
                "sb_low_cl" INTEGER,
                "sb_caution_low_cl" INTEGER,
                "sb_caution_high_cl" INTEGER,
                "sb_high_cl" INTEGER,
                "sb_low_ph" INTEGER,
                "sb_caution_low_ph" INTEGER,
                "sb_caution_high_ph" INTEGER,
                "sb_high_ph" INTEGER,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_peak = r#"
            CREATE TABLE "message_peak" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "peaknum" INTEGER,
                "peakstart1" INTEGER,
                "peakend1" INTEGER,
                "peakstart2" INTEGER,
                "peakend2" INTEGER,
                "midpeaknum" INTEGER,
                "midpeakstart1" INTEGER,
                "midpeakend1" INTEGER,
                "midpeakstart2" INTEGER,
                "midpeakend2" INTEGER,
                "offpeakstart" INTEGER,
                "offpeakend" INTEGER,
                "offset" INTEGER,
                "peakheater" BOOLEAN,
                "peakfilter" BOOLEAN,
                "peakozone" BOOLEAN,
                "midpeakheater" BOOLEAN,
                "midpeakfilter" BOOLEAN,
                "midpeakozone" BOOLEAN,
                "sat" BOOLEAN,
                "sun" BOOLEAN,
                "mon" BOOLEAN,
                "tue" BOOLEAN,
                "wed" BOOLEAN,
                "thu" BOOLEAN,
                "fri" BOOLEAN,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_peripheral = r#"
            CREATE TABLE "message_peripheral" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "guid" TEXT,
                "hardware_version" INTEGER,
                "firmware_version" INTEGER,
                "product_code" INTEGER,
                "connected" BOOLEAN,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_message_settings = r#"
            CREATE TABLE "message_settings" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "message_received_at" DATETIME,

                "max_filtration_frequency" INTEGER,
                "min_filtration_frequency" INTEGER,
                "filtration_frequency" INTEGER,
                "max_filtration_duration" INTEGER,
                "min_filtration_duration" INTEGER,
                "filtration_duration" INTEGER,
                "max_onzen_hours" INTEGER,
                "min_onzen_hours" INTEGER,
                "onzen_hours" INTEGER,
                "max_onzen_cycles" INTEGER,
                "min_onzen_cycles" INTEGER,
                "onzen_cycles" INTEGER,
                "max_ozone_hours" INTEGER,
                "min_ozone_hours" INTEGER,
                "ozone_hours" INTEGER,
                "max_ozone_cycles" INTEGER,
                "min_ozone_cycles" INTEGER,
                "ozone_cycles" INTEGER,
                "filter_suspension" BOOLEAN,
                "flash_lights_on_error" BOOLEAN,
                "temperature_offset" INTEGER,
                "sauna_duration" INTEGER,
                "min_temperature" INTEGER,
                "max_temperature" INTEGER,
                "filtration_offset" INTEGER,
                "spaboy_hours" INTEGER,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;
        let sql_create_table_command_request = r#"
            CREATE TABLE "command_request" (
                "id" INTEGER PRIMARY KEY,
                "created_at" DATETIME,
                "process_run_id" INTEGER,
                "connection_session_id" INTEGER,
                "command_sent_at" DATETIME,

                "name" TEXT,
                "value" TEXT,

                FOREIGN KEY ("process_run_id")
                    REFERENCES "process_run" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE

                FOREIGN KEY ("connection_session_id")
                    REFERENCES "connection_session" ("id")
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )
        "#;

        self.conn.execute(sql_create_table_process_run, [])?;
        self.conn.execute(sql_create_table_connection_session, [])?;
        self.conn.execute(sql_create_table_message_clock, [])?;
        self.conn.execute(sql_create_table_message_configuration, [])?;
        self.conn.execute(sql_create_table_message_error, [])?;
        self.conn.execute(sql_create_table_message_filter, [])?;
        self.conn.execute(sql_create_table_message_information, [])?;
        self.conn.execute(sql_create_table_message_live, [])?;
        self.conn.execute(sql_create_table_message_onzen_live, [])?;
        self.conn.execute(sql_create_table_message_onzen_settings, [])?;
        self.conn.execute(sql_create_table_message_peak, [])?;
        self.conn.execute(sql_create_table_message_peripheral, [])?;
        self.conn.execute(sql_create_table_message_settings, [])?;
        self.conn.execute(sql_create_table_command_request, [])?;

        Ok(())
    }

    fn create_process_run(&mut self) -> Result<(), rusqlite::Error> {
        let sql_insert = r#"
            INSERT INTO process_run (created_at)
            VALUES (datetime('now'))
        "#;
        self.conn.execute(sql_insert, [])?;
        let process_run_id = self.conn.last_insert_rowid();
        self.process_run_id = Some(process_run_id);
        log::debug!("created process_run: id={}", process_run_id);
        Ok(())
    }

    pub fn create_connection_session(&mut self, host: &str) -> Result<(), rusqlite::Error> {
        let sql_insert = r#"
            INSERT INTO connection_session (created_at, process_run_id, host)
            VALUES (datetime('now'), ?, ?)
        "#;
        let process_run_id = self.process_run_id.ok_or(rusqlite::Error::InvalidQuery)?;
        self.conn.execute(sql_insert, params![process_run_id, host])?;
        let connection_session_id = self.conn.last_insert_rowid();
        self.connection_session_id = Some(connection_session_id);
        log::debug!("created connection_session: id={}", connection_session_id);
        Ok(())
    }

    fn current_run_and_session_ids(&self) -> Result<(i64, i64), rusqlite::Error> {
        let process_run_id = self.process_run_id.ok_or(rusqlite::Error::InvalidQuery)?;
        let connection_session_id = self.connection_session_id.ok_or(rusqlite::Error::InvalidQuery)?;
        Ok((process_run_id, connection_session_id))
    }

    pub fn insert_message_clock(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_clock().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_clock (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "year",
                    "month",
                    "day",
                    "hour",
                    "minute",
                    "second"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.year(),
                msg.month(),
                msg.day(),
                msg.hour(),
                msg.minute(),
                msg.second()
            ],
        )?;

        log::debug!("inserted message_clock: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_configuration(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_configuration().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_configuration (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "pump1",
                    "pump2",
                    "pump3",
                    "pump4",
                    "pump5",
                    "blower1",
                    "blower2",
                    "lights",
                    "stereo",
                    "heater1",
                    "heater2",
                    "filter",
                    "onzen",
                    "ozone_peak_1",
                    "ozone_peak_2",
                    "exhaust_fan",
                    "powerlines",
                    "breaker_size",
                    "smart_onzen",
                    "fogger",
                    "sds",
                    "yess"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.pump1(),
                msg.pump2(),
                msg.pump3(),
                msg.pump4(),
                msg.pump5(),
                msg.blower1(),
                msg.blower2(),
                msg.lights(),
                msg.stereo(),
                msg.heater1(),
                msg.heater2(),
                msg.filter(),
                msg.onzen(),
                msg.ozone_peak_1(),
                msg.ozone_peak_2(),
                msg.exhaust_fan(),
                msg.powerlines().value(),
                msg.breaker_size(),
                msg.smart_onzen(),
                msg.fogger(),
                msg.sds(),
                msg.yess()
            ],
        )?;

        log::debug!("inserted message_configuration: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_error(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_error().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_error (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "no_flow",
                    "flow_switch",
                    "heater_over_temperature",
                    "spa_over_temperature",
                    "spa_temperature_probe",
                    "spa_high_limit",
                    "eeprom",
                    "freeze_protect",
                    "ph_high",
                    "heater_probe_disconnected"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.no_flow(),
                msg.flow_switch(),
                msg.heater_over_temperature(),
                msg.spa_over_temperature(),
                msg.spa_temperature_probe(),
                msg.spa_high_limit(),
                msg.eeprom(),
                msg.freeze_protect(),
                msg.ph_high(),
                msg.heater_probe_disconnected()
            ],
        )?;

        log::debug!("inserted message_error: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_filter(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_filter().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_filter (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "serial_nums",
                    "filter_state",
                    "install_dates"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.serial_nums(),
                msg.filter_state().value(),
                msg.install_dates()
            ],
        )?;

        log::debug!("inserted message_filter: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_information(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_information().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_information (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "pack_serial_number",
                    "pack_firmware_version",
                    "pack_hardware_version",
                    "pack_product_id",
                    "pack_board_id",
                    "topside_product_id",
                    "topside_software_version",
                    "guid",
                    "spa_type",
                    "website_registration",
                    "website_registration_confirm",
                    "mac_address",
                    "firmware_version",
                    "product_code",
                    "var_software_version",
                    "spaboy_firmware_version",
                    "spaboy_hardware_version",
                    "spaboy_product_id",
                    "spaboy_serial_number",
                    "rfid_firmware_version",
                    "rfid_hardware_version",
                    "rfid_product_id",
                    "rfid_serial_number"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.pack_serial_number(),
                msg.pack_firmware_version(),
                msg.pack_hardware_version(),
                msg.pack_product_id(),
                msg.pack_board_id(),
                msg.topside_product_id(),
                msg.topside_software_version(),
                msg.guid(),
                msg.spa_type().value(),
                msg.website_registration(),
                msg.website_registration_confirm(),
                msg.mac_address(),
                msg.firmware_version(),
                msg.product_code(),
                msg.var_software_version(),
                msg.spaboy_firmware_version(),
                msg.spaboy_hardware_version(),
                msg.spaboy_product_id(),
                msg.spaboy_serial_number(),
                msg.rfid_firmware_version(),
                msg.rfid_hardware_version(),
                msg.rfid_product_id(),
                msg.rfid_serial_number()
            ],
        )?;

        log::debug!("inserted message_information: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn check_message_live_changed(&self, message: &proto::Live::Live) -> Result<bool, rusqlite::Error> {
        // let msg = message.as_live().ok_or(rusqlite::Error::InvalidQuery)?;

        let msg = message;

        let last_msg: proto::Live::Live = self.conn.query_row(
            r#"
                SELECT
                    temperature_fahrenheit,
                    temperature_setpoint_fahrenheit,
                    pump_1,
                    pump_2,
                    pump_3,
                    pump_4,
                    pump_5,
                    blower_1,
                    blower_2,
                    lights,
                    stereo,
                    heater_1,
                    heater_2,
                    filter,
                    onzen,
                    ozone,
                    exhaust_fan,
                    sauna,
                    heater_adc,
                    sauna_time_remaining,
                    economy,
                    current_adc,
                    all_on,
                    fogger,
                    error,
                    alarm,
                    status,
                    ph,
                    orp,
                    sds,
                    yess
                FROM message_live
                WHERE process_run_id = ? AND connection_session_id = ?
                ORDER BY created_at DESC
                LIMIT 1
            "#,
            params![
                self.process_run_id.ok_or(rusqlite::Error::InvalidQuery)?,
                self.connection_session_id.ok_or(rusqlite::Error::InvalidQuery)?
            ],
            |row| {
                let mut msg = proto::Live::Live::new();

                msg.set_temperature_fahrenheit(row.get(0)?);
                msg.set_temperature_setpoint_fahrenheit(row.get(1)?);
                msg.set_pump_1(proto::Live::live::PumpStatus::from_i32(row.get(2)?).unwrap());
                msg.set_pump_2(proto::Live::live::PumpStatus::from_i32(row.get(3)?).unwrap());
                msg.set_pump_3(proto::Live::live::PumpStatus::from_i32(row.get(4)?).unwrap());
                msg.set_pump_4(proto::Live::live::PumpStatus::from_i32(row.get(5)?).unwrap());
                msg.set_pump_5(proto::Live::live::PumpStatus::from_i32(row.get(6)?).unwrap());
                msg.set_blower_1(proto::Live::live::PumpStatus::from_i32(row.get(7)?).unwrap());
                msg.set_blower_2(proto::Live::live::PumpStatus::from_i32(row.get(8)?).unwrap());
                msg.set_lights(row.get::<_, i32>(9)? != 0);
                msg.set_stereo(row.get::<_, i32>(10)? != 0);
                msg.set_heater_1(proto::Live::live::HeaterStatus::from_i32(row.get(11)?).unwrap());
                msg.set_heater_2(proto::Live::live::HeaterStatus::from_i32(row.get(12)?).unwrap());
                msg.set_filter(proto::Live::live::FilterStatus::from_i32(row.get(13)?).unwrap());
                msg.set_onzen(row.get::<_, i32>(14)? != 0);
                msg.set_ozone(proto::Live::live::OzoneStatus::from_i32(row.get(15)?).unwrap());
                msg.set_exhaust_fan(row.get::<_, i32>(16)? != 0);
                msg.set_sauna(proto::Live::live::SaunaStatus::from_i32(row.get(17)?).unwrap());
                msg.set_heater_adc(row.get(18)?);
                msg.set_sauna_time_remaining(row.get(19)?);
                msg.set_economy(row.get::<_, i32>(20)? != 0);
                msg.set_current_adc(row.get(21)?);
                msg.set_all_on(row.get::<_, i32>(22)? != 0);
                msg.set_fogger(row.get::<_, i32>(23)? != 0);
                msg.set_error(row.get(24)?);
                msg.set_alarm(row.get(25)?);
                msg.set_status(row.get(26)?);
                msg.set_ph(row.get(27)?);
                msg.set_orp(row.get(28)?);
                msg.set_sds(row.get(29)?);
                msg.set_yess(row.get(30)?);

                Ok(msg)
            }
        )?;

        let mut has_changed = false;

        if last_msg.temperature_fahrenheit() != msg.temperature_fahrenheit() {
            has_changed = true;
            log::debug!("temperature_fahrenheit: old={}, new={}", last_msg.temperature_fahrenheit(), msg.temperature_fahrenheit());
        }
        if last_msg.temperature_setpoint_fahrenheit() != msg.temperature_setpoint_fahrenheit() {
            has_changed = true;
            log::debug!("temperature_setpoint_fahrenheit: old={}, new={}", last_msg.temperature_setpoint_fahrenheit(), msg.temperature_setpoint_fahrenheit());
        }
        if last_msg.pump_1() != msg.pump_1() {
            has_changed = true;
            log::debug!("pump_1: old={:?}, new={:?}", last_msg.pump_1(), msg.pump_1());
        }
        if last_msg.pump_2() != msg.pump_2() {
            has_changed = true;
            log::debug!("pump_2: old={:?}, new={:?}", last_msg.pump_2(), msg.pump_2());
        }
        if last_msg.pump_3() != msg.pump_3() {
                has_changed = true;
                log::debug!("pump_3: old={:?}, new={:?}", last_msg.pump_3(), msg.pump_3());
            }
        if last_msg.pump_4() != msg.pump_4() {
            has_changed = true;
            log::debug!("pump_4: old={:?}, new={:?}", last_msg.pump_4(), msg.pump_4());
        }
        if last_msg.pump_5() != msg.pump_5() {
            has_changed = true;
            log::debug!("pump_5: old={:?}, new={:?}", last_msg.pump_5(), msg.pump_5());
        }
        if last_msg.blower_1() != msg.blower_1() {
            has_changed = true;
            log::debug!("blower_1: old={:?}, new={:?}", last_msg.blower_1(), msg.blower_1());
        }
        if last_msg.blower_2() != msg.blower_2() {
            has_changed = true;
            log::debug!("blower_2: old={:?}, new={:?}", last_msg.blower_2(), msg.blower_2());
        }
        if last_msg.lights() != msg.lights() {
            has_changed = true;
            log::debug!("lights: old={}, new={}", last_msg.lights(), msg.lights());
        }
        if last_msg.stereo() != msg.stereo() {
            has_changed = true;
            log::debug!("stereo: old={}, new={}", last_msg.stereo(), msg.stereo());
        }
        if last_msg.heater_1() != msg.heater_1() {
            has_changed = true;
            log::debug!("heater_1: old={:?}, new={:?}", last_msg.heater_1(), msg.heater_1());
        }
        if last_msg.heater_2() != msg.heater_2() {
            has_changed = true;
            log::debug!("heater_2: old={:?}, new={:?}", last_msg.heater_2(), msg.heater_2());
        }
        if last_msg.filter() != msg.filter() {
            has_changed = true;
            log::debug!("filter: old={:?}, new={:?}", last_msg.filter(), msg.filter());
        }
        if last_msg.onzen() != msg.onzen() {
            has_changed = true;
            log::debug!("onzen: old={}, new={}", last_msg.onzen(), msg.onzen());
        }
        if last_msg.ozone() != msg.ozone() {
            has_changed = true;
            log::debug!("ozone: old={:?}, new={:?}", last_msg.ozone(), msg.ozone());
        }
        if last_msg.exhaust_fan() != msg.exhaust_fan() {
            has_changed = true;
            log::debug!("exhaust_fan: old={}, new={}", last_msg.exhaust_fan(), msg.exhaust_fan());
        }
        if last_msg.sauna() != msg.sauna() {
            has_changed = true;
            log::debug!("sauna: old={:?}, new={:?}", last_msg.sauna(), msg.sauna());
        }
        if last_msg.heater_adc() != msg.heater_adc() {
            has_changed = true;
            log::debug!("heater_adc: old={}, new={}", last_msg.heater_adc(), msg.heater_adc());
        }
        if last_msg.sauna_time_remaining() != msg.sauna_time_remaining() {
            has_changed = true;
            log::debug!("sauna_time_remaining: old={}, new={}", last_msg.sauna_time_remaining(), msg.sauna_time_remaining());
        }
        if last_msg.economy() != msg.economy() {
            has_changed = true;
            log::debug!("economy: old={}, new={}", last_msg.economy(), msg.economy());
        }
        if last_msg.current_adc() != msg.current_adc() {
            has_changed = true;
            log::debug!("current_adc: old={}, new={}", last_msg.current_adc(), msg.current_adc());
        }
        if last_msg.all_on() != msg.all_on() {
            has_changed = true;
            log::debug!("all_on: old={}, new={}", last_msg.all_on(), msg.all_on());
        }
        if last_msg.fogger() != msg.fogger() {
            has_changed = true;
            log::debug!("fogger: old={}, new={}", last_msg.fogger(), msg.fogger());
        }
        if last_msg.error() != msg.error() {
            has_changed = true;
            log::debug!("error: old={}, new={}", last_msg.error(), msg.error());
        }
        if last_msg.alarm() != msg.alarm() {
            has_changed = true;
            log::debug!("alarm: old={}, new={}", last_msg.alarm(), msg.alarm());
        }
        if last_msg.status() != msg.status() {
            has_changed = true;
            log::debug!("status: old={}, new={}", last_msg.status(), msg.status());
        }
        if last_msg.ph() != msg.ph() {
            has_changed = true;
            log::debug!("ph: old={}, new={}", last_msg.ph(), msg.ph());
        }
        if last_msg.orp() != msg.orp() {
            has_changed = true;
            log::debug!("orp: old={}, new={}", last_msg.orp(), msg.orp());
        }
        if last_msg.sds() != msg.sds() {
            has_changed = true;
            log::debug!("sds: old={}, new={}", last_msg.sds(), msg.sds());
        }
        if last_msg.yess() != msg.yess() {
            has_changed = true;
            log::debug!("yess: old={}, new={}", last_msg.yess(), msg.yess());
        }

        if !has_changed {
            log::debug!("no changes since last message_live");
        }

        Ok(has_changed)
    }

    pub fn insert_message_live(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_live().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_live (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "temperature_fahrenheit",
                    "temperature_setpoint_fahrenheit",
                    "pump_1",
                    "pump_2",
                    "pump_3",
                    "pump_4",
                    "pump_5",
                    "blower_1",
                    "blower_2",
                    "lights",
                    "stereo",
                    "heater_1",
                    "heater_2",
                    "filter",
                    "onzen",
                    "ozone",
                    "exhaust_fan",
                    "sauna",
                    "heater_adc",
                    "sauna_time_remaining",
                    "economy",
                    "current_adc",
                    "all_on",
                    "fogger",
                    "error",
                    "alarm",
                    "status",
                    "ph",
                    "orp",
                    "sds",
                    "yess"
                )
                VALUES
                (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.temperature_fahrenheit(),
                msg.temperature_setpoint_fahrenheit(),
                msg.pump_1().value(),
                msg.pump_2().value(),
                msg.pump_3().value(),
                msg.pump_4().value(),
                msg.pump_5().value(),
                msg.blower_1().value(),
                msg.blower_2().value(),
                msg.lights(),
                msg.stereo(),
                msg.heater_1().value(),
                msg.heater_2().value(),
                msg.filter().value(),
                msg.onzen(),
                msg.ozone().value(),
                msg.exhaust_fan(),
                msg.sauna().value(),
                msg.heater_adc(),
                msg.sauna_time_remaining(),
                msg.economy(),
                msg.current_adc(),
                msg.all_on(),
                msg.fogger(),
                msg.error(),
                msg.alarm(),
                msg.status(),
                msg.ph(),
                msg.orp(),
                msg.sds(),
                msg.yess()
            ]
        )?;

        log::debug!("inserted message_live: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_onzen_live(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_onzen_live().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_onzen_live (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "guid",
                    "orp",
                    "ph_100",
                    "current",
                    "voltage",
                    "current_setpoint",
                    "voltage_setpoint",
                    "pump1",
                    "pump2",
                    "orp_state_machine",
                    "electrode_state_machine",
                    "electrode_id",
                    "electrode_polarity",
                    "electrode_1_resistance_1",
                    "electrode_1_resistance_2",
                    "electrode_2_resistance_1",
                    "electrode_2_resistance_2",
                    "command_mode",
                    "electrode_mah",
                    "ph_color",
                    "orp_color",
                    "electrode_wear"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.guid(),
                msg.orp(),
                msg.ph_100(),
                msg.current(),
                msg.voltage(),
                msg.current_setpoint(),
                msg.voltage_setpoint(),
                msg.pump1(),
                msg.pump2(),
                msg.orp_state_machine(),
                msg.electrode_state_machine(),
                msg.electrode_id(),
                msg.electrode_polarity().value(),
                msg.electrode_1_resistance_1(),
                msg.electrode_1_resistance_2(),
                msg.electrode_2_resistance_1(),
                msg.electrode_2_resistance_2(),
                msg.command_mode(),
                msg.electrode_mAH(),
                msg.ph_color().value(),
                msg.orp_color().value(),
                msg.electrode_wear()
            ],
        )?;

        log::debug!("inserted message_onzen_live: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_onzen_settings(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_onzen_settings().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_onzen_settings (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "guid",
                    "over_voltage",
                    "under_voltage",
                    "over_current",
                    "under_current",
                    "orp_high",
                    "orp_low",
                    "ph_high",
                    "ph_low",
                    "pwm_pump1_time_on",
                    "pwm_pump1_time_off",
                    "sampling_interval",
                    "sampling_duration",
                    "pwm_pump2_time_on",
                    "pwm_pump2_time_off",
                    "sb_low_cl",
                    "sb_caution_low_cl",
                    "sb_caution_high_cl",
                    "sb_high_cl",
                    "sb_low_ph",
                    "sb_caution_low_ph",
                    "sb_caution_high_ph",
                    "sb_high_ph"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.guid(),
                msg.over_voltage(),
                msg.under_voltage(),
                msg.over_current(),
                msg.under_current(),
                msg.orp_high(),
                msg.orp_low(),
                msg.ph_high(),
                msg.ph_low(),
                msg.pwm_pump1_time_on(),
                msg.pwm_pump1_time_off(),
                msg.sampling_interval(),
                msg.sampling_duration(),
                msg.pwm_pump2_time_on(),
                msg.pwm_pump2_time_off(),
                msg.sb_low_cl(),
                msg.sb_caution_low_cl(),
                msg.sb_caution_high_cl(),
                msg.sb_high_cl(),
                msg.sb_low_ph(),
                msg.sb_caution_low_ph(),
                msg.sb_caution_high_ph(),
                msg.sb_high_ph()
            ],
        )?;

        log::debug!("inserted message_onzen_settings: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_peak(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_peak().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_peak (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "peaknum",
                    "peakstart1",
                    "peakend1",
                    "peakstart2",
                    "peakend2",
                    "midpeaknum",
                    "midpeakstart1",
                    "midpeakend1",
                    "midpeakstart2",
                    "midpeakend2",
                    "offpeakstart",
                    "offpeakend",
                    "offset",
                    "peakheater",
                    "peakfilter",
                    "peakozone",
                    "midpeakheater",
                    "midpeakfilter",
                    "midpeakozone",
                    "sat",
                    "sun",
                    "mon",
                    "tue",
                    "wed",
                    "thu",
                    "fri"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.peaknum(),
                msg.peakstart1(),
                msg.peakend1(),
                msg.peakstart2(),
                msg.peakend2(),
                msg.midpeaknum(),
                msg.midpeakstart1(),
                msg.midpeakend1(),
                msg.midpeakstart2(),
                msg.midpeakend2(),
                msg.offpeakstart(),
                msg.offpeakend(),
                msg.offset(),
                msg.peakheater(),
                msg.peakfilter(),
                msg.peakozone(),
                msg.midpeakheater(),
                msg.midpeakfilter(),
                msg.midpeakozone(),
                msg.sat(),
                msg.sun(),
                msg.mon(),
                msg.tue(),
                msg.wed(),
                msg.thu(),
                msg.fri()
            ],
        )?;

        log::debug!("inserted message_peak: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_peripheral(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_peripheral().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_peripheral (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "guid",
                    "hardware_version",
                    "firmware_version",
                    "product_code",
                    "connected"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.guid(),
                msg.hardware_version(),
                msg.firmware_version(),
                msg.product_code().value(),
                msg.connected()
            ],
        )?;

        log::debug!("inserted message_peripheral: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message_settings(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        let msg = message.as_settings().ok_or(rusqlite::Error::InvalidQuery)?;
        let msg_rec_at_str = message.received_at_formatted(None);

        let (process_run_id, connection_session_id) = self.current_run_and_session_ids()?;

        self.conn.execute(
            r#"
                INSERT INTO message_settings (
                    "created_at",
                    "process_run_id",
                    "connection_session_id",
                    "message_received_at",

                    "max_filtration_frequency",
                    "min_filtration_frequency",
                    "filtration_frequency",
                    "max_filtration_duration",
                    "min_filtration_duration",
                    "filtration_duration",
                    "max_onzen_hours",
                    "min_onzen_hours",
                    "onzen_hours",
                    "max_onzen_cycles",
                    "min_onzen_cycles",
                    "onzen_cycles",
                    "max_ozone_hours",
                    "min_ozone_hours",
                    "ozone_hours",
                    "max_ozone_cycles",
                    "min_ozone_cycles",
                    "ozone_cycles",
                    "filter_suspension",
                    "flash_lights_on_error",
                    "temperature_offset",
                    "sauna_duration",
                    "min_temperature",
                    "max_temperature",
                    "filtration_offset",
                    "spaboy_hours"
                )
                VALUES (
                    datetime('now'),
                    ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?, ?, ?, ?, ?, ?, ?,
                    ?, ?
                )
            "#,
            params![
                process_run_id,
                connection_session_id,
                msg_rec_at_str,

                msg.max_filtration_frequency(),
                msg.min_filtration_frequency(),
                msg.filtration_frequency(),
                msg.max_filtration_duration(),
                msg.min_filtration_duration(),
                msg.filtration_duration(),
                msg.max_onzen_hours(),
                msg.min_onzen_hours(),
                msg.onzen_hours(),
                msg.max_onzen_cycles(),
                msg.min_onzen_cycles(),
                msg.onzen_cycles(),
                msg.max_ozone_hours(),
                msg.min_ozone_hours(),
                msg.ozone_hours(),
                msg.max_ozone_cycles(),
                msg.min_ozone_cycles(),
                msg.ozone_cycles(),
                msg.filter_suspension(),
                msg.flash_lights_on_error(),
                msg.temperature_offset(),
                msg.sauna_duration(),
                msg.min_temperature(),
                msg.max_temperature(),
                msg.filtration_offset(),
                msg.spaboy_hours()
            ],
        )?;

        log::debug!("inserted message_settings: received_at={}", msg_rec_at_str);

        Ok(())
    }

    pub fn insert_message(&self, message: &ProtoMessage) -> Result<(), rusqlite::Error> {
        match message.message_type() {
            MessageType::Clock => self.insert_message_clock(message),
            MessageType::Command => {
                log::warn!("received Command message, but insert_message_command is not implemented, skipping");
                Ok(())
            },
            MessageType::Configuration => self.insert_message_configuration(message),
            MessageType::Error => self.insert_message_error(message),
            MessageType::Filter => self.insert_message_filter(message),
            MessageType::Information => self.insert_message_information(message),
            MessageType::Live => self.insert_message_live(message),
            MessageType::OnzenLive => self.insert_message_onzen_live(message),
            MessageType::OnzenSettings => self.insert_message_onzen_settings(message),
            MessageType::Peak => self.insert_message_peak(message),
            MessageType::Peripheral => self.insert_message_peripheral(message),
            MessageType::Router => {
                log::warn!("received Router message, but insert_message_router is not implemented, skipping");
                Ok(())
            },
            MessageType::Settings => self.insert_message_settings(message),
            MessageType::Heartbeat => {
                log::warn!("received Heartbeat message, but insert_message_heartbeat is not implemented, skipping");
                Ok(())
            },
        }
    }
}
