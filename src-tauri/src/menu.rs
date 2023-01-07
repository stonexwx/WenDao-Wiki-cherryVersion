/*!
    按钮大集合
 */
use tauri::{CustomMenuItem, Menu, Submenu};

pub fn get_menu() -> Menu{
    let file = Submenu::new("文件", Menu::new()
        .add_item(CustomMenuItem::new("open".to_string(),"打开"))
        .add_item(CustomMenuItem::new("new".to_string(),"新建"))
        .add_item(CustomMenuItem::new("save".to_string(),"另存为"))
        .add_item(CustomMenuItem::new("update".to_string(),"保存"))

    );
    let menu = Menu::new()
        .add_submenu(file);
    menu
}