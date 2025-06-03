use serde::{Serialize, Deserialize};

use crate::structs::Personnage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ennemie {
    personnage: Personnage,
    dialogues: Vec<String>,
    droptable: Vec<String>,
}

impl Ennemie {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }

    pub fn get_position(&self) -> String { self.personnage.position.clone() }

    pub fn get_pronom(&self) -> String { self.personnage.pronom.clone() }

    pub fn get_niveau(&self) -> u8 { self.personnage.niveau.clone() }

    pub fn get_pv(&self) -> u16 { self.personnage.pv.clone() }

    pub fn get_force(&self) -> u16 { self.personnage.force.clone() }

    pub fn get_dexterite(&self) -> u16 { self.personnage.dexterite.clone() }

    pub fn get_intelligence(&self) -> u16 { self.personnage.intelligence.clone() }

    pub fn get_vitesse(&self) -> u16 { self.personnage.vitesse.clone() }

    pub fn get_esquive(&self) -> u16 { self.personnage.esquive.clone() }

    pub fn get_chance(&self) -> u16 { self.personnage.chance.clone() }

    pub fn get_resistance_physique(&self) -> u16 { self.personnage.resistance_physique.clone() }

    pub fn get_resistance_magique(&self) -> u16 { self.personnage.resistance_magique.clone() }

    pub fn get_dialogues(&self) -> Vec<String> { self.dialogues.clone() }

    pub fn get_droptable(&self) -> Vec<String> { self.droptable.clone() }

    fn str_dialogues(&self) -> String {
        let mut res = String::new();
        for i in 0..self.dialogues.len()-1 {
            res.push_str(&self.dialogues[i].to_string());
            res.push_str(", ");
        }
        res.push_str(&self.dialogues[self.dialogues.len()-1].to_string());
        res
    }

    fn str_drop_table(&self) -> String {
        let mut res = String::new();
        for i in 0..self.droptable.len()-1 {
            res.push_str(&self.droptable[i].to_string());
            res.push_str(", ");
        }
        res.push_str(&self.droptable[self.droptable.len()-1].to_string());
        res
    }

    pub fn get_personnage(&self) -> &Personnage {
        &self.personnage
    }
    pub fn set_personnage(&mut self, personnage: Personnage) {
        self.personnage = personnage;
    }
}

impl std::fmt::Display for Ennemie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ennemie : personnage = [{}], dialogues = [{}], droptable = [{}]", self.personnage, self.str_dialogues(), self.str_drop_table())
    }
}