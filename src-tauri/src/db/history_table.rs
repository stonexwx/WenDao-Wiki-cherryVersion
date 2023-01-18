/*!
 文档打开历史记录
 */
use std::error::Error;
use chrono::prelude::*;
use rusqlite::Connection;

struct History {
    file_name:String,
    path:String,
    date: DateTime<Local>
}

impl History{
    pub fn new(file_name:String, path:String) -> History{
        History{
            file_name,
            path,
            date:Local::now()
        }
    }
}

pub fn create_history_table(connect: &Connection) -> Result<(),Box<dyn Error>>{
    connect.execute("CREATE TABLE history(h_id INTEGER PRIMARY KEY AUTOINCREMENT,file_name VARCHAR NOT NULL,path VARCHAR NOT NULL,date TEXT NOT NULL)", ())?;
    Ok(())
}




