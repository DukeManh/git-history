#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod git;

use commands::{get_commits, git_show, read_repo, git_show_diff};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_repo, get_commits, git_show, git_show_diff])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
