use std::io::{Read, Write};
use std::net::TcpStream;
use crate::structure::{StructStoneHeader, StructRawStonePayload, StructStone};

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
            println!("보낸거");
            println!("{:?}", ssp);
            let ssh = StructStoneHeader::from(&ssp);
            println!("{:?}", ssh);
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
    fn recv_stone(&mut self) ->  Result<StructStoneHeader, ()>;

}

impl Client for Session {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error> {
        self.socket.write_all(stone)?;
        Ok(())
    }

    fn recv_stone(&mut self) -> Result<StructStoneHeader, ()> {
        let mut buffer = [0; 16];

        match self.socket.read_exact(&mut buffer) {
            Ok(_) => {
                let ssh = StructStoneHeader {
                    stone_status: Vec::from(&buffer[0..4]),
                    stone_type: Vec::from(&buffer[4..8]),
                    stone_size: Vec::from(&buffer[8..16]),
                };

                Ok(ssh)
            },
            Err(_) => Err(())
        }
    }
}




