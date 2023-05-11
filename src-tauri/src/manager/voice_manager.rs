use crate::errors::voice_error::VoiceError;
use crate::utils::audio_player::AudioPlayer;
use crate::{protocol::serialize::message_container::FrontendMessage, utils::varint::parse_varint};
use opus::Decoder;
use serde::Serialize;
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};
use tokio::sync::broadcast::Sender;
use tracing::{error, warn};

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
    decoder: Decoder,
}

impl VoiceManager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> VoiceManager {
        let mut player = AudioPlayer::new();
        if let Err(error) = player.start() {
            error!("Failed to start audio player: {}", error);
        }
        // Always pretend Stereo mode is true by default. since opus will convert mono stream to stereo stream.
        // https://tools.ietf.org/html/rfc6716#section-2.1.2
        let decoder = Decoder::new(SAMPLE_RATE, CHANNELS);
        if decoder.is_err() {
            error!(
                "Failed to create opus decoder: {:?}",
                decoder.as_ref().err()
            );
        }

        VoiceManager {
            frontend_channel: send_to,
            _server_channel: server_channel,
            user_audio_info: HashMap::new(),
            audio_player: player,
            decoder: decoder.unwrap(),
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

    pub fn notify_audio(&mut self, audio_data: &[u8]) -> Result<(), Box<dyn Error>> {
        let audio_header = audio_data[0];

        let audio_type = (audio_header & 0xE0) >> 5;
        //let audio_target = audio_header & 0x1F;
        if audio_type != 4 {
            warn!("Received audio data with unknown type: {:?}", audio_type);
            return Ok(());
        }
        let mut position = 1;

        let session_id = parse_varint(&audio_data[position..])?;
        position += session_id.1 as usize;

        let sequence_number = parse_varint(&audio_data[position..])?;
        position += sequence_number.1 as usize;

        let opus_header = parse_varint(&audio_data[position..])?;
        position += opus_header.1 as usize;

        let talking = (opus_header.0 & 0x2000) <= 0;
        let user_id = session_id.0 as u32;

        self.send_taking_information(user_id, talking);

        // = SampleRate * 60ms = 48000Hz * 0.06s = 2880, ~12KB
        let mut audio_buffer_size = SAMPLE_RATE * 60 / 1000;
        if CHANNELS == opus::Channels::Stereo {
            audio_buffer_size *= 2;
        }
        let mut decoded_data = vec![0; audio_buffer_size as usize];

        let payload_size = opus_header.0 & 0x1FFF;
        let payload = &audio_data[position..position + payload_size as usize];
        let num_decoded_samples = self.decoder.decode(payload, &mut decoded_data, false)?;
        decoded_data.truncate(num_decoded_samples);

        if let Err(error) = self.audio_player.add_to_queue(decoded_data, user_id) {
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
