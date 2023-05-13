#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod connection;
mod errors;
mod manager;
mod mumble;
mod protocol;
mod utils;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use commands::ConnectionState;
use tokio::sync::Mutex;

use tauri::Manager;
use tracing::Level;
use tracing_subscriber::fmt;

use crate::commands::{
    change_user_state, connect_to_server, join_channel, like_message, logout, send_message,
    set_user_image,
};

fn init_logging() {
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact(); // use the `Compact` formatting style.

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .event_format(format)
        .init();
}

fn main() {
    init_logging();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(ConnectionState {
                connection: Mutex::new(None),
                window: Mutex::new(app.get_window("main").unwrap()),
                package_info: Mutex::new(app.package_info().clone()),
                message_handler: Mutex::new(HashMap::new()),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect_to_server,
            send_message,
            logout,
            like_message,
            join_channel,
            set_user_image,
            change_user_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
