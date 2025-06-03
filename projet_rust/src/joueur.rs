use serde::{Serialize, Deserialize};

use crate::structs::Personnage;
use std::collections::HashMap;
use crate::equipement::Categorie;
use crate::json_manager::MasterFile;

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

    pub fn get_equipement(&self) -> HashMap<Categorie, Option<String>> { self.personnage.equipement.clone() }

    pub fn get_inventaire(&self) -> std::collections::HashMap<String, u32> { self.personnage.inventaire.clone() }
    

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



    pub fn add_equipement(&mut self, categorie: &Categorie, equipement: &String) {
        let eq = self.personnage.equipement.entry(categorie.clone()).or_insert(None);
        if eq.is_some() {
            println!("Un équipement est déjà équipé dans la catégorie {:?}: {:?}", categorie, eq.as_ref().unwrap());
        } else {
            *eq = Some(equipement.clone());
            println!("Équipement équipé dans la catégorie {:?}", categorie);
            self.remove_inventaire(equipement, 1);
        }
    }

    pub fn remove_equipement(&mut self, categorie: &Categorie) {
        match self.personnage.equipement.get_mut(categorie) {
            Some(equipement) => {
                if let Some(eq) = equipement.take() {
                    println!("Équipement retiré de la catégorie {:?}: {:?}", categorie, eq);
                    self.add_inventaire(eq, 1);
                } else {
                    println!("Aucun équipement de la catégorie {:?} à retirer.", categorie);
                }
            }
            None => {
                println!("Catégorie {:?} inconnue dans l'équipement.", categorie);
            }
        }
    }

    pub fn add_inventaire(&mut self, item: String, quantite: u32) {
        let entry = self.personnage.inventaire.entry(item).or_insert(0);
        *entry += quantite;
    }

    pub fn remove_inventaire(&mut self, item: &String, quantite: u32){
        if let Some(entry) = self.personnage.inventaire.get_mut(item) {
            if *entry >= quantite {
                *entry -= quantite;
                if *entry == 0 {
                    self.personnage.inventaire.remove(item);
                }
            } else {
                println!("Quantité insuffisante pour retirer {} de {}.", quantite, item);
            }
        } else {
            println!("L'item {} n'est pas dans l'inventaire.", item);
        }
    }

    pub fn appliquer_effets_items(&mut self, effets: Vec<u16>) {
        self.personnage.force += effets[0];
        self.personnage.dexterite += effets[1];
        self.personnage.intelligence += effets[2];
        self.personnage.vitesse += effets[3];
        self.personnage.esquive += effets[4];
        self.personnage.chance += effets[5];
        self.personnage.resistance_physique += effets[6];
        self.personnage.resistance_magique += effets[7];
    }

    pub fn utiliser_item(&mut self, item: &String) {
        let master_file = MasterFile::new();
        match master_file.prendre_consommable_id(item) {
            Ok(consommable) => {
                let effets = consommable.get_effets().clone();
                let should_apply = {
                    let inventaire = &mut self.personnage.inventaire;
                    if let Some(quantite) = inventaire.get_mut(item) {
                        if *quantite > 0 {
                            *quantite -= 1;
                            if *quantite == 0 {
                                self.personnage.inventaire.remove(item);
                            }
                            true
                        } else {
                            println!("Quantité de {} insuffisante pour l'utiliser.", item);
                            false
                        }
                    } else {
                        println!("L'item {} n'est pas dans l'inventaire.", item);
                        false
                    }
                };

                if should_apply {
                    self.appliquer_effets_items(effets);
                    println!("Item {} utilisé.", item);
                }
            }
            _ => {
                println!("L'item {} n'est pas utilisable", item);
            }
        }
    }
}

impl std::fmt::Display for Joueur {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Joueur : personnage = [{}], temps = {}, reputation = {}, multiplicateur_xp = {}, quetes = {}", self.personnage, self.temps, self.str_reputations(), self.multiplicateur_xp, self.str_quetes())
    }
}