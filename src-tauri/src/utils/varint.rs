use std::error::Error;

use crate::errors::voice_error::VoiceError;

fn create_voice_eoi(on: &str) -> Box<dyn Error> {
    Box::new(VoiceError::new(format!(
        "Unexpected end of input for {}",
        on
    )))
}

pub fn parse_varint(bytes: &[u8]) -> Result<(i128, u32), Box<dyn Error>> {
    if bytes.is_empty() {
        return Err(Box::new(VoiceError::new("Unexpected end of input")));
    }

    if bytes.is_empty() {
        return Err(create_voice_eoi("varint"));
    }

    let first_byte = bytes[0];
    let value = match first_byte {
        // 7-bit positive number
        0..=127 => (u64::from(first_byte) as i128, 1),
        // 14-bit positive number
        128..=191 => {
            if bytes.len() < 2 {
                return Err(create_voice_eoi("14-bit positive number"));
            }
            let value = u64::from(first_byte & 0b0011_1111) << 8 | u64::from(bytes[1]);
            (value as i128, 2)
        }
        // 21-bit positive number
        192..=223 => {
            if bytes.len() < 3 {
                return Err(create_voice_eoi("21-bit positive number"));
            }
            let value = u64::from(first_byte & 0b0001_1111) << 16
                | u64::from(bytes[1]) << 8
                | u64::from(bytes[2]);
            (value as i128, 3)
        }
        // 28-bit positive number
        224..=239 => {
            if bytes.len() < 4 {
                return Err(create_voice_eoi("28-bit positive number"));
            }
            let value = u64::from(first_byte & 0b0000_1111) << 24
                | u64::from(bytes[1]) << 16
                | u64::from(bytes[2]) << 8
                | u64::from(bytes[3]);
            (value as i128, 4)
        }

        // 32-bit positive number
        0b11110000..=0b11110011 => {
            if bytes.len() < 5 {
                return Err(create_voice_eoi("32-bit positive number"));
            }
            (u32::from_be_bytes(bytes[1..=4].try_into()?) as i128, 5)
        }

        // 64-bit positive number
        0b11110100..=0b11110111 => {
            if bytes.len() < 9 {
                return Err(create_voice_eoi("64-bit positive number"));
            }
            (u64::from_be_bytes(bytes[1..=8].try_into()?) as i128, 9)
        }

        // Negative recursive varint
        0b11111000..=0b11111011 => {
            let value = parse_varint(&bytes[1..])?;
            (-value.0, value.1 + 1)
        }

        // inverted negative two bit number
        0b11111100..=0b11111111 => (!((first_byte) & 0b0000_0011) as i128, 1),
    };

    Ok(value)
}