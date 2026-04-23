#![allow(dead_code)]

use std::net::TcpStream;
use std::time::Duration;
use std::io::{self, Error, ErrorKind, Read, Write};


const HEADER_SIZE: usize = 20;
const HEADER_PREAMBLE: [u8; 4] = [171, 173, 29, 58];
const HEADER_MAGIC: i32 = -1414718150;


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

    pub fn write_packet(&mut self, message_type: u16, payload: Vec<u8>) -> Result<(), Error> {
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

    pub fn read_packets(&mut self) -> Result<(), Error> {
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
                let mut buf_accumulator: Vec<u8> = vec![];

                println!("reading stream");

                loop {
                    match stream.read(&mut buf) {
                        Ok(n) => {
                            println!("read {} bytes", n);
                            for i in 0..n {
                                buf_accumulator.push(buf[i]);
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
                println!("total bytes read: {}", buf_accumulator.len());

                println!("parsing bytes into packets");

                let total_bytes_read: u32 = buf_accumulator.len() as u32;
                let mut parsed_byte_count: u32 = 0;

                while parsed_byte_count < total_bytes_read {
                    let mut packet = Packet::new();
                    let mut buf_parse: Vec<u8> = vec![];
                    buf_parse.extend_from_slice(&buf_accumulator[parsed_byte_count as usize..total_bytes_read as usize]);
                    match packet.deserialize(&buf_parse) {
                        Ok(_) => {
                            parsed_byte_count += packet.packet_size as u32;
                            println!("parsed {} / {} bytes", parsed_byte_count, total_bytes_read);
                        },
                        Err(e) => {
                            println!("error deserializing packet: {}", e);
                            return Err(e);
                        }
                    }
                }
            },
            None => {
                println!("not connected!");
                return Err(Error::new(ErrorKind::NotConnected, format!("not connected")));
            }
        }
        return Ok(());
    }

    pub fn test_packet(&self) -> () {
        let mut packet: Packet = Packet::new();
        packet.serialize(2, vec![]);
    }

}
