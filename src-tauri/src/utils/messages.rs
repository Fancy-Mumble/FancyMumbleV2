use byteorder::{BigEndian, ByteOrder};
use prost::{DecodeError, Message};
use serde::Serialize;
use std::any::Any;
use tokio::sync::broadcast::Sender;

pub mod mumble {
    pub mod proto {
        include!(concat!(env!("OUT_DIR"), "/mumble_proto.rs"));
    }
}

#[derive(Debug)]
pub struct MessageInfo {
    pub message_type: MessageTypes,
    pub message_data: Box<dyn Any>,
}

#[derive(Debug, Serialize)]
pub struct MessageSendData<T>
where
    T: Clone
{
    message_type: MessageTypes,
    data: T,
}

pub trait NetworkMessage {
    fn message_type(&self) -> u16;
}

macro_rules! message_builder {
    ($($value:expr => $proto:ident),*) => {
        $(impl NetworkMessage for mumble::proto::$proto {
            fn message_type(&self) -> u16 {
                $value
            }
        })*

        #[derive(Debug, Clone, Serialize)]
        pub enum MessageTypes {
            $( $proto ),*
        }

        pub fn downcast_message(data: Box<dyn Any>, message_type: MessageTypes, send_to: Sender<String>) {
                match message_type {
                    $( MessageTypes::$proto => {
                        match data.downcast::<mumble::proto::$proto>() {
                            Ok(b) => {
                                let data = MessageSendData{message_type, data: b.as_ref() };
                                if let Ok(v) = serde_json::to_string(&data) {
                                    _ = send_to.send(v);
                                }
                            }
                            Err(e) => {
                                println!("Type not yet implemented: {:?}, {e:?}", message_type);
                            }
                        };
                    }
                ),*
                }
        }

        pub fn get_message(id: u16, buf: &[u8]) -> Result<MessageInfo, DecodeError> {
            match id {
                $( $value => {
                    let value = <mumble::proto::$proto>::decode(buf);
                    match value {
                        Ok(v) => Ok(MessageInfo{ message_type: MessageTypes::$proto, message_data: Box::new(v)} ),
                        Err(e) => Err(e)
                    }
                } ),*
                _ => Err(DecodeError::new("Invalid message")),
            }
        }
    };
}

message_builder! {
    0 => Version,
    1 => UdpTunnel,
    2 => Authenticate,
    3 => Ping,
    4 => Reject,
    5 => ServerSync,
    6 => ChannelRemove,
    7 => ChannelState,
    8 => UserRemove,
    9 => UserState,
    10 => BanList,
    11 => TextMessage,
    12 => PermissionDenied,
    13 => Acl,
    14 => QueryUsers,
    15 => CryptSetup,
    16 => ContextActionModify,
    17 => ContextAction,
    18 => UserList,
    19 => VoiceTarget,
    20 => PermissionQuery,
    21 => CodecVersion,
    22 => UserStats,
    23 => RequestBlob,
    24 => ServerConfig,
    25 => SuggestConfig
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
