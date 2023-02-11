/*!
用于封装rusqlite的一些操作，暂时搁置，进修一下
*/

// use std::any::Any;
// use std::error::Error;
// use rusqlite::{Connection, Params};
//
// //查询返回List集合
// pub fn get_list<T,P: Params>(connect: &Connection,t:T,sql:&str,params:P) -> Result<Vec<T>,Box<dyn Error>>{
//
//     let mut stmt = connect.prepare(sql)?;
//     stmt.query_map(params,|row|{
//         t.type_id()
//     })
// }