use std::collections::HashMap;

use crate::{
    connection::{connection_traits::Shutdown, Connection},
    protocol::message_transmitter::MessageTransmitter,
};
use tauri::State;
use tokio::sync::Mutex;
use tracing::{error, info, trace};

pub struct ConnectionState {
    pub connection: Mutex<Option<Connection>>,
    pub window: Mutex<tauri::Window>,
    pub message_handler: Mutex<HashMap<String, Box<dyn Shutdown + Send>>>,
}

async fn add_message_handler(
    state: &State<'_, ConnectionState>,
    name: String,
    handler: Box<dyn Shutdown + Send>,
) {
    state.message_handler.lock().await.insert(name, handler);
}

#[tauri::command]
pub async fn connect_to_server(
    server_host: String,
    server_port: u16,
    username: String,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    info!("Connecting to server: {server_host}:{server_port}");

    let mut guard = state.connection.lock().await;
    if guard.is_some() {
        // close old connection
        if let Err(e) = guard.as_mut().unwrap().shutdown().await {
            return Err(format!("{e:?}"));
        }
    }

    let connection = guard.insert(Connection::new(&server_host, server_port, &username));
    if let Err(e) = connection.connect().await {
        return Err(format!("{e:?}"));
    }

    let window = state.window.lock().await;

    let mut transmitter = MessageTransmitter::new(connection.get_message_channel(), window.clone());
    transmitter.start_message_transmit_handler().await;
    add_message_handler(&state, "transmitter".to_string(), Box::new(transmitter)).await;

    Ok(())
}

#[tauri::command]
pub async fn send_message(
    chat_message: String,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    let guard = state.connection.lock().await;
    if let Some(guard) = guard.as_ref() {
        if let Err(e) = guard.send_message(&chat_message).await {
            return Err(format!("{e:?}"));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn logout(state: State<'_, ConnectionState>) -> Result<(), String> {
    info!("Got logout request");
    let mut connection = state.connection.lock().await;

    if connection.is_none() {
        return Err("Called logout, but there is no connection!".to_string());
    }

    for (name, thread) in state.message_handler.lock().await.iter_mut() {
        if let Err(e) = thread.shutdown().await {
            error!("Failed to shutdown thread {}: {}", name, e);
        }
        trace!("Joined {}", name.to_string());
    }

    if let Err(e) = connection.as_mut().unwrap().shutdown().await {
        return Err(format!("{e:?}"));
    }

    *connection = None;

    Ok(())
}
