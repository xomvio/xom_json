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
    "selected_weapon": 1
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

  assert_eq!(12468536u32,player.id);
  assert_eq!("xomvio", player.username);
  assert_eq!(100.0f32, player.health);
  assert_eq!(1u8, player.selected_weapon);
  assert_eq!("AK-47", player.weapons[0].name);
  assert_eq!(14, player.weapons[0].bullet);
  assert_eq!(35, player.weapons[0].power);
  assert_eq!("Revolver", player.weapons[1].name);
  assert_eq!(6, player.weapons[1].bullet);
  assert_eq!(20, player.weapons[1].power);
  assert_eq!("Knife", player.weapons[2].name);
  assert_eq!(1, player.weapons[2].bullet);
  assert_eq!(3, player.weapons[2].power);

  let jtext:String = r#"
  {
    "id": 12468536,
    "username": "xomvio",
    "health": 100.0
  }
  "#.to_string();

  let hmap = xom_json::json_parse(jtext);
  
  let players_username = hmap["username"].to_string();
  let players_health = hmap["health"].to_string().parse::<f32>().unwrap();

  assert_eq!(players_username,"xomvio");
  assert_eq!(players_health,100.0);
}