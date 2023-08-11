// clippy is detecting '_ as a underscore binding, which it shouldn't
#![allow(clippy::used_underscore_binding)]

mod helper;
pub mod settings_cmd;
pub mod web_cmd;
pub mod zip_cmd;

use std::{borrow::BorrowMut, collections::HashMap};

use crate::{
    connection::{traits::Shutdown, Connection},
    errors::string_convertion::ErrorString,
    manager::user::UpdateableUserState,
    protocol::message_transmitter::MessageTransmitter,
    utils::audio::device_manager::AudioDeviceManager,
};
use tauri::State;
use tokio::sync::Mutex;
use tracing::{debug, error, info, trace};

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
    info!("Connecting to server: {server_host}:{server_port}!");

    let mut guard = state.connection.lock().await;
    if let Some(guard) = guard.as_mut() {
        debug!("Found old connection, closing it");
        // close old connection
        if let Err(e) = guard.shutdown() {
            return Err(format!("{e:?}"));
        }
    }

    trace!("Finished closing old connection, creating new one");
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

    transmitter.start_message_transmit_handler();
    add_message_handler(&state, "transmitter".to_string(), Box::new(transmitter)).await;

    Ok(())
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
            if let Err(e) = thread.shutdown() {
                error!("Failed to shutdown thread {}: {}", name, e);
            }
            trace!("Joined {}", name.to_string());
        }
    }

    if let Err(e) = connection
        .as_mut()
        .ok_or("No connection is available, but logout called")?
        .shutdown()
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
