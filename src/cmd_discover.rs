use crate::discovery;


pub fn discover_devices() -> Vec<String> {
    println!("searching for devices on the local network...");
    discovery::search(None, 24, 1000, 100)
}

pub fn display_devices(devices: &Vec<String>) -> () {
    if devices.is_empty() {
        println!("no devices found on the network.");
    } else {
        println!("discovered devices:");
        for device in devices {
            println!("* {}", device);
        }
    }
}
