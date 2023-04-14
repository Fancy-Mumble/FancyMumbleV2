#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod connection;
mod protocol;
mod utils;
use tokio::sync::Mutex;

use tauri::State;

use crate::connection::Connection;

struct ConnectionState {
    connection: Mutex<Option<Connection>>,
}

#[tauri::command]
async fn connect_to_server(
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

    Ok(())
}

#[tauri::command]
async fn send_message(
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

fn main() {
    tauri::Builder::default()
        .manage(ConnectionState {
            connection: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![connect_to_server, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
