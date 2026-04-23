use std::env;

mod asdc;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: arctic-spa-dc-rust \"{{target_ip_address}}\" {{message_type}}");
        return;
    }

    let arg_ip_address: String = String::from(&args[1]);
    let arg_message_type: u16 = args[2].parse().unwrap();

    let message_type: asdc::MessageType = asdc::int_to_message_type(arg_message_type).unwrap();

    let mut network_client = asdc::NetworkClient::new();

    network_client.connect(&arg_ip_address).unwrap();
    network_client.request_message(message_type).unwrap();
    let messages: Vec<asdc::ProtoMessage> = network_client.read_messages().unwrap();

    for msg in messages {
        match msg {
            asdc::ProtoMessage::Live(m) => println!("{m:?}"),
            asdc::ProtoMessage::Command(m) => println!("{m:?}"),
            asdc::ProtoMessage::Settings(m) => println!("{m:?}"),
            asdc::ProtoMessage::Configuration(m) => println!("{m:?}"),
            asdc::ProtoMessage::Peak(m) => println!("{m:?}"),
            asdc::ProtoMessage::Clock(m) => println!("{m:?}"),
            asdc::ProtoMessage::Information(m) => println!("{m:?}"),
            asdc::ProtoMessage::Error(m) => println!("{m:?}"),
            asdc::ProtoMessage::Router(m) => println!("{m:?}"),
            asdc::ProtoMessage::Filter(m) => println!("{m:?}"),
            asdc::ProtoMessage::Peripheral(m) => println!("{m:?}"),
            asdc::ProtoMessage::OnzenLive(m) => println!("{m:?}"),
            asdc::ProtoMessage::OnzenSettings(m) => println!("{m:?}")
        }
    }
}
