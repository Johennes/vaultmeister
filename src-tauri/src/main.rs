// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use matrix_sdk::Client;
use std::{process::{self}, thread};
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
    .manage(client)
    .invoke_handler(tauri::generate_handler![version, homeserver, sign_in, sign_out])
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
fn homeserver(client: tauri::State<Client>) -> String {
  client.homeserver().to_string()
}

#[tauri::command]
async fn sign_in(client: tauri::State<'_, Client>, username: String, password: String) -> Result<(), String> {
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
async fn sign_out(client: tauri::State<'_, Client>) -> Result<(), String> {
  // Ensure that we're not signed out already as otherwise the SDK will panic and kill the thread
  if client.session_meta().is_none() {
    return Ok(());
  }

  match client.matrix_auth().logout().await {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string())
  }
}
