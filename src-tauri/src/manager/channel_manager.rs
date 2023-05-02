use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};

use serde::Serialize;
use tracing::{error, info};

use crate::{
    protocol::serialize::message_container::FrontendMessage,
    utils::messages::mumble::{self},
};

use super::Update;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Default, Serialize)]
pub struct Channel {
    pub channel_id: u32,
    pub parent: u32,
    pub name: String,
    pub links: Vec<u32>,
    pub description: String,
    pub links_add: Vec<u32>,
    pub links_remove: Vec<u32>,
    pub temporary: bool,
    pub position: i32,
    pub description_hash: Vec<u8>,
    pub max_users: u32,
    pub is_enter_restricted: bool,
    pub can_enter: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct ChannelBlobData {
    pub user_id: u32,
    pub data: String,
}

impl Update<mumble::proto::ChannelState> for Channel {
    fn update_from(&mut self, other: mumble::proto::ChannelState) -> &Self {
        self.links = other.links;
        self.links_add = other.links_add;
        self.links_remove = other.links_remove;

        Self::update_if_some(&mut self.channel_id, other.channel_id);
        Self::update_if_some(&mut self.parent, other.parent);
        Self::update_if_some(&mut self.name, other.name);
        Self::update_if_some(&mut self.description, other.description);
        Self::update_if_some(&mut self.position, other.position);
        Self::update_if_some(&mut self.description_hash, other.description_hash);
        Self::update_if_some(&mut self.max_users, other.max_users);
        Self::update_if_some(&mut self.temporary, other.temporary);
        Self::update_if_some(&mut self.is_enter_restricted, other.is_enter_restricted);
        Self::update_if_some(&mut self.can_enter, other.can_enter);

        self
    }
}

pub struct ChannelManager {
    channels: HashMap<u32, Channel>,
    frontend_channel: Sender<String>,
    _server_channel: Sender<Vec<u8>>,
}

impl ChannelManager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> ChannelManager {
        ChannelManager {
            channels: HashMap::new(),
            frontend_channel: send_to,
            _server_channel: server_channel,
        }
    }

    fn send_to_frontend<T: Serialize + Clone>(&self, msg: &FrontendMessage<T>) {
        match serde_json::to_string(&msg) {
            Ok(json) => {
                if let Err(e) = self.frontend_channel.send(json) {
                    error!("Failed to send user list to frontend: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to serialize user list: {}", e);
            }
        }
    }

    fn notify(&self, channel_id: &u32) {
        if let Some(user) = self.channels.get(channel_id) {
            let msg = FrontendMessage::new("channel_update", &user);

            self.send_to_frontend(&msg);
        }
    }

    pub fn update_channel(
        &mut self,
        channel_info: mumble::proto::ChannelState,
    ) -> Result<(), Box<dyn Error>> {
        let channel_id = channel_info.channel_id();

        match self.channels.entry(channel_id) {
            Entry::Occupied(mut o) => {
                info!("Updating channel");
                o.get_mut().update_from(channel_info);
            }
            Entry::Vacant(v) => {
                let mut channel = Channel::default();
                info!("Adding channel");
                channel.update_from(channel_info);
                v.insert(channel);
            }
        };

        self.notify(&channel_id);

        Ok(())
    }

    pub fn remove_channel(&mut self, user_info: mumble::proto::ChannelRemove) {
        let session = user_info.channel_id;

        self.channels.remove(&session);
        self.notify(&session);
    }
}
