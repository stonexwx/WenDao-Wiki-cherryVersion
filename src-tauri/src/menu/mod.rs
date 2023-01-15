/*!
    按钮操作的一些方法
 */
use std::collections::HashMap;
use std::error::Error;
use serde_json::map;

mod file;
//打开操作

pub fn open_menu() -> Result<HashMap<String,String>,Box<dyn Error>>{
    file::open_file()
}
//
// pub fn save_menu(text: String) -> Result<String,Box<dyn Error>>{
//
// }

