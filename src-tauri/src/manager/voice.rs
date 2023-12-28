use crate::commands::Settings;
use crate::errors::application_error::ApplicationError;
use crate::errors::AnyError;
use crate::mumble;
use crate::protocol::serialize::message_container::FrontendMessage;
use crate::utils::audio;
use crate::utils::audio::player::Player;
use crate::utils::audio::recorder::Recorder;
use crate::utils::messages::message_builder;
use crate::{connection::traits::Shutdown, errors::voice_error::VoiceError};
use async_trait::async_trait;
use serde::Serialize;
use std::collections::{hash_map::Entry, HashMap};
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::error;

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: opus::Channels = opus::Channels::Mono;

#[derive(Debug, Serialize, Clone)]
struct AudioInfo {
    talking: bool,
    user_id: u32,
}

pub struct Manager {
    frontend_channel: Sender<String>,
    server_channel: Sender<Vec<u8>>,
    user_audio_info: HashMap<u32, AudioInfo>,
    audio_player: Player,
    recoder: Recorder,
    decoder: audio::decoder::Decoder,
}

impl Manager {
    pub fn new(
        send_to: Sender<String>,
        server_channel: Sender<Vec<u8>>,
        settings_channel: Receiver<Settings>,
    ) -> AnyError<Self> {
        let mut player = Player::new();
        if let Err(error) = player.start() {
            return Err(Box::new(ApplicationError::new(&format!(
                "Failed to start audio player: {error}"
            ))));
        }

        let server_channel_clone = server_channel.clone();

        let mut recoder = audio::recorder::Recorder::new(server_channel_clone, settings_channel);
        //if enable_recorder {
        if let Err(error) = recoder.start() {
            return Err(Box::new(ApplicationError::new(&format!(
                "Failed to start audio recorder: {error}"
            ))));
        }
        //}

        Ok(Self {
            frontend_channel: send_to,
            server_channel,
            user_audio_info: HashMap::new(),
            audio_player: player,
            recoder,
            decoder: audio::decoder::Decoder::new(SAMPLE_RATE, CHANNELS),
        })
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

    pub fn notify_audio(&mut self, audio_data: &[u8]) -> AnyError<()> {
        let audio_data = self.decoder.decode_audio(audio_data)?;
        self.send_taking_information(audio_data.user_id, audio_data.talking);
        if let Err(error) = self.audio_player.add_to_queue(audio_data) {
            return Err(VoiceError::new(format!("Failed to add audio to queue: {error}")).into());
        }

        Ok(())
    }

    fn send_taking_information(&mut self, user_id: u32, talking: bool) {
        match self.user_audio_info.entry(user_id) {
            Entry::Occupied(o) => {
                if o.get().talking != talking {
                    let audio_info = AudioInfo { talking, user_id };
                    {
                        o.remove_entry();
                    }
                    self.send_to_frontend(&FrontendMessage::new("audio_info", &audio_info));
                }
            }
            Entry::Vacant(v) => {
                let audio_info = AudioInfo { talking, user_id };
                let audio_info_clone = audio_info.clone();

                {
                    v.insert(audio_info);
                }
                self.send_to_frontend(&FrontendMessage::new("audio_info", &audio_info_clone));
            }
        };
    }

    pub(crate) fn deafen(&self) -> AnyError<()> {
        let blob_request = mumble::proto::UserState {
            self_deaf: Some(true),
            self_mute: Some(true),
            ..Default::default()
        };
        self.server_channel.send(message_builder(&blob_request)?)?;

        Ok(())
    }
}

#[async_trait]
impl Shutdown for Manager {
    async fn shutdown(&mut self) -> AnyError<()> {
        self.audio_player.stop();
        self.recoder.stop();

        Ok(())
    }
}
