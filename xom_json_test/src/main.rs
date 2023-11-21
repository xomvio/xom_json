use xom_json::Val;


#[derive(Debug)]
struct Player {
    id:u32,
    username:String,
    health:f32,
    weapons:Vec<Weapon>,
    selected_weapon: u8
}
#[derive(Debug)]
struct Weapon {
  name:String,
  bullet:u8,
  power:i8
}

fn main(){}

#[test]
fn tt() {
  let jtext:String = r#"
  {
    "id": 12468536,
    "username": "xomvio",
    "health": 100.0,
    "weapons":[
      { "name": "AK-47", "bullet": 14, "power": 35 },
      { "name": "Revolver", "bullet": 6, "power": 20 },
      { "name": "Knife", "bullet": 1, "power":3 }
    ],
    "selected_weapon": 2
  }
  "#.to_string();
  
  let json_obj = xom_json::json_parse(jtext.clone());

  let mut player:Player = Player { 
    id: json_obj["id"].to_string().parse::<u32>().unwrap(),
    username: json_obj["username"].to_string(),
    health: json_obj["health"].to_string().parse::<f32>().unwrap(),
    weapons:vec![],
    selected_weapon: json_obj["selected_weapon"].to_string().parse::<u8>().unwrap()
  };

  match &json_obj["weapons"] {
    Val::Array(varray)=>{
      for val in varray {
        match val {
          Val::Object(vobject)=>{
            player.weapons.push(Weapon { 
                name: vobject["name"].to_string(),
                bullet: vobject["bullet"].to_string().parse::<u8>().unwrap(), 
                power: vobject["power"].to_string().parse::<i8>().unwrap()
              })
          }
          _=>{}
        }
      }
    }
    _=>{}
  }

  println!("{:?}", player);
}