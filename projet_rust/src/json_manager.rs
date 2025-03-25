use crate::{Joueur, Pnj, Ennemie, Lieu};

use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterFile {
    Joueur: Joueur,
    Pnj : Vec<Pnj>,
    Ennemie : Vec<Ennemie>,
    Lieux: Vec<Lieu>,
}

impl MasterFile {
    ////MasterFile////
    pub fn new() -> Self {
        let data = fs::read_to_string("masterFile.json").unwrap();
        let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
        Self {
            Joueur: master_file.Joueur,
            Pnj: master_file.Pnj,
            Ennemie: master_file.Ennemie,
            Lieux: master_file.Lieux,
        }
    }

    ////Lieu////

    pub fn prendre_lieu_id(&self, id: &str) -> Result<Lieu, String> {
        for lieu in self.Lieux.clone() {
            if lieu.get_id() == id {
                return Ok(lieu);
            }
        }
        return Err("Lieu introuvable".to_string());
    }


    ////Joueur////

    pub fn get_joueur(&self) -> Joueur {
        self.Joueur.clone()
    }

    pub fn sauvegarder(&mut self, joueur: &Joueur) {
        self.Joueur = joueur.clone();
        let updated_data = serde_json::to_string_pretty(&self).unwrap();// Sauvegarder les données dans le fichier
        fs::write("masterFile.json", updated_data).unwrap();
    }


    ////PNJ////

    pub fn prendre_pnj_id(&self, id: &str) -> Result<Pnj, String> {
        for pnj in self.Pnj.clone() {
            if pnj.get_id() == id {
                return Ok(pnj);
            }
        }
        return Err("PNJ introuvable".to_string());
    }


    ////Ennemie////

    pub fn prendre_ennemie_id(&self, id: &str) -> Result<Ennemie, String> {
        for ennemie in self.Ennemie.clone() {
            if ennemie.get_id() == id {
                return Ok(ennemie);
            }
        }
        return Err("Ennemie introuvable".to_string());
    }
}


