/*!
    由于Tauri不在提供自定义路径文件操作，故
    file包封装了文件操作的一些方法
 */
use std::error::Error;
use std::fs::{File, read};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use rfd::FileDialog;
use tauri::api::file;


//用于打开文件时拉起系统文件管理
fn get_open_file_path() -> PathBuf{
    let path_buf_o = FileDialog::new()
        .add_filter("markdown", &["md"])
        .pick_file();
    let mut path_buf = PathBuf::new();
    if let Some(data) = path_buf_o{
        path_buf = data
    }
    path_buf
}

//用于保存文件时拉起系统文件管理
fn get_save_file_path() -> PathBuf{
    let path_buf_o = FileDialog::new()
        .add_filter("markdown", &["md"])
        .add_filter("html",&["html"])
        .add_filter("pdf",&["pdf"])
        .set_file_name("未命名")
        .save_file();
    let mut path_buf = PathBuf::new();
    if let Some(data) = path_buf_o{
        path_buf = data
    }
    path_buf
}

//打开指定文件
pub fn open_file() -> Result<String,Box<dyn Error>> {
    let mut file = File::open(get_open_file_path())?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

//保存指定文件
pub fn save_file(text: String) ->Result<String,Box<dyn Error>>{
    let path = get_save_file_path();
    let mut file = File::create(path)?;
    file.write_all(text.as_ref())?;
    Ok(String::from("保存成功！！"))
}

//保存修改后的文件
pub fn update_file(path:String, text:String) ->Result<String,Box<dyn Error>>{
    let mut file = File::open(path)?;
    file.write_all(text.as_ref())?;
    Ok(String::from("保存成功！！"))
}

