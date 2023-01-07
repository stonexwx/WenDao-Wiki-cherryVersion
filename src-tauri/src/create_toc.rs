use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Poc {
    level: u32,
    id: String,
    text: String
}

impl Poc{
    fn from(json:&str)->Vec<Poc>{
        let poc:Vec<Poc> = serde_json::from_str(json).unwrap();
        poc
    }

    pub fn create(json:&str)->String{
        let poc = Poc::from(json);
        // let li = "<ul style=\"list-style:none; padding:0; margin:0;\" >";
        let mut res: String = String::new();
        for a in poc{
            match a.level {
                1 => {
                    res.push_str("<li style=\"border-bottom: 1px ridge #dfe6ee;list-style: none;\">");
                    res.push_str(&a.text);
                    res.push_str("</li>");
                },
                2 => {
                    res.push_str("<li style=\"border-bottom: 1px ridge #dfe6ee;list-style: none;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                    res.push_str(&a.text);
                    res.push_str("</li>");
                },
                3 => {
                    res.push_str("<li style=\"border-bottom: 1px ridge #dfe6ee;list-style: none;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                    res.push_str(&a.text);
                    res.push_str("</li>");
                } ,
                4 => {
                    res.push_str("<li style=\"border-bottom: 1px ridge #dfe6ee;list-style: none;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                    res.push_str(&a.text);
                    res.push_str("</li>");
                },
                5 => {
                    res.push_str("<li style=\"border-bottom: 1px ridge #dfe6ee;list-style: none;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                    res.push_str(&a.text);
                    res.push_str("</li>");
                },
                6 => {
                    res.push_str("<li style=\"border-bottom: 1px ridge #dfe6ee;list-style: none;\">&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
                    res.push_str(&a.text);
                    res.push_str("</li>");
                },
                _ => {}
            }
        }
        // res.push_str("</ul>");
        res
    }
}


