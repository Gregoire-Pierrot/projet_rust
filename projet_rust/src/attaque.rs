use serde::{Serialize, Deserialize};

use crate::structs::Entite;
use crate::equipement::Arme;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attaque {
    entite: Entite,
    degats: u16,
    pourcent_bonus_degats: u16,
    categorie: Arme
}

impl Attaque {
    pub fn get_id(&self) -> String { self.entite.id.clone() }
    pub fn get_description(&self) -> String { self.entite.description.clone() }
    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_degats(&self) -> u16 { self.degats }

    pub fn get_pourcent_bonus_degats(&self) -> u16 { self.pourcent_bonus_degats }

    pub fn get_categorie(&self) -> Arme { self.categorie.clone() }
}

impl std::fmt::Display for Attaque {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Attaque : entite = [{}], degats = {}, pourcent_bonus_degats = {}, categorie = {}",
               self.entite, self.degats, self.pourcent_bonus_degats, self.categorie)
    }
}