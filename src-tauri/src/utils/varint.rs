use std::error::Error;

use crate::errors::{voice_error::VoiceError, AnyError};

fn create_voice_eoi(on: &str) -> Box<dyn Error> {
    Box::new(VoiceError::new(format!("Unexpected end of input for {on}")))
}

pub struct Builder {
    bytes: Option<Vec<u8>>,
    parsed_value: Option<i128>,
    parsed_bytes: Option<u32>,
}

impl From<&[u8]> for Builder {
    fn from(bytes: &[u8]) -> Self {
        let (parsed_value, parsed_bytes) = match Varint::parse(bytes) {
            Ok((value, bytes)) => (Some(value), Some(bytes)),
            Err(_) => (None, None),
        };

        Self {
            bytes: Some(bytes.to_vec()),
            parsed_value,
            parsed_bytes,
        }
    }
}

impl Builder {
    pub fn build(self) -> AnyError<Varint> {
        Ok(Varint {
            _bytes: self.bytes.ok_or_else(|| VoiceError::new("No bytes"))?,
            parsed_value: self
                .parsed_value
                .ok_or_else(|| VoiceError::new("No parsed value"))?,
            parsed_bytes: self
                .parsed_bytes
                .ok_or_else(|| VoiceError::new("No parsed bytes"))?,
        })
    }
}

pub struct Varint {
    //TODO: Implement trait into
    _bytes: Vec<u8>,
    pub parsed_value: i128,
    pub parsed_bytes: u32,
}

impl Varint {
    pub const fn parsed_pair(&self) -> (i128, u32) {
        (self.parsed_value, self.parsed_bytes)
    }

    fn parse(bytes: &[u8]) -> AnyError<(i128, u32)> {
        if bytes.is_empty() {
            return Err(Box::new(VoiceError::new("Unexpected end of input")));
        }

        if bytes.is_empty() {
            return Err(create_voice_eoi("varint"));
        }

        let first_byte = bytes[0];
        let value = match first_byte {
            // 7-bit positive number
            0..=127 => (i128::from(u64::from(first_byte)), 1),
            // 14-bit positive number
            128..=191 => {
                if bytes.len() < 2 {
                    return Err(create_voice_eoi("14-bit positive number"));
                }
                let value = u64::from(first_byte & 0b0011_1111) << 8 | u64::from(bytes[1]);
                (i128::from(value), 2)
            }
            // 21-bit positive number
            192..=223 => {
                if bytes.len() < 3 {
                    return Err(create_voice_eoi("21-bit positive number"));
                }
                let value = u64::from(first_byte & 0b0001_1111) << 16
                    | u64::from(bytes[1]) << 8
                    | u64::from(bytes[2]);
                (i128::from(value), 3)
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
                (i128::from(value), 4)
            }

            // 32-bit positive number
            0b1111_0000..=0b1111_0011 => {
                if bytes.len() < 5 {
                    return Err(create_voice_eoi("32-bit positive number"));
                }
                (i128::from(u32::from_be_bytes(bytes[1..=4].try_into()?)), 5)
            }

            // 64-bit positive number
            0b1111_0100..=0b1111_0111 => {
                if bytes.len() < 9 {
                    return Err(create_voice_eoi("64-bit positive number"));
                }
                (i128::from(u64::from_be_bytes(bytes[1..=8].try_into()?)), 9)
            }

            // Negative recursive varint
            0b1111_1000..=0b1111_1011 => {
                let value = Self::parse(&bytes[1..])?;
                (-value.0, value.1 + 1)
            }

            // inverted negative two bit number
            0b1111_1100..=0b1111_1111 => (i128::from(!((first_byte) & 0b0000_0011)), 1),
        };

        Ok(value)
    }
}
