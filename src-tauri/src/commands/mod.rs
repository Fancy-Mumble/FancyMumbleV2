use crate::{connection::Connection, protocol::message_transmitter::MessageTransmitter};
use tauri::State;
use tokio::sync::Mutex;

pub struct ConnectionState {
    pub connection: Mutex<Option<Connection>>,
    pub window: Mutex<tauri::Window>,
}

#[tauri::command]
pub async fn connect_to_server(
    server_host: String,
    server_port: u16,
    username: String,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    println!("Connecting to server: {server_host}:{server_port}");

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
    let transmitter = MessageTransmitter::new(connection.get_message_channel(), window.clone());
    transmitter.message_transmit_handler().await;

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
    println!("Got logout request");
    let mut connection = state.connection.lock().await;

    if connection.is_none() {
        return Err("Called logout, but there is no connection!".to_string());
    }

    if let Err(e) = connection.as_mut().unwrap().shutdown().await {
        return Err(format!("{e:?}"));
    }

    *connection = None;

    Ok(())
}
