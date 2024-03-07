// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use matrix_sdk::{config::SyncSettings, ruma::{api::client::room::create_room::{self, v3::CreationContent}}, Client};
use std::{process, sync::Arc, thread};
use tauri::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  launch_dendrite().await;

  let client = Client::builder()
    .homeserver_url(Url::parse("https://localhost:8448")?)
    .disable_ssl_verification()
    .build()
    .await?;

  tauri::Builder::default()
    .manage(Arc::new(client))
    .invoke_handler(tauri::generate_handler![version, homeserver, sign_in, sign_out, start_sync, get_rooms, create_vault])
    .run(tauri::generate_context!())
    .expect("tauri application should not cause error");

  Ok(())
}

async fn launch_dendrite() {
  // Request /versions to check if dendrite is already running (e.g. due to hot reload)
  let running = match reqwest::get("http://localhost:8008/_matrix/client/versions").await {
    Ok(response) => response.status().is_success(),
    Err(_) => false
  };

  if running {
    return;
  }

  // Start subprocess
  let mut subprocess = process::Command::new("../dendrite-src/bin/dendrite")
    .arg("-config").arg("../dendrite.yaml")
    .arg("-tls-cert").arg("../dendrite-data/server.crt")
    .arg("-tls-key").arg("../dendrite-data/server.key")
    .arg("-really-enable-open-registration")
    .spawn()
    .expect("dendrite should be running");

  // Monitor subprocess and exit if it dies
  thread::spawn(move || {
    let result = subprocess.wait();
    if result.is_err() || !result.unwrap().success() {
      eprintln!("dendrite should not exit");
      process::exit(1);
    }
  });
}

#[tauri::command]
fn version() -> String {
  env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn homeserver(client: tauri::State<Arc<Client>>) -> String {
  client.homeserver().to_string()
}

#[tauri::command]
async fn sign_in(client: tauri::State<'_, Arc<Client>>, username: String, password: String) -> Result<(), String> {
  // Ensure that we're not signed in already as otherwise the SDK will panic and kill the thread
  if client.session_meta().is_some() {
    return Ok(());
  }

  match client.matrix_auth().login_username(username, &password).send().await {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string()),
  }
}

#[tauri::command]
async fn sign_out(client: tauri::State<'_, Arc<Client>>) -> Result<(), String> {
  // Ensure that we're not signed out already as otherwise the SDK will panic and kill the thread
  if client.session_meta().is_none() {
    return Ok(());
  }

  match client.matrix_auth().logout().await {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string())
  }
}

#[tauri::command]
async fn start_sync(client: tauri::State<'_, Arc<Client>>) -> Result<(), String> {
  let result = client.sync_once(SyncSettings::default()).await;
  if result.is_err() {
    return Err(result.unwrap_err().to_string());
  }

  let thread_client = Arc::clone(&client);
  tauri::async_runtime::spawn(async move {
    let _ = thread_client.sync(SyncSettings::default().token(result.unwrap().next_batch)).await;
  });

  Ok(())
}

#[derive(serde::Serialize)]
struct FrontendRoom {
  id: String,
  name: Option<String>
}

#[tauri::command]
fn get_rooms(client: tauri::State<'_, Arc<Client>>) -> Vec<FrontendRoom> {
  client.rooms().into_iter().map(|room| FrontendRoom { id: room.room_id().to_string(), name: room.name() }).collect()
}

#[tauri::command]
async fn create_vault(client: tauri::State<'_, Arc<Client>>, name: String) -> Result<String, String> {
  let mut request = create_room::v3::Request::new();
  request.name = Some(name.to_string());
  let mut creation_content = CreationContent::default();
  creation_content.room_type = Some("org.matrix.msc4114.vault".into());
  match client.create_room(request).await {
    Ok(room) => Ok(room.room_id().to_string()),
    Err(error) => Err(error.to_string())
  }
}
