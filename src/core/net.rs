#![allow(dead_code)]


use std::net::TcpStream;
use std::time::{Duration, SystemTime};
use std::io::{Error, ErrorKind, Read, Write};

use chrono::{DateTime, Utc};

use crate::proto;
use protobuf::Message;
use serde::{Deserialize, Serialize};


const HEADER_SIZE: usize = 20;
const HEADER_PREAMBLE: [u8; 4] = [171, 173, 29, 58];
const HEADER_MAGIC: i32 = -1414718150;


#[derive(Deserialize, Serialize)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MessageType {
    Live = 0,
    Command = 1,
    Settings = 2,
    Configuration = 3,
    Peak = 4,
    Clock = 5,
    Information = 6,
    Error = 7,
    Router = 9,
    Heartbeat = 10,
    Filter = 13,
    Peripheral = 16,
    OnzenLive = 48,
    OnzenSettings = 50
}

impl From<MessageType> for u16 {
    fn from(mt: MessageType) -> u16 {
        mt as u16
    }
}

impl TryFrom<u16> for MessageType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Error> {
        match value {
            0 => Ok(MessageType::Live),
            1 => Ok(MessageType::Command),
            2 => Ok(MessageType::Settings),
            3 => Ok(MessageType::Configuration),
            4 => Ok(MessageType::Peak),
            5 => Ok(MessageType::Clock),
            6 => Ok(MessageType::Information),
            7 => Ok(MessageType::Error),
            9 => Ok(MessageType::Router),
            10 => Ok(MessageType::Heartbeat),
            13 => Ok(MessageType::Filter),
            16 => Ok(MessageType::Peripheral),
            48 => Ok(MessageType::OnzenLive),
            50 => Ok(MessageType::OnzenSettings),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid message type"))
        }
    }
}


#[derive(Debug)]
struct Packet {
    preamble: [u8; 4],
    checksum: [u8; 4],
    counter: u32,
    unused: u32,
    message_type_value: u16,
    message_type: Option<MessageType>,
    payload_size: u16,
    payload: Vec<u8>,
    packet_size: u16,
    received_at: SystemTime
}

impl Packet {
    pub fn new() -> Self {
        Self {
            preamble: [0, 0, 0, 0],
            checksum: [0, 0, 0, 0],
            counter: 0,
            unused: 0,
            message_type_value: 0,
            message_type: None,
            payload_size: 0,
            payload: vec![],
            packet_size: HEADER_SIZE as u16,
            received_at: SystemTime::now()
        }
    }

    pub fn serialize(&mut self, message_type_value: u16, payload: Vec<u8>) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        log::trace!("serializing packet: message_type_value={:?}", message_type_value);

        self.preamble = HEADER_PREAMBLE;
        let padding: u32 = 0;  // 4 bytes reserved for checksum after calculation
        self.counter = 0;
        self.unused = 0;
        self.message_type_value = message_type_value;
        self.message_type = MessageType::try_from(message_type_value).ok();
        self.payload_size = payload.len() as u16;
        self.payload = payload;
        self.packet_size = HEADER_SIZE as u16 + self.payload_size;
        self.received_at = SystemTime::now();

        ret.extend_from_slice(&self.preamble);
        ret.extend_from_slice(&padding.to_be_bytes());
        ret.extend_from_slice(&self.counter.to_be_bytes());
        ret.extend_from_slice(&self.unused.to_be_bytes());
        ret.extend_from_slice(&self.message_type_value.to_be_bytes());
        ret.extend_from_slice(&self.payload_size.to_be_bytes());
        ret.extend_from_slice(&self.payload);

        log::trace!("packet bytes before checksum: len={}, value={:?}", ret.len(), ret);

        // Calculate checksum on the current packet
        const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let checksum_value = CRC32.checksum(&ret);
        let checksum_bytes = checksum_value.to_be_bytes();
        self.checksum = checksum_bytes;

        log::trace!("calculated checksum: value={:?}, bytes={:?}", checksum_value, checksum_bytes);

        // Replace the padding field (bytes 4-8) with the checksum
        ret[4..8].copy_from_slice(&checksum_bytes);

        log::trace!("packet bytes: len={}, value={:?}", ret.len(), ret);

        log::debug!("successfully serialized packet: message_type={:?}, payload_size={}", self.message_type, self.payload_size);
        log::trace!("packet: {:?}", self);

