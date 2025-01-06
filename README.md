Just use serde man...

# xom_json
It's a JSON parser which returns JObject or JArray.

### Serialization
JObject or JArray .to_string() also means serialization.

### Deserialization
unfortunately deserialization must be done manually.




## Example:
More detailed usages shown at src/tests.rs <br>
```Rust
let jtext = r#"
{
    "id": 12468536,
    "username": "xomvio",
    "health": 100.0
}
"#.to_string();

let jobj = to_jobject(jtext).unwrap();

let players_id = jobj.get("id").unwrap().as_i32().unwrap();
let players_username = jobj.get("username").unwrap().as_string().unwrap();
let players_health = jobj.get("health").unwrap().as_f64().unwrap();

assert_eq!(players_id, 12468536);
assert_eq!(players_username, "xomvio");
assert_eq!(players_health, 100.0);
```

<b>Note:</b> when getting a string, do not use get("key").unwrap().to_string() because it gives value with "quotes". Use as_string() instead.
