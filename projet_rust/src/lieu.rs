use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::Entite;
use crate::equipement::Categorie;
use crate::structs::Personnage;
use crate::ennemie::Ennemie;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Meteo {
    Soleil,
    Pluie,
    Neige,
    Interieur,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnnemieStats {
    id: String,
    position: String,
    niveau: u8,
    pv: u16,
    force: u16,
    dexterite: u16,
    intelligence: u16,
    vitesse: u16,
    esquive: u16,
    chance: u16,
    resistance_physique: u16,
    resistance_magique: u16,
    equipement: HashMap<Categorie, Option<String>>,
    inventaire: HashMap<String, u32>,
    droptable: HashMap<String, f32>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ContenuLieu {
    Reference(String),        // Les entitées
    Stats(EnnemieStats), // surcharge d’un ennemi avec ces stats de lieu
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
    contient_ressources: HashMap<String, u32>,
    contient_ennemies: HashMap<String, Vec<u16>>,
    contient_pnj: Vec<String>
}

impl Lieu {
    pub fn new(entite: Entite, destinations: Vec<String>, meteo: String, contient_ressources: HashMap<String, u32>, contient_ennemies: HashMap<String, Vec<u16>>, contient_pnj: Vec<String>) -> Self {
        Self {
            entite,
            destinations,
            meteo: match meteo.as_str() {
                "Soleil" => Meteo::Soleil,
                "Pluie" => Meteo::Pluie,
                "Neige" => Meteo::Neige,
                "Interieur" => Meteo::Interieur,
                _ => panic!("Météo inconnue : {}", meteo)
            },
            contient_ressources,
            contient_ennemies,
            contient_pnj
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_destinations(&self) -> Vec<String> { self.destinations.clone() }

    pub fn get_meteo(&self) -> Meteo { self.meteo.clone() }

    pub fn get_contient_ressources(&self) -> HashMap<String, u32> { self.contient_ressources.clone() }

    pub fn get_contient_ennemies(&self) -> HashMap<String, Vec<u16>> { self.contient_ennemies.clone() }

    pub fn get_contient_pnj(&self) -> Vec<String> { self.contient_pnj.clone() }

    fn str_destinations(&self) -> String {
        let mut str_destinations = String::new();
        for i in 0..self.destinations.len()-1 {
            str_destinations.push_str(self.destinations[i].as_str());
            str_destinations.push_str(", ");
        }
        str_destinations.push_str(self.destinations[self.destinations.len()-1].as_str());
        str_destinations
    }

    fn str_contient_ressources(&self) -> String {
        let mut str_contient_ressources = String::new();
        for (key, value) in &self.contient_ressources {
            str_contient_ressources.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_contient_ressources.is_empty() {
            str_contient_ressources.pop(); // Remove last space
            str_contient_ressources.pop(); // Remove last space
        }
        str_contient_ressources
    }

    fn str_contient_ennemies(&self) -> String {
        let mut str_contient_ennemies = String::new();
        for (key, value) in &self.contient_ennemies {
            str_contient_ennemies.push_str(&format!("{}: {:?}, ", key, value));
        }
        if !str_contient_ennemies.is_empty() {
            str_contient_ennemies.pop(); // Remove last space
            str_contient_ennemies.pop(); // Remove last space
        }
        str_contient_ennemies
    }

    fn str_contient_pnj(&self) -> String {
        let mut str_contient_pnj = String::new();
        for i in 0..self.contient_pnj.len()-1 {
            str_contient_pnj.push_str(self.contient_pnj[i].as_str());
            str_contient_pnj.push_str(", ");
        }
        str_contient_pnj.push_str(self.contient_pnj[self.contient_pnj.len()-1].as_str());
        str_contient_pnj
    }

    pub fn get_stats_ennemie(&self, id: &str) -> Option<EnnemieStats> {
        for contenu in &self.contient {
            if let ContenuLieu::Stats(stats) = contenu {
                if stats.id == id {
                    return Some(stats.clone());
                }
            }
        }
        None
    }

    pub fn attribuer_stats_ennemie(&self, base: &Personnage, stats: &EnnemieStats) -> Personnage {
        Personnage {
            entite: base.entite.clone(),
            position: stats.position.clone(),
            pronom: base.pronom.clone(),
            niveau: stats.niveau,
            pv: stats.pv,
            force: stats.force,
            dexterite: stats.dexterite,
            intelligence: stats.intelligence,
            vitesse: stats.vitesse,
            esquive: stats.esquive,
            chance: stats.chance,
            resistance_physique: stats.resistance_physique,
            resistance_magique: stats.resistance_magique,
            attaques: base.attaques.clone(),
            equipement: stats.equipement.clone(),
            inventaire: stats.inventaire.clone(),
        }
    }

    pub fn synchro_ennemie(&self, ennemie: &mut Ennemie){
        if let Some(stats_ennemie) = self.get_stats_ennemie(&ennemie.get_id()) {
            ennemie.set_personnage(self.attribuer_stats_ennemie(ennemie.get_personnage(),&stats_ennemie));
            ennemie.set_droptable(stats_ennemie.droptable.clone());
        }
    }

}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lieu : entite = [{}], destinations = {}, meteo = {}, contient_ressources = [{}], contient_ennemies = [{}], contient_pnj [{}]",self.entite, self.str_destinations(), self.meteo, self.str_contient_ressources(), self.str_contient_ennemies(), self.str_contient_pnj())
    }
}