        return ret;
    }

    pub fn deserialize(&mut self, data: &Vec<u8>) -> Result<(), Error> {
        log::trace!("deserializing packet: bytes_len={}", data.len());

        if data.len() < HEADER_SIZE {
            return Err(
                Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid input data: expected {} bytes, got {} bytes", HEADER_SIZE, data.len())
                )
            );
        }
        if data.get(0..4) != HEADER_PREAMBLE.get(0..4) {
            return Err(
                Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid preamble: expected {:?}, got {:?}", HEADER_PREAMBLE.get(0..4), data.get(0..4))
                )
            );
        }

        self.preamble.copy_from_slice(&data[0..4]);
        self.checksum.copy_from_slice(&data[4..8]);

        self.counter = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
        self.unused = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);

        self.message_type_value = u16::from_be_bytes([data[16], data[17]]);
        self.message_type = MessageType::try_from(self.message_type_value).ok();
        self.payload_size = u16::from_be_bytes([data[18], data[19]]);
        self.payload.extend_from_slice(&data[HEADER_SIZE..HEADER_SIZE + self.payload_size as usize]);
        self.packet_size = HEADER_SIZE as u16 + self.payload_size;
        self.received_at = SystemTime::now();

        log::debug!("successfully deserialized packet: message_type={:?}, payload_size={}", self.message_type, self.payload_size);
        log::trace!("packet: {:?}", self);

        Ok(())
    }
}


#[derive(Debug)]
pub enum ProtoMessage {
    Live { message: proto::Live::Live, received_at: SystemTime },
    Command { message: proto::Command::Command, received_at: SystemTime },
    Settings { message: proto::Settings::Settings, received_at: SystemTime },
    Configuration { message: proto::Configuration::Configuration, received_at: SystemTime },
    Peak { message: proto::Peak::Peak, received_at: SystemTime },
    Clock { message: proto::Clock::Clock, received_at: SystemTime },
    Information { message: proto::Information::Information, received_at: SystemTime },
    Error { message: proto::Error::Error, received_at: SystemTime },
    Router { message: proto::Router::Router, received_at: SystemTime },
    Filter { message: proto::Filter::Filter, received_at: SystemTime },
    Peripheral { message: proto::Peripheral::Peripheral, received_at: SystemTime },
    OnzenLive { message: proto::OnzenLive::OnzenLive, received_at: SystemTime },
    OnzenSettings { message: proto::OnzenSettings::OnzenSettings, received_at: SystemTime }
}

