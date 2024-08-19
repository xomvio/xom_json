use crate::common::JArray;

//add tests
use super::{Val,to_jobject,to_jarray};

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
  power:i8,
}

#[test]
fn test() {
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
    
    let json_obj = to_jobject(jtext.clone()).unwrap();
  
    let mut player:Player = Player {
      id: json_obj.get("id").unwrap().as_u32().unwrap(),
      username: json_obj.get("username").unwrap().as_string().unwrap(),
      health: json_obj.get("health").unwrap().as_f32().unwrap(),
      weapons: vec![],
      selected_weapon: json_obj.get("selected_weapon").unwrap().as_u8().unwrap()
    };

    if let Val::Array(weapon_array) = &json_obj.get("weapons").unwrap() {
      for val in weapon_array.iter() {
        if let Val::Object(weapon) = val {
          player.weapons.push( Weapon {
            name: weapon.get("name").unwrap().as_string().unwrap().to_string(),
            bullet: weapon.get("bullet").unwrap().as_u8().unwrap(),
            power: weapon.get("power").unwrap().as_i8().unwrap()
          })
        }
      }
    }
  
    // GIVES THE SAME RESULT WITH THAT`IF LET`
    /*match &json_obj.get("weapons").unwrap() {
      Val::Array(varray)=>{
        for val in varray.iter() {
          match val {
            Val::Object(vobject)=>{
              player.weapons.push(Weapon { 
                  name: vobject.get("name").unwrap().to_string(),
                  bullet: vobject.get("bullet").unwrap().as_u8().unwrap(),
                  power: vobject.get("power").unwrap().as_i8().unwrap()
                })
            }
            _=>{}
          }
        }
      }
      _=>{}
    }*/
  
    println!("{}",player.username);
  
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
  
}