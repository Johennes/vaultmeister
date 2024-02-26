// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use matrix_sdk::Client;
use std::{process::{self}, thread};
use tauri::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Launch dendrite
  let mut dendrite = process::Command::new("../dendrite-src/bin/dendrite")
    .arg("-config")
    .arg("../dendrite.yaml")
    .arg("-really-enable-open-registration")
    .spawn()
    .expect("dendrite should be running");

  // Exit if dendrite dies
  thread::spawn(move || {
    let result = dendrite.wait();
    if result.is_err() || !result.unwrap().success() {
      eprintln!("dendrite should not exit");
      process::exit(1);
    }
  });

  // Set up Matrix client
  let _client = Client::new(Url::parse("localhost:8008")?).await?;

  // Run tauri
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("tauri application should not cause error");

  Ok(())
}
