// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process::{self, ExitCode}, thread};

fn main() {
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

  // Run tauri
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("tauri application should not cause error");
}
