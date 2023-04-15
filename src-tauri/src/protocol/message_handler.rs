use crate::{
    connection::MessageChannels,
    utils::messages::{mumble::proto::TextMessage, MessageInfo, downcast_message},
};

pub struct MessageHandler {
    sender: MessageChannels,
}

impl MessageHandler {
    pub fn new(sender: MessageChannels) -> MessageHandler {
        MessageHandler { sender }
    }

    pub fn recv_message(&self, message: MessageInfo) {
        //println!("Incomming: {message:?}");

        downcast_message(message.message_data, message.message_type, self.sender.message_channel.clone())
    }
}
