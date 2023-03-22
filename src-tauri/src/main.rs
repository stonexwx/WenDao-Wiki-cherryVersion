#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use rusqlite::Connection;
use tauri::{Manager, Window};
use crate::create_toc::Poc;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod create_toc;
mod menu;
mod db;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
//目录生成command
#[tauri::command]
fn create_toc(json:&str,label:&str,window:Window)->Result<(),String>{
    window.emit_to(label, "toc", Payload { message: Poc::create(json) }).map_err(|err| err.to_string())
}

// 打开文件获取内容
#[tauri::command]
fn open()->Result<HashMap<String,String>,String>{
    menu::open_menu().map_err(|err| err.to_string())
}

//根据路劲打开文件获取内容
#[tauri::command]
fn open_file_for_path(path:&str)->Result<HashMap<String,String>,String>{
    menu::get_file_content(path).map_err(|err| err.to_string())
}

//选择文件
#[tauri::command]
fn choose_file()->Result<HashMap<String,String>,String>{
    menu::choose_file().map_err(|err| err.to_string())
}

//另存为
#[tauri::command]
fn save_as(text:&str) ->Result<String,String>{
    menu::save_as_menu(text).map_err(|err| err.to_string())
}

//保存
#[tauri::command]
fn save(text:&str,path:&str) ->Result<String,String>{
    menu::save(text,path).map_err(|err| err.to_string())
}

//插入历史记录
#[tauri::command]
fn set_open_history(path:&str) ->Result<(),String>{
    let connect = Connection::open("./db/cherry.db").unwrap();
    db::set_open_history(&connect, path).map_err(|err| err.to_string())?;
    connect.close().unwrap();
    Ok(())
}

//获取历史记录
#[tauri::command]
fn get_open_history() -> String {
    let connect = Connection::open("./db/cherry.db").unwrap();
    db::get_open_history(&connect)
}


fn main() {
    let connect = Connection::open("./db/cherry.db").unwrap();
    db::init(&connect);
    db::get_open_history(&connect);
    connect.close().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_toc,
            open,
            open_file_for_path,
            choose_file,
            save_as,
            save,
            get_open_history,
            set_open_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
