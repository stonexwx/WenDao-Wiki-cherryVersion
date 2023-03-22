/*!
 文档打开历史记录
 */
use std::collections::HashMap;
use std::error::Error;
use chrono::prelude::*;
use rusqlite::{Connection, params};
use serde_json::json;
use serde::{Serialize,Deserialize};
#[derive(Debug,Serialize, Deserialize)]
pub struct History {
    h_id: usize,
    path: String,
    date: String
}

impl History{
    pub fn new(path:String) -> History{
        History{
            h_id:0,
            path,
            date:Local::now().to_string()
        }
    }

}

//创建open_history表
pub fn create_history_table(connect: &Connection) -> Result<(),Box<dyn Error>>{
    connect.execute("CREATE TABLE open_history(h_id INTEGER PRIMARY KEY AUTOINCREMENT,path VARCHAR NOT NULL,date TEXT NOT NULL)", ())?;
    Ok(())
}

//向数据库中保存打开历史记录
pub fn set_history(path: &str,connect: &Connection) -> Result<(),Box<dyn Error>>{

    let mut stmt = connect.prepare("select open_history.* from open_history ORDER BY open_history.date ASC")?;
    let history_iter = stmt.query_map([],|row| {
        Ok(History{
            h_id:row.get(0)?,
            path:row.get(1)?,
            date:row.get(2)?
        })
    })?;

    let mut res:Vec<History> = Vec::new();
    for (index,history) in history_iter.enumerate(){
        let path2 = match history {
            Ok(path) => path,
            _=>History::new("".to_string()),
        };
        println!("6666:{}",index);

        if path2.path.eq(&path){
            connect.execute("UPDATE open_history set date =?1 where rowid = ?2 ",
                            params![Local::now().to_string(),path2.h_id])?;
            return Ok(());
        }
        res.push(path2);
    }
    return if res.len() < 10 {
        connect.execute("INSERT INTO open_history(path,date) values(?1,?2)",
                        params![path,Local::now().to_string()])?;
        Ok(())
    } else {
        connect.execute("UPDATE open_history set path= ?1 , date =?2 where rowid = ?3 ",
                        params![path,Local::now().to_string(),res[0].h_id])?;
        Ok(())
    }
}

//获取打开历史记录
pub fn get_history(connect: &Connection) -> Result<String,Box<dyn Error>> {

    let mut stmt = connect.prepare("select h_id,path,date from open_history")?;
    let history_iter = stmt.query_map([],|row|{
        Ok(History{
            h_id: row.get(0)?,
            path: row.get(1)?,
            date: row.get(2)?,
        })
    })?;
    let mut res:HashMap<usize,History> = HashMap::new();
    for (i,history) in history_iter.into_iter().enumerate(){
        res.insert(i,history?);
    }
    Ok(serde_json::to_string(&res)?)
}

//清楚所有历史记录
pub fn clear_history(connect: &Connection) -> Result<(),Box<dyn Error>>{
    connect.execute("DELETE from open_history", ())?;
    Ok(())
}




