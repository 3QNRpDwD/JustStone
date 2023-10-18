use core::slice::SlicePattern;
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::structure::{StructStoneHeader, StructRawStonePayload, StructStone, StructStonePayload};

pub struct Session {
    ip_port: String,
    socket: TcpStream,
}

impl Session {
    pub fn new(ip_port: String) -> Session {
        let mut socket;

        if let Ok(s) = TcpStream::connect(ip_port.clone()) {
            socket = s;

            let ssp= StructRawStonePayload::to_vec(
                                    StructRawStonePayload {
                                            sysinfo: String::from("sysinfo.."),
                                            command_input: String::from("command_input.."),
                                            command_output: String::from("command_output.."),
                                            stone_chain: String::from("stone_chain.."),
                                    });
            println!("보낸거 : {:?}", ssp);
            let ssh = StructStoneHeader::from(&ssp);
            println!("보낸거 : {:?}", ssh);
            let stone    = StructStone::from(ssh, ssp);

            socket.write_all(&stone.stone).expect("TODO: panic message");

            Session { ip_port, socket }
        } else {
            Self::new(ip_port)
        }
    }
}

pub trait Client {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error>;
    fn parsing_packet(&mut self, packet: StructStone) -> StructStonePayload;
    fn recv_stone(&mut self) ->  Result<StructStone, ()>;

}

impl Client for Session {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error> {
        self.socket.write_all(stone)?;
        Ok(())
    }

    fn parsing_packet(&mut self, packet: StructStone) -> StructStonePayload {

        println!("{}", u32::from_ne_bytes(packet.header.stone_size[8..12].try_into().unwrap()));

        StructStonePayload{
            sysinfo: vec![],
            command_input: vec![],
            command_output: vec![],
            stone_chain: vec![],
        }
    }

    fn recv_stone(&mut self) -> Result<StructStone, ()> {
        let mut buffer = [0; 12];

        match self.socket.read_exact(&mut buffer) {
            Ok(_) => {

                if buffer.len() == 12 {

                    let header = StructStoneHeader {
                        stone_status: Vec::from(&buffer[0..4]),
                        stone_type: Vec::from(&buffer[4..8]),
                        stone_size: Vec::from(&buffer[8..12]),
                    };

                    let temp_payload = StructStonePayload {
                        sysinfo: vec![],
                        command_input: vec![],
                        command_output: vec![],
                        stone_chain: vec![],
                    };

                    Ok(StructStone::from(header, temp_payload))
                } else {

                    let temp_header = StructStoneHeader {
                        stone_status: vec![],
                        stone_type: vec![],
                        stone_size: vec![],
                    };

                    let payload = StructStonePayload {
                        sysinfo: vec![],
                        command_input: vec![],
                        command_output: vec![],
                        stone_chain: vec![],
                    };

                    Ok(StructStone::from(temp_header, payload))
                }
            },
            Err(_) => Err(())
        }
    }
}




