use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
};

use serde::Serialize;
use tracing::{error, warn};

use crate::{protocol::serialize::message_container::FrontendMessage, utils::varint::parse_varint};

use tokio::sync::broadcast::Sender;

#[derive(Debug, Serialize, Clone)]
struct AudioInfo {
    talking: bool,
    user_id: u32,
}

pub struct VoiceManager {
    frontend_channel: Sender<String>,
    _server_channel: Sender<Vec<u8>>,
    user_audio_info: HashMap<u32, AudioInfo>,
}

impl VoiceManager {
    pub fn new(send_to: Sender<String>, server_channel: Sender<Vec<u8>>) -> VoiceManager {
        VoiceManager {
            frontend_channel: send_to,
            _server_channel: server_channel,
            user_audio_info: HashMap::new(),
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

    pub fn notify_audio(&mut self, audio_data: &Vec<u8>) -> Result<(), Box<dyn Error>> {
        //trace!("Received audio data: {:?}", audio_data);
        let audio_header = audio_data.as_slice()[0];

        let audio_type = (audio_header & 0xE0) >> 5;
        let audio_target = audio_header & 0x1F;
        if audio_type != 4 {
            warn!("Received audio data with unknown type: {:?}", audio_type);
            return Ok(());
        }
        let mut position = 1 as usize;

        let session_id = parse_varint(&audio_data[position..])?;
        position += session_id.1 as usize;

        let sequence_number = parse_varint(&audio_data[position..])?;
        position += sequence_number.1 as usize;

        let opus_header = parse_varint(&audio_data[position..])?;
        //position += opus_header.1 as usize;

        let talking = (opus_header.0 & 0x2000) <= 0;
        let user_id = session_id.0 as u32;

        /*trace!(
            "Type: {:?} | Target: {:?} | Session: {:?} | Sequence: {:?} | Opus: {:?}, EOF: {:?}",
            audio_type,
            audio_target,
            user_id,
            sequence_number.0,
            opus_header.0 & 0x1FFF,
            talking
        );*/

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

        Ok(())
    }
}
