use byteorder::{BigEndian, ReadBytesExt};
use prost::Message;
use std::io::Cursor;

use crate::utils::messages::get_message;

pub struct StreamReader {
    stream_buffer: Vec<u8>,
}

impl StreamReader {
    pub fn new() -> StreamReader {
        StreamReader {
            stream_buffer: Vec::<u8>::new(),
        }
    }

    pub fn read(&mut self, data: &mut Vec<u8>) {
        self.stream_buffer.append(data);
        while let Some(result) = self.try_read() {
            println!("{result:?}");
        }
    }

    fn try_read(&mut self) -> Option<Box<dyn Message>> {
        if self.stream_buffer.len() < 6 {
            return None;
        }

        let message_type = Cursor::new(self.get_n(2)).read_u16::<BigEndian>().ok()?;
        let message_size = Cursor::new(self.get_n_from(4, 2))
            .read_u32::<BigEndian>()
            .ok()?;

        let message_size = message_size as usize;
        if message_size + 6 > self.stream_buffer.len() {
            return None;
        }

        // remove the first 6, because we already peaked at them
        self.stream_buffer.drain(0..6);

        let buffer = self.stream_buffer.drain(0..message_size);

        get_message(message_type, buffer.as_slice()).ok()
    }

    fn get_n(&self, n: usize) -> &[u8] {
        self.get_n_from(n, 0)
    }

    fn get_n_from(&self, n: usize, start: usize) -> &[u8] {
        &self.stream_buffer[start..(n + start)]
    }
}
