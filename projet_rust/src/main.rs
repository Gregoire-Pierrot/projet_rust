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
    id: String,
    nom: String,
    desc: String,
    Connections: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Connection {
    destination: String,
}

fn PrendreInfoJoueur() -> Result<Joueur, String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();
    items.iter()
        .filter(|item| item.get("type").and_then(|t| t.as_str()) == Some("Joueur"))
        .filter_map(|item| serde_json::from_value(item.clone()).ok()) //converti chaque élément du Vec<Value> en un objet Joueur ???
        .next()
        .ok_or_else(|| "Aucun joueur".to_string())
}

fn PrendreLieuId(id: &str) -> Result<Lieu, String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();
    items.iter()
        .filter(|item| item.get("type").and_then(|t| t.as_str()) == Some("Lieu"))
        .filter_map(|item| serde_json::from_value(item.clone()).ok()) // Convertit chaque élément du Vec<Value> en un objet Lieu ?????
        .find(|lieu: &Lieu| lieu.id == id) 
        .ok_or_else(|| format!("Aucun lieu trouvé avec : {}", id))
}

fn PrendreToutesInfo() {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let items: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();
    for item in items {
        match item.get("type").and_then(|t| t.as_str()) {
            Some("Joueur") => {
                let joueur: Joueur = serde_json::from_value(item).unwrap();
                println!("Joueur: {}", joueur.description);
                println!("Joueur: {}", joueur.position);
            }
            Some("Lieu") => {
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

fn ChangerPositionJoueur(position: &str) -> Result<(), String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let mut items: Vec<serde_json::Value> = serde_json::from_str(&data).unwrap();

    let Some(joueur_json) = items.iter_mut().find(|item| item.get("type").and_then(|t| t.as_str()) == Some("Joueur"))// On cherche le joueur
        else {
            return Err("Aucun joueur trouvé".to_string());
        };
    
    if let Some(nouvelle_position) = joueur_json.get_mut("position") {
        *nouvelle_position = serde_json::Value::String(position.to_string());// Modifier la position du joueur
    }
        
    let updated_data = serde_json::to_string_pretty(&items).unwrap();// Sauvegarder les données dans le fichier
    fs::write("masterFile.json", updated_data).unwrap();
    Ok(())
}

fn main(){
    PrendreToutesInfo();

    let joueur = PrendreInfoJoueur();
    println!("Joueur trouvé: {:?}", joueur);

    let lieu = PrendreLieuId("pièce2");
    println!("Lieu trouvé: {:?}", lieu);

    match ChangerPositionJoueur("pièce1") {
        Ok(_) => println!("Position du joueur modifié"),
        Err(e) => println!("Erreur : {}", e),
    }
    
}