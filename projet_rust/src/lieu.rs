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
    contient: Vec<ContenuLieu>
}

impl Lieu {
    pub fn new(entite: Entite, destinations: Vec<String> /*entites: Vec<String>*/, meteo: String, contient: Vec<ContenuLieu>) -> Self {
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

    pub fn get_contient(&self) -> Vec<ContenuLieu> { self.contient.clone() }

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
        for i in 0..self.contient.len() {
            match &self.contient[i] {
                ContenuLieu::Reference(id) => res.push_str(id),
                ContenuLieu::Stats(stats) => res.push_str(&format!("EnnemieStats({})", stats.id)),
            }

            if i < self.contient.len() - 1 {
                res.push_str(", ");
            }
        }
        res
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
        write!(f, "Lieu : entite = [{}], destinations = {}, meteo = {}, contient [{}]",self.entite, self.str_destinations(), self.meteo, self.str_contient())
    }
}