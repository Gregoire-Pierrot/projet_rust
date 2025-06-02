use crate::{Joueur, Pnj, Ennemie, Lieu, Quete, Consommable, Equipement, Attaque};

use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

use crate::structs::Ressource;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterFile {
    Joueur: Joueur,
    Pnj : Vec<Pnj>,
    Ennemie : Vec<Ennemie>,
    Lieu: Vec<Lieu>,
    Quete: Vec<Quete>,
    Consommable: Vec<Consommable>,
    Ressource: Vec<Ressource>,
    Equipement: Vec<Equipement>,
    Attaque: Vec<Attaque>
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
            Lieu: master_file.Lieu,
            Quete: master_file.Quete,
            Consommable: master_file.Consommable,
            Ressource: master_file.Ressource,
            Equipement: master_file.Equipement,
            Attaque: master_file.Attaque
        }
    }

    ////Lieu////

    pub fn prendre_lieu_id(&self, id: &str) -> Result<Lieu, String> {
        for lieu in self.Lieu.clone() {
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


    ////Consommable////

    pub fn prendre_consommable_id(&self, id: &str) -> Result<Consommable, String> {
        for consommable in self.Consommable.clone() {
            if consommable.get_id() == id {
                return Ok(consommable);
            }
        }
        return Err("Consommable introuvable".to_string());
    }

    ////Ressource////

    pub fn prendre_ressource_id(&self, id: &str) -> Result<Ressource, String> {
        for ressource in self.Ressource.clone() {
            if ressource.get_id() == id {
                return Ok(ressource);
            }
        }
        return Err("Ressource introuvable".to_string());
    }

    ////Equipement////

    pub fn prendre_equipement_id(&self, id: &str) -> Result<Equipement, String> {
        for equipement in self.Equipement.clone() {
            if equipement.get_id() == id {
                return Ok(equipement);
            }
        }
        return Err("Equipement introuvable".to_string());
    }

    ////Quete////

    pub fn prendre_quete_id(&self, id: &str) -> Result<Quete, String> {
        for quete in self.Quete.clone() {
            if quete.get_id() == id {
                return Ok(quete);
            }
        }
        return Err("Quête introuvable".to_string());
    }
}
