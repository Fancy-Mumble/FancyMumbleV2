use std::error::Error;

use tracing::trace;

use crate::{
    connection::MessageChannels,
    errors::application_error::ApplicationError,
    manager::user_manager::UserManager,
    utils::messages::{mumble, MessageInfo},
};

pub struct MessageHandler {
    user_manager: UserManager,
}

impl MessageHandler {
    pub fn new(sender: MessageChannels) -> MessageHandler {
        MessageHandler {
            user_manager: UserManager::new(sender.message_channel.clone()),
        }
    }

    fn handle_downcast<T: 'static>(&self, message_info: MessageInfo) -> Result<T, Box<dyn Error>> {
        if let Ok(a) = message_info.message_data.downcast::<T>() {
            return Ok(*a);
        }
        return Err(Box::new(ApplicationError::new("Invalid message type")));
    }

    //TODO: create a message distributor
    pub fn recv_message(&mut self, message: MessageInfo) -> Result<(), Box<dyn Error>> {
        // self.sender.message_channel.send("{}".to_string());
        trace!("Received message: {:?}", message);

        match message.message_type {
            crate::utils::messages::MessageTypes::Version => {}
            crate::utils::messages::MessageTypes::UdpTunnel => {}
            crate::utils::messages::MessageTypes::Authenticate => {}
            crate::utils::messages::MessageTypes::Ping => {}
            crate::utils::messages::MessageTypes::Reject => {}
            crate::utils::messages::MessageTypes::ServerSync => {}
            crate::utils::messages::MessageTypes::ChannelRemove => {}
            crate::utils::messages::MessageTypes::ChannelState => {}
            crate::utils::messages::MessageTypes::UserRemove => {
                let removed_user = self.handle_downcast::<mumble::proto::UserRemove>(message)?;
                self.user_manager.remove_user(removed_user);
            }
            crate::utils::messages::MessageTypes::UserState => {
                let changed_user = self.handle_downcast::<mumble::proto::UserState>(message)?;
                self.user_manager.update_user(changed_user);
            }
            crate::utils::messages::MessageTypes::BanList => {}
            crate::utils::messages::MessageTypes::TextMessage => {}
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
        };

        /*downcast_message(
            message.message_data,
            message.message_type,
            self.sender.message_channel.clone(),
        );*/

        Ok(())
    }
}
