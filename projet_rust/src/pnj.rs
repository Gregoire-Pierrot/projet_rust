use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::Personnage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pnj {
    personnage: Personnage,
    dialogues: Vec<String>,
    commerce_table: HashMap<String, u32>,
}

impl Pnj {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }

    pub fn get_pv(&self) -> u16 { self.personnage.pv.clone() }

    pub fn get_force(&self) -> u16 { self.personnage.force.clone() }

    pub fn get_dexterite(&self) -> u16 { self.personnage.dexterite.clone() }

    pub fn get_intelligence(&self) -> u16 { self.personnage.intelligence.clone() }

    pub fn get_vitesse(&self) -> u16 { self.personnage.vitesse.clone() }

    pub fn get_esquive(&self) -> u16 { self.personnage.esquive.clone() }

    pub fn get_chance(&self) -> u16 { self.personnage.chance.clone() }

    pub fn get_resistance_physique(&self) -> u16 { self.personnage.resistance_physique.clone() }

    pub fn get_resistance_magique(&self) -> u16 { self.personnage.resistance_magique.clone() }

    pub fn get_attaques(&self) -> Vec<String> { self.personnage.attaques.clone() }

    pub fn get_equipement(&self) -> HashMap<String, Option<String>> { self.personnage.equipement.clone() }

    pub fn get_inventaire(&self) -> HashMap<String, u32> { self.personnage.inventaire.clone() }

    pub fn get_dialogues(&self) -> Vec<String> { self.dialogues.clone() }

    pub fn get_commerce_table(&self) -> HashMap<String, u32> { self.commerce_table.clone() }

    fn str_dialogues(&self) -> String {
        let mut str_dialogues = String::new();
        for i in 0..self.dialogues.len()-1 {
            str_dialogues.push_str(&self.dialogues[i].to_string());
            str_dialogues.push_str(", ");
        }
        str_dialogues.push_str(&self.dialogues[self.dialogues.len()-1].to_string());
        str_dialogues
    }

    fn str_commerce_table(&self) -> String {
        let mut str_commerce_table = String::new();
        for (key, value) in &self.commerce_table {
            str_commerce_table.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_commerce_table.is_empty() {
            str_commerce_table.pop(); // Remove last space
            str_commerce_table.pop(); // Remove last space
        }
        str_commerce_table
    }
}

impl std::fmt::Display for Pnj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pnj : personnage = [{}], dialogues = [{}], commerce_table = [{}]", self.personnage, self.str_dialogues(), self.str_commerce_table())
    }
}