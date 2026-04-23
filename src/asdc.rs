#![allow(dead_code)]


use std::net::TcpStream;
use std::time::Duration;
use std::io::{self, Error, ErrorKind, Read, Write};

use protobuf::Message;
mod proto {
    include!("./proto/mod.rs");
}


const HEADER_SIZE: usize = 20;
const HEADER_PREAMBLE: [u8; 4] = [171, 173, 29, 58];
const HEADER_MAGIC: i32 = -1414718150;


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

pub fn int_to_message_type (message_type_int: u16) -> Result<MessageType, Error> {
    if message_type_int == MessageType::Live as u16 {
        return Ok(MessageType::Live);
    } else if message_type_int == MessageType::Command as u16 {
        return Ok(MessageType::Command);
    } else if message_type_int == MessageType::Settings as u16 {
        return Ok(MessageType::Settings);
    } else if message_type_int == MessageType::Configuration as u16 {
        return Ok(MessageType::Configuration);
    } else if message_type_int == MessageType::Peak as u16 {
        return Ok(MessageType::Peak);
    } else if message_type_int == MessageType::Clock as u16 {
        return Ok(MessageType::Clock);
    } else if message_type_int == MessageType::Information as u16 {
        return Ok(MessageType::Information);
    } else if message_type_int == MessageType::Router as u16 {
        return Ok(MessageType::Router);
    } else if message_type_int == MessageType::Heartbeat as u16 {
        return Ok(MessageType::Heartbeat);
    } else if message_type_int == MessageType::Filter as u16 {
        return Ok(MessageType::Filter);
    } else if message_type_int == MessageType::Peripheral as u16 {
        return Ok(MessageType::Peripheral);
    } else if message_type_int == MessageType::OnzenLive as u16 {
        return Ok(MessageType::OnzenLive);
    } else if message_type_int == MessageType::OnzenSettings as u16 {
        return Ok(MessageType::OnzenSettings);
    }
    return Err(Error::new(ErrorKind::InvalidData, "invalid message type"))
}


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


