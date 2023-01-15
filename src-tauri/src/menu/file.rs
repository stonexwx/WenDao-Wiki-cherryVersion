/*!
    由于Tauri不在提供自定义路径文件操作，故
    file包封装了文件操作的一些方法
 */
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File};
use std::io::{Read, Write};
use std::path::{PathBuf};
use rfd::FileDialog;


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
pub fn open_file() -> Result<HashMap<String,String>,Box<dyn Error>> {
    let path = get_open_file_path();
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut map: HashMap<String,String> = HashMap::with_capacity(2);
    map.insert("text".to_string(),s);
    map.insert("name".to_string(),get_file_name(&path));
    if let Some(res) = path.to_str(){
        map.insert("path".to_string(),res.to_string());
    }
    Ok(map)
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
//获取文件名
fn get_file_name(path: &PathBuf) -> String{
    let mut s = String::new();
    if let Some(name) = path.file_name(){
        if let Some(res) = name.to_str(){
            let a: Vec<&str> = res.split(".").collect();
            s = a[0].to_string();
        }
    }
    s
}

