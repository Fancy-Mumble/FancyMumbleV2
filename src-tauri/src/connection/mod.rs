pub mod threads;
pub mod traits;
use crate::connection::traits::Shutdown;
use crate::errors::AnyError;
use crate::manager::user::UpdateableUserState;
use crate::mumble;
use crate::protocol::init_connection;
use crate::protocol::stream_reader::StreamReader;
use crate::utils::certificate_store::CertificateBuilder;
use crate::utils::file::read_image_as_thumbnail;
use crate::utils::messages::message_builder;
use async_trait::async_trait;
use base64::engine::general_purpose;
use base64::Engine;
use native_tls::TlsConnector;
use std::collections::HashMap;
use std::error::Error;
use std::net::TcpStream;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel as StdChannel;
use std::sync::mpsc::Receiver as StdReceiver;
use std::sync::mpsc::Sender as StdSender;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::thread::JoinHandle as StdJoinHandle;
use tauri::PackageInfo;
use threads::{InputThread, MainThread, OutputThread, PingThread};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
//use tokio_native_tls::native_tls::TlsConnector;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use tracing::{info, trace};

use self::threads::ConnectionThread;

const QUEUE_SIZE: usize = 256;
const BUFFER_SIZE: usize = 8192;

struct ServerData {
    username: String,
    server_host: String,
    server_port: u16,
}

#[derive(Debug)]
pub struct MessageChannels {
    pub tx_message_channel: StdSender<String>,
    pub rx_message_channel: Option<StdReceiver<String>>,
}

#[derive(Debug, Clone)]
pub struct TextMessage {
    message: String,
    channel_id: Option<u32>,
    reciever: Option<u32>,
}

pub struct Connection {
    server_data: ServerData,
    tx_in: StdSender<Vec<u8>>,
    rx_in: Option<StdReceiver<Vec<u8>>>,
    tx_out: StdSender<Vec<u8>>,
    rx_out: Option<StdReceiver<Vec<u8>>>,

    tx_message_channel: StdSender<TextMessage>,
    rx_message_channel: Option<StdReceiver<TextMessage>>,

    running: Arc<AtomicBool>,
    threads: HashMap<ConnectionThread, StdJoinHandle<()>>,
    message_channels: MessageChannels,
    package_info: PackageInfo,
    stream_reader: Arc<StdMutex<Option<StreamReader>>>,
}

impl Connection {
    pub fn new(
        server_host: &str,
        server_port: u16,
        username: &str,
        package_info: PackageInfo,
    ) -> Self {
        let (tx_in, rx_in): (StdSender<Vec<u8>>, StdReceiver<Vec<u8>>) = StdChannel();
        let (tx_out, rx_out): (StdSender<Vec<u8>>, StdReceiver<Vec<u8>>) = StdChannel();
        let (tx_message_channel, rx_message_channel): (
            StdSender<TextMessage>,
            StdReceiver<TextMessage>,
        ) = StdChannel();
        let (tx_message_channel_group, rx_message_channel_group): (
            StdSender<String>,
            StdReceiver<String>,
        ) = StdChannel();

        Self {
            package_info,
            server_data: ServerData {
                username: username.to_string(),
                server_host: server_host.to_string(),
                server_port,
            },
            tx_in,
            rx_in: Some(rx_in),
            tx_out,
            rx_out: Some(rx_out),
            tx_message_channel,
            rx_message_channel: Some(rx_message_channel),
            running: Arc::new(AtomicBool::new(false)),
            threads: HashMap::new(),
            message_channels: MessageChannels {
                tx_message_channel: tx_message_channel_group,
                rx_message_channel: Some(rx_message_channel_group),
            },
            stream_reader: Arc::new(StdMutex::new(None)),
        }
    }

    fn setup_connection(&mut self) -> AnyError<Option<native_tls::TlsStream<TcpStream>>> {
        let server_uri = format!(
            "{}:{}",
            self.server_data.server_host, self.server_data.server_port
        );

        let mut certificate_store = CertificateBuilder::new()
            .load_or_generate_new(true)
            .store_to_project_dir(true)
            .build()?;

        let socket = TcpStream::connect(server_uri)?;
        let cx = TlsConnector::builder()
            .identity(certificate_store.get_client_certificate()?)
            .danger_accept_invalid_certs(true)
            .build()?;
        let cx = native_tls::TlsConnector::from(cx);

        Ok(Some(cx.connect(&self.server_data.server_host, socket)?))
    }

    pub async fn connect(&mut self) -> AnyError<()> {
        {
            self.running
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        trace!("Connecting to server: setup_connection()");
        let stream = self.setup_connection()?;

        self.spawn_ping_thread();
        self.spawn_input_thread();
        self.spawn_output_thread();

        self.init_main_thread(stream)?;
        init_connection(&self.server_data.username, &self.tx_out, &self.package_info);

        Ok(())
    }

    pub fn send_message(
        &self,
        channel_id: Option<u32>,
        reciever: Option<u32>,
        message: &str,
    ) -> AnyError<()> {
        self.tx_message_channel.send(TextMessage {
            message: message.to_string(),
            channel_id,
            reciever,
        })?;

        Ok(())
    }

    pub fn get_message_channel(&mut self) -> Arc<StdMutex<StdReceiver<String>>> {
        Arc::new(StdMutex::new(
            self.message_channels.rx_message_channel.take().unwrap(),
        ))
    }

    //TODO: Move to output Thread
    pub fn like_message(&self, message_id: &str) -> AnyError<()> {
        let like_message = mumble::proto::PluginDataTransmission {
            sender_session: None,
            receiver_sessions: Vec::new(),
            data: Some(message_id.as_bytes().to_vec()),
            data_id: None,
        };
        self.tx_out.send(message_builder(&like_message)?)?;

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
                self.tx_out
                    .send(message_builder(&set_profile_background)?)?;
            }
            "profile" => {
                let image_vec = Some(image_data);
                let set_profile_background = mumble::proto::UserState {
                    texture: image_vec,
                    ..Default::default()
                };
                self.tx_out
                    .send(message_builder(&set_profile_background)?)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub fn update_user_info(&self, user: &mut UpdateableUserState) -> AnyError<()> {
        let updated_state = mumble::proto::UserState {
            session: None,
            actor: None,
            name: std::mem::take(&mut user.name),
            user_id: None, /*Some(user.id)*/
            channel_id: user.channel_id,
            mute: user.mute,
            deaf: user.deaf,
            suppress: user.suppress,
            self_mute: user.self_mute,
            self_deaf: user.self_deaf,
            texture: None,
            plugin_context: None,
            plugin_identity: None,
            comment: user.comment.take(),
            hash: None,
            comment_hash: None,
            texture_hash: None,
            priority_speaker: user.priority_speaker,
            recording: user.recording,
            temporary_access_tokens: Vec::new(),
            listening_channel_add: Vec::new(),
            listening_channel_remove: Vec::new(),
            listening_volume_adjustment: Vec::new(),
        };
        self.tx_out.send(message_builder(&updated_state)?)?;

        Ok(())
    }
}

#[async_trait]
impl Shutdown for Connection {
    fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Sending Shutdown Request");
        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        trace!("Joining Threads");
        /*if let Some(mut reader) = self.stream_reader.lock()?.take() {
            reader.shutdown();
        }

        for (name, thread) in &mut self.threads {
            thread.join();
            trace!("Joined {}", name.to_string());
        }*/

        self.threads.clear();

        Ok(())
    }
}
