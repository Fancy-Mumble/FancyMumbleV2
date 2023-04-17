#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod protocol;
mod utils;

use commands::ConnectionState;
use tokio::sync::Mutex;

use tauri::Manager;

use crate::commands::{connect_to_server, send_message, logout};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(ConnectionState {
                connection: Mutex::new(None),
                window: Mutex::new(app.get_window("main").unwrap()),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect_to_server,
            send_message,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
