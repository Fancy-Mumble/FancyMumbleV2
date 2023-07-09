// clippy is detecting '_ as a underscore binding, which it shouldn't
#![allow(clippy::used_underscore_binding)]

mod helper;

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::{Seek, SeekFrom, Write},
};

use crate::{
    connection::{traits::Shutdown, Connection},
    errors::string_convertion::ErrorString,
    manager::user::UpdateableUserState,
    protocol::message_transmitter::MessageTransmitter,
    utils::{
        audio::device_manager::AudioDeviceManager, constants::get_project_dirs, server::Server,
    },
};
use base64::{engine::general_purpose, Engine};
use serde_json::json;
use tauri::State;
use tokio::sync::Mutex;
use tracing::{error, info, trace};
use webbrowser::{Browser, BrowserOptions};

use self::helper::OpenGraphCrawler;

pub struct ConnectionState {
    pub connection: Mutex<Option<Connection>>,
    pub window: Mutex<tauri::Window>,
    pub package_info: Mutex<tauri::PackageInfo>,
    pub message_handler: Mutex<HashMap<String, Box<dyn Shutdown + Send>>>,
    pub device_manager: Mutex<Option<AudioDeviceManager>>,
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
    if let Some(guard) = guard.as_mut() {
        // close old connection
        if let Err(e) = guard.shutdown().await {
            return Err(format!("{e:?}"));
        }
    }

    let app_info = state.package_info.lock().await.clone();
    let connection = guard.insert(Connection::new(
        &server_host,
        server_port,
        &username,
        app_info,
    ));
    if let Err(e) = connection.connect().await {
        return Err(format!("{e:?}"));
    }

    let window = state.window.lock().await;

    let mut transmitter = MessageTransmitter::new(connection.get_message_channel(), window.clone());
    drop(guard);

    transmitter.start_message_transmit_handler().await;
    add_message_handler(&state, "transmitter".to_string(), Box::new(transmitter)).await;

    Ok(())
}

#[tauri::command]
pub fn save_server(
    description: &str,
    server_host: &str,
    server_port: u16,
    username: &str,
) -> Result<(), String> {
    info!("Saving server: {server_host}:{server_port}");
    let project_dirs = get_project_dirs().ok_or("Unable to load project dir")?;

    let data_dir = project_dirs.config_dir();

    // create config dir if it doesn't exist
    std::fs::create_dir_all(data_dir).map_err(|e| format!("{e:?}"))?;

    // open server.json or create it if it doesn't exist
    let mut server_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(data_dir.join("server.json"))
        .map_err(|e| format!("Error opening file: {e:?}"))?;

    // read the json content using serde and append the new server
    let mut server_list =
        serde_json::from_reader::<&std::fs::File, Vec<Server>>(&server_file).unwrap_or_default();

    // check if the server is already in the list
    for server in &server_list {
        if server.host == server_host && server.port == server_port {
            return Err("Server already exists".to_string());
        }
    }

    server_list.push(Server {
        description: description.to_string(),
        host: server_host.to_string(),
        port: server_port,
        username: username.to_string(),
    });

    trace!("Server list: {:#?}", server_list);

    // write the new json content
    server_file
        .seek(SeekFrom::Start(0))
        .map_err(|e| format!("{e:?}"))?;
    server_file
        .write_all(
            serde_json::to_string_pretty(&server_list)
                .map_err(|e| format!("{e:?}"))?
                .as_bytes(),
        )
        .map_err(|e| format!("{e:?}"))?;

    Ok(())
}

#[tauri::command]
pub fn get_server_list() -> Result<Vec<Server>, String> {
    info!("Getting server list");
    let project_dirs = get_project_dirs().ok_or("Unable to load project dir")?;

    let data_dir = project_dirs.config_dir();

    // create config dir if it doesn't exist
    std::fs::create_dir_all(data_dir).map_err(|e| format!("{e:?}"))?;

    // open server.json or create it if it doesn't exist
    let server_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(data_dir.join("server.json"))
        .map_err(|e| format!("Error opening file: {e:?}"))?;

    // read the json content using serde
    let server_list =
        serde_json::from_reader::<&std::fs::File, Vec<Server>>(&server_file).unwrap_or_default();

    trace!("Server list: {:#?}", server_list);

    Ok(server_list)
}

#[tauri::command]
pub async fn send_message(
    chat_message: String,
    channel_id: Option<u32>,
    reciever: Option<u32>,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    let guard = state.connection.lock().await;
    if let Some(guard) = guard.as_ref() {
        if let Err(e) = guard.send_message(channel_id, reciever, &chat_message) {
            return Err(format!("{e:?}"));
        }
    }

    Ok(())
}

