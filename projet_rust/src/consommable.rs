use serde::{Serialize, Deserialize};

use crate::structs::{Ressource, Rarete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Consommable {
    ressource: Ressource,
    effets: Vec<u16>,
}

impl Consommable {
    pub fn new(ressource: Ressource, effets: Vec<u16>) -> Self {
        if effets.len() != 9 {
            panic!("Erreur sur le consomable : id={}, le nombre d'effets doit être de 8.", ressource.entite.id);
        } else {
            Self { ressource, effets }
        }
    }

    pub fn get_id(&self) -> String { self.ressource.entite.id.clone() }

    pub fn get_description(&self) -> String { self.ressource.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.ressource.entite.nom.clone() }

    pub fn get_prix(&self) -> u32 { self.ressource.prix.clone() }

    pub fn get_rarete(&self) -> Rarete { self.ressource.rarete.clone() }

    pub fn get_effets(&self) -> Vec<u16> { self.effets.clone() }

    fn str_effets(&self) -> String {
        let mut effets_str = String::new();
        for i in 0..self.effets.len()-1 {
            effets_str.push_str(&self.effets[i].to_string());
            effets_str.push_str(", ");
        }
        effets_str.push_str(&self.effets[self.effets.len()-1].to_string());
        effets_str
    }
}

impl std::fmt::Display for Consommable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Consommable : ressource = [{}], effets = [{}]", self.ressource, self.str_effets())
    }
}