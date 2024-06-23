use std::fmt::Write;
use std::mem::replace;

use crate::{
    structure::{
        utils::{
            structs::define::EncryptionInfo,
            enums::{
                StatusCode,
                StoneTransferProtocol,
            },
            structs::define::{
                SecureHandshakePacket,
                StructStoneHeader,
                StructStonePayload,
            },
            traits::{
                define::{
                    Detector,
                    ProtocolCodec
                }
            }
        }
    }
};
use crate::structure::utils::enums::{EncryptType, HandshakeType};

impl Detector for SecureHandshakePacket {
    fn display(&self) {
        let mut output = String::new();

        writeln!(output, "
        handshake_type: {:?}
        encrypt_type:   {:?}
            Header:
                Status: {:?}
                Type:   {:?}
                Size:   {:?}
            Payload:
                System information: {:?}
                Command input:      {:?}
                Response:           {:?}
                file:               {:?}",
                 self.handshake_type,
                 self.encrypt_type,
                 self.get_type(),
                 self.get_type(),
                 self.get_size(),
                 self.take_sysinfo(),
                 self.take_command(),
                 self.take_response(),
                 self.take_file()
        ).unwrap();
        print!("{}", output)
    }

    fn get_status(&self) -> StatusCode {
        StatusCode::get_type(&self.origin_packet.header.stone_status)
    }

    fn get_type(&self) -> StoneTransferProtocol {
        StoneTransferProtocol::get_type(&self.origin_packet.header.stone_type)
    }

    fn get_size(&self) -> usize {
        let length = u32::from_le_bytes([
            self.encrypt_data_block_length[0],
            self.encrypt_data_block_length[1],
            self.encrypt_data_block_length[2],
            self.encrypt_data_block_length[3],
        ]) as usize
            + u32::from_le_bytes([
            self.encrypt_data_block_length[4],
            self.encrypt_data_block_length[5],
            self.encrypt_data_block_length[6],
            self.encrypt_data_block_length[7],
        ]) as usize;
        usize::from(length)
    }

    fn get_encryption(&self) -> EncryptionInfo {
        EncryptionInfo {
            Activated: self.is_encryption(),
            Type: EncryptType::get_type(&self.encrypt_type),
            Handshake_Type: HandshakeType::get_type(&self.handshake_type),
        }
    }

    fn get_header(&mut self) -> StructStoneHeader { replace(&mut self.origin_packet.header, Default::default()) }
    fn get_payload(&mut self) -> StructStonePayload { replace(&mut self.origin_packet.payload, Default::default()) }
    fn get_sysinfo(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.sysinfo, Default::default()) }
    fn get_command(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.command_input, Default::default()) }
    fn get_response(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.response, Default::default()) }
    fn get_file(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.file, Default::default()) }
    fn get_stone(&mut self) -> Option<Vec<u8>> { Option::from(replace(&mut self.encrypted_packet, Default::default())) }
    fn take_header(&self) -> Option<&StructStoneHeader> {
        Option::from(&self.origin_packet.header)
    }
    fn take_payload(&self) -> Option<&StructStonePayload> {
        Option::from(&self.origin_packet.payload)
    }
    fn take_sysinfo(&self) -> Option<&Vec<u8>> {
        Option::from(&self.origin_packet.payload.sysinfo)
    }
    fn take_command(&self) -> Option<&Vec<u8>> {
        Option::from(&self.origin_packet.payload.command_input)
    }
    fn take_response(&self) -> Option<&Vec<u8>> {
        Option::from(&self.origin_packet.payload.response)
    }
    fn take_file(&self) -> Option<&Vec<u8>> {
        Option::from(&self.origin_packet.payload.file)
    }
    fn take_stone(&self) -> Option<&Vec<u8>> {
        Option::from(&self.encrypted_packet)
    }
    fn is_compression(&self) -> bool {
        self.origin_packet.header.is_compression()
    }
    fn is_encryption(&self) -> bool {
        self.origin_packet.header.is_encrypted()
    }
}