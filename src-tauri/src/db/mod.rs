/*!
操作sqllite的一些方法
 */

use lazy_static::lazy_static;
use rusqlite::{Connection, params, Result};
use std::sync::Mutex;

mod open_history_table;
mod file_history_table;

//检测表是否存在
fn check_table_existed(table_name : &str, con : &Connection) -> bool {
    let sql: &str = "SELECT COUNT(`name`) FROM `sqlite_master` WHERE `type` = 'table' AND `name` = ?";
    let mut stmt = con.prepare(sql).unwrap();
    let rs = stmt.query_row(params![table_name], |row | {
        return row.get(0) as Result<i32>;
    });

    let count = rs.unwrap();

    return count > 0;
}

pub fn init(connect: &Connection){
    if !check_table_existed("history",connect){
        if let Err(e) = open_history_table::create_history_table(connect){
            println!("{:?}",e);
        }
    }
}