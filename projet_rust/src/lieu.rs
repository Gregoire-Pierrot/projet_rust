use serde::{Serialize, Deserialize};

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Meteo {
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
    destinations: Vec<String>,
    meteo: Meteo,
    contient: Vec<String>
}

impl Lieu {
    pub fn new(entite: Entite, destinations: Vec<String> /*entites: Vec<String>*/, meteo: String, contient: Vec<String>) -> Self {
        Self {
            entite,
            destinations,
            meteo: match meteo.as_str() {
                "Soleil" => Meteo::Soleil,
                "Pluie" => Meteo::Pluie,
                "Neige" => Meteo::Neige,
                "Interieur" => Meteo::Interieur,
                _ => Meteo::Interieur
            },
            contient
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_destinations(&self) -> Vec<String> { self.destinations.clone() }

    pub fn get_meteo(&self) -> Meteo { self.meteo.clone() }

    pub fn get_contient(&self) -> Vec<String> { self.contient.clone() }

    fn str_destinations(&self) -> String {
        let mut res = String::new();
        for i in 0..self.destinations.len()-1 {
            res.push_str(self.destinations[i].as_str());
            res.push_str(", ");
        }
        res.push_str(self.destinations[self.destinations.len()-1].as_str());
        res
    }

    fn str_contient(&self) -> String {
        let mut res = String::new();
        for i in 0..self.contient.len()-1 {
            res.push_str(self.contient[i].as_str());
            res.push_str(", ");
        }
        res.push_str(self.contient[self.contient.len()-1].as_str());
        res
    }
}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lieu : entite = [{}], destinations = {}, meteo = {}, contient [{}]",self.entite, self.str_destinations(), self.meteo, self.str_contient())
    }
}