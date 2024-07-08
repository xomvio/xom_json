mod fmt;
use std::collections::HashMap;
use std::io::Error;
use std::str::Chars;


enum Readstat{
    Objbegin,
    Keybegin,
    Keyend,
    Middle,
    Valbegin,
    Valend
}

pub enum Val {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<Val>),
    Object(HashMap<String, Val>),
}

impl Val {
    pub fn to_owned(&self) -> Val {
        match &self {
            Val::Null=> Val::Null,
            Val::Bool(a)=> Val::Bool(*a),
            Val::Number(a)=> Val::Number(a.clone()),
            Val::String(a)=> Val::String(a.clone()),
            Val::Array(a)=>{
                let mut ownedvec:Vec<Val>=vec![];
                for val in a {
                    ownedvec.push(val.to_owned());
                }
                Val::Array(ownedvec)
            },
            Val::Object(a)=>{
                let mut ownedmap: HashMap<String,Val> = HashMap::new();
                for (k, v) in a {
                    ownedmap.insert(k.clone(), v.to_owned());
                }
                Val::Object(ownedmap)
            }
        }
    }    
}


fn get_arr(mut jchars:Chars) -> Result<(Chars, Vec<Val>), Error>{

    let mut cevval:Vec<Val>=vec![];

    while let Some(c) = jchars.next(){
        match c {
            '"'=>{
                let mut str:String = String::from("");
                while let Some(c2) = jchars.next(){
                    match c2 {
                        '"'=>{
                            break;
                        }
                        '\\'=>{
                            jchars.next();
                            str.push(c2);
                        }
                        _=>{
                            str.push(c2);
                        }
                    }
                }
                cevval.push(Val::String(str));
            }
            '{'=>{
                let hmap:HashMap<String, Val>;
                (jchars, hmap) = match read_json(jchars) {
                    Ok((jchars, hmap)) => (jchars, hmap),
                    Err(e) => return Err(e),                    
                };
                cevval.push(Val::Object(hmap));
            }
            '['=>{
                let v:Vec<Val>;
                (jchars,v) = match get_arr(jchars) {
                    Ok((jchars, v)) => (jchars, v),
                    Err(e) => return Err(e),                    
                };
                cevval.push(Val::Array(v));
            }
            ','|' '|'\t'|'\r'|'\n'=>{}
            ']'=>{
                break;
            }
            'n'=>{
                let mut n:String = String::from("");
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                if n=="ull"{
                    cevval.push(Val::Null);
                }
                else {
                    Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected characters \"n{}\"",n)))?;
                }
            }
            't'=>{
                let mut n:String = String::from("");
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                if n=="rue"{
                    cevval.push(Val::Bool(true));
                }
                else {
                    Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected characters \"n{}\"",n)))?;
                }
            }
            'f'=>{
                let mut n:String = String::from("");
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                n.push(jchars.next().unwrap());
                if n=="alse"{
                    cevval.push(Val::Bool(false));
                }
                else {
                    Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected characters \"f{}\"",n)))?;
                }
            }
            _=>{
                if c.is_numeric(){
                    let mut num:String = String::from(c);
                    for c2 in jchars.by_ref(){
                        match c2 {
                            ','=>{
                                break;
                            }
                            ' '|'\t'|'\r'|'\n'=>{}
                            _=>{
                                num.push(c2);
                            }
                        }
                    }
                    cevval.push(Val::Number(num));
                }
                else {
                    Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"",c)))?;
                }
            }
        }
    }
    Ok((jchars, cevval))
}



