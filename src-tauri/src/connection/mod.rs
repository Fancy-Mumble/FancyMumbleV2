pub mod connection_traits;
use crate::protocol::init_connection;
use crate::protocol::stream_reader::StreamReader;
use crate::utils::messages::{message_builder, mumble, MessageTypes};
use crate::connection::connection_traits::Shutdown;
use std::cmp;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time;
use tokio_native_tls::native_tls::TlsConnector;

use self::connection_traits::HandleMessage;

const QUEUE_SIZE: usize = 256;
const PING_INTERVAL: Duration = Duration::from_millis(5000);
const DEADMAN_INTERVAL: Duration = Duration::from_millis(2000);
const BUFFER_SIZE: usize = 1024;
const MAX_SEND_SIZE: usize = 1024;

struct ThreadReferenceHolder {
    ping_thread: Option<JoinHandle<()>>,
    message_thread: Option<JoinHandle<()>>,
    output_thread: Option<JoinHandle<()>>,
    main_thread: Option<JoinHandle<()>>,
}

#[derive(Debug, Clone)]
pub struct MessageChannels {
    pub message_channel: Sender<String>,
}

pub struct Connection {
    username: String,
    server_host: String,
    server_port: u16,

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
            username: username.to_string(),
            server_host: server_host.to_string(),
            server_port,
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

    fn spawn_message_thread(&mut self) {
        let mut rx_in = self.tx_in.subscribe();
        let running_clone = self.running.clone();
        let message_channels = self.message_channels.clone();

        self.threads.message_thread = Some(tokio::spawn(async move {
            let mut interval = time::interval(DEADMAN_INTERVAL);
            let mut reader = StreamReader::new(message_channels);

            while *running_clone.read().unwrap() {
                select! {
                    Ok(mut result) = rx_in.recv() => {
                        reader.read_next(&mut result);
                    }

                    _ = interval.tick() => {}
                }
            }
        }));
    }

    fn spawn_ping_thread(&mut self) {
        let tx_a = self.tx_out.clone();
        let running_clone = self.running.clone();

        // timer thread
        self.threads.ping_thread = Some(tokio::spawn(async move {
            let mut interval = time::interval(PING_INTERVAL);
            while *running_clone.read().unwrap() {
                let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
                if now.is_err() {
                    println!("Unable to send Ping!");
                    continue;
                }

                //todo: Add actual ping statistics
                let ping = mumble::proto::Ping {
                    timestamp: Some(now.unwrap().as_secs()),
                    good: Some(0),
                    late: Some(0),
                    lost: Some(999),
                    resync: Some(0),
                    tcp_packets: Some(1),
                    tcp_ping_avg: Some(1.2f32),
                    tcp_ping_var: Some(2.0f32),
                    udp_packets: Some(1),
                    udp_ping_avg: Some(2.3f32),
                    udp_ping_var: Some(5.6f32),
                };

                interval.tick().await;
                let buffer = message_builder(ping);
                _ = tx_a.send(buffer);
            }
        }));
    }

    fn spawn_output_thread(&mut self) {
        let tx_b = self.tx_out.clone();
        let running_clone = self.running.clone();
        let mut rx_message_channel = self.tx_message_channel.subscribe();

        self.threads.output_thread = Some(tokio::spawn(async move {
            let mut interval = time::interval(DEADMAN_INTERVAL);

            while *running_clone.read().unwrap() {
                select! {
                    Ok(result) = rx_message_channel.recv() => {
                        let message = mumble::proto::TextMessage {
                            actor: None,
                            session: Vec::new(),
                            channel_id: vec![60u32],
                            tree_id: Vec::new(),
                            message: result,
                        };
                        let buffer = message_builder(message);
                        _ = tx_b.send(buffer);
                    }

                    _ = interval.tick() => {}
                }
            }
        }));
    }

    async fn init_main_thread(
        &mut self,
        stream: Option<tokio_native_tls::TlsStream<TcpStream>>,
    ) -> Result<(), Box<dyn Error>> {
        let mut buffer = [0; BUFFER_SIZE];
        let (reader, mut writer) = tokio::io::split(stream.unwrap());
        let mut reader = BufReader::new(reader);

        let tx_in = self.tx_in.clone();
        let mut rx_out = self.tx_out.subscribe();
        let running_clone = self.running.clone();

        self.threads.main_thread = Some(tokio::spawn(async move {
            while *running_clone.read().unwrap() {
                select! {
                    Ok(size) = reader.read(&mut buffer) => {
                        if size == 0 {
                            return;
                        }

                        match tx_in.send((&buffer[0..size]).to_vec()) {
                            Ok(_) => {},
                            Err(e) => println!("Error while channeling incomming data: {e:?}"),
                        }
                    }
                    Ok(result) = rx_out.recv() => {
                        if result.len() < MAX_SEND_SIZE {
                            println!("Sending to server: {result:?}");
                        }

                        let chunks = result.chunks(cmp::max(1, result.len() / MAX_SEND_SIZE));
                        for chunk in chunks {
                            _ = writer.write(&chunk).await;
                        }
                    }
                }
            }
        }));

        Ok(())
    }

    async fn setup_connection(
        &mut self,
    ) -> Result<Option<tokio_native_tls::TlsStream<TcpStream>>, Box<dyn Error>> {
        let server_uri = format!("{}:{}", self.server_host, self.server_port);
        let socket = TcpStream::connect(server_uri).await?;
        let cx = TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let cx = tokio_native_tls::TlsConnector::from(cx);

        Ok(Some(cx.connect(&self.server_host, socket).await?))
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        {
            if let Ok(mut running) = self.running.write() {
                *running = true;
            }
        }
        let stream = self.setup_connection().await?;

        self.spawn_ping_thread();
        self.spawn_message_thread();
        self.spawn_output_thread();

        self.init_main_thread(stream).await?;
        init_connection(&self.username, self.tx_out.clone()).await;

        Ok(())
    }
    pub async fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        self.tx_message_channel.send(message.to_string())?;

        Ok(())
    }

    pub fn get_message_channel(&self) -> Receiver<String> {
        self.message_channels.message_channel.subscribe()
    }

    pub fn addListener(&self, message_type: MessageTypes, message: Box<dyn HandleMessage>) {

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