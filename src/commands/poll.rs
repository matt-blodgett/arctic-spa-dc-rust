#![allow(dead_code)]


use crate::proto;
use crate::core::db;


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

    let mut msg_live = proto::Live::Live::new();
    msg_live.set_temperature_fahrenheit(102);
    msg_live.set_temperature_setpoint_fahrenheit(104);
    msg_live.set_pump_1(proto::Live::live::PumpStatus::PUMP_LOW);
    msg_live.set_pump_2(proto::Live::live::PumpStatus::PUMP_HIGH);
    msg_live.set_pump_3(proto::Live::live::PumpStatus::PUMP_OFF);
    msg_live.set_pump_4(proto::Live::live::PumpStatus::PUMP_OFF);
    msg_live.set_pump_5(proto::Live::live::PumpStatus::PUMP_OFF);
    msg_live.set_blower_1(proto::Live::live::PumpStatus::PUMP_OFF);
    msg_live.set_blower_2(proto::Live::live::PumpStatus::PUMP_OFF);
    msg_live.set_lights(false);
    msg_live.set_stereo(false);
    msg_live.set_heater_1(proto::Live::live::HeaterStatus::HEATER_HEATING);
    msg_live.set_heater_2(proto::Live::live::HeaterStatus::HEATER_IDLE);
    msg_live.set_filter(proto::Live::live::FilterStatus::FILTER_IDLE);
    msg_live.set_onzen(true);
    msg_live.set_ozone(proto::Live::live::OzoneStatus::OZONE_ACTIVE);
    msg_live.set_exhaust_fan(false);
    msg_live.set_sauna(proto::Live::live::SaunaStatus::SAUNA_NORMAL);
    msg_live.set_heater_adc(20);
    msg_live.set_sauna_time_remaining(0);
    msg_live.set_economy(false);
    msg_live.set_current_adc(0);
    msg_live.set_all_on(false);
    msg_live.set_fogger(false);
    msg_live.set_error(0);
    msg_live.set_alarm(24);
    msg_live.set_status(67);
    msg_live.set_ph(712);
    msg_live.set_orp(650);
    msg_live.set_sds(false);
    msg_live.set_yess(false);

    if let Err(e) = db_client.insert_message_live(&msg_live) {
        log::error!("failed to insert live data: {}", e);
        return;
    }
}
