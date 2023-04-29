use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};
use base64::{engine::general_purpose, Engine as _};

use serde::Serialize;
use tracing::{error, info, trace};

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
    profile_picture_hash: Vec<u8>,
    #[serde(skip_serializing)] // We don't want to send such a big blob to the frontend
    profile_picture: Vec<u8>,
    #[serde(skip_serializing)] // We don't want to send such a big blob to the frontend
    comment: String,
}

#[derive(Debug, Default, Serialize)]
pub struct UserImage {
    pub user_id: u32,
    pub image: String,
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
        Self::update_if_some(&mut self.profile_picture_hash, other.texture_hash);
        Self::update_if_some(&mut self.comment, other.comment);

        self
    }
}

pub struct UserManager {
    users: HashMap<u32, User>,
    frontend_channel: Sender<String>,
    server_channel: Sender<Vec<u8>>,
}

impl UserManager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> UserManager {
        UserManager {
            users: HashMap::new(),
            frontend_channel: send_to,
            server_channel,
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

    fn notify_user_image(&self, session: u32) {
        if let Some(user) = self.users.get(&session) {
            let base64 = format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&user.profile_picture));

            let user_image = UserImage {
                user_id: user.id,
                image: base64,
            };
            let msg = FrontendMessage::new("user_image", &user_image);

            match serde_json::to_string(&msg) {
                Ok(json) => {
                    info!("Sending user image to frontend: {}", json.len());
                    if let Err(e) = self.frontend_channel.send(json) {
                        error!("Failed to send user image to frontend: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to serialize user image: {}", e);
                }
            }
        }
    }

    fn fill_user_images(
        &mut self,
        user_info: &mumble::proto::UserState,
    ) -> Result<(), Box<dyn Error>> {
        let user_session = user_info.session();
        let user_texture_hash = user_info.texture_hash.clone().unwrap_or_default();
        let user_profile_picture_hash = &self
            .users
            .entry(user_session)
            .or_default()
            .profile_picture_hash;

        if &user_texture_hash == user_profile_picture_hash {
            return Ok(());
        }

        if user_info.texture.is_none() {
            let blob_request = mumble::proto::RequestBlob {
                session_texture: vec![user_session],
                ..Default::default()
            };
            self.server_channel.send(message_builder(blob_request))?;
        }

        Ok(())
    }

    pub fn update_user(
        &mut self,
        user_info: mumble::proto::UserState,
    ) -> Result<(), Box<dyn Error>> {
        let has_texture = user_info.texture.is_some();
        let session = user_info.session();
        self.fill_user_images(&user_info)?;

        match self.users.entry(session) {
            Entry::Occupied(mut o) => {
                info!("Updating user: {:?}", o.get().name);
                o.get_mut().update_from(user_info);
            }
            Entry::Vacant(v) => {
                let mut user = User::default();
                trace!("Adding user: {:?}", user.name);
                user.update_from(user_info);
                v.insert(user);
            }
        };

        self.notify();

        if has_texture {
            info!("Notifying user image: {}", session);
            self.notify_user_image(session);
        }

        Ok(())
    }

    pub fn remove_user(&mut self, user_info: mumble::proto::UserRemove) {
        self.users.remove(&user_info.session);
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }
}
