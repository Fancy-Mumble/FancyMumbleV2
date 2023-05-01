use std::{error::Error};

use tokio::sync::broadcast::Sender;
use tracing::{error, trace};

use crate::{
    connection::MessageChannels,
    errors::application_error::ApplicationError,
    manager::{text_message_manager::TextMessageManager, user_manager::UserManager, channel_manager::ChannelManager},
    utils::messages::{mumble, MessageInfo},
};

pub struct MessageRouter {
    user_manager: UserManager,
    channel_manager: ChannelManager,
    text_manager: TextMessageManager,
}

impl MessageRouter {
    pub fn new(sender: MessageChannels, server_channel: Sender<Vec<u8>>) -> MessageRouter {
        MessageRouter {
            user_manager: UserManager::new(sender.message_channel.clone(), server_channel.clone()),
            channel_manager: ChannelManager::new(sender.message_channel.clone(), server_channel.clone()),
            text_manager: TextMessageManager::new(sender.message_channel.clone()),
        }
    }

    fn handle_downcast<T: 'static>(&self, message_info: MessageInfo) -> Result<T, Box<dyn Error>> {
        if let Ok(a) = message_info.message_data.downcast::<T>() {
            return Ok(*a);
        }
        return Err(Box::new(ApplicationError::new("Invalid message type")));
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
                error!("Received text message without actor");
            }
        }
        Ok(())
    }

    pub fn recv_message(&mut self, message: MessageInfo) -> Result<(), Box<dyn Error>> {
        trace!("Received message: {:<100?}", message);

        match message.message_type {
            crate::utils::messages::MessageTypes::Version => {}
            crate::utils::messages::MessageTypes::UdpTunnel => {}
            crate::utils::messages::MessageTypes::Authenticate => {}
            crate::utils::messages::MessageTypes::Ping => {}
            crate::utils::messages::MessageTypes::Reject => {}
            crate::utils::messages::MessageTypes::ServerSync => {}
            crate::utils::messages::MessageTypes::ChannelRemove => {
                let removed_channel = self.handle_downcast::<mumble::proto::ChannelRemove>(message)?;
                self.channel_manager.remove_channel(removed_channel);
            }
            crate::utils::messages::MessageTypes::ChannelState => {
                let changed_channel = self.handle_downcast::<mumble::proto::ChannelState>(message)?;
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
            crate::utils::messages::MessageTypes::PluginDataTransmission => {},
        };

        /*downcast_message(
            message.message_data,
            message.message_type,
            self.sender.message_channel.clone(),
        );*/

        Ok(())
    }
}
