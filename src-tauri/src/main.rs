#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod connection;
mod protocol;
mod utils;

use crate::connection::Connection;

#[tauri::command]
async fn connect_to_server(server_host: String, server_port: u16, username: String) {
  println!("Connecting to server: {server_host}:{server_port}");

  let mut connection = Connection::new(&server_host, server_port, &username);

  connection.connect().await;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![connect_to_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
