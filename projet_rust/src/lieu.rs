use serde::{Serialize, Deserialize};

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Meteo {
    Soleil,
    Pluie,
    Neige,
    Interieur,
}

impl std::fmt::Display for Meteo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lieu {
    entite: Entite,
    destinations_id: Vec<String>,
    //entites: Vec<String>,
    meteo: Meteo,
}

impl Lieu {
    pub fn new(entite: Entite, destinations_id: Vec<String> /*entites: Vec<String>*/, meteo: String) -> Self {
        Self {
            entite,
            destinations_id,
            //entites,
            meteo: match meteo.as_str() {
                "Soleil" => Meteo::Soleil,
                "Pluie" => Meteo::Pluie,
                "Neige" => Meteo::Neige,
                "Interieur" => Meteo::Interieur,
                _ => Meteo::Interieur
            }
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_destinations_id(&self) -> Vec<String> { self.destinations_id.clone() }

    fn str_destination_id(&self) -> String {
        let mut res = String::new();
        for i in 0..self.destinations_id.len()-1 {
            res.push_str(self.destinations_id[i].as_str());
            res.push_str(", ");
        }
        res.push_str(self.destinations_id[self.destinations_id.len()-1].as_str());
        res
    }

    //fn get_entites(&self) -> Vec<Entite> { self.entites.clone() }

    fn get_meteo(&self) -> Meteo { self.meteo.clone() }

    pub fn add_destination_id(&mut self, id: String) { self.destinations_id.push(id); }
}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lieu : id = {}, description = {}, nom = {}, destinations = {}, meteo = {}",self.entite.id, self.entite.description, self.entite.nom, self.str_destination_id(), self.meteo)
    }
}