use std::io::Error;
pub mod tests;
mod common;
use common::{get_arr, read_json, JObject, JArray};
pub use common::Val;

pub fn to_jobject(jtext:String) -> Result<JObject, Error> {
    let mut jchars = jtext.chars();
    let mut jobj: JObject = JObject::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'|'['=>{
                (jchars, jobj) = match read_json(jchars) {
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
    Ok(jobj)
}

pub fn to_jarray(jtext:String) -> Result<JArray, Error> {
    let mut jchars = jtext.chars();
    let mut jarr: JArray = JArray::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'|'['=>{
                (jchars, jarr) = match get_arr(jchars) {
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
    Ok(jarr)
}


/*pub fn from_json(jtext:String) -> Result<HashMap<String, Val>, Error> {
    let mut jchars = jtext.chars();
    let mut hmap:HashMap<String, Val> = HashMap::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'=>{
                (jchars, hmap) = match read_json(jchars) {
                    Ok((jchars, hmap))=>(jchars, hmap),
                    Err(e)=>return Err(e),                    
                };
            },
            '['=>{
                (jchars, hmap) = match get_arr(jchars) {
                    Ok((jchars, hmap))=>(jchars, hmap),
                    Err(e)=>return Err(e),
                };
            }
            ' '|'\t'|'\r'|'\n'=>{},
            _=>{
                Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"", c)))?;
            }
        }
    }
    Ok(hmap)
}*/