struct Packet {
    preamble: [u8; 4],
    checksum: [u8; 4],
    counter: u32,
    unused: u32,
    message_type: u16,
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
            message_type: 0,
            payload_size: 0,
            payload: vec![],
            packet_size: HEADER_SIZE as u16
        }
    }

    pub fn serialize(&mut self, message_type: u16, payload: Vec<u8>) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        self.preamble = HEADER_PREAMBLE;
        let padding: u32 = 0;
        self.counter = 0;
        self.unused = 0;
        self.message_type = message_type;
        self.payload_size = payload.len() as u16;
        self.payload = payload;
        self.packet_size = HEADER_SIZE as u16 + self.payload_size;

        ret.extend_from_slice(&self.preamble);
        ret.extend_from_slice(&padding.to_be_bytes());
        ret.extend_from_slice(&self.counter.to_be_bytes());
        ret.extend_from_slice(&self.unused.to_be_bytes());
        ret.extend_from_slice(&self.message_type.to_be_bytes());
        ret.extend_from_slice(&self.payload_size.to_be_bytes());
        ret.extend_from_slice(&self.payload);

        // println!("packet before checksum: {ret:?}");
        // let packet_len_before_checksum = ret.len();
        // println!("packet len before checksum: {packet_len_before_checksum:}");

        // Calculate checksum on the current packet
        const CRC32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let checksum_value = CRC32.checksum(&ret);
        let checksum_bytes = checksum_value.to_be_bytes();
        self.checksum = checksum_bytes;

        // Replace the padding field (bytes 4-8) with the checksum
        ret[4..8].copy_from_slice(&checksum_bytes);

        // println!("checksum: {checksum_value:?}");
        // println!("packet after checksum: {ret:?}");
        // let packet_len = ret.len();
        // println!("packet len: {packet_len:}");

        return ret;
    }

    pub fn deserialize(&mut self, data: &Vec<u8>) -> Result<(), Error> {
        if data.len() < HEADER_SIZE {
            return Err(Error::new(ErrorKind::InvalidData, format!("got {} bytes; expected at least {}", data.len(), HEADER_SIZE)));
        }
        if data.get(0..4) != HEADER_PREAMBLE.get(0..4) {
            return Err(Error::new(ErrorKind::InvalidData, format!("invalid preamble!")));
        }

        self.preamble.copy_from_slice(&data[0..4]);
        self.checksum.copy_from_slice(&data[4..8]);

        self.counter = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
        self.unused = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);

        self.message_type = u16::from_be_bytes([data[16], data[17]]);
        self.payload_size = u16::from_be_bytes([data[18], data[19]]);
        self.payload.extend_from_slice(&data[HEADER_SIZE..HEADER_SIZE + self.payload_size as usize]);
        self.packet_size = HEADER_SIZE as u16 + self.payload_size;

        println!("-------------------------");
        let payload = &self.payload;
        println!("deserialized packet:");
        println!("message_type={}", self.message_type);
        println!("packet_size={}", self.packet_size);
        println!("payload_size={}", self.payload_size);
        println!("payload={payload:?}");
        println!("-------------------------");

        return Ok(());
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
        println!("connecting to host \"{addr:?}\"");
        let stream = TcpStream::connect(addr)?;
        println!("successfully connected");
        self.tcp_stream = Some(stream);
        return Ok(());
    }

    fn write_packet(&mut self, message_type: u16, payload: Vec<u8>) -> Result<(), Error> {
        match &mut self.tcp_stream {
            Some(stream) => {
                let mut packet = Packet::new();
                let packet_bytes = packet.serialize(message_type, payload);
                println!("writing packet - message type \"{message_type:?}\"");
                stream.write_all(&packet_bytes)?;
            },
            None => {
                println!("not connected!");
                return Err(Error::new(ErrorKind::NotConnected, format!("not connected")));
            }
        }
        return Ok(());
    }

    pub fn request_message(&mut self, message_type: MessageType) -> Result<(), Error> {
        self.write_packet(message_type as u16, vec![])?;
        return Ok(());
    }

    fn packets_to_messages(&self, packets: &Vec<Packet>) -> Result<Vec<ProtoMessage>, Error> {
        let mut ret: Vec<ProtoMessage> = vec![];

        println!("parsing packets to messages");

        for packet in packets.iter() {
            let message_type: MessageType = int_to_message_type(packet.message_type)?;

            match message_type {
                MessageType::Live => {
                    let msg = proto::Live::Live::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Live(msg));
                },
                MessageType::Command => {
                    let msg = proto::Command::Command::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Command(msg));
                },
                MessageType::Settings => {
                    let msg = proto::Settings::Settings::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Settings(msg));
                },
                MessageType::Configuration => {
                    let msg = proto::Configuration::Configuration::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Configuration(msg));
                },
                MessageType::Peak => {
                    let msg = proto::Peak::Peak::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Peak(msg));
                },
                MessageType::Clock => {
                    let msg = proto::Clock::Clock::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Clock(msg));
                },
                MessageType::Information => {
                    let msg = proto::Information::Information::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Information(msg));
                },
                MessageType::Error => {
                    let msg = proto::Error::Error::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Error(msg));
                },
                MessageType::Router => {
                    let msg = proto::Router::Router::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Router(msg));
                },
                MessageType::Heartbeat => {},
                MessageType::Filter => {
                    let msg = proto::Filter::Filter::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Filter(msg));
                },
                MessageType::Peripheral => {
                    let msg = proto::Peripheral::Peripheral::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::Peripheral(msg));
                },
                MessageType::OnzenLive => {
                    let msg = proto::OnzenLive::OnzenLive::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::OnzenLive(msg));
                },
                MessageType::OnzenSettings => {
                    let msg = proto::OnzenSettings::OnzenSettings::parse_from_bytes(&packet.payload)?;
                    ret.push(ProtoMessage::OnzenSettings(msg));
                }
            }
        }

        return Ok(ret);
    }

    fn bytes_to_packets(&self, bytes_array: &Vec<u8>) -> Result<Vec<Packet>, Error> {
        let mut ret: Vec<Packet> = vec![];

        println!("parsing bytes into packets");

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
                    println!("parsed {} / {} bytes", parsed_byte_count, total_bytes_read);
                },
                Err(e) => {
                    println!("error deserializing packet: {}", e);
                    return Err(e);
                }
            }
        }
        return Ok(ret);
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut ret: Vec<u8> = vec![];

        match &mut self.tcp_stream {
            Some(stream) => {
                const READ_TIMEOUT_DURATION_SECONDS: u64 = 5;
                println!("setting read timeout duration seconds = {}", READ_TIMEOUT_DURATION_SECONDS);
                stream.set_read_timeout(Some(Duration::new(READ_TIMEOUT_DURATION_SECONDS, 0)))?;

                const NON_BLOCKING: bool = true;
                println!("setting non blocking = {}", NON_BLOCKING);
                stream.set_nonblocking(NON_BLOCKING)?;

                const TWO: i16 = 2;
                const LEN_READ_BYTES: usize = TWO.pow(8) as usize;
                let mut buf: [u8; LEN_READ_BYTES] = [0; LEN_READ_BYTES];

                println!("reading stream");

                loop {
                    match stream.read(&mut buf) {
                        Ok(n) => {
                            println!("read {} bytes", n);
                            for i in 0..n {
                                ret.push(buf[i]);
                            }
                            if n < LEN_READ_BYTES {
                                break
                            }
                        },
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // No data available right now
                            println!("no data available; trying again in 1 second");
                            std::thread::sleep(Duration::new(1, 0));
                        }
                        Err(e) => return Err(e)
                    }
                }
                println!("total bytes read: {}", ret.len());
            },
            None => {
                println!("not connected!");
                return Err(Error::new(ErrorKind::NotConnected, format!("not connected")));
            }
        }
        return Ok(ret);
    }

    pub fn read_messages(&mut self) -> Result<Vec<ProtoMessage>, Error> {
        let bytes_array: Vec<u8> = self.read_bytes()?;
        let packets: Vec<Packet> = self.bytes_to_packets(&bytes_array)?;
        let messages: Vec<ProtoMessage> = self.packets_to_messages(&packets)?;
        return Ok(messages);
    }
}
