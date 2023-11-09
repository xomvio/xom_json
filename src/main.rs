mod xom_json;

fn main(){
    let jtext:String = "
    {
        \"isok\": null,
        \"isturu\": true,
        \"isfals\": false,
        \"name\": \"yasir\",
        \"arr\": [\"Cyberpunk\",\"2077\"]
    }".to_string();
    let json_obj = xom_json::read_json(jtext);

    let mut i = 0;
    for (key, value) in &json_obj {
        print!("{}",i);
        i+=1;
        println!("{}: {}", key, value);
    }

    /*match &json_obj["arr"] {
        xom_json::Val::Array(x)=>{
            for a in x {
                println!("Array verisi: {}",a);
            }
        }
        _=>{}
    }*/
}