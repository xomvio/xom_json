mod xom_json;

fn main(){
    let jtext:String = "
    {\"name\":\"John\",\"age\":30,\"cars\":[[ \"a\", \"b\", \"c\" ],[ \"m\", \"n\", \"o\" ],[ \"x\", \"y\", \"z\" ]]}".to_string();
    let json_obj = xom_json::read_json(jtext);

    let mut i = 0;
    for (key, value) in &json_obj {
        print!("{}",i);
        i+=1;
        println!("{}: {}", key, value);
    }

    match &json_obj["cars"] {
        xom_json::Val::Array(x)=>{
            for a in x {
                println!("Array verisi: {}",a);
                match a {
                    xom_json::Val::Array(y)=>{
                        for r in y {
                            println!("Array verisi: {}",r);
                        }
                    }
                    _=>{}
                }
                
            }
        }
        _=>{}
    }
}