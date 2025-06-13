use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::{Ressource, Rarete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parchemin {
    ressource: Ressource,
    attaque: String,
}

impl Parchemin {
    pub fn new(ressource: Ressource, attaque: String) -> Self {
        Self { ressource, attaque }
    }

    pub fn get_id(&self) -> String { self.ressource.entite.id.clone() }

    pub fn get_description(&self) -> String { self.ressource.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.ressource.entite.nom.clone() }

    pub fn get_prix(&self) -> u32 { self.ressource.prix.clone() }

    pub fn get_rarete(&self) -> Rarete { self.ressource.rarete.clone() }

    pub fn get_value_rarete(&self) -> f32{
        match self.get_rarete(){
            Rarete::Commun => 0.4,             
            Rarete::PeuCommun => 0.3,
            Rarete::Rare => 0.2,
            Rarete::TresRare => 0.1,
            Rarete::Epique => 0.01,
            Rarete::Legendaire => 0.005,
            Rarete::Mythique => 0.0001,
            Rarete::Divin => 0.00001
        }
    }

    pub fn get_ressource(&self) -> Ressource { self.ressource.clone() }

    pub fn get_ressources(&self) -> HashMap<String, u32> { self.ressource.get_ressource() }

    pub fn get_attaque(&self) -> String { self.attaque.clone() }
}

impl std::fmt::Display for Parchemin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parchemin : ressource = [{}], attaque = [{}]", self.ressource, self.attaque)
    }
}