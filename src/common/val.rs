use std::io::Error;

use super::{JObject,JArray};

pub enum Val {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(JArray),
    Object(JObject),
}

impl Val {
    pub fn to_owned(&self) -> Val {
        match &self {
            Val::Null=> Val::Null,
            Val::Bool(a)=> Val::Bool(*a),
            Val::Number(a)=> Val::Number(a.clone()),
            Val::String(a)=> Val::String(a.clone()),
            Val::Array(a)=>{
                let mut ownedvec:JArray=JArray::new();
                for val in a.iter() {
                    ownedvec.push(val.to_owned());
                }
                Val::Array(ownedvec)
            },
            Val::Object(a)=>{
                let mut ownedmap: JObject = JObject::new();
                for (k, v) in a.iter() {
                    ownedmap.push((k.clone(), v.to_owned()));
                    //ownedmap.insert(k.clone(), v.to_owned());
                }
                Val::Object(ownedmap)
            }
        }
    }

    pub fn is_null(&self) -> bool {
        match &self {
            Val::Null=> true,
            _=>false
        }
    }

    pub fn as_bool(&self) -> Result<bool, Error> {        
        match &self {
            Val::Bool(b)=> Ok(*b),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not a bool")),
        }
    }

    pub fn as_string(&self) -> Result<String, Error> {
        Ok(self.to_string().trim_matches('\"').to_string())
    }

    pub fn as_i8(&self) -> Result<i8, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<i8>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<i8>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an i8")),
        }
    }

    pub fn as_i16(&self) -> Result<i16, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<i16>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<i16>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an i16")),
        }
    }

    pub fn as_i32(&self) -> Result<i32, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<i32>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<i32>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an i32")),
        }
    }

    pub fn as_i64(&self) -> Result<i64, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<i64>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<i64>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an i64")),
        }
    }

    pub fn as_u8(&self) -> Result<u8, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<u8>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<u8>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an u8")),
        }
    }

    pub fn as_u16(&self) -> Result<u16, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<u16>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<u16>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an u16")),
        }
    }

    pub fn as_u32(&self) -> Result<u32, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<u32>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<u32>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an u32")),
        }
    }

    pub fn as_u64(&self) -> Result<u64, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<u64>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<u64>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not an u64")),
        }
    }

    pub fn as_f32(&self) -> Result<f32, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<f32>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<f32>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not a float")),
        }
    }

    pub fn as_f64(&self) -> Result<f64, Error> {
        match &self {
            Val::String(s)=> Ok(s.trim_matches('\"').parse::<f64>().unwrap()),
            Val::Number(s)=> Ok(s.parse::<f64>().unwrap()),
            _=>Err(Error::new(std::io::ErrorKind::Other, "not a float")),
        }
    }
}