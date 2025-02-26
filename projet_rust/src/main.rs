use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize,Deserialize,Debug)]

struct Joueur {
    description:String,
    nom: String,
    pronom: String,
    niveau: String,
    position:String,
}

impl Joueur {
    pub fn new(description: String, nom: String, pronom: String, niveau: String, position: String) -> Joueur {
        Joueur {
            description,
            nom,
            pronom,
            niveau,
            position,
        }
    }
}

fn main(){
    let data = fs::read_to_string("masterFile.json").unwrap();
    let joueur: Joueur = serde_json::from_str(&data).unwrap();
    println!("{}",joueur.description);
}