use opus::Channels;

use crate::utils::varint;

use super::microphone::DeviceConfig;

const MAXIMUM_SAMPLES_PER_TALK: u64 = 600;
const QUALITY: opus::Application = opus::Application::Audio;

pub struct Encoder {
    encoder: opus::Encoder,
    audio_buffer_size: usize,
}

impl Encoder {
    pub fn new(config: DeviceConfig) -> Self {
        let opus_channels = match config.channels {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            _ => panic!("Unsupported channel count"),
        };

        let encoder = opus::Encoder::new(config.sample_rate, opus_channels, QUALITY)
            .expect("Failed to create opus encoder");

        Self {
            encoder,
            audio_buffer_size: config.buffer_size,
        }
    }

    pub fn encode_audio(&mut self, data: &[f32], sequence_number: &mut u64) -> Vec<u8> {
        let output = self
            .encoder
            .encode_vec_float(data, self.audio_buffer_size)
            .expect("Failed to encode audio data");

        let mut audio_buffer = Vec::new();

        let opus_audio_codec = 4u8 << 5;
        let target = 0b0000_0000u8;
        let first_byte = opus_audio_codec | target;
        audio_buffer.push(first_byte);

        let sequence_number_bytes = varint::Builder::new()
            .number(&*sequence_number)
            .build()
            .expect("Failed to build sequence number");
        audio_buffer.extend(sequence_number_bytes.parsed_vec());
        *sequence_number += 1;

        let size_pre = (output.len() as i128) | 1 << 14;

        if *sequence_number > MAXIMUM_SAMPLES_PER_TALK {
            *sequence_number = 0;
        }
        let size = varint::Builder::new()
            .number(&size_pre)
            .minimum_bytes(2)
            .build()
            .expect("Failed to build size");

        audio_buffer.extend(size.parsed_vec());

        audio_buffer.extend(output);

        audio_buffer
    }
}
