use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};

use serde::Serialize;
use tracing::{debug, error, info};

use crate::{
    protocol::serialize::message_container::FrontendMessage,
    utils::messages::{
        message_builder,
        mumble::{self},
    },
};

use super::Update;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Default, Serialize)]
pub struct Channel {
    pub channel_id: u32,
    pub parent: u32,
    pub name: String,
    pub links: Vec<u32>,
    #[serde(skip_serializing)] // We don't want to send such a big blob to the frontend
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
    pub channel_id: u32,
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
        //Self::update_if_some(&mut self.description_hash, other.description_hash);
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
    server_channel: Sender<Vec<u8>>,
}

impl ChannelManager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> ChannelManager {
        ChannelManager {
            channels: HashMap::new(),
            frontend_channel: send_to,
            server_channel: server_channel,
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

    fn fill_channel_description(
        &self,
        channel_info: &Channel,
        description_hash: &Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let channel_id = channel_info.channel_id;
        let cached_channel_description_hash =
            &self.channels.get(&channel_id).unwrap().description_hash;

        if description_hash == cached_channel_description_hash {
            debug!(
                "Channel description is up to date {:?} vs {:?}",
                description_hash, cached_channel_description_hash
            );
            return Ok(());
        }
        debug!(
            "Channel description is not up to date for channel {}",
            channel_id
        );

        let no_comment_available = cached_channel_description_hash.is_empty();
        let comment_in_current_message = !description_hash.is_empty();

        if no_comment_available && comment_in_current_message {
            let blob_request = mumble::proto::RequestBlob {
                channel_description: vec![channel_id],
                ..Default::default()
            };
            self.server_channel.send(message_builder(blob_request))?;
        }

        Ok(())
    }

    fn notify_channel_description(&self, channel_id: u32) {
        if let Some(user) = self.channels.get(&channel_id) {
            let channel_description = ChannelBlobData {
                channel_id: channel_id,
                data: user.description.clone(),
            };
            let msg = FrontendMessage::new("channel_description", &channel_description);

            self.send_to_frontend(&msg);
        }
    }

    pub fn update_channel(
        &mut self,
        channel_info: mumble::proto::ChannelState,
    ) -> Result<(), Box<dyn Error>> {
        let has_description = channel_info.description.is_some()
            && !channel_info.description.as_ref().unwrap().is_empty();
        let channel_id = channel_info.channel_id();
        let description_hash = channel_info.description_hash.clone().unwrap_or_default();

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

        if let Some(channel) = self.channels.get(&channel_id) {
            debug!("Updating channel description: {}", channel_id);
            self.fill_channel_description(channel, &description_hash)?;
        }

        self.notify(&channel_id);

        if has_description {
            self.notify_channel_description(channel_id);
        }

        Ok(())
    }

    pub fn remove_channel(&mut self, user_info: mumble::proto::ChannelRemove) {
        let session = user_info.channel_id;

        self.channels.remove(&session);
        self.notify(&session);
    }
}
