use async_trait::async_trait;

mod input_thread;
mod main_thread;
mod output_thread;
mod ping_thread;
use std::{error::Error, time::Duration};
use tokio::net::TcpStream;

pub const DEADMAN_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConnectionThread {
    Ping,
    Output,
    Input,
    Main,
}

impl ToString for ConnectionThread {
    fn to_string(&self) -> String {
        match self {
            Self::Ping => "PingThread".to_string(),
            Self::Output => "OutputThread".to_string(),
            Self::Input => "InputThread".to_string(),
            Self::Main => "MainThread".to_string(),
        }
    }
}

pub trait PingThread {
    fn spawn_ping_thread(&mut self);
}

pub trait OutputThread {
    fn spawn_output_thread(&mut self);
}

pub trait InputThread {
    fn spawn_input_thread(&mut self);
}

#[async_trait]
pub trait MainThread {
    async fn init_main_thread(
        &mut self,
        stream: Option<tokio_native_tls::TlsStream<TcpStream>>,
    ) -> Result<(), Box<dyn Error>>;
}
