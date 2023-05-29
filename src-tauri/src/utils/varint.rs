use std::error::Error;

use crate::errors::{voice_error::VoiceError, AnyError};

fn create_voice_eoi(on: &str) -> Box<dyn Error> {
    Box::new(VoiceError::new(format!("Unexpected end of input for {on}")))
}

pub struct Builder {
    bytes: Option<Vec<u8>>,
    parsed_value: Option<i128>,
    parsed_bytes: Option<u32>,
    minimum_bytes: Option<u32>,
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
            minimum_bytes: None,
        }
    }
}

impl From<i128> for Builder {
    fn from(value: i128) -> Self {
        let bytes = Varint::encode(value, 1);
        let size = bytes.as_ref().map(|b| b.len() as u32);

        Self {
            bytes,
            parsed_value: Some(value),
            parsed_bytes: size,
            minimum_bytes: None,
        }
    }
}

impl Builder {
    pub fn new(value: i128) -> Self {
        Self {
            bytes: None,
            parsed_value: Some(value),
            parsed_bytes: None,
            minimum_bytes: None,
        }
    }

    pub fn minimum_bytes(mut self, minimum_bytes: u32) -> Self {
        self.minimum_bytes = Some(minimum_bytes);
        self
    }

    pub fn encode_build(self) -> AnyError<Varint> {
        let bytes = Varint::encode(
            self.parsed_value.unwrap_or(0),
            self.minimum_bytes.unwrap_or(1),
        );
        let size = bytes.as_ref().map(|b| b.len() as u32);

        Ok(Varint {
            bytes: bytes.ok_or_else(|| VoiceError::new("No bytes"))?,
            parsed_value: self
                .parsed_value
                .ok_or_else(|| VoiceError::new("No parsed value"))?,
            parsed_bytes: size.ok_or_else(|| VoiceError::new("No parsed bytes"))?,
        })
    }

    pub fn build(self) -> AnyError<Varint> {
        Ok(Varint {
            bytes: self.bytes.ok_or_else(|| VoiceError::new("No bytes"))?,
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
    bytes: Vec<u8>,
    pub parsed_value: i128,
    pub parsed_bytes: u32,
}

impl Varint {
    pub const fn parsed_pair(&self) -> (i128, u32) {
        (self.parsed_value, self.parsed_bytes)
    }

    pub const fn parsed_vec(&self) -> &Vec<u8> {
        &self.bytes
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

    fn encode(value: i128, minimum_length: u32) -> Option<Vec<u8>> {
        let mut bytes = Vec::new();

        match value {
            0..=127 if minimum_length <= 1 => {
                bytes.push(value as u8);
            }
            ..=16_383 if minimum_length <= 2 => {
                let byte1 = ((value >> 8) & 0b0011_1111) as u8 | 0b1000_0000;
                let byte2 = (value & 0xFF) as u8;
                bytes.push(byte1);
                bytes.push(byte2);
            }
            ..=2_097_151 if minimum_length <= 3 => {
                let byte1 = ((value >> 16) & 0b0001_1111) as u8 | 0b1100_0000;
                let byte2 = ((value >> 8) & 0xFF) as u8;
                let byte3 = (value & 0xFF) as u8;
                bytes.push(byte1);
                bytes.push(byte2);
                bytes.push(byte3);
            }
            ..=268_435_455 if minimum_length <= 4 => {
                let byte1 = ((value >> 24) & 0b0000_1111) as u8 | 0b1110_0000;
                let byte2 = ((value >> 16) & 0xFF) as u8;
                let byte3 = ((value >> 8) & 0xFF) as u8;
                let byte4 = (value & 0xFF) as u8;
                bytes.push(byte1);
                bytes.push(byte2);
                bytes.push(byte3);
                bytes.push(byte4);
            }
            ..=4_294_967_295 if minimum_length <= 5 => {
                bytes.push(0b1111_0000);
                bytes.extend_from_slice(&(value as u32).to_be_bytes());
            }
            ..=18_446_744_073_709_551_615 if minimum_length <= 9 => {
                bytes.push(0b1111_0100);
                bytes.extend_from_slice(&(value as u64).to_be_bytes());
            }
            ..=-5 => {
                let inverted_value = !value as u128;
                bytes.push((inverted_value & 0b0011_1111) as u8 | 0b1111_1000);
                bytes.extend_from_slice(&Self::encode(-value - 1, minimum_length)?);
            }
            -4..=-1 => {
                let inverted_value = !value as u8;
                bytes.push(inverted_value | 0b1111_1100);
            }
            _ => return None, // Value out of supported range
        }

        Some(bytes)
    }
}
