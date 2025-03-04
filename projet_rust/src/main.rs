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
  
mod structs;

use structs::{Joueur, Lieu, Meteo};

fn main() {
    println!("Début du test");

    let joueur = Joueur::new(
        "Le joueur".to_string(),
        "C'est vous !".to_string(),
        "je suis un pronom".to_string(),
        1,
        Lieu::new(
            "Salle c137".to_string(),
            "c137".to_string(),
            Vec::new(),
            Meteo::Interieur,
        ),
    );

    println!("Joueur : {:#?}", joueur);

    println!("Fin du test");
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
struct MasterFile {
    Joueur: Joueur,
    Lieux: Vec<Lieu>,
}

fn get_joueur() -> Joueur {
    let data = fs::read_to_string("masterFile.json").unwrap();
    println!("Contenu du fichier : {}", data);
    let master_file : MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur
}


fn prendre_lieu_id(id: &str) -> Result<Lieu, String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    println!("data = {}", data);
    let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    for lieu in master_file.Lieux {
        if lieu.id == id {
            return Ok(lieu);
        }
    }
    return Err("Lieu introuvable".to_string());
}


fn ChangerPositionJoueur(position: &str) -> Result<(), String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let mut master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur.position = position.to_string();
    let updated_data = serde_json::to_string_pretty(&master_file).unwrap();// Sauvegarder les données dans le fichier
    fs::write("masterFile.json", updated_data).unwrap();
    Ok(())
}
}