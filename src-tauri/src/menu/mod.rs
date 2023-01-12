/*!
    按钮操作的一些方法
 */
use std::collections::HashMap;
use serde_json::map;

mod file;
//打开操作

pub fn open_menu() -> HashMap<String,String>{
    file::open_file().unwrap()
}

