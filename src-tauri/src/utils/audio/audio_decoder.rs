use std::error::Error;

use opus::Decoder;

use crate::utils::varint::parse_varint;

pub struct DecodedMessage {
    pub user_id: u32,
    pub talking: bool,
    pub data: Vec<i16>,
}

pub struct AudioDecoder {
    decoder: Decoder,
    sample_rate: u32,
    channels: opus::Channels,
}

impl AudioDecoder {
    pub fn new(sample_rate: u32, channels: opus::Channels) -> Result<AudioDecoder, Box<dyn Error>> {
        let decoder = Decoder::new(sample_rate, channels)?;
        Ok(AudioDecoder {
            decoder,
            sample_rate,
            channels,
        })
    }

    pub fn decode_audio(&mut self, audio_data: &[u8]) -> Result<DecodedMessage, Box<dyn Error>> {
        let audio_header = audio_data[0];

        let audio_type = (audio_header & 0xE0) >> 5;
        //let audio_target = audio_header & 0x1F;
        if audio_type != 4 {
            return Err(format!("Received audio data with unknown type: {:?}", audio_type).into());
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

        //self.send_taking_information(user_id, talking);

        // = SampleRate * 60ms = 48000Hz * 0.06s = 2880, ~12KB
        let mut audio_buffer_size = self.sample_rate * 60 / 1000;
        if self.channels == opus::Channels::Stereo {
            audio_buffer_size *= 2;
        }
        let mut decoded_data = vec![0; audio_buffer_size as usize];

        let payload_size = opus_header.0 & 0x1FFF;
        let payload = &audio_data[position..position + payload_size as usize];
        let num_decoded_samples = self.decoder.decode(payload, &mut decoded_data, false)?;
        decoded_data.truncate(num_decoded_samples);

        /*if let Err(error) = self.audio_player.add_to_queue(decoded_data, user_id) {
            return Err(VoiceError::new(format!("Failed to add audio to queue: {}", error)).into());
        }*/

        Ok(DecodedMessage {
            user_id,
            talking,
            data: decoded_data,
        })
    }
}
