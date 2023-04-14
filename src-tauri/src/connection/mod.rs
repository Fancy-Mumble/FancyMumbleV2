use crate::protocol::init_connection;
use crate::protocol::stream_reader::StreamReader;
use crate::utils::messages::{message_builder, mumble};
use std::error::Error;
use std::time::{Duration, SystemTime};
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::time;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::TlsStream;

const QUEUE_SIZE: usize = 256;
const PING_INTERVAL: Duration = Duration::from_millis(15000);
const BUFFER_SIZE: usize = 1024;

pub struct Connection {
    username: String,
    server_host: String,
    server_port: u16,

    tx_in: Sender<Vec<u8>>,
    tx_out: Sender<Vec<u8>>,
    rx_out: Receiver<Vec<u8>>,

    stream: Option<TlsStream<TcpStream>>,
}

impl Connection {
    pub fn new(server_host: &str, server_port: u16, username: &str) -> Connection {
        let (tx_in, _): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = broadcast::channel(QUEUE_SIZE);
        let (tx_out, rx_out): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = broadcast::channel(QUEUE_SIZE);

        Connection {
            username: username.to_string(),
            server_host: server_host.to_string(),
            server_port,
            tx_in,
            tx_out,
            rx_out,
            stream: None,
        }
    }

    fn spawn_message_thread(&mut self) {
        let mut rx_in = self.tx_in.subscribe();

        tokio::spawn(async move {
            let mut reader = StreamReader::new();

            loop {
                let result = rx_in.recv().await;
                match result {
                    Ok(mut data) => reader.read(&mut data),
                    Err(_) => { print!("Corrupted stream!"); break; },
                }
            }
        });
    }

    fn spawn_ping_thread(&self) {
        let tx_a = self.tx_out.clone();

        // timer thread
        tokio::spawn(async move {
            let mut interval = time::interval(PING_INTERVAL);
            loop {
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
                println!("Writing Ping");
                let buffer = message_builder(ping);
                _ = tx_a.send(buffer);
            }
        });
    }

    fn spawn_output_thread(&self) {
        let tx_b = self.tx_out.clone();
        tokio::spawn(async move {
            loop {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        let message = mumble::proto::TextMessage {
                            actor: None,
                            session: Vec::new(),
                            channel_id: vec![60u32],
                            tree_id: Vec::new(),
                            message: input.clone(),
                        };
                        let buffer = message_builder(message);
                        _ = tx_b.send(buffer);
                    }
                    Err(_) => {}
                };
            }
        });
    }

    async fn init_main_thread(&mut self) -> Result<(), Box<dyn Error>> {
        assert!(self.stream.is_some());

        let mut buffer = [0; BUFFER_SIZE];
        let (reader, mut writer) = tokio::io::split(self.stream.as_mut().unwrap());
        let mut reader = BufReader::new(reader);

        loop {
            select! {
                result = reader.read(&mut buffer) => {
                    let size = result?;
                    if size == 0 {
                        return Err("We didn't get any data from our stream".into());
                    }

                    match self.tx_in.send((&buffer[0..size]).to_vec()) {
                        Ok(_) => {},
                        Err(e) => println!("Error while channeling incomming data: {e:?}"),
                    }
                }
                result = self.rx_out.recv() => {
                    // println!("Sending to server: {msg:?}");
                    writer.write(&result?).await?;
                }
            }
        }
    }

    async fn setup_connection(&mut self) -> Result<(), Box<dyn Error>> {
        let server_uri = format!("{}:{}", self.server_host, self.server_port);
        let socket = TcpStream::connect(server_uri).await?;
        let cx = TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let cx = tokio_native_tls::TlsConnector::from(cx);

        self.stream = Some(cx.connect(&self.server_host, socket).await?);
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.setup_connection().await?;

        init_connection(&self.username, &self.tx_out).await;

        self.spawn_ping_thread();
        self.spawn_message_thread();
        self.spawn_output_thread();

        self.init_main_thread().await
    }
}
