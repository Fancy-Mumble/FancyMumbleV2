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
    PingThread,
    OutputThread,
    InputThread,
    MainThread,
}

impl ToString for ConnectionThread {
    fn to_string(&self) -> String {
        match self {
            ConnectionThread::PingThread => "PingThread".to_string(),
            ConnectionThread::OutputThread => "OutputThread".to_string(),
            ConnectionThread::InputThread => "InputThread".to_string(),
            ConnectionThread::MainThread => "MainThread".to_string(),
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
