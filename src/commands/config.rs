use chrono::format::Pad;
use clap::ValueEnum;
use serde::{Deserialize, Serialize, de};
use std::time::SystemTime;
use crate::{commands::mock_server, core::net::{MessageType, NetworkClient, ProtoMessage}};
use std::collections::HashMap;


use crate::core::config::AppConfigManager;







pub fn test() -> bool {
    let mut config = AppConfigManager::load_or_create().unwrap();

    // println!("current config: {:#?}", config.data);

    // config.set_path("mock_server.enabled", serde_json::Value::Bool(true)).unwrap();
    config.set_path_value("mock_server.ip_address", serde_json::Value::String("127.0.0.1".to_string())).unwrap();
    let mock_server_ip_address = config.get_path_value("mock_server.ip_address").unwrap_or(serde_json::Value::Null);
    println!("mock_server.ip_address: {:?}", mock_server_ip_address);

    // config.set_path("ip_address", serde_json::Value::String("192.168.2.17".to_string())).unwrap();

    // let value1 = config.get_path("mock_server.enabled").unwrap_or(serde_json::Value::Null);
    // println!("mock_server.enabled: {:?}", value1);

    // let value2 = config.get_path("ip_address").unwrap_or(serde_json::Value::Null);
    // println!("ip_address: {:?}", value2);

    return true;
}











// #[derive(Serialize, Deserialize, Debug)]
// struct PollConfig {
//     ip_address: String,
//     message_intervals: MessageIntervals,
// }


// pub fn test_poll_config() {

//     let poll_config = PollConfig {
//         ip_address: "".to_string(),
//         message_intervals: MessageIntervals::from([
//             (MessageType::Clock, PollInterval {
//                 refresh_interval_ms: 5_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Configuration, PollInterval {
//                 refresh_interval_ms: 10_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Error, PollInterval {
//                 refresh_interval_ms: 15_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Filter, PollInterval {
//                 refresh_interval_ms: 20_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Information, PollInterval {
//                 refresh_interval_ms: 25_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Live, PollInterval {
//                 refresh_interval_ms: 30_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::OnzenLive, PollInterval {
//                 refresh_interval_ms: 35_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::OnzenSettings, PollInterval {
//                 refresh_interval_ms: 40_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Peak, PollInterval {
//                 refresh_interval_ms: 45_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Peripheral, PollInterval {
//                 refresh_interval_ms: 50_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Router, PollInterval {
//                 refresh_interval_ms: 55_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//             (MessageType::Settings, PollInterval {
//                 refresh_interval_ms: 60_000,
//                 last_refresh_time: Some(SystemTime::now())
//             }),
//         ]),
//     };
//     println!("poll_config: {:#?}", poll_config);

//     println!("test {:?}", poll_config.message_intervals[&MessageType::Live]);
// }



// pub fn test() -> Result<(), Box<dyn std::error::Error>> {
    // let mut config = AppConfigManager::load_or_create()?;

    // config.data.nested = Some(NestedConfig {
    //     txt: "hello".to_string(),
    //     num: "123".to_string(),
    // });


    // config.data.array_of_objects = Some(vec![
    //     NestedConfig {
    //         txt: "first".to_string(),
    //         num: "1".to_string(),
    //         message_type: MessageType::Clock,
    //     },
    //     NestedConfig {
    //         txt: "second".to_string(),
    //         num: "2".to_string(),
    //         message_type: MessageType::Configuration,
    //     },
    // ]);

    // for nested_config in config.data.array_of_objects.as_ref().unwrap_or(&vec![]) {
    //     println!("nested config: {:#?}", nested_config);
    //     println!("{:?}={:?} -> {}", nested_config.message_type, MessageType::Configuration, nested_config.message_type == MessageType::Configuration);
    // }


    // config.data.message_intervals.unwrap_or(MessageIntervals::new()).insert(MessageType::Live, PollInterval {
    //     refresh_interval_ms: 30_000,
    //     last_refresh_time: Some(SystemTime::now()),
    // });

    // let message_intervals = MessageIntervals::from([
    //     (MessageType::Clock, PollInterval {
    //         refresh_interval_ms: 5_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Configuration, PollInterval {
    //         refresh_interval_ms: 10_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Error, PollInterval {
    //         refresh_interval_ms: 15_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Filter, PollInterval {
    //         refresh_interval_ms: 20_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Information, PollInterval {
    //         refresh_interval_ms: 25_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Live, PollInterval {
    //         refresh_interval_ms: 30_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::OnzenLive, PollInterval {
    //         refresh_interval_ms: 35_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::OnzenSettings, PollInterval {
    //         refresh_interval_ms: 40_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Peak, PollInterval {
    //         refresh_interval_ms: 45_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Peripheral, PollInterval {
    //         refresh_interval_ms: 50_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Router, PollInterval {
    //         refresh_interval_ms: 55_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    //     (MessageType::Settings, PollInterval {
    //         refresh_interval_ms: 60_000,
    //         last_refresh_time: Some(SystemTime::now())
    //     }),
    // ]);

    // config.data.message_intervals = Some(message_intervals);

    // let m: serde_json::Map<String, serde_json::Value> = [
    //     ("key1".to_string(), serde_json::Value::String("value1".to_string())),
    //     ("key2".to_string(), serde_json::Value::String("value2".to_string())),
    // ]
    // .into_iter()
    // .collect();

    // println!("msgs: {:#?}", m);

    // config.data.msgs = Some(m);

    // config.data.msgs.as_mut().map(|msgs| {
    //     msgs["key1"] = serde_json::Value::String("updated_value1".to_string());
    // });

    // config.data.msgs.as_ref().map(|msgs| {
    //     for (key, value) in msgs {
    //         println!("msg key: {}, value: {}", key, value);
    //     }
    // });
    // msgs: {
    //     "key1": String("value1"),
    //     "key2": String("value2"),
    // }
    // msg key: key1, value: "updated_value1"
    // msg key: key2, value: "value2"
    // test result: Ok(())



    // let mut message_intervals = config.data.message_intervals.unwrap_or(MessageIntervals::new());

    // message_intervals.insert(MessageType::Live, PollInterval {
    //     refresh_interval_ms: 30_000,
    //     last_refresh_time: Some(SystemTime::now()),
    // });


    // println!("config: {:#?}", config.data);

//     config.save()?;

//     Ok(())
// }



