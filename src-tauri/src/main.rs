#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod errors;
mod protocol;
mod utils;

use std::collections::HashMap;

use commands::ConnectionState;
use tokio::sync::Mutex;

use tauri::Manager;
use tracing_subscriber;
use tracing_subscriber::fmt;

use crate::commands::{connect_to_server, logout, send_message};

fn init_logging() {
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact(); // use the `Compact` formatting style.

    tracing_subscriber::fmt().event_format(format).init();
}

fn main() {
    init_logging();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(ConnectionState {
                connection: Mutex::new(None),
                window: Mutex::new(app.get_window("main").unwrap()),
                message_handler: Mutex::new(HashMap::new()),
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
