use serde::{Serialize, Deserialize};

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attaque {
    entite: Entite,
    force: u16,
    dexterite: u16,
    intelligence: u16,
    pourcent_bonus_force: u16,
    pourcent_bonus_dexterite: u16,
    pourcent_bonus_intelligence: u16,
}

impl Attaque {
    pub fn get_id(&self) -> String { self.entite.id.clone() }
    pub fn get_description(&self) -> String { self.entite.description.clone() }
    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_force(&self) -> u16 { self.force }
    
    pub fn get_dexterite(&self) -> u16 { self.dexterite }
    
    pub fn get_intelligence(&self) -> u16 { self.intelligence }
    
    pub fn get_pourcent_bonus_force(&self) -> u16 { self.pourcent_bonus_force }
    
    pub fn get_pourcent_bonus_dexterite(&self) -> u16 { self.pourcent_bonus_dexterite }
    
    pub fn get_pourcent_bonus_intelligence(&self) -> u16 { self.pourcent_bonus_intelligence }
}

impl std::fmt::Display for Attaque {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Attaque : entite = [{}], force = {}, dexterite = {}, intelligence = {}, pourcent_bonus_force = {}, pourcent_bonus_dexterite = {}, pourcent_bonus_intelligence = {}",
               self.entite, self.force, self.dexterite, self.intelligence, self.pourcent_bonus_force, self.pourcent_bonus_dexterite, self.pourcent_bonus_intelligence)
    }
}