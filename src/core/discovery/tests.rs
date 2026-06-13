use super::*;

#[test]
fn test_enumerate_hosts_basic() {
    let hosts = enumerate_hosts("192.168.1.0", 24);
    assert_eq!(hosts.len(), 254); // 192.168.1.1 through 192.168.1.254
    assert_eq!(hosts[0], "192.168.1.1");
    assert_eq!(hosts[hosts.len() - 1], "192.168.1.254");
}

#[test]
fn test_enumerate_hosts_small_subnet() {
    let hosts = enumerate_hosts("192.168.1.0", 30);
    // /30 has 4 addresses: network, 2 usable, broadcast
    assert_eq!(hosts.len(), 2); // .1 and .2 are usable
}

#[test]
fn test_enumerate_hosts_invalid_ip() {
    let hosts = enumerate_hosts("invalid", 24);
    assert!(hosts.is_empty());
}

#[test]
fn test_ipv4_to_string() {
    let ip: Ipv4Addr = "192.168.1.1".parse().unwrap();
    assert_eq!(ip.to_string(), "192.168.1.1");
}
