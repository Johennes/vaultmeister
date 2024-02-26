// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use matrix_sdk::Client;
use std::{process::{self}, thread};
use tauri::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  launch_dendrite().await;

  let _client = Client::new(Url::parse("localhost:8008")?).await?;

  tauri::Builder::default()
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
    .arg("-config")
    .arg("../dendrite.yaml")
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
