use byteorder::{BigEndian, ByteOrder};
use prost::{DecodeError, Message};

pub mod mumble {
    pub mod proto {
        include!(concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
    }
}

pub trait NetworkMessage {
    fn message_type(&self) -> u16;
}

macro_rules! message_builder {
    ($($value:expr => $proto:ty),*) => {
        $(impl NetworkMessage for $proto {
            fn message_type(&self) -> u16 {
                $value
            }
        })*

        pub fn get_message(id: u16, buf: &[u8]) -> Result<Box<dyn Message>, DecodeError> {
            match id {
                $( $value => {
                    let value = <$proto>::decode(buf);
                    match value {
                        Ok(v) => Ok(Box::new(v)),
                        Err(e) => Err(e)
                    }
                } ),*
                _ => Err(DecodeError::new("Invalid message")),
            }
        }
    };
}

message_builder! {
    0 => mumble::proto::Version,
    1 => mumble::proto::UdpTunnel,
    2 => mumble::proto::Authenticate,
    3 => mumble::proto::Ping,
    4 => mumble::proto::Reject,
    5 => mumble::proto::ServerSync,
    6 => mumble::proto::ChannelRemove,
    7 => mumble::proto::ChannelState,
    8 => mumble::proto::UserRemove,
    9 => mumble::proto::UserState,
    10 => mumble::proto::BanList,
    11 => mumble::proto::TextMessage,
    12 => mumble::proto::PermissionDenied,
    13 => mumble::proto::Acl,
    14 => mumble::proto::QueryUsers,
    15 => mumble::proto::CryptSetup,
    16 => mumble::proto::ContextActionModify,
    17 => mumble::proto::ContextAction,
    18 => mumble::proto::UserList,
    19 => mumble::proto::VoiceTarget,
    20 => mumble::proto::PermissionQuery,
    21 => mumble::proto::CodecVersion,
    22 => mumble::proto::UserStats,
    23 => mumble::proto::RequestBlob,
    24 => mumble::proto::ServerConfig,
    25 => mumble::proto::SuggestConfig
}

pub fn message_builder<T>(message: T) -> Vec<u8>
where
    T: NetworkMessage + Message,
{
    let message_type = message.message_type();
    let payload = message.encode_to_vec();
    let length = payload.len() as u32;

    let mut new_buffer = vec![0; (length + 6) as usize];
    BigEndian::write_u16(&mut new_buffer, message_type);
    BigEndian::write_u32(&mut new_buffer[2..], length);
    new_buffer[6..].copy_from_slice(&payload);

    new_buffer
}
