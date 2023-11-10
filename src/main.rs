mod xom_json;
fn main(){
    let jtext:String = "
    {
      \"id\": \"0001\",
      \"type\": \"donut\",
      \"name\": \"Cake\",
      \"ppu\": 0.55,
      \"batters\":
        {
          \"batter\":
            [
              { \"id\": \"1001\", \"type\": \"Regular\" },
              { \"id\": \"1002\", \"type\": \"Chocolate\" },
              { \"id\": \"1003\", \"type\": \"Blueberry\" },
              { \"id\": \"1004\", \"type\": \"Devil's Food\" }
            ]
        },
      \"topping\":
        [
          { \"id\": \"5001\", \"type\": \"None\" },
          { \"id\": \"5002\", \"type\": \"Glazed\" },
          { \"id\": \"5005\", \"type\": \"Sugar\" },
          { \"id\": \"5007\", \"type\": \"Powdered Sugar\" },
          { \"id\": \"5006\", \"type\": \"Chocolate with Sprinkles\" },
          { \"id\": \"5003\", \"type\": \"Chocolate\" },
          { \"id\": \"5004\", \"type\": \"Maple\" }
        ]
    }
    ".to_string();
    let json_obj = xom_json::json_parse(jtext.clone());
    let mut i = 0;
    for (key, value) in &json_obj {
        print!("{}-> ",i);
        i+=1;
        println!("{}: {}", key, value);
    }

    let json_obj2 = xom_json::Val::Object(xom_json::json_parse(jtext));
    println!("{}",json_obj2);
}