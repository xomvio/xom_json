use std::fmt;
use super::super::{Val, JObject, JArray};

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null=>write!(f,"null"),
            Self::Bool(x)=>write!(f,"{}",x),
            Self::Number(x)=>write!(f,"{}",x),
            Self::String(x)=>write!(f,"{}",x),
            Self::Array(x)=>{
                let mut res:String ="[ ".to_string();
                for item in x.iter() {
                    res += item.to_string().as_str();
                    res.push(',');
                }
                res.pop();
                write!(f,"{} ]",res)
            }
            Self::Object(x)=>{
                let mut res:String = "{ ".to_string();
                for (key, val) in x.iter() {
                    res+= format!("\"{key}\": {val},").as_str();
                }
                res.pop();
                write!(f,"{} }}",res)
            }
        }
    }
}

impl fmt::Display for JObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res:String = "{ ".to_string();
        for (key, val) in self.iter() {
            res+= format!("\"{key}\": {val},").as_str();
        }
        res.pop();
        write!(f,"{} }}",res)
    }
}

impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res:String ="[ ".to_string();
        for item in self.iter() {
            res += item.to_string().as_str();
            res.push(',');
        }
        res.pop();
        write!(f,"{} ]",res)
    }    
}