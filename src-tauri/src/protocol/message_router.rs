use std::{collections::VecDeque, error::Error};

use tokio::sync::broadcast::Sender;
use tracing::{error, trace};

use crate::{
    connection::MessageChannels,
    errors::application_error::ApplicationError,
    manager::{
        channel_manager::ChannelManager, connection_manager::ConnectionManager,
        text_message_manager::TextMessageManager, user_manager::UserManager,
    },
    utils::{
        messages::{mumble, MessageInfo},
        varint::parse_varint,
    },
};

pub struct MessageRouter {
    user_manager: UserManager,
    channel_manager: ChannelManager,
    text_manager: TextMessageManager,
    connection_manager: ConnectionManager,
}

impl MessageRouter {
    pub fn new(sender: MessageChannels, server_channel: Sender<Vec<u8>>) -> MessageRouter {
        MessageRouter {
            user_manager: UserManager::new(sender.message_channel.clone(), server_channel.clone()),
            channel_manager: ChannelManager::new(
                sender.message_channel.clone(),
                server_channel.clone(),
            ),
            text_manager: TextMessageManager::new(sender.message_channel.clone()),
            connection_manager: ConnectionManager::new(
                sender.message_channel.clone(),
                server_channel.clone(),
            ),
        }
    }

    fn handle_downcast<T: 'static>(&self, message_info: MessageInfo) -> Result<T, Box<dyn Error>> {
        match message_info.message_data.downcast::<T>() {
            Ok(a) => return Ok(*a),
            Err(e) => Err(Box::new(ApplicationError::new(
                format!("Invalid message type {:?}", e).as_str(),
            ))),
        }
    }

    fn handle_text_message(&mut self, message: MessageInfo) -> Result<(), Box<dyn Error>> {
        let text_message = self.handle_downcast::<mumble::proto::TextMessage>(message)?;
        match text_message.actor {
            Some(actor) => {
                let actor = self
                    .user_manager
                    .get_user_by_id(actor)
                    .ok_or_else(|| Box::new(ApplicationError::new("msg")) as Box<dyn Error>)
                    .map_err(|e| e)?;
                self.text_manager.add_text_message(text_message, actor)?;
            }
            None => {
                error!("Received text message without actor: {:?}", text_message);
            }
        }
        Ok(())
    }

    pub fn recv_message(&mut self, message: MessageInfo) -> Result<(), Box<dyn Error>> {
        if message.message_type != crate::utils::messages::MessageTypes::UdpTunnel {
            trace!("Received message: {:<100?}", message);
        }

        match message.message_type {
            crate::utils::messages::MessageTypes::Version => {}
            crate::utils::messages::MessageTypes::UdpTunnel => {
                let mut audio_data = self.handle_downcast::<VecDeque<u8>>(message)?;
                //trace!("Received audio data: {:?}", audio_data);
                let audio_header = audio_data.pop_front().unwrap();

                let audio_type = (audio_header & 0xE0) >> 5;
                let audio_target = audio_header & 0x1F;
                if audio_type != 4 {
                    return Ok(());
                }

                let session_id = parse_varint(audio_data.make_contiguous())?;
                audio_data.drain(0..(session_id.1 as usize));

                let sequence_number = parse_varint(audio_data.make_contiguous())?;
                audio_data.drain(0..(sequence_number.1 as usize));

                let opus_header = parse_varint(audio_data.make_contiguous())?;
                audio_data.drain(0..(opus_header.1 as usize));

                /*trace!(
                    "Type: {:?} | Target: {:?} | Session: {:?} | Sequence: {:?} | Opus: {:?}",
                    audio_type,
                    audio_target,
                    session_id.0,
                    sequence_number.0,
                    opus_header.0
                );*/
            }
            crate::utils::messages::MessageTypes::Authenticate => {}
            crate::utils::messages::MessageTypes::Ping => {}
            crate::utils::messages::MessageTypes::Reject => {
                let reject = self.handle_downcast::<mumble::proto::Reject>(message)?;
                self.connection_manager.notify_disconnected(&reject.reason);
                return Err(Box::new(ApplicationError::new(
                    format!("Received reject message: {:?}", reject.reason).as_str(),
                )));
            }
            crate::utils::messages::MessageTypes::ServerSync => {
                let server_sync = self.handle_downcast::<mumble::proto::ServerSync>(message)?;
                self.user_manager.notify_current_user(server_sync);
                self.connection_manager.notify_connected();
            }
            crate::utils::messages::MessageTypes::ChannelRemove => {
                let removed_channel =
                    self.handle_downcast::<mumble::proto::ChannelRemove>(message)?;
                self.channel_manager.remove_channel(removed_channel);
            }
            crate::utils::messages::MessageTypes::ChannelState => {
                let changed_channel =
                    self.handle_downcast::<mumble::proto::ChannelState>(message)?;
                self.channel_manager.update_channel(changed_channel)?;
            }
            crate::utils::messages::MessageTypes::UserRemove => {
                let removed_user = self.handle_downcast::<mumble::proto::UserRemove>(message)?;
                self.user_manager.remove_user(removed_user);
            }
            crate::utils::messages::MessageTypes::UserState => {
                let changed_user = self.handle_downcast::<mumble::proto::UserState>(message)?;
                self.user_manager.update_user(changed_user)?;
            }
            crate::utils::messages::MessageTypes::BanList => {}
            crate::utils::messages::MessageTypes::TextMessage => {
                self.handle_text_message(message)?;
            }
            crate::utils::messages::MessageTypes::PermissionDenied => {}
            crate::utils::messages::MessageTypes::Acl => {}
            crate::utils::messages::MessageTypes::QueryUsers => {}
            crate::utils::messages::MessageTypes::CryptSetup => {}
            crate::utils::messages::MessageTypes::ContextActionModify => {}
            crate::utils::messages::MessageTypes::ContextAction => {}
            crate::utils::messages::MessageTypes::UserList => {}
            crate::utils::messages::MessageTypes::VoiceTarget => {}
            crate::utils::messages::MessageTypes::PermissionQuery => {}
            crate::utils::messages::MessageTypes::CodecVersion => {}
            crate::utils::messages::MessageTypes::UserStats => {}
            crate::utils::messages::MessageTypes::RequestBlob => {}
            crate::utils::messages::MessageTypes::ServerConfig => {}
            crate::utils::messages::MessageTypes::SuggestConfig => {}
            crate::utils::messages::MessageTypes::PluginDataTransmission => {}
        };

        Ok(())
    }
}
