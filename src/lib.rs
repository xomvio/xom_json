use std::{collections::HashMap, io::Error};
pub mod tests;
mod common;
use common::{Val, read_json};

pub fn json_parse(jtext:String) -> Result<HashMap<String, Val>, Error> {
    let mut jchars = jtext.chars();
    let mut hmap:HashMap<String, Val> = HashMap::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'=>{
                (jchars, hmap) = match read_json(jchars) {
                    Ok((jchars, hmap))=>(jchars, hmap),
                    Err(e)=>return Err(e),                    
                };
            }
            ' '|'\t'|'\r'|'\n'=>{}
            _=>{
                Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"", c)))?;
            }
        }
    }
    Ok(hmap)
}

pub fn parse_as_jobject(jtext:String) -> Result<Val, Error> {
    let mut jchars = jtext.chars();
    let mut hmap:HashMap<String, Val> = HashMap::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'=>{
                (jchars, hmap) = match read_json(jchars) {
                    Ok((jchars, hmap))=>(jchars, hmap),
                    Err(e)=>return Err(e),
                };
            }
            ' '|'\t'|'\r'|'\n'=>{}
            _=>{
                Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"", c)))?;
            }
        }
    }
    Ok(Val::Object(hmap))
}
