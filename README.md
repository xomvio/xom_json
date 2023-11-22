# xom_json
It's a json parser written in Rust. There is no deserializing and serializing, yet. Because macro derives are confused me and I had to study for my midterm exams. Deserializing ahead!

More detailed usages shown at xom_json_test

Example:
```Rust
  let jtext = r#"
  {
    "id": 12468536,
    "username": "xomvio",
    "health": 100.0,
  }
  "#.to_string();

  let hmap = xom_json::json_parse(jtext);
  
  let players_username = hmap["username"].to_string();
  let players_health = hmap["health"].to_string().parse::<f32>().unwrap();

  assert_eq!(players_username, "xomvio");
  assert_eq!(players_health, 100.0);
```
