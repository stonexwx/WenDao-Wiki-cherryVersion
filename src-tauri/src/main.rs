#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::error::Error;
use std::thread::sleep;
use std::time;
use lazy_static::lazy_static;
use tauri::{Manager, Window};
use crate::create_toc::Poc;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod create_toc;
mod menu;
//目录生成command
#[tauri::command]
fn create_toc(json:&str) -> String{
    Poc::create(json)
}

#[tauri::command]
fn open()->Result<HashMap<String,String>,String>{
    menu::open_menu().map_err(|err| err.to_string())
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_toc,open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
