use crate::{Joueur, Pnj, Ennemie, Lieu, Quete, Consommable, Equipement, Attaque};

use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::collections::HashMap;

use crate::structs::Ressource;

use std::sync::{OnceLock, Mutex};

pub enum Item {
    Ressource(Ressource),
    Consommable(Consommable),
    Equipement(Equipement),
}

impl Item {

    pub fn get_ressources(&self) -> HashMap<String, u32> {
        match self {
            Item::Ressource(r) => r.get_ressource(),
            Item::Consommable(c) => c.get_ressource().get_ressource(),
            Item::Equipement(e) => e.get_ressource().get_ressource(),
        }
    }
}


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

static INSTANCE: OnceLock<Mutex<MasterFile>> = OnceLock::new();

impl MasterFile {
    ////MasterFile////
    pub fn get_instance() -> &'static Mutex<MasterFile> {
        INSTANCE.get_or_init(|| {
            let data = fs::read_to_string("masterFile.json").unwrap();
            let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
            Mutex::new(master_file)
        })
    }

    pub fn sauvegarder(&mut self) {
        let updated_data = serde_json::to_string_pretty(&self).unwrap(); // Serialiser les données
        fs::write("masterFile.json", updated_data).unwrap(); // Sauvegarder les données dans le fichier
    }

    pub fn recharger(&mut self) {
        let data = fs::read_to_string("masterFile.json").unwrap();
        let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
        self.Joueur = master_file.Joueur;
        self.Pnj = master_file.Pnj;
        self.Ennemie = master_file.Ennemie;
        self.Lieu = master_file.Lieu;
        self.Quete = master_file.Quete;
        self.Consommable = master_file.Consommable;
        self.Ressource = master_file.Ressource;
        self.Equipement = master_file.Equipement;
        self.Attaque = master_file.Attaque;
    }

    //pub fn new() -> Self {
    //    let data = fs::read_to_string("masterFile.json").unwrap();
    //    let master_file: MasterFile = serde_json::from_str(&data).expect("Erreur de parsing");
    //    Self {
    //        Joueur: master_file.Joueur,
    //        Pnj: master_file.Pnj,
    //        Ennemie: master_file.Ennemie,
    //        Lieu: master_file.Lieu,
    //        Quete: master_file.Quete,
    //        Consommable: master_file.Consommable,
    //        Ressource: master_file.Ressource,
    //        Equipement: master_file.Equipement,
    //        Attaque: master_file.Attaque
    //    }
    //}

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

    pub fn get_joueur_mut(&mut self) -> &mut Joueur {
        &mut self.Joueur
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

    pub fn prendre_pnj_id_string(&self, id: String) -> Pnj {
        for pnj in self.Pnj.clone() {
            if pnj.get_id() == id {
                return pnj;
            }
        }
        panic!("PNJ introuvable {}",id);
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

    pub fn prendre_quete_mut(&mut self, id: &str) -> Result<&mut Quete, String> {
        for quete in self.Quete.iter_mut() {
            if quete.get_id() == id {
                return Ok(quete);
            }
        }
        Err("Quête introuvable".to_string())
    }

    ////Attaque////

    pub fn prendre_attaque_id(&self, id: &str) -> Result<Attaque, String> {
        for attaque in self.Attaque.clone() {
            if attaque.get_id() == id {
                return Ok(attaque);
            }
        }
        return Err("Attaque introuvable".to_string());
    }


    ////item////
    pub fn prendre_item_id(&self, id: &str) -> Result<Item, String> {
        if let Ok(consommable) = self.prendre_consommable_id(id) {
            Ok(Item::Consommable(consommable))
        } else if let Ok(equipement) = self.prendre_equipement_id(id) {
            Ok(Item::Equipement(equipement))
        } else if let Ok(ressource) = self.prendre_ressource_id(id) {
            Ok(Item::Ressource(ressource))
        } else {
            Err(format!("Item avec id '{}' introuvable", id))
        }
    }
}
