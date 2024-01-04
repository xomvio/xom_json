use std::{collections::HashMap, fmt, str::Chars};

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
    Strink(String),
    Array(Vec<Val>),
    Object(HashMap<String, Val>),
}

fn val_own(v:&Val) -> Val {
    match &v {
        Val::Null=>return Val::Null,
        Val::Bool(a)=>return Val::Bool(a.to_owned()),
        Val::Number(a)=>return Val::Number(a.to_owned()),
        Val::Strink(a)=>return Val::Strink(a.to_owned()),
        Val::Array(a)=>{
            let mut ownedvec:Vec<Val>=vec![];
            for i in a {
                ownedvec.push(val_own(i));
            }
            Val::Array(ownedvec)
        },
        Val::Object(a)=>{
            let mut ownedmap: HashMap<String,Val> = HashMap::new();
            for i in a {
                ownedmap.insert(i.0.to_owned(), val_own(i.1));
            }
            Val::Object(ownedmap)
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null=>write!(f,"null"),
            Self::Bool(x)=>write!(f,"{}",x),
            Self::Number(x)=>write!(f,"{}",x),
            Self::Strink(x)=>write!(f,"\"{}\"",x),
            Self::Array(x)=>{
                let mut res:String ="[ ".to_string();
                for i in 0..x.len() {
                    res += x[i].to_string().as_str();
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

fn get_arr(mut jchars:Chars) -> (Chars, Vec<Val>){

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
                cevval.push(Val::Strink(str));
            }
            '{'=>{
                let hmap:HashMap<String, Val>;
                (jchars, hmap) = read_json(jchars);
                cevval.push(Val::Object(hmap));
            }
            '['=>{
                let v:Vec<Val>;
                (jchars,v) = get_arr(jchars);
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
                    panic!("json validation failed. unexpected characters \"n{}\"", n);
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
                    panic!("json validation failed. unexpected characters \"t{}\"", n);
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
                    panic!("json validation failed. unexpected characters \"f{}\"", n);
                }
            }
            _=>{
                if c.is_numeric(){
                    let mut num:String = String::from(c);
                    while let Some(c2) = jchars.next(){
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
                    panic!("json validation failed. unexpected character \'{}\'", c);
                }
            }
        }
    }
    (jchars, cevval)
}

pub fn json_parse(jtext:String) -> HashMap<String, Val> {
    let mut jchars = jtext.chars();
    let mut hmap:HashMap<String, Val> = HashMap::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'=>{
                (jchars, hmap) = read_json(jchars);
            }
            ' '|'\t'|'\r'|'\n'=>{}
            _=>{
                panic!("json validation failed. unexpected character \'{}\'", c);
            }
        }
    }
    hmap
}

pub fn parse_as_jobject(jtext:String) -> Val {
    let mut jchars = jtext.chars();
    let mut hmap:HashMap<String, Val> = HashMap::new();
    while let Some(c) = jchars.next(){
        match c {
            '{'=>{
                (jchars, hmap) = read_json(jchars);
            }
            ' '|'\t'|'\r'|'\n'=>{}
            _=>{
                panic!("json validation failed. unexpected character \'{}\'", c);
            }
        }
    }
    Val::Object(hmap)
}

fn read_json(mut jchars:Chars) -> (Chars, HashMap<String, Val>) {

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
                        panic!("json validation failed. unexpected character \'{}\'", c);
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
                        panic!("error: unexpected character \'{}\'", c);
                    }
                }
            }
            Readstat::Middle=>{
                match c {
                    '"'=>{
                        stat=Readstat::Valbegin;
                        val=Val::Strink(String::from(""));
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
                            panic!("json validation failed. unexpected characters \"n{}\"", n);
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
                            panic!("json validation failed. unexpected characters \"t{}\"", n);
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
                            panic!("json validation failed. unexpected characters \"f{}\"", n);
                        }
                    }
                    '['=>{
                        val = Val::Array(vec![]);
                        match &mut val {
                            Val::Array(a)=>{                        
                                let xa: Vec<Val>;
                                (jchars, xa)=get_arr(jchars);
                                for x in xa {
                                    a.push(x);
                                }
        
                                stat=Readstat::Valend;
                            }                            
                            _=>{}//impossible
                        }

                    }
                    '{'=>{
                        let tmap:HashMap<String,Val>;
                        (jchars,tmap) = read_json(jchars);
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
                            panic!("json validation failed. unexpected character \'{}\'", c);
                        }
                    }
                    
                }
            }
            Readstat::Valbegin=>{
                match &mut val {
                    Val::Strink(s)=>{
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
                                jobject.insert(key.clone(), val_own(&val));
                                break;
                            }
                            ','=>{
                                jobject.insert(key.clone(), val_own(&val));
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
                                    panic!("json validation failed. unexpected character \'{}\'. expected numeric.", c);
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
                        if key!=""{
                            jobject.insert(key.clone(), val_own(&val));
                        }
                        break;
                    }
                    ','=>{
                        stat=Readstat::Objbegin;
                        jobject.insert(key.clone(), val_own(&val));
                        key = String::new();
                    }
                    ' '|'\t'|'\r'|'\n'=>{}
                    _=>{
                        panic!("json validation failed. unexpected character \'{}\'", c);
                    }
                }
            }
        }
    }
    (jchars,jobject)
}