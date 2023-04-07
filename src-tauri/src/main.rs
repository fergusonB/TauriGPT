// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use openai::set_key;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn set_api_key(api_key: String) {
    set_key(api_key);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
