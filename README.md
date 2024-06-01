# xom_json
It's a json parser written in Rust. Sorry for terrible deserializing ^^

More detailed usages shown at xom_json_test

Example:
```Rust
  let jtext = r#"
  {
    "id": 12468536,
    "username": "xomvio",
    "health": 100.0
  }
  "#.to_string();

  let hmap = xom_json::json_parse(jtext);
  
  let players_username = hmap["username"].to_string();
  let players_health = hmap["health"].to_string().parse::<f32>().unwrap();

  assert_eq!(players_username, "xomvio");
  assert_eq!(players_health, 100.0);
```
