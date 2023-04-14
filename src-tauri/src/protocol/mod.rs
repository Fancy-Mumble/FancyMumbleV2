pub mod stream_reader;
pub mod message_handler;
pub mod message_transmitter;
pub mod serialize;

use crate::utils::messages::{message_builder, mumble};
use tokio::sync::broadcast::Sender;

pub async fn init_connection(username: &str, channel: Sender<Vec<u8>>) {
    let version = mumble::proto::Version {
        version_v1: Some((2 << 16) | (0 << 8)),
        os: Some("Rust".to_string()),
        os_version: Some("11".to_string()),
        release: Some("Mumble Rust without scroll bug".to_string()),
        version_v2: None,
    };

    let buffer = message_builder(version);
    _ = channel.send(buffer);

    let auth = mumble::proto::Authenticate {
        opus: Some(true),
        celt_versions: vec![-2147483637, -2147483632],
        password: None,
        tokens: vec![],
        username: Some(username.to_string()),
        client_type: Some(0),
    };

    let buffer = message_builder(auth);
    _ = channel.send(buffer);
}
