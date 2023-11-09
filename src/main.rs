mod xom_json;

fn main(){
    let jtext:String = "
    {
        \"isok\": null,
        \"isturu\": true,
        \"isfals\": false,
        \"name\": \"yasir\",
        \"age\": 31
    }".to_string();
    let json_obj = xom_json::read_json(jtext);

    for (key, value) in &json_obj {
        println!("{}: {}", key, value);
    }
}