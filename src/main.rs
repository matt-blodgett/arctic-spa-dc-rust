use std::env;

mod tcp_client;
use crate::tcp_client::NetworkClient;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: arctic-spa-dc-rust \"{{target_ip_address}}\" {{message_type}}");
        return;
    }

    let ip_address: String = String::from(&args[1]);
    let message_type: u16 = args[2].parse().unwrap();

    if message_type != 1 || message_type != 2 {
        println!("Invalid message type");
        return;
    }

    let mut network_client = NetworkClient::new();

    match network_client.connect(&ip_address) {
        Ok(_) => {},
        Err(e) => panic!("error connecting to host: {e}")
    };

    match network_client.write_packet(message_type, vec![]) {
        Ok(_) => {},
        Err(e) => panic!("error writing packet: {e}")
    }

    match network_client.read_packets() {
        Ok(_) => {},
        Err(e) => panic!("error reading packets: {e}")
    }
}
