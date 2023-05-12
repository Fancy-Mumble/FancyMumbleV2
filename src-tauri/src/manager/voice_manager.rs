use crate::errors::voice_error::VoiceError;
use crate::protocol::serialize::message_container::FrontendMessage;
use crate::utils::audio::audio_decoder::AudioDecoder;
use crate::utils::audio::audio_player::AudioPlayer;
use crate::utils::audio::audio_recorder::AudioRecorder;
use serde::Serialize;
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};
use tokio::sync::broadcast::Sender;
use tracing::error;

const SAMPLE_RATE: u32 = 48000;
const CHANNELS: opus::Channels = opus::Channels::Mono;

#[derive(Debug, Serialize, Clone)]
struct AudioInfo {
    talking: bool,
    user_id: u32,
}

pub struct VoiceManager {
    frontend_channel: Sender<String>,
    _server_channel: Sender<Vec<u8>>,
    user_audio_info: HashMap<u32, AudioInfo>,
    audio_player: AudioPlayer,
    decoder: AudioDecoder,
}

impl VoiceManager {
    pub fn new(
        send_to: Sender<String>,
        server_channel: Sender<Vec<u8>>,
    ) -> Result<VoiceManager, Box<dyn Error>> {
        let mut player = AudioPlayer::new();
        if let Err(error) = player.start() {
            error!("Failed to start audio player: {}", error);
        }

        let mut recoder = AudioRecorder::new();
        if let Err(error) = recoder.start() {
            error!("Failed to start audio recorder: {}", error);
        }

        Ok(VoiceManager {
            frontend_channel: send_to,
            _server_channel: server_channel,
            user_audio_info: HashMap::new(),
            audio_player: player,
            decoder: AudioDecoder::new(SAMPLE_RATE, CHANNELS)?,
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

    pub fn notify_audio(&mut self, audio_data: &[u8]) -> Result<(), Box<dyn Error>> {
        let audio_data = self.decoder.decode_audio(audio_data)?;
        self.send_taking_information(audio_data.user_id, audio_data.talking);
        if let Err(error) = self
            .audio_player
            .add_to_queue(audio_data.data, audio_data.user_id)
        {
            return Err(VoiceError::new(format!("Failed to add audio to queue: {}", error)).into());
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

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        self.audio_player.stop();

        Ok(())
    }
}
