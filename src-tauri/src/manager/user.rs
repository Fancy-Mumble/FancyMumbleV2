use base64::{engine::general_purpose, Engine as _};
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    mem,
};

use serde::Serialize;
use tracing::{debug, error, info, trace};

use crate::{
    mumble, protocol::serialize::message_container::FrontendMessage,
    utils::messages::message_builder,
};

use super::Update;
use tokio::sync::broadcast::Sender;

//for now we allow this, because we want to keep the struct as close to the protobuf as possible
#[allow(clippy::struct_excessive_bools)]
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
    comment_hash: Vec<u8>,
    #[serde(skip_serializing)] // We don't want to send such a big blob to the frontend
    comment: String,
}

#[derive(Debug, Default, Serialize)]
pub struct BlobData {
    pub user_id: u32,
    pub data: String,
}

impl Update<mumble::proto::UserState> for User {
    fn update_from(&mut self, other: &mut mumble::proto::UserState) -> &Self {
        Self::update_if_some(&mut self.id, &mut other.session);
        Self::update_if_some(&mut self.name, &mut other.name);
        Self::update_if_some(&mut self.channel_id, &mut other.channel_id);
        Self::update_if_some(&mut self.mute, &mut other.mute);
        Self::update_if_some(&mut self.deaf, &mut other.deaf);
        Self::update_if_some(&mut self.suppress, &mut other.suppress);
        Self::update_if_some(&mut self.self_mute, &mut other.self_mute);
        Self::update_if_some(&mut self.self_deaf, &mut other.self_deaf);
        Self::update_if_some(&mut self.priority_speaker, &mut other.priority_speaker);
        Self::update_if_some(&mut self.recording, &mut other.recording);
        Self::update_if_some(&mut self.profile_picture, &mut other.texture);
        //Self::update_if_some(&mut self.profile_picture_hash, other.texture_hash);
        Self::update_if_some(&mut self.comment, &mut other.comment);
        //Self::update_if_some(&mut self.comment_hash, other.comment_hash);

        self
    }
}

pub struct Manager {
    users: HashMap<u32, User>,
    current_user_id: Option<u32>,
    frontend_channel: Sender<String>,
    server_channel: Sender<Vec<u8>>,
}

impl Manager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> Self {
        Self {
            users: HashMap::new(),
            current_user_id: None,
            frontend_channel: send_to,
            server_channel,
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

    fn notify(&self, session: u32) {
        if let Some(user) = self.users.get(&session) {
            let msg = FrontendMessage::new("user_update", &user);

            self.send_to_frontend(&msg);
        }
    }

    fn notify_remove(&self, session: u32) {
        let msg = FrontendMessage::new("user_remove", session);

        self.send_to_frontend(&msg);
    }

    fn notify_user_image(&self, session: u32) {
        if let Some(user) = self.users.get(&session) {
            let base64 = format!(
                "data:image/png;base64,{}",
                general_purpose::STANDARD.encode(&user.profile_picture)
            );

            let user_image = BlobData {
                user_id: user.id,
                data: base64,
            };
            let msg = FrontendMessage::new("user_image", &user_image);

            self.send_to_frontend(&msg);
        }
    }

    fn notify_user_comment(&self, session: u32) {
        if let Some(user) = self.users.get(&session) {
            let user_image = BlobData {
                user_id: user.id,
                data: user.comment.clone(),
            };
            let msg = FrontendMessage::new("user_comment", &user_image);

            self.send_to_frontend(&msg);
        }
    }

    fn fill_user_images(&self, user_id: u32, texture_hash: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        let user = self.users.get(&user_id);
        if user.is_none() {
            return Err(format!("User {user_id} not found").into());
        }

        let cached_user_texture_hash = &user.unwrap().profile_picture_hash;

        if texture_hash == cached_user_texture_hash {
            trace!(
                "User image is up to date: {:?} vs {:?}",
                texture_hash,
                cached_user_texture_hash
            );
            return Ok(());
        }
        trace!("User image is not up to date for user {}", user_id);

        let no_texture_hash_available = cached_user_texture_hash.is_empty();
        let texture_hash_in_current_message = !texture_hash.is_empty();

        if no_texture_hash_available && texture_hash_in_current_message {
            let blob_request = mumble::proto::RequestBlob {
                session_texture: vec![user_id],
                ..Default::default()
            };
            self.server_channel.send(message_builder(&blob_request))?;
        }

        Ok(())
    }

    fn fill_user_comment(
        &self,
        user_id: u32,
        comment_hash: &Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let cached_user_comment_hash = &self.users.get(&user_id).unwrap().comment_hash;

        if comment_hash == cached_user_comment_hash {
            trace!(
                "User comment is up to date {:?} vs {:?}",
                comment_hash,
                cached_user_comment_hash
            );
            return Ok(());
        }
        trace!("User comment is not up to date for user {user_id}");

        let no_comment_available = cached_user_comment_hash.is_empty();
        let comment_in_current_message = !comment_hash.is_empty();

        if no_comment_available && comment_in_current_message {
            let blob_request = mumble::proto::RequestBlob {
                session_comment: vec![user_id],
                ..Default::default()
            };
            self.server_channel.send(message_builder(&blob_request))?;
        }

        Ok(())
    }

    pub fn update_user(
        &mut self,
        user_info: &mut mumble::proto::UserState,
    ) -> Result<(), Box<dyn Error>> {
        let has_texture =
            user_info.texture.is_some() && !user_info.texture.as_ref().unwrap().is_empty();

        let texture_hash = &mut user_info.texture_hash;
        let texture_hash = mem::take(texture_hash).unwrap_or_default();

        let has_comment =
            user_info.comment.is_some() && !user_info.comment.as_ref().unwrap().is_empty();
        let session = user_info.session();

        let comment_hash = &mut user_info.comment_hash;
        let comment_hash = mem::take(comment_hash).unwrap_or_default();

        match self.users.entry(session) {
            Entry::Occupied(mut o) => {
                info!("Updating user: {:?}", o.get().name);
                o.get_mut().update_from(user_info);
            }
            Entry::Vacant(v) => {
                let mut user = User::default();
                info!("Adding user: {:?}", user_info.name);
                user.update_from(user_info);
                v.insert(user);
            }
        };

        if let Some(user) = self.users.get(&session) {
            self.fill_user_images(user.id, &texture_hash)?;
            self.fill_user_comment(user.id, &comment_hash)?;
        }

        self.notify(session);

        if has_texture {
            debug!("Notifying user image: {}", session);
            self.notify_user_image(session);
        }

        if has_comment {
            debug!("Notifying user comment: {}", session);
            self.notify_user_comment(session);
        }

        Ok(())
    }

    pub fn remove_user(&mut self, user_info: &mumble::proto::UserRemove) {
        let session = user_info.session;

        self.users.remove(&session);
        self.notify_remove(session);
    }

    pub fn get_user_by_id(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn notify_current_user(&mut self, sync_info: &mumble::proto::ServerSync) {
        if sync_info.session.is_some() {
            self.current_user_id = sync_info.session;

            let message = FrontendMessage::new("current_user_id", self.current_user_id);
            self.send_to_frontend(&message);
        }
    }
}
