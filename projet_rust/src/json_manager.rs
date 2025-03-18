use crate::structs;

use structs::{Joueur, Lieu};

use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct MasterFile {
    Joueur: Joueur,
    Lieux: Vec<Lieu>,
}

////Joueur////

pub fn get_joueur() -> Joueur {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let master_file : MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur
}

pub fn changer_nom_joueur(nom: &str) -> Result<(), String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let mut master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur.set_nom(nom.to_string());
    let updated_data = serde_json::to_string_pretty(&master_file).unwrap();// Sauvegarder les données dans le fichier
    fs::write("masterFile.json", updated_data).unwrap();
    Ok(())
}

pub fn changer_pronom_joueur(pronom: &str) -> Result<(), String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let mut master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur.set_pronom(pronom.to_string());
    let updated_data = serde_json::to_string_pretty(&master_file).unwrap();// Sauvegarder les données dans le fichier
    fs::write("masterFile.json", updated_data).unwrap();
    Ok(())
}

pub fn changer_niveau_joueur(niveau: &u8) -> Result<(), String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let mut master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur.add_niveau(*niveau);
    let updated_data = serde_json::to_string_pretty(&master_file).unwrap();// Sauvegarder les données dans le fichier
    fs::write("masterFile.json", updated_data).unwrap();
    Ok(())
}

pub fn changer_position_joueur(position: &str) -> Result<(), String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let mut master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    master_file.Joueur.set_position(position.to_string());
    let updated_data = serde_json::to_string_pretty(&master_file).unwrap();// Sauvegarder les données dans le fichier
    fs::write("masterFile.json", updated_data).unwrap();
    Ok(())
}

////Lieu////

pub fn prendre_lieu_id(id: &str) -> Result<Lieu, String> {
    let data = fs::read_to_string("masterFile.json").unwrap();
    let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    for lieu in master_file.Lieux {
        if lieu.get_id() == id {
            return Ok(lieu);
        }
    }
    return Err("Lieu introuvable".to_string());
}
