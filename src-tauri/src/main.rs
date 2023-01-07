#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::error::Error;
use tauri::{Manager, WindowBuilder, WindowMenuEvent, Wry};
use crate::create_toc::Poc;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod create_toc;
mod menu;
mod file;


#[tauri::command]
fn create_toc(json:&str) -> String{
    Poc::create(json)
}


fn main() {
      tauri::Builder::default()
        .menu(menu::get_menu())
        .on_menu_event(|event|{
            match event.menu_item_id() {
                "open"=>{
                    let handle = event.window().app_handle();
                    std::thread::spawn(move || {
                        let local_window = WindowBuilder::new(
                            &handle,
                            "exeee",
                            tauri::WindowUrl::App("index.html".into())
                        ).build().unwrap();
                    });
                }
                _=>{}
            };
        })
        .invoke_handler(tauri::generate_handler![create_toc])
        // .run(tauri::generate_context!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
