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

impl SecureHandshakePacket {
    pub fn build(mut source: StructStone, handshake_type: &HandshakeType, encrypt_type: &EncryptType) -> Result<SecureHandshakePacket, ParseError> {
        let packet = SecureHandshakePacket::new();

        if encrypt_type != &EncryptType::AesGcmSiv {
            return Err(ParseError::Unimplemented("Encryption algorithms other than AesGcmSiv have not yet been implemented.".to_string()));
        }

        let mut handshake_method = match handshake_type {
            &HandshakeType::RSA => RsaCrypto::default(),
            &HandshakeType::DiffieHellman => return Err(ParseError::Unimplemented("The handshake method using DiffieHellman algorithm is still incomplete. Please use the RSA handshake method.".to_string())),
            &HandshakeType::NoHandshake => return Err(ParseError::Unimplemented("No Handshake".to_string())),
        };

        source.stone = handshake_method.encrypt(source.stone);
        packet.set(source.stone.len(), handshake_type.to_vec(), encrypt_type.to_vec(), source)
    }
}