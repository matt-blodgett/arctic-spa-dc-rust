use clap::ValueEnum;


// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ConfigPropertyName {
    /// IP Address
    IpAddress,
    /// Verbosity
    Verbosity
}

impl ConfigPropertyName {
    pub fn as_str(&self) -> &str {
        match self {
            ConfigPropertyName::IpAddress => "ip_address",
            ConfigPropertyName::Verbosity => "verbosity",
        }
    }
}
