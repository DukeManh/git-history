#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod git;

use commands::{get_commits, read_repo};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_repo, get_commits])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
