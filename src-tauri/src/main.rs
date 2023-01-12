#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::collections::HashMap;
use std::ops::Add;
use std::sync::Mutex;
use std::thread::sleep;
use std::time;
use tauri::{InvokePayload, Manager, Window, WindowBuilder};
use uuid::Uuid;
use crate::create_toc::Poc;
use lazy_static::lazy_static;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod create_toc;
mod menu;
mod file;

//目录生成command
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
