use std::collections::{hash_map::Entry, HashMap};

use serde::Serialize;
use tracing::{error, info};

use crate::{protocol::serialize::message_container::FrontendMessage, utils::messages::mumble};

use super::Update;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Default, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    channel_id: u32,
    mute: bool,
    deaf: bool,
    suppress: bool,
    self_mute: bool,
    self_deaf: bool,
    priority_speaker: bool,
    recording: bool,
    profile_picture: Vec<u8>,
    comment: String,
}

impl Update<mumble::proto::UserState> for User {
    fn update_from(&mut self, other: mumble::proto::UserState) -> &Self {
        Self::update_if_some(&mut self.id, other.session);
        Self::update_if_some(&mut self.id, other.session);
        Self::update_if_some(&mut self.name, other.name);
        Self::update_if_some(&mut self.channel_id, other.channel_id);
        Self::update_if_some(&mut self.mute, other.mute);
        Self::update_if_some(&mut self.deaf, other.deaf);
        Self::update_if_some(&mut self.suppress, other.suppress);
        Self::update_if_some(&mut self.self_mute, other.self_mute);
        Self::update_if_some(&mut self.self_deaf, other.self_deaf);
        Self::update_if_some(&mut self.priority_speaker, other.priority_speaker);
        Self::update_if_some(&mut self.recording, other.recording);
        Self::update_if_some(&mut self.profile_picture, other.texture);
        Self::update_if_some(&mut self.comment, other.comment);

        self
    }
}

pub struct UserManager {
    users: HashMap<u32, User>,
    frontend_channel: Sender<String>,
}

impl UserManager {
    pub fn new(send_to: Sender<String>) -> UserManager {
        UserManager {
            users: HashMap::new(),
            frontend_channel: send_to,
        }
    }

    fn notify(&self) {
        let msg = FrontendMessage::new("user_list", &self.users);

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

    pub fn update_user(&mut self, user_info: mumble::proto::UserState) {
        match self.users.entry(user_info.session()) {
            Entry::Occupied(mut o) => {
                info!("Updating user: {:?}", o.get());
                o.get_mut().update_from(user_info);
            }
            Entry::Vacant(v) => {
                let mut user = User::default();
                user.update_from(user_info);
                info!("Adding user: {:?}", user);
                v.insert(user);
            }
        };

        self.notify();
    }

    pub fn remove_user(&mut self, user_info: mumble::proto::UserRemove) {
        self.users.remove(&user_info.session);
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }
}
