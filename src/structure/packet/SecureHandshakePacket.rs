use crate::{
    structure::utils::{
        enums::{
            EncryptType,
            HandshakeType,
            ParseError,
        },
        structs::define::{
            SecureHandshakePacket,
            StructStone,
        },
        traits::define::ProtocolCodec,
    },
    utility::secure::{
        crypto::Crypto,
        utils::RsaCrypto,
    },
};
use crate::structure::utils::structs::define::EncryptionInfo;

impl SecureHandshakePacket {
    pub fn build(mut source: StructStone, encryption_info: &EncryptionInfo) -> Result<SecureHandshakePacket, ParseError> {
        let packet = SecureHandshakePacket::new();

        println!("{:?}", encryption_info);

        if encryption_info.encryption_type != EncryptType::AesGcmSiv {
            return Err(ParseError::Unimplemented("EncryptionInfo algorithms other than AesGcmSiv have not yet been implemented.".to_string()));
        }

        let mut handshake_method = match encryption_info.handshake_type {
            HandshakeType::RSA => RsaCrypto::default(),
            HandshakeType::DiffieHellman => return Err(ParseError::Unimplemented("The handshake method using DiffieHellman algorithm is still incomplete. Please use the RSA handshake method.".to_string())),
            HandshakeType::NoHandshake => return Err(ParseError::Unimplemented("No Handshake".to_string())),
        };

        source.stone = handshake_method.encrypt(source.stone);
        packet.set(source.stone.len(), encryption_info.handshake_type.to_vec(), encryption_info.encryption_type.to_vec(), source)
    }
}