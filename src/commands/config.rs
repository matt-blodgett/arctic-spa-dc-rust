use clap::ValueEnum;


// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ConfigPropertyName {
    /// Hot tub IP Address
    IpAddress,
    /// Log Level
    LogLevel,
    /// Use local running mock server
    MockServerMode,
    /// Mock server IP Address
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
