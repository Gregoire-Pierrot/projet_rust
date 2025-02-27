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

#[derive(Serialize, Deserialize, Debug)]
struct Lieu {
    ID: String,
    nom: String,
    Desc: String,
    Connections: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Connection {
    destination: String,
}


fn main(){
    let data = fs::read_to_string("masterFile.json").unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();
    for item in items {
        match item.get("type").and_then(|t| t.as_str()) {
            Some("Joueur") => {
                let joueur: Joueur = serde_json::from_value(item).unwrap();
                println!("Joueur: {}", joueur.description);
                println!("Joueur: {}", joueur.position);
            }
            Some("lieu") => {
                let lieu: Lieu = serde_json::from_value(item).unwrap();
                println!("Lieu: {}", lieu.nom);
                let connections: Vec<Connection> = lieu.Connections;
                for connection in connections{
                    println!("Lieu possibles: {}", connection.destination);
                }
            }
            _ => println!("Type inconnu trouvé dans les données."),
        }
        println!("");
    }
}