#[allow(clippy::significant_drop_tightening)]
#[tauri::command]
pub async fn logout(state: State<'_, ConnectionState>) -> Result<(), String> {
    info!("Got logout request");
    let mut connection = state.connection.lock().await;

    if connection.is_none() {
        return Err("Called logout, but there is no connection!".to_string());
    }

    {
        let mut lock_guard = state.message_handler.lock().await;
        #[allow(clippy::significant_drop_in_scrutinee)]
        for (name, thread) in lock_guard.iter_mut() {
            if let Err(e) = thread.shutdown().await {
                error!("Failed to shutdown thread {}: {}", name, e);
            }
            trace!("Joined {}", name.to_string());
        }
    }

    if let Err(e) = connection
        .as_mut()
        .ok_or("No connection is available, but logout called")?
        .shutdown()
        .await
    {
        return Err(format!("{e:?}"));
    }

    *connection = None;

    Ok(())
}

#[tauri::command]
pub async fn like_message(
    message_id: String,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    let guard = state.connection.lock().await;
    if let Some(guard) = guard.as_ref() {
        if let Err(e) = guard.like_message(&message_id) {
            return Err(format!("{e:?}"));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn set_user_image(
    image_path: &str,
    image_type: String,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    let connection = &state.connection;
    let guard = connection.lock().await;

    if let Some(guard) = guard.as_ref() {
        if let Err(error) = guard.set_user_image(image_path, &image_type) {
            return Err(format!("{error:?}"));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn change_user_state(
    mut user_state: UpdateableUserState,
    state: State<'_, ConnectionState>,
) -> Result<(), String> {
    let connection = &state.connection;
    let guard = connection.lock().await;
    trace!("Got change user state change request");

    if let Some(guard) = guard.as_ref() {
        if let Err(error) = guard.update_user_info(user_state.borrow_mut()) {
            return Err(format!("{error:?}"));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_audio_devices(
    state: State<'_, ConnectionState>,
) -> Result<Vec<String>, ErrorString> {
    let device_manager = &state.device_manager;
    let mut guard = device_manager.lock().await;

    if guard.is_none() {
        *guard = Some(AudioDeviceManager::new());
    }

    if let Some(guard) = guard.as_mut() {
        if let Ok(mut devices) = guard.get_audio_device() {
            return Ok(devices.drain().map(|d| d.1).collect());
        }
    }

    Err(ErrorString("Failed to get audio devices".to_string()))
}

#[tauri::command]
pub fn zip_data_to_utf8(data: &str, quality: u32) -> Result<String, String> {
    trace!("zipping data {:?}", data);

    let mut buffer = Vec::new();
    let lg_windows_size = 22;

    {
        let cursor = std::io::Cursor::new(&mut buffer);
        let mut writer = brotli::CompressorWriter::new(cursor, 4096, quality, lg_windows_size);
        writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
    }

    let encoded = general_purpose::STANDARD.encode(buffer);
    Ok(encoded)
}

#[tauri::command]
pub fn unzip_data_from_utf8(data: &str) -> Result<String, String> {
    let decoded_data = general_purpose::STANDARD
        .decode(data)
        .map_err(|e| e.to_string())?;
    let mut writer = brotli::DecompressorWriter::new(Vec::new(), 4096);
    writer.write_all(&decoded_data).map_err(|e| e.to_string())?;
    let output = writer.into_inner().map_err(|_| "Decompress Error")?;

    let result = String::from_utf8(output).map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
pub fn open_browser(url: &str) -> Result<(), String> {
    if let Err(e) = webbrowser::open_browser_with_options(
        Browser::Default,
        url,
        BrowserOptions::new().with_suppress_output(false),
    ) {
        return Err(format!("{e:?}"));
    }

    Ok(())
}

pub struct CrawlerState {
    pub crawler: Mutex<Option<OpenGraphCrawler>>,
}

#[tauri::command]
pub async fn get_open_graph_data_from_website(
    state: State<'_, CrawlerState>,
    url: &str,
) -> Result<String, String> {
    // setup crawler if not already done
    let result = {
        let mut client = state.crawler.lock().await;
        if client.is_none() {
            *client = OpenGraphCrawler::try_new();
        }

        client
            .as_ref()
            .ok_or_else(|| "Failed to read website body".to_string())?
            .crawl(url)
            .await
    };

    let result = json!(result);

    Ok(result.to_string())
}
