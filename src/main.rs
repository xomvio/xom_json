use std::{collections::HashMap, fmt};


enum Readstat{
    None,
    Objbegin,
    Keybegin,
    Keyend,
    Middle,
    Valbegin,
    Valend,
    Objend
}

enum Val {
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
            Self::Strink(x)=>write!(f,"{}",x),
            Self::Array(x)=>{
                let mut res:String ="[ ".to_string();
                for i in x {
                    res+=i.to_string().as_str();
                    res.push(',');
                }
                res.pop();
                write!(f,"{} ]",res)
            }
            _=>{write!(f,"erorke")}
        }
    }
}

fn main(){
    let mut jtext = "
    {
         \"name\": \"yasir\",
         \"age\": 31
    }".chars();
    let mut jobject:HashMap<String, Val> = HashMap::new(); 


    let mut key = String::from("");
    let mut val = Val::Null;
    let mut stat = Readstat::None;


    while let Some(c) = jtext.next(){
        match stat {
            Readstat::None=>{
                match c {
                    '{'=>stat=Readstat::Objbegin,
                    ' '=>{}
                    _=>{}//error unexpected char
                }
            }
            Readstat::Objbegin=>{
                match c {
                    '"'=>stat=Readstat::Keybegin,
                    ' '=>{}
                    _=>{}//error unexpected char
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
                    ' '=>{}
                    _=>{}//error unexpected char
                }
            }
            Readstat::Middle=>{
                match c {
                    '"'=>{
                        stat=Readstat::Valbegin;
                        val=Val::Strink(String::from(""));
                    }
                    ' '=>{}
                    _=>{
                        if c.is_numeric() {
                            stat=Readstat::Valbegin;
                            val=Val::Number(c.to_string());
                        }
                        else{}//error unexpected char
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
                                let nextc = jtext.next().unwrap();
                                match nextc {
                                    '"'=>{
                                        s.push(nextc);
                                    }
                                    _=>{}
                                }
                            }
                            _=>{
                                s.push(c);
                            }
                        }
                    }
                    Val::Number(n)=>{
                        match c {
                            ','|'}'=>{
                                stat=Readstat::Valend;
                                jobject.insert(key.clone(), val_own(&val));
                                key = String::new();
                            }
                            ' '=>{},
                            '.'=>n.push(c),
                            _=>{
                                if c.is_numeric(){
                                    n.push(c);
                                }
                                else {}//error char is not a number
                            }
                        }                        
                    }
                    _=>{}
                }
            }
            Readstat::Valend=>{
                match c {
                    '}'=>{
                        stat=Readstat::Objend;
                        jobject.insert(key.clone(), val_own(&val));
                        key = String::new();
                    }
                    '"'=>{
                        stat=Readstat::Keybegin;
                        jobject.insert(key.clone(), val_own(&val));
                        key = String::new();
                    }
                    ' '=>{}
                    _=>{}//error unexpected char
                }
            }
            Readstat::Objend=>{
                println!("warning: json main object has ended but document continues.");
                break;
            }
        }
    }
    for (key, value) in &jobject {
        println!("{}: {}", key, value);
    }
}