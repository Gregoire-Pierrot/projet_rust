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
    description: String,
    Connections: Vec<Connection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Connection { 
    destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Lieux {
    Lieux: Vec<Lieu>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MasterFile {
    Joueur: Vec<Joueur>,
    Lieu: Vec<Lieu>,
}

fn PrendreInfoJoueur() -> Result<Joueur, String>{
    let data = fs::read_to_string("masterFile.json").unwrap();
    println!("Contenu du fichier : {}", data);
    let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    for joueur in master_file.Joueur {
       return Ok(joueur);
    }
    return Err("erreur".to_string());
}


fn prendre_lieu_id(id: &str) -> Result<Lieu, String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    println!("data = {}", data);
    let lieux: Lieux = serde_json::from_str(&data).expect("Erreur de parsing");
    for lieu in lieux.Lieux {
        if lieu.id == id {
            return Ok(lieu);
        }
    }
    return Err("Lieu introuvable".to_string());
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