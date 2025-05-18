use async_trait::async_trait;

mod input_thread;
mod main_thread;
mod output_thread;
mod ping_thread;
use std::time::Duration;
use tokio::net::TcpStream;

use crate::errors::AnyError;
use std::fmt;

pub const DEADMAN_INTERVAL: Duration = Duration::from_millis(500);
pub const MAX_PING_FAILURES: u8 = 3;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConnectionThread {
    Ping,
    Output,
    Input,
    Main,
}

impl fmt::Display for ConnectionThread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let thread_name = match self {
            Self::Ping => "PingThread",
            Self::Output => "OutputThread",
            Self::Input => "InputThread",
            Self::Main => "MainThread",
        };
        write!(f, "{thread_name}")
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
    ) -> AnyError<()>;
}