impl ProtoMessage {
    pub fn as_live(&self) -> Option<&proto::Live::Live> {
        match self {
            ProtoMessage::Live { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_command(&self) -> Option<&proto::Command::Command> {
        match self {
            ProtoMessage::Command { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_settings(&self) -> Option<&proto::Settings::Settings> {
        match self {
            ProtoMessage::Settings { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_configuration(&self) -> Option<&proto::Configuration::Configuration> {
        match self {
            ProtoMessage::Configuration { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_peak(&self) -> Option<&proto::Peak::Peak> {
        match self {
            ProtoMessage::Peak { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_clock(&self) -> Option<&proto::Clock::Clock> {
        match self {
            ProtoMessage::Clock { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_information(&self) -> Option<&proto::Information::Information> {
        match self {
            ProtoMessage::Information { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_error(&self) -> Option<&proto::Error::Error> {
        match self {
            ProtoMessage::Error { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_router(&self) -> Option<&proto::Router::Router> {
        match self {
            ProtoMessage::Router { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_filter(&self) -> Option<&proto::Filter::Filter> {
        match self {
            ProtoMessage::Filter { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_peripheral(&self) -> Option<&proto::Peripheral::Peripheral> {
        match self {
            ProtoMessage::Peripheral { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_onzen_live(&self) -> Option<&proto::OnzenLive::OnzenLive> {
        match self {
            ProtoMessage::OnzenLive { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn as_onzen_settings(&self) -> Option<&proto::OnzenSettings::OnzenSettings> {
        match self {
            ProtoMessage::OnzenSettings { message, .. } => Some(message),
            _ => None,
        }
    }

    pub fn message_type(&self) -> MessageType {
        match self {
            ProtoMessage::Live { .. } => MessageType::Live,
            ProtoMessage::Command { .. } => MessageType::Command,
            ProtoMessage::Settings { .. } => MessageType::Settings,
            ProtoMessage::Configuration { .. } => MessageType::Configuration,
            ProtoMessage::Peak { .. } => MessageType::Peak,
            ProtoMessage::Clock { .. } => MessageType::Clock,
            ProtoMessage::Information { .. } => MessageType::Information,
            ProtoMessage::Error { .. } => MessageType::Error,
            ProtoMessage::Router { .. } => MessageType::Router,
            ProtoMessage::Filter { .. } => MessageType::Filter,
            ProtoMessage::Peripheral { .. } => MessageType::Peripheral,
            ProtoMessage::OnzenLive { .. } => MessageType::OnzenLive,
            ProtoMessage::OnzenSettings { .. } => MessageType::OnzenSettings,
        }
    }

    pub fn received_at(&self) -> &SystemTime {
        match self {
            ProtoMessage::Live { received_at, .. } => received_at,
            ProtoMessage::Command { received_at, .. } => received_at,
            ProtoMessage::Settings { received_at, .. } => received_at,
            ProtoMessage::Configuration { received_at, .. } => received_at,
            ProtoMessage::Peak { received_at, .. } => received_at,
            ProtoMessage::Clock { received_at, .. } => received_at,
            ProtoMessage::Information { received_at, .. } => received_at,
            ProtoMessage::Error { received_at, .. } => received_at,
            ProtoMessage::Router { received_at, .. } => received_at,
            ProtoMessage::Filter { received_at, .. } => received_at,
            ProtoMessage::Peripheral { received_at, .. } => received_at,
            ProtoMessage::OnzenLive { received_at, .. } => received_at,
            ProtoMessage::OnzenSettings { received_at, .. } => received_at,
        }
    }

    pub fn received_at_datetime(&self) -> DateTime<Utc> {
        let received_at = self.received_at();
        let received_at_dt: DateTime<Utc> = (*received_at).into();
        received_at_dt
    }

    pub fn received_at_formatted(&self, fmt_string: Option<&str>) -> String {
        let received_at_dt = self.received_at_datetime();
        let fmt = fmt_string.unwrap_or("%Y-%m-%d %H:%M:%S");
        let received_at_fmt = received_at_dt.format(fmt).to_string();
        received_at_fmt
    }
}


impl TryFrom<&Packet> for ProtoMessage {
    type Error = Error;

    fn try_from(value: &Packet) -> Result<Self, Error> {
        log::trace!("parsing packet payload to protobuf message: message_type_value={:?}, payload_size={}", value.message_type_value, value.payload_size);

        let message_type = value.message_type
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid message type in packet"))?;

        match message_type {
            MessageType::Live => Ok(ProtoMessage::Live { message: proto::Live::Live::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Command => Ok(ProtoMessage::Command { message: proto::Command::Command::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Settings => Ok(ProtoMessage::Settings { message: proto::Settings::Settings::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Configuration => Ok(ProtoMessage::Configuration { message: proto::Configuration::Configuration::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Peak => Ok(ProtoMessage::Peak { message: proto::Peak::Peak::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Clock => Ok(ProtoMessage::Clock { message: proto::Clock::Clock::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Information => Ok(ProtoMessage::Information { message: proto::Information::Information::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Error => Ok(ProtoMessage::Error { message: proto::Error::Error::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Router => Ok(ProtoMessage::Router { message: proto::Router::Router::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Heartbeat => Err(Error::new(ErrorKind::Unsupported, "no corresponding proto for message type Heartbeat")),
            MessageType::Filter => Ok(ProtoMessage::Filter { message: proto::Filter::Filter::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::Peripheral => Ok(ProtoMessage::Peripheral { message: proto::Peripheral::Peripheral::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::OnzenLive => Ok(ProtoMessage::OnzenLive { message: proto::OnzenLive::OnzenLive::parse_from_bytes(&value.payload)?, received_at: value.received_at }),
            MessageType::OnzenSettings => Ok(ProtoMessage::OnzenSettings { message: proto::OnzenSettings::OnzenSettings::parse_from_bytes(&value.payload)?, received_at: value.received_at })
        }
    }
}


pub struct NetworkClient {
    host: String,
    port: &'static str,
    tcp_stream: Option<TcpStream>
}

impl NetworkClient {
    pub fn new() -> Self {
        Self {
            host: "".to_string(),
            port: "65534",
            tcp_stream: None
        }
    }

    pub fn connect(ip_address: &str) -> Result<Self, Error> {
        let mut client = Self::new();
        client.set_connection(ip_address)?;
        Ok(client)
    }

    pub fn set_connection(&mut self, host: &str) -> Result<(), Error> {
        self.host = host.to_string();

        log::info!("creating tcp connection: host={:?}, port={:?}", self.host, self.port);

        let addr = format!("{}:{}", self.host, self.port);
        let stream = TcpStream::connect(&addr)?;

        log::debug!("setting stream to blocking mode");
        stream.set_nonblocking(false)?;

        self.tcp_stream = Some(stream);
        let default_timeout_ms = 3_000;
        self.set_timeout_ms(default_timeout_ms)?;

        log::info!("successfully connected to {:?}", &addr);

        Ok(())
    }

    fn get_stream_mut(&mut self) -> Result<&mut TcpStream, Error> {
        self.tcp_stream.as_mut()
            .ok_or_else(|| Error::new(ErrorKind::NotConnected, "not connected"))
    }

    pub fn set_timeout_ms(&mut self, ms: u64) -> Result<(), Error> {
        log::debug!("setting stream read / write timeout to {}ms", ms);
        let stream = self.get_stream_mut()?;
        let duration = Duration::from_millis(ms);
        stream.set_read_timeout(Some(duration))?;
        stream.set_write_timeout(Some(duration))?;
        Ok(())
    }

    fn write_packet(&mut self, message_type_value: u16, payload: Vec<u8>) -> Result<(), Error> {
        log::debug!("writing packet: message_type_value={:?}, payload_size={:?}", message_type_value, payload.len());
        let stream = self.get_stream_mut()?;

        let mut packet = Packet::new();
        let packet_bytes = packet.serialize(message_type_value, payload);

        stream.write_all(&packet_bytes)?;

        match MessageType::try_from(message_type_value) {
            Ok(message_type) => log::debug!("successfully wrote packet for message type {:?}", message_type),
            Err(_) => log::debug!("successfully wrote packet for message type value {}", message_type_value),
        };

        Ok(())
    }

    pub fn request_message(&mut self, message_type: MessageType) -> Result<(), Error> {
        log::debug!("requesting message {:?}", message_type);
        self.write_packet(message_type.into(), vec![])?;
        Ok(())
    }

    fn packets_to_messages(&self, packets: &Vec<Packet>) -> Result<Vec<ProtoMessage>, Error> {
        let mut ret: Vec<ProtoMessage> = vec![];

        log::trace!("parsing {} packets to messages", packets.len());

        for packet in packets.iter() {
            let msg = match ProtoMessage::try_from(packet) {
                Ok(m) => m,
                Err(e) => {
                    if packet.message_type_value != MessageType::Heartbeat as u16 {
                        log::warn!("error parsing message: {}", e);
                    }
                    continue;
                }
            };
            ret.push(msg);
        }

        Ok(ret)
    }

    fn bytes_to_packets(&self, bytes_array: &Vec<u8>) -> Result<Vec<Packet>, Error> {
        let mut ret: Vec<Packet> = vec![];

        log::trace!("parsing {} bytes into packets", bytes_array.len());

        let total_bytes_read: u32 = bytes_array.len() as u32;
        let mut parsed_byte_count: u32 = 0;

        while parsed_byte_count < total_bytes_read {
            let mut packet = Packet::new();
            let mut buf_parse: Vec<u8> = vec![];
            buf_parse.extend_from_slice(&bytes_array[parsed_byte_count as usize..total_bytes_read as usize]);
            match packet.deserialize(&buf_parse) {
                Ok(_) => {
                    parsed_byte_count += packet.packet_size as u32;
                    ret.push(packet);
                    log::trace!("parsed {} / {} bytes", parsed_byte_count, total_bytes_read);
                },
                Err(e) => {
                    log::error!("error parsing packet: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(ret)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut ret: Vec<u8> = vec![];

        log::trace!("reading bytes from stream");

        let stream = self.get_stream_mut()?;

        const LEN_READ_BYTES: usize = 256;
        let mut buf: [u8; LEN_READ_BYTES] = [0; LEN_READ_BYTES];

        loop {
            let n = stream.read(&mut buf)?;
            log::trace!("bytes read: {}", n);
            for i in 0..n {
                ret.push(buf[i]);
            }
            if n < LEN_READ_BYTES {
                break
            }
        }

        log::trace!("total bytes read: {}", ret.len());

        Ok(ret)
    }

    fn read_packets(&mut self) -> Result<Vec<Packet>, Error> {
        let bytes_array = self.read_bytes()?;
        let packets = self.bytes_to_packets(&bytes_array)?;
        Ok(packets)
    }

    pub fn read_messages(&mut self) -> Result<Vec<ProtoMessage>, Error> {
        let packets = self.read_packets()?;
        let messages = self.packets_to_messages(&packets)?;
        Ok(messages)
    }

    pub fn request_message_and_await_response(&mut self, message_type: MessageType) -> Result<ProtoMessage, Error> {
        self.request_message(message_type)?;

        let max_attempts = 3;
        let mut current_attempt = 0;
        while current_attempt < max_attempts {
            let packets = self.read_packets()?;
            for packet in packets {
                // if packet.message_type_value == message_type.into() {
                if packet.message_type_value == <MessageType as Into<u16>>::into(message_type) {
                    let message = ProtoMessage::try_from(&packet)?;
                    return Ok(message);
                }
            }
            current_attempt += 1;
        }
        return Err(Error::new(ErrorKind::NotFound, "did not receive requested message"));
    }

    pub fn send_command(&mut self, command_message: proto::Command::Command) -> Result<(), Error> {
        let payload = command_message.write_to_bytes()?;
        self.write_packet(MessageType::Command.into(), payload)?;
        Ok(())
    }
}
