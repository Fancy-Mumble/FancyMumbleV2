pub mod threads;
pub mod traits;
use crate::connection::traits::Shutdown;
use crate::errors::AnyError;
use crate::mumble;
use crate::protocol::init_connection;
use crate::protocol::stream_reader::StreamReader;
use crate::utils::certificate_store::get_client_certificate;
use crate::utils::file::read_image_as_thumbnail;
use crate::utils::messages::message_builder;
use async_trait::async_trait;
use base64::engine::general_purpose;
use base64::Engine;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::PackageInfo;
use threads::{InputThread, MainThread, OutputThread, PingThread};
use tokio::net::TcpStream;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_native_tls::native_tls::TlsConnector;
use tracing::{info, trace};

use self::threads::ConnectionThread;

const QUEUE_SIZE: usize = 256;
const BUFFER_SIZE: usize = 8192;

struct ServerData {
    username: String,
    server_host: String,
    server_port: u16,
}

#[derive(Debug, Clone)]
pub struct MessageChannels {
    pub message_channel: Sender<String>,
}

#[derive(Debug, Clone)]
pub struct TextMessage {
    message: String,
    channel_id: Option<u32>,
}

pub struct Connection {
    server_data: ServerData,
    tx_in: Sender<Vec<u8>>,
    tx_out: Sender<Vec<u8>>,

    tx_message_channel: Sender<TextMessage>,

    running: Arc<AtomicBool>,
    threads: HashMap<ConnectionThread, JoinHandle<()>>,
    message_channels: MessageChannels,
    package_info: PackageInfo,
    stream_reader: Arc<Mutex<Option<StreamReader>>>,
}

impl Connection {
    pub fn new(
        server_host: &str,
        server_port: u16,
        username: &str,
        package_info: PackageInfo,
    ) -> Self {
        let (tx_in, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = broadcast::channel(QUEUE_SIZE);
        let (tx_out, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = broadcast::channel(QUEUE_SIZE);
        let (tx_message_channel, _): (Sender<TextMessage>, Receiver<TextMessage>) =
            broadcast::channel(QUEUE_SIZE);
        let (message_channel, _): (Sender<String>, Receiver<String>) =
            broadcast::channel(QUEUE_SIZE);

        Self {
            package_info,
            server_data: ServerData {
                username: username.to_string(),
                server_host: server_host.to_string(),
                server_port,
            },
            tx_in,
            tx_out,
            tx_message_channel,
            running: Arc::new(AtomicBool::new(false)),
            threads: HashMap::new(),
            message_channels: MessageChannels { message_channel },
            stream_reader: Arc::new(Mutex::new(None)),
        }
    }

    async fn setup_connection(
        &mut self,
    ) -> AnyError<Option<tokio_native_tls::TlsStream<TcpStream>>> {
        let server_uri = format!(
            "{}:{}",
            self.server_data.server_host, self.server_data.server_port
        );

        let socket = TcpStream::connect(server_uri).await?;
        let cx = TlsConnector::builder()
            .identity(get_client_certificate()?)
            .danger_accept_invalid_certs(true)
            .build()?;
        let cx = tokio_native_tls::TlsConnector::from(cx);

        Ok(Some(
            cx.connect(&self.server_data.server_host, socket).await?,
        ))
    }

    pub async fn connect(&mut self) -> AnyError<()> {
        {
            self.running
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        let stream = self.setup_connection().await?;

        self.spawn_ping_thread();
        self.spawn_input_thread();
        self.spawn_output_thread();

        self.init_main_thread(stream).await?;
        init_connection(&self.server_data.username, &self.tx_out, &self.package_info);

        Ok(())
    }

    pub fn send_message(&self, channel_id: Option<u32>, message: &str) -> AnyError<()> {
        self.tx_message_channel.send(TextMessage {
            message: message.to_string(),
            channel_id,
        })?;

        Ok(())
    }

    pub fn get_message_channel(&self) -> Receiver<String> {
        self.message_channels.message_channel.subscribe()
    }

    //TODO: Move to output Thread
    pub fn like_message(&self, message_id: &str) -> AnyError<()> {
        let like_message = mumble::proto::PluginDataTransmission {
            sender_session: None,
            receiver_sessions: Vec::new(),
            data: Some(message_id.as_bytes().to_vec()),
            data_id: None,
        };
        self.tx_out.send(message_builder(&like_message))?;

        Ok(())
    }

    pub fn set_user_image(&self, image_path: &str, image_type: &str) -> AnyError<()> {
        let image = read_image_as_thumbnail(image_path, 512)?;
        let image_data = image.data;
        if image_data.len() > 1024 * 8192 {
            return Err("Image is too big".into());
        }

        match image_type {
            "background" => {
                let mime_type = image.format;
                let background = general_purpose::STANDARD.encode(image_data);
                let img: Option<String> = Some(format!(
                    "<img src='data:image/{mime_type};base64,{background}' />"
                ));

                let set_profile_background = mumble::proto::UserState {
                    comment: img,
                    ..Default::default()
                };
                self.tx_out.send(message_builder(&set_profile_background))?;
            }
            "profile" => {
                let image_vec = Some(image_data);
                let set_profile_background = mumble::proto::UserState {
                    texture: image_vec,
                    ..Default::default()
                };
                self.tx_out.send(message_builder(&set_profile_background))?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn join_channel(&self, channel_id: u32) -> AnyError<()> {
        let join_channel = mumble::proto::UserState {
            session: None,
            actor: None,
            name: None,
            user_id: None,
            channel_id: Some(channel_id),
            mute: None,
            deaf: None,
            suppress: None,
            self_mute: None,
            self_deaf: None,
            texture: None,
            plugin_context: None,
            plugin_identity: None,
            comment: None,
            hash: None,
            comment_hash: None,
            texture_hash: None,
            priority_speaker: None,
            recording: None,
            temporary_access_tokens: Vec::new(),
            listening_channel_add: Vec::new(),
            listening_channel_remove: Vec::new(),
            listening_volume_adjustment: Vec::new(),
        };
        self.tx_out.send(message_builder(&join_channel))?;

        Ok(())
    }

    pub fn update_user_info(&self) -> AnyError<()> {
        todo!()
    }
}

#[async_trait]
impl Shutdown for Connection {
    async fn shutdown(&mut self) -> AnyError<()> {
        info!("Sending Shutdown Request");
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        trace!("Joining Threads");
        if let Some(mut reader) = self.stream_reader.lock().await.take() {
            reader.shutdown().await?;
        }

        for (name, thread) in &mut self.threads {
            thread.await?;
            trace!("Joined {}", name.to_string());
        }

        self.threads.clear();

        Ok(())
    }
}