pub fn read_json(mut jchars:Chars) -> Result<(Chars, HashMap<String, Val>), Error> {

    let mut jobject:HashMap<String, Val> = HashMap::new(); 

    let mut key = String::from("");
    let mut val = Val::Null;
    let mut stat = Readstat::Objbegin;

    while let Some(c) = jchars.next(){
        match stat {
            Readstat::Objbegin=>{
                match c {
                    '"'=>stat=Readstat::Keybegin,
                    ' '|'\t'|'\r'|'\n'=>{}
                    _=>{
                        Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"",c)))?;
                    }
                }
            }
            Readstat::Keybegin=>{
                match c {
                    '"'=>stat=Readstat::Keyend,
                    _=>key.push(c)
                }
            }
            Readstat::Keyend=>{
                match c {
                    ':'=>stat=Readstat::Middle,
                    ' '|'\t'|'\r'|'\n'=>{}
                    _=>{
                        Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"",c)))?;
                    }
                }
            }
            Readstat::Middle=>{
                match c {
                    '"'=>{
                        stat=Readstat::Valbegin;
                        val=Val::String(String::from(""));
                    }
                    'n'=>{
                        let mut n:String = String::from("");
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        if n=="ull"{
                            stat=Readstat::Valend;
                            val=Val::Null;
                        }
                        else{
                            Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected characters \"n{}\"",n)))?;
                        }
                    }
                    't'=>{
                        let mut n:String = String::from("");
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        if n=="rue"{
                            stat=Readstat::Valend;
                            val=Val::Bool(true);
                        }
                        else{
                            Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected characters \"t{}\"",n)))?;
                        }
                    }
                    'f'=>{
                        let mut n:String = String::from("");
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        if n=="alse"{
                            stat=Readstat::Valend;
                            val=Val::Bool(false);
                        }
                        else{
                            Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected characters \"f{}\"",n)))?;
                        }
                    }
                    '['=>{
                        val = Val::Array(vec![]);
                        if let Val::Array(a) = &mut val {
                            let xa: Vec<Val>;
                            (jchars, xa)= match get_arr(jchars) {
                                Ok((jchars, xa)) => (jchars, xa),
                                Err(e) => return Err(e),
                            };
                            for x in xa {
                                a.push(x);
                            }

                            stat=Readstat::Valend;
                        }
                    }
                    '{'=>{
                        let tmap:HashMap<String,Val>;                        
                        (jchars,tmap) = match read_json(jchars) {
                            Ok((jchars, tmap)) => (jchars, tmap),
                            Err(e) => return Err(e),
                        };
                        val = Val::Object(tmap);
                        stat=Readstat::Valend;
                    }
                    ' '|'\t'|'\r'|'\n'=>{}
                    _=>{
                        if c.is_numeric() {
                            stat=Readstat::Valbegin;
                            val=Val::Number(c.to_string());
                        }
                        else{
                            Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"",c)))?;
                        }
                    }
                    
                }
            }
            Readstat::Valbegin=>{
                match &mut val {
                    Val::String(s)=>{
                        match c {
                            '"'=>{
                                stat=Readstat::Valend;
                            }
                            '\\'=>{
                                jchars.next();
                                s.push(c);
                            }
                            _=>{
                                s.push(c);
                            }
                        }
                    }
                    Val::Number(n)=>{
                        match c {
                            '}'=>{
                                jobject.insert(key.clone(), val.to_owned());
                                break;
                            }
                            ','=>{
                                jobject.insert(key.clone(), val.to_owned());
                                stat=Readstat::Objbegin;
                                key = String::new();
                            }
                            ' '|'\t'|'\r'|'\n'=>{}
                            '.'=>n.push(c),
                            _=>{
                                if c.is_numeric(){
                                    n.push(c);
                                }
                                else {
                                    Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\". expected numeric",c)))?;
                                }
                            }
                        }                        
                    }
                    _=>{}//impossible error
                }
            }
            Readstat::Valend=>{
                match c {
                    '}'=>{
                        if !key.is_empty(){
                            jobject.insert(key.clone(), val.to_owned());
                        }
                        break;
                    }
                    ','=>{
                        stat=Readstat::Objbegin;
                        jobject.insert(key.clone(), val.to_owned());
                        key = String::new();
                    }
                    ' '|'\t'|'\r'|'\n'=>{}
                    _=>{
                        Err(Error::new(std::io::ErrorKind::Other, format!("json validation failed. unexpected character \"{}\"",c)))?;
                    }
                }
            }
        }
    }
    Ok((jchars,jobject))
}