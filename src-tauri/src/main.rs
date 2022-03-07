#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::path::Path;

#[tauri::command]
fn read_repo(local_repo: String) {
  // Create a path to the desired file
  let path = Path::new(&local_repo);
  let display = path.display();

  // Open the path in read-only mode, returns `io::Result<File>`
  match fs::read_dir(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(files) => {
      for file in files {
        println!("{:?}", file.unwrap().path());
      }
    }
  };
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_repo])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
