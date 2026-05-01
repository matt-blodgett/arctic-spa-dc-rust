use std::io::Error;

use crate::asdc;


pub fn get_message(ip_address: &str, message_type: asdc::MessageType) -> Result<asdc::ProtoMessage, Error> {
    log::info!("querying host {:?} for message type {:?}", ip_address, message_type);
    let mut network_client = asdc::NetworkClient::new();
    network_client.connect(ip_address)?;
    let message = network_client.request_message_and_await_response(message_type)?;
    Ok(message)
}

pub fn display_message(message: asdc::ProtoMessage) -> () {
    log::info!("outputting message data");

    // let output_string = match message {
    //     asdc::ProtoMessage::Live(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Command(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Settings(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Configuration(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Peak(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Clock(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Information(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Error(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Router(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Filter(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::Peripheral(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::OnzenLive(msg) => protobuf::text_format::print_to_string(&msg),
    //     asdc::ProtoMessage::OnzenSettings(msg) => protobuf::text_format::print_to_string(&msg),
    // };
    let output_string = match message {
        asdc::ProtoMessage::Live(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Command(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Settings(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Configuration(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Peak(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Clock(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Information(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Error(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Router(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Filter(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::Peripheral(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::OnzenLive(msg) => protobuf::text_format::print_to_string_pretty(&msg),
        asdc::ProtoMessage::OnzenSettings(msg) => protobuf::text_format::print_to_string_pretty(&msg),
    };
    println!("{:#?}", output_string);
}
