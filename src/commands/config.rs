use clap::ValueEnum;


// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ConfigPropertyName {
    /// Hot tub IP Address
    #[value(alias = "ip-address", alias = "ip_address")]
    IpAddress,
    /// Log Level
    #[value(alias = "log-level", alias = "log_level")]
    LogLevel,
    /// Use local running mock server
    #[value(alias = "mock-server.mode", alias = "mock-server-mode", alias = "mock_server_mode")]
    MockServerMode,
    /// Mock server IP Address
    #[value(alias = "mock-server.ip-address", alias = "mock-server-ip-address", alias = "mock_server_ip_address")]
    MockServerIpAddress,
}

impl ConfigPropertyName {
    pub fn as_str(&self) -> &str {
        match self {
            ConfigPropertyName::IpAddress => "ip_address",
            ConfigPropertyName::LogLevel => "log_level",
            ConfigPropertyName::MockServerMode => "mock_server_mode",
            ConfigPropertyName::MockServerIpAddress => "mock_server_ip_address",
        }
    }
}
