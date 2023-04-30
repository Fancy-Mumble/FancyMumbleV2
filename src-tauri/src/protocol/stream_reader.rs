use byteorder::{BigEndian, ReadBytesExt};
use tracing::{error, trace};
use std::io::Cursor;

use crate::{utils::messages::{get_message, MessageInfo}};

use super::message_router::MessageRouter;

pub struct StreamReader {
    stream_buffer: Vec<u8>,
    message_handler: MessageRouter,
}

impl StreamReader {
    pub fn new(message_handler: MessageRouter) -> StreamReader {
        StreamReader {
            stream_buffer: Vec::<u8>::new(),
            message_handler,
        }
    }

    pub fn read_next(&mut self, data: &mut Vec<u8>) {
        self.stream_buffer.append(data);
        while let Some(result) = self.try_read() {
            if let Err(e) = self.message_handler.recv_message(result) {
                error!("Error handling message: {}", e);
            }
        }
    }

    fn try_read(&mut self) -> Option<MessageInfo> {
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
