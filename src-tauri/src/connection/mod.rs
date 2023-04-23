pub mod connection_threads;
pub mod connection_traits;
use crate::connection::connection_traits::Shutdown;
use crate::protocol::init_connection;
use async_trait::async_trait;
use connection_threads::{InputThread, MainThread, OutputThread, PingThread};
use std::error::Error;
use std::sync::{Arc, RwLock};
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_native_tls::native_tls::TlsConnector;

const QUEUE_SIZE: usize = 256;
const BUFFER_SIZE: usize = 1024;

struct ThreadReferenceHolder {
    ping_thread: Option<JoinHandle<()>>,
    message_thread: Option<JoinHandle<()>>,
    output_thread: Option<JoinHandle<()>>,
    main_thread: Option<JoinHandle<()>>,
}

struct ServerData {
    username: String,
    server_host: String,
    server_port: u16,
}

#[derive(Debug, Clone)]
pub struct MessageChannels {
    pub message_channel: Sender<String>,
}

pub struct Connection {
    server_data: ServerData,
    tx_in: Sender<Vec<u8>>,
    tx_out: Sender<Vec<u8>>,

    tx_message_channel: Sender<String>,

    running: Arc<RwLock<bool>>,
    threads: ThreadReferenceHolder,
    message_channels: MessageChannels,
}

impl Connection {
    pub fn new(server_host: &str, server_port: u16, username: &str) -> Connection {
        let (tx_in, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = broadcast::channel(QUEUE_SIZE);
        let (tx_out, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = broadcast::channel(QUEUE_SIZE);
        let (tx_message_channel, _): (Sender<String>, Receiver<String>) =
            broadcast::channel(QUEUE_SIZE);
        let (message_channel, _): (Sender<String>, Receiver<String>) =
            broadcast::channel(QUEUE_SIZE);

        Connection {
            server_data: ServerData {
                username: username.to_string(),
                server_host: server_host.to_string(),
                server_port,
            },
            tx_in,
            tx_out,
            tx_message_channel,
            running: Arc::new(RwLock::new(false)),
            threads: ThreadReferenceHolder {
                ping_thread: None,
                message_thread: None,
                output_thread: None,
                main_thread: None,
            },
            message_channels: MessageChannels {
                message_channel: message_channel,
            },
        }
    }

    async fn setup_connection(
        &mut self,
    ) -> Result<Option<tokio_native_tls::TlsStream<TcpStream>>, Box<dyn Error>> {
        let server_uri = format!(
            "{}:{}",
            self.server_data.server_host, self.server_data.server_port
        );
        let socket = TcpStream::connect(server_uri).await?;
        let cx = TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let cx = tokio_native_tls::TlsConnector::from(cx);

        Ok(Some(
            cx.connect(&self.server_data.server_host, socket).await?,
        ))
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        {
            if let Ok(mut running) = self.running.write() {
                *running = true;
            }
        }
        let stream = self.setup_connection().await?;

        self.spawn_ping_thread();
        self.spawn_input_thread();
        self.spawn_output_thread();

        self.init_main_thread(stream).await?;
        init_connection(&self.server_data.username, self.tx_out.clone()).await;

        Ok(())
    }

    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        self.tx_message_channel.send(message.to_string())?;

        Ok(())
    }

    pub fn get_message_channel(&self) -> Receiver<String> {
        self.message_channels.message_channel.subscribe()
    }
}

#[async_trait]
impl Shutdown for Connection {
    async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Sending Shutdown Request");
        if let Ok(mut running) = self.running.write() {
            *running = false;
        }
        println!("Joining Threads");

        if let Some(main_thread) = self.threads.main_thread.as_mut() {
            main_thread.await?;
            println!("Joined main_thread");
        }

        if let Some(message_thread) = self.threads.message_thread.as_mut() {
            message_thread.await?;
            println!("Joined message_thread");
        }

        if let Some(output_thread) = self.threads.output_thread.as_mut() {
            output_thread.await?;
            println!("Joined output_thread");
        }

        if let Some(ping_thread) = self.threads.ping_thread.as_mut() {
            ping_thread.await?;
            println!("Joined ping_thread");
        }

        self.threads.main_thread = None;
        self.threads.message_thread = None;
        self.threads.output_thread = None;
        self.threads.ping_thread = None;

        Ok(())
    }
}
