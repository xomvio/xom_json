use std::{collections::HashMap, fmt, str::Chars};

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
            Self::Strink(x)=>write!(f,"{}",x),
            Self::Array(x)=>{
                let mut res:String ="[ ".to_string();
                for i in 0..x.len() {
                    res += x[i].to_string().as_str();
                    res.push(',');
                }
                res.pop();
                write!(f,"{} ]",res)
            }
            _=>{write!(f,"erorke")}
        }
    }
}

fn get_arr(mut jchars:Chars) -> (Chars, Vec<Val>){

    let mut cevval:Vec<Val>=vec![];

    while let Some(c) = jchars.next(){
        //let mut val = Val::Null;
        match c {
            '"'=>{
                let mut str:String = String::from("");
                while let Some(c2) = jchars.next(){
                    match c2 {
                        '"'=>{
                            break;
                        }
                        _=>{
                            str.push(c2);
                        }
                    }
                }
                //val = Val::Strink(str);
                cevval.push(Val::Strink(str));
            }

            '['=>{
                let v:Vec<Val>;
                (jchars,v) = get_arr(jchars);
                //val = Val::Array(v);
                cevval.push(Val::Array(v));
            }
            ','=>{
                //cevval.push(val);
            }
            ']'=>{
                //cevval.push(val);
                break;
            }
            _=>{}//sayi falan
        }
    }
    (jchars, cevval)
}


pub fn read_json(jtext:String) -> HashMap<String, Val>  {

    let mut jchars=jtext.chars();


    let mut jobject:HashMap<String, Val> = HashMap::new(); 

    let mut key = String::from("");
    let mut val = Val::Null;
    let mut stat = Readstat::None;

    while let Some(c) = jchars.next(){
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
                    'n'=>{
                        let mut n:String = String::from("");
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        n.push(jchars.next().unwrap());
                        if n=="ull"{
                            stat=Readstat::Valend;
                            val=Val::Null;
                        }
                        else{}//error unexpected char
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
                        else{}//error unexpected char
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
                        else{}//error unexpected char
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
                                //jobject.insert(key.clone(), val_own(&val));
                                //key = String::new();
                            }                            
                            _=>{}//imkansiz error
                        }

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
                            /*'\\'=>{
                                let nextc = jchars.next().unwrap();
                                match nextc {
                                    '"'=>{
                                        s.push(nextc);
                                    }
                                    _=>{}
                                }
                            }*/
                            _=>{
                                s.push(c);
                            }
                        }
                    }
                    Val::Number(n)=>{
                        match c {
                            '}'=>{
                                stat=Readstat::Objend;
                                jobject.insert(key.clone(), val_own(&val));
                                key = String::new();                                
                            }
                            ','=>{
                                stat=Readstat::Objbegin;
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

                    //buraya object gelecek
                    _=>{}
                }
            }
            Readstat::Valend=>{
                match c {
                    '}'=>{
                        stat=Readstat::Objend;
                        if key!=""{
                            jobject.insert(key.clone(), val_own(&val));
                        }
                        key = String::new();
                    }
                    ','=>{
                        stat=Readstat::Objbegin;
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
    jobject
}