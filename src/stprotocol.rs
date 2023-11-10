use std::io::{Read, Write};
use std::net::TcpStream;
use std::u8;
use crate::structure::{StructStoneHeader, StructRawStonePayload, StructStone, StructStonePayload, Generator};

pub struct Session {
    ip_port: String,
    socket: TcpStream,
    packet: StructStone,
}

impl Session {
    pub fn new(ip_port: String) -> Session {
        let mut socket;

        if let Ok(s) = TcpStream::connect(ip_port.clone()) {
            socket = s;

            let packet = StructRawStonePayload {
                sysinfo: String::from("sysinfo.."),
                command_input: String::from("command_input.."),
                command_output: String::from("command_output.."),
                stone_chain: String::from("stone_chain.."),
            }.generator();


            socket.write_all(&packet.stone).expect("TODO: panic message");


            Session { ip_port, socket , packet }
        } else {
            Self::new(ip_port)
        }
    }

    pub fn set() {

    }
}

pub trait Client {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error>;
    fn parsing_packet(&mut self, packet: Vec<u8>) -> StructStonePayload;
    fn detect_header_type(&mut self, header: Vec<u8>) -> bool;
    fn receiving(&mut self) -> StructStone;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
}

impl Client for Session {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error> {
        self.socket.write_all(stone)?;
        Ok(())
    }

    fn parsing_packet(&mut self, packet: Vec<u8>) -> StructStonePayload {
        let sting_packet = String::from_utf8_lossy(&packet).to_string();
        let split_packet = sting_packet.split("..");

        println!("{:?}", split_packet);

        StructStonePayload::default()

    }

    fn detect_header_type(&mut self, header: Vec<u8>) -> bool {
        todo!()
    }
    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {

        let mut buffer : Vec<u8> = vec![0; buffer_size];

        match self.socket.read_exact(&mut buffer) {
            Ok(_) => {
                buffer
            }
            Err(_) => {}
        }
    }

    fn receiving(&mut self) -> StructStone {
        let mut header = StructStoneHeader::default();
        let packet = self.recv(12);

        if packet {
                header = StructStoneHeader {
                    stone_status: Vec::from(&buffer[0..4]),
                    stone_type: Vec::from(&buffer[4..8]),
                    stone_size: Vec::from(&buffer[8..12]),
                };
                self.packet = StructStone::new(header.clone(), StructStonePayload::default(), Vec::new());
                self.recv_payload();
            }

        header
        let length_bytes: &[u8] = &self.packet.header.stone_size;
        let length = u32::from_le_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]);
        let mut buffer : Vec<u8> = vec![0; length as usize];
        println!("{:?}", buffer);

        StructStone::default()
    }
}




