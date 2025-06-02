use serde::{Serialize, Deserialize};

use crate::structs::Personnage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur {
    personnage: Personnage,
    temps: u32,
    reputations: Vec<u16>,
    multiplicateur_xp: u16,
    quetes: Vec<String>
}

impl Joueur {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }
    pub fn set_nom(&mut self, nom: String) { self.personnage.entite.nom = nom; }

    pub fn get_position(&self) -> String { self.personnage.position.clone() }
    pub fn set_position(&mut self, lieu: String) { self.personnage.position = lieu; }

    pub fn get_pronom(&self) -> String { self.personnage.pronom.clone() }
    pub fn set_pronom(&mut self, pronom: String) { self.personnage.pronom = pronom; }

    pub fn get_niveau(&self) -> u8 { self.personnage.niveau.clone() }
    pub fn add_niveau(&mut self, niveau: u8) { self.personnage.niveau += niveau; }

    pub fn get_pv(&self) -> u16 { self.personnage.pv.clone() }
    pub fn set_pv(&mut self, pv: u16) { self.personnage.pv = pv; }

    pub fn get_force(&self) -> u16 { self.personnage.force.clone() }
    pub fn set_force(&mut self, force: u16) { self.personnage.force = force; }

    pub fn get_dexterite(&self) -> u16 { self.personnage.dexterite.clone() }
    pub fn set_dexterite(&mut self, dexterite: u16) { self.personnage.dexterite = dexterite; }

    pub fn get_intelligence(&self) -> u16 { self.personnage.intelligence.clone() }
    pub fn set_intelligence(&mut self, intelligence: u16) { self.personnage.intelligence = intelligence; }

    pub fn get_vitesse(&self) -> u16 { self.personnage.vitesse.clone() }
    pub fn set_vitesse(&mut self, vitesse: u16) { self.personnage.vitesse = vitesse; }

    pub fn get_esquive(&self) -> u16 { self.personnage.esquive.clone() }
    pub fn set_esquive(&mut self, esquive: u16) { self.personnage.esquive = esquive; }

    pub fn get_chance(&self) -> u16 { self.personnage.chance.clone() }
    pub fn set_chance(&mut self, chance: u16) { self.personnage.chance = chance; }

    pub fn get_resistance_physique(&self) -> u16 { self.personnage.resistance_physique.clone() }
    pub fn set_resistance_physique(&mut self, resistance_physique: u16) { self.personnage.resistance_physique = resistance_physique; }

    pub fn get_resistance_magique(&self) -> u16 { self.personnage.resistance_magique.clone() }
    pub fn set_resistance_magique(&mut self, resistance_magique: u16) { self.personnage.resistance_magique = resistance_magique; }

    pub fn get_attaques(&self) -> Vec<String> { self.personnage.attaques.clone() }
    pub fn add_attaque(&mut self, attaque: String) {
        if !self.personnage.attaques.contains(&attaque) {
            self.personnage.attaques.push(attaque);
        }
    }
    pub fn remove_attaque(&mut self, attaque: &String) {
        if let Some(pos) = self.personnage.attaques.iter().position(|x| x == attaque) {
            self.personnage.attaques.remove(pos);
        }
    }

    pub fn get_equipement(&self) -> Vec<String> { self.personnage.equipement.clone() }
    // fn add_equipement(&mut self, equipement: String)
    pub fn remove_equipement(&mut self, equipement: &String) {
        if let Some(pos) = self.personnage.equipement.iter().position(|x| x == equipement) {
            self.personnage.equipement.remove(pos);
            self.add_inventaire(equipement.clone(), 1);
        }
    }

    pub fn get_inventaire(&self) -> std::collections::HashMap<String, u32> { self.personnage.inventaire.clone() }
    pub fn add_inventaire(&mut self, item: String, quantite: u32) {
        let entry = self.personnage.inventaire.entry(item).or_insert(0);
        *entry += quantite;
    }
    //pub fn remove_inventaire(&mut self, item: &String, quantite: u32) 

    pub fn get_temps(&self) -> u32 { self.temps.clone() }
    pub fn set_temps(&mut self, temps: u32) { self.temps = temps; }

    pub fn get_reputations(&self) -> Vec<u16> { self.reputations.clone() }
    pub fn set_reputations(&mut self, reputation: Vec<u16>) { self.reputations = reputation; }

    pub fn get_multiplicateur_xp(&self) -> u16 { self.multiplicateur_xp.clone() }
    pub fn set_multiplicateur_xp(&mut self, multiplicateur_xp: u16) { self.multiplicateur_xp = multiplicateur_xp; }
    
    pub fn get_quetes(&self) -> Vec<String> { self.quetes.clone() }
    pub fn add_quete(&mut self, quete: String) {
        if !self.quetes.contains(&quete) {
            self.quetes.push(quete);
        }
    }
    
    fn str_reputations(&self) -> String {
        let mut res = String::new();
        for i in 0..self.reputations.len()-1 {
            res.push_str(&self.reputations[i].to_string());
            res.push_str(", ");
        }
        res.push_str(&self.reputations[self.reputations.len()-1].to_string());
        res
    }

    fn str_quetes(&self) -> String {
        let mut res = String::new();
        for i in 0..self.quetes.len()-1 {
            res.push_str(&self.quetes[i]);
            res.push_str(", ");
        }
        res.push_str(&self.quetes[self.quetes.len()-1]);
        res
    }
}

impl std::fmt::Display for Joueur {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Joueur : personnage = [{}], temps = {}, reputation = {}, multiplicateur_xp = {}, quetes = {}", self.personnage, self.temps, self.str_reputations(), self.multiplicateur_xp, self.str_quetes())
    }
}