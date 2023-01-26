/*!
    按钮操作的一些方法
 */
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use serde_json::map;

mod file;
/**打开操作
文件打开操作分成了两部分，由于tauri的事件传输速度比vue渲染速度快，导致新窗口很有可能接收不到事件信息
故打开文件分为：1. 创建窗口打开文件。2.在原窗口打开文件
1的情况需要先选文件，然后由前端将目标文件路径返回给Rust读取文件内容
2直接打开文件获取内容即可
**/

//打开按钮情况2
pub fn open_menu() -> Result<HashMap<String,String>,Box<dyn Error>>{
    file::open_file()
}

//打开按钮情况1
pub fn choose_file() -> Result<HashMap<String,String>,Box<dyn Error>>{
    let path = file::get_open_file_path();
    let name  = file::get_file_name(&path);
    let mut map:HashMap<String,String> = HashMap::with_capacity(2);
    map.insert("name".to_string(),name);
    if let Some(res) = path.to_str(){
        map.insert("path".to_string(),res.to_string());
    }
    Ok(map)
}
pub fn get_file_content(path: &str) -> Result<HashMap<String,String>,Box<dyn Error>>{
    file::open_file_for_path(path)
}

//另存为
pub fn save_as_menu(text: &str) -> Result<String,Box<dyn Error>>{
    file::save_file(text.to_string())
}

//保存
pub fn save(text: &str,path:&str) -> Result<String,Box<dyn Error>>{
    file::update_file(path.to_string(),text.to_string())
}

