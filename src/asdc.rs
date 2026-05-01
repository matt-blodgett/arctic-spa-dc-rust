#![allow(dead_code)]


use std::net::TcpStream;
use std::time::Duration;
use std::io::{Error, ErrorKind, Read, Write};

use protobuf::Message;

use crate::proto;


const HEADER_SIZE: usize = 20;
const HEADER_PREAMBLE: [u8; 4] = [171, 173, 29, 58];
const HEADER_MAGIC: i32 = -1414718150;


// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
    packet_size: u16
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
            packet_size: HEADER_SIZE as u16
        }
    }

    pub fn serialize(&mut self, message_type_value: u16, payload: Vec<u8>) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        log::debug!("serializing packet: message_type_value={:?}", message_type_value);

        self.preamble = HEADER_PREAMBLE;
        let padding: u32 = 0;  // 4 bytes reserved for checksum after calculation
        self.counter = 0;
        self.unused = 0;
        self.message_type_value = message_type_value;
        self.message_type = MessageType::try_from(message_type_value).ok();
        self.payload_size = payload.len() as u16;
        self.payload = payload;
        self.packet_size = HEADER_SIZE as u16 + self.payload_size;

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
        log::debug!("deserializing packet: bytes_len={}", data.len());

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

        log::debug!("successfully deserialized packet: message_type={:?}, payload_size={}", self.message_type, self.payload_size);
        log::trace!("packet: {:?}", self);

        Ok(())
    }
}


#[derive(Debug)]
pub enum ProtoMessage {
    Live(proto::Live::Live),
    Command(proto::Command::Command),
    Settings(proto::Settings::Settings),
    Configuration(proto::Configuration::Configuration),
    Peak(proto::Peak::Peak),
    Clock(proto::Clock::Clock),
    Information(proto::Information::Information),
    Error(proto::Error::Error),
    Router(proto::Router::Router),
    Filter(proto::Filter::Filter),
    Peripheral(proto::Peripheral::Peripheral),
    OnzenLive(proto::OnzenLive::OnzenLive),
    OnzenSettings(proto::OnzenSettings::OnzenSettings)
}

// impl ProtoMessage {
//     pub fn as_live(&self) -> Option<&proto::Live::Live> {
//         match self {
//             ProtoMessage::Live(msg) => Some(msg),
//             _ => None,
//         }
//     }

//     pub fn as_settings(&self) -> Option<&proto::Settings::Settings> {
//         match self {
//             ProtoMessage::Settings(msg) => Some(msg),
//             _ => None,
//         }
//     }
//     // ... etc for each variant
// }

impl TryFrom<&Packet> for ProtoMessage {
    type Error = Error;

    fn try_from(value: &Packet) -> Result<Self, Error> {
        log::debug!("parsing packet payload to protobuf message: message_type_value={:?}, payload_size={}", value.message_type_value, value.payload_size);

        let message_type = value.message_type
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid message type in packet"))?;

        match message_type {
            MessageType::Live => Ok(ProtoMessage::Live(proto::Live::Live::parse_from_bytes(&value.payload)?)),
            MessageType::Command => Ok(ProtoMessage::Command(proto::Command::Command::parse_from_bytes(&value.payload)?)),
            MessageType::Settings => Ok(ProtoMessage::Settings(proto::Settings::Settings::parse_from_bytes(&value.payload)?)),
            MessageType::Configuration => Ok(ProtoMessage::Configuration(proto::Configuration::Configuration::parse_from_bytes(&value.payload)?)),
            MessageType::Peak => Ok(ProtoMessage::Peak(proto::Peak::Peak::parse_from_bytes(&value.payload)?)),
            MessageType::Clock => Ok(ProtoMessage::Clock(proto::Clock::Clock::parse_from_bytes(&value.payload)?)),
            MessageType::Information => Ok(ProtoMessage::Information(proto::Information::Information::parse_from_bytes(&value.payload)?)),
            MessageType::Error => Ok(ProtoMessage::Error(proto::Error::Error::parse_from_bytes(&value.payload)?)),
            MessageType::Router => Ok(ProtoMessage::Router(proto::Router::Router::parse_from_bytes(&value.payload)?)),
            MessageType::Heartbeat => Err(Error::new(ErrorKind::Unsupported, "no corresponding proto for message type Heartbeat")),
            MessageType::Filter => Ok(ProtoMessage::Filter(proto::Filter::Filter::parse_from_bytes(&value.payload)?)),
            MessageType::Peripheral => Ok(ProtoMessage::Peripheral(proto::Peripheral::Peripheral::parse_from_bytes(&value.payload)?)),
            MessageType::OnzenLive => Ok(ProtoMessage::OnzenLive(proto::OnzenLive::OnzenLive::parse_from_bytes(&value.payload)?)),
            MessageType::OnzenSettings => Ok(ProtoMessage::OnzenSettings(proto::OnzenSettings::OnzenSettings::parse_from_bytes(&value.payload)?))
        }
    }
}


pub struct NetworkClient {
    host: String,
    port: String,
    tcp_stream: Option<TcpStream>
}

impl NetworkClient {
    pub fn new() -> Self {
        Self {
            host: "".to_string(),
            port: "65534".to_string(),
            tcp_stream: None
        }
    }

    pub fn connect(&mut self, host: &str) -> Result<(), Error> {
        self.host = host.to_string();
        let addr = format!("{}:{}", self.host, self.port);
        log::info!("creating tcp connection: host={:?}, port={:?}", self.host, self.port);
        let stream = TcpStream::connect(addr)?;
        log::debug!("setting stream to blocking mode");
        stream.set_nonblocking(false)?;
        self.tcp_stream = Some(stream);
        let default_timeout_ms = 3_000;
        self.set_timeout_ms(default_timeout_ms)?;
        log::info!("successfully connected to {:?}", host);
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
        Ok(())
    }

    pub fn request_message(&mut self, message_type: MessageType) -> Result<(), Error> {
        log::info!("requesting message {:?}", message_type);
        self.write_packet(message_type.into(), vec![])?;
        Ok(())
    }

    fn packets_to_messages(&self, packets: &Vec<Packet>) -> Result<Vec<ProtoMessage>, Error> {
        let mut ret: Vec<ProtoMessage> = vec![];

        log::info!("parsing {} packets to messages", packets.len());

        for packet in packets.iter() {
            let msg = match ProtoMessage::try_from(packet) {
                Ok(m) => m,
                Err(e) => {
                    log::error!("error parsing message: {}", e);
                    continue;
                }
            };
            ret.push(msg);
        }

        Ok(ret)
    }

    fn bytes_to_packets(&self, bytes_array: &Vec<u8>) -> Result<Vec<Packet>, Error> {
        let mut ret: Vec<Packet> = vec![];

        log::info!("parsing {} bytes into packets", bytes_array.len());

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
                    return Err(e);
                }
            }
        }

        Ok(ret)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut ret: Vec<u8> = vec![];

        log::info!("reading bytes from stream");

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

        log::info!("total bytes read: {}", ret.len());

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
                if packet.message_type_value == message_type.into() {
                    let message = ProtoMessage::try_from(&packet)?;
                    return Ok(message);
                }
            }
            current_attempt += 1;
        }
        return Err(Error::new(ErrorKind::NotFound, "did not receive requested message"));
    }
}
