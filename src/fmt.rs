use std::fmt;
use super::*;

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null=>write!(f,"null"),
            Self::Bool(x)=>write!(f,"{}",x),
            Self::Number(x)=>write!(f,"{}",x),
            Self::String(x)=>write!(f,"{}",x),
            Self::Array(x)=>{
                let mut res:String ="[ ".to_string();
                for item in x {
                    res += item.to_string().as_str();
                    res.push(',');
                }
                res.pop();
                write!(f,"{} ]",res)
            }
            Self::Object(x)=>{
                let mut res:String = "{ ".to_string();
                for (key, val) in x{
                    res+= format!("\"{key}\": {val},").as_str();
                    /*res+=": ";
                    res+= val.to_string().as_str();
                    res.push(',');*/
                }
                res.pop();
                write!(f,"{} }}",res)
            }
        }
    }
}
