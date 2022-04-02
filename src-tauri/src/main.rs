#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod git;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![commands::read_repo])
    //.invoke_handler(tauri::generate_handler![commands::get_log])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
