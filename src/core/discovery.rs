use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Returns the local IPv4 address that would be used for the default route
fn local_ipv4_via_default_route() -> String {
    // Attempt to connect to a dummy address to determine local route
    if let Ok(sock) = UdpSocket::bind("0.0.0.0:0") {
        sock.set_read_timeout(Some(Duration::from_millis(1))).ok();
        // Doesn't need to be reachable; forces kernel to pick a route
        let dummy_dst: SocketAddr = "10.254.254.254:1".parse().unwrap();
        if sock.connect(dummy_dst).is_ok() {
            if let Ok(addr) = sock.local_addr() {
                if let IpAddr::V4(ipv4) = addr.ip() {
                    let ip_string = ipv4.to_string();
                    // Validate it's not loopback
                    if !ipv4.is_loopback() {
                        return ip_string;
                    }
                }
            }
        }
    }

    // Fallback: try to find first non-loopback IPv4 from pnet or use default
    // For simplicity without external deps, return localhost
    "127.0.0.1".to_string()
}

/// Enumerates all host IP addresses in a subnet given an IP and CIDR prefix
fn enumerate_hosts(ip_address: &str, prefix: u8) -> Vec<String> {
    // Parse the IP address
    let ip: Ipv4Addr = match ip_address.parse() {
        Ok(addr) => addr,
        Err(_) => return Vec::new(),
    };

    let ip_u32 = u32::from(ip);

    // Build mask from CIDR
    let mask = if prefix == 0 {
        0u32
    } else {
        0xFFFFFFFFu32 << (32 - prefix)
    };

    let network = ip_u32 & mask;
    let broadcast = network | !mask;

    let start = if prefix <= 30 { network + 1 } else { network };
    let end = if prefix <= 30 { broadcast - 1 } else { broadcast };

    if start > end {
        return Vec::new();
    }

    let mut hosts = Vec::new();
    for addr_u32 in start..=end {
        let addr = Ipv4Addr::from(addr_u32);
        hosts.push(addr.to_string());
    }

    hosts
}

/// Sends a UDP probe to a host and checks for the expected response
fn udp_probe(host: &str, query: &[u8], query_port: u16, response_prefix: &[u8], timeout: Duration) -> bool {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return false,
    };

    if socket.set_read_timeout(Some(timeout)).is_err() {
        return false;
    }

    let dst_addr = match format!("{}:{}", host, query_port).parse::<SocketAddr>() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    if socket.send_to(query, dst_addr).is_err() {
        return false;
    }

    let mut buf = [0u8; 1024];
    let start = std::time::Instant::now();

    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, addr)) => {
                // Check if sender matches and response starts with expected prefix
                if addr.ip().to_string() == host && buf[..len].starts_with(response_prefix) {
                    return true;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Timeout
                if start.elapsed() >= timeout {
                    break;
                }
            }
            Err(_) => {
                // Other error, stop trying
                break;
            }
        }

        // Check if total timeout exceeded
        if start.elapsed() >= timeout {
            break;
        }
    }

    false
}

/// Searches for Arctic Spa devices on the network
///
/// # Arguments
/// * `ip_address` - The IP address to scan from. If None, uses the local IPv4 default route
/// * `netmask_cidr` - The CIDR netmask (e.g., 24 for /24)
/// * `timeout_ms` - Timeout for each UDP probe in milliseconds
/// * `max_workers` - Maximum number of concurrent threads to use
///
/// # Returns
/// A vector of IP addresses of discovered devices
pub fn search(ip_address: Option<&str>, netmask_cidr: u8, timeout_ms: u64, max_workers: usize) -> Vec<String> {
    const QUERY: &[u8] = b"Query,BlueFalls,";
    const RESPONSE: &[u8] = b"Response,BlueFalls,";
    const QUERY_PORT: u16 = 9131;

    log::info!("initializing search");

    let ip_addr = match ip_address {
        Some(addr) if !addr.is_empty() => addr.to_string(),
        _ => {
            log::debug!("no ip address supplied, getting local ipv4 default");
            let local_ip = local_ipv4_via_default_route();
            log::debug!("using local ipv4 default route: {}", local_ip);
            local_ip
        }
    };

    log::info!(
        "scanning network: ip_supplied={}, ip_fallback={}, netmask={}, timeout={}ms, workers={}",
        ip_address.unwrap_or("<none>"),
        ip_addr,
        netmask_cidr,
        timeout_ms,
        max_workers
    );

    let hosts = enumerate_hosts(&ip_addr, netmask_cidr);
    if hosts.is_empty() {
        log::warn!("no hosts found to search; exiting");
        return Vec::new();
    }

    log::debug!("found {} hosts to query; starting udp probes", hosts.len());

    let found = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    // Create thread pool with max_workers limit
    let semaphore = Arc::new(Mutex::new(0usize));

    for host in hosts {
        let found_clone = Arc::clone(&found);
        let semaphore_clone = Arc::clone(&semaphore);
        let timeout_duration = Duration::from_millis(timeout_ms);

        let handle = thread::spawn(move || {
            // Simple rate limiting using counter
            {
                let mut count = semaphore_clone.lock().unwrap();
                while *count >= max_workers {
                    drop(count);
                    thread::sleep(Duration::from_millis(1));
                    count = semaphore_clone.lock().unwrap();
                }
                *count += 1;
            }

            if udp_probe(&host, QUERY, QUERY_PORT, RESPONSE, timeout_duration) {
                let mut found_list = found_clone.lock().unwrap();
                found_list.push(host);
            }

            let mut count = semaphore_clone.lock().unwrap();
            *count -= 1;
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    let result = Arc::try_unwrap(found)
        .map(|mutex| mutex.into_inner().unwrap())
        .unwrap_or_else(|arc| arc.lock().unwrap().clone());

    // Sort results for consistent output
    let mut result = result;
    result.sort();

    log::info!("discovered {} valid host[s]", result.len());
    if !result.is_empty() {
        log::info!("discovered hosts: {:?}", result);
    }

    result
}

#[cfg(test)]
mod tests {
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
}
