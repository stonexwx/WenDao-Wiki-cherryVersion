#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::create_toc::Poc;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//目录结构解析
mod create_toc;

#[tauri::command]
fn create_toc(json:&str) -> String{
    Poc::create(json)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_toc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
