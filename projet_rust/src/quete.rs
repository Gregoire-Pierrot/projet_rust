use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatutQuete {
    NonCommencee,
    EnCours,
    Terminee,
}

impl std::fmt::Display for StatutQuete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinDeQuete {
    Combat,
    Dialogue,
    Obtention,
    Interaction
}

impl std::fmt::Display for FinDeQuete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quete {
    entite: Entite,
    lieu: String,
    recompense: HashMap<String, u32>,
    quetes_suivantes: Vec<String>,
    ajout_quete: String,
    statut: StatutQuete,
    fin_de_quete: HashMap<FinDeQuete, String>
}

impl Quete {
    pub fn new(entite: Entite, lieu: String, recompense: HashMap<String, u32>, quetes_suivantes: Vec<String>, ajout_quete: String, statut: String, fin_de_quete: HashMap<String, String>) -> Self {
        Self {
            entite,
            lieu,
            recompense,
            quetes_suivantes,
            ajout_quete,
            statut: match statut.as_str() {
                "NonCommencee" => StatutQuete::NonCommencee,
                "EnCours" => StatutQuete::EnCours,
                "Terminee" => StatutQuete::Terminee,
                _ => panic!("Statut de quête inconnu : {}", statut)
            },
            fin_de_quete: fin_de_quete.into_iter().map(|(k, v)| {
                let key = match k.as_str() {
                    "Combat" => FinDeQuete::Combat,
                    "Dialogue" => FinDeQuete::Dialogue,
                    "Obtention" => FinDeQuete::Obtention,
                    "Interaction" => FinDeQuete::Interaction,
                    _ => panic!("Type de fin de quête inconnu : {}", k)
                };
                (key, v)
            }).collect()
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }
    pub fn get_description(&self) -> String { self.entite.description.clone() }
    pub fn get_nom(&self) -> String { self.entite.nom.clone() }
    
    pub fn get_lieu(&self) -> String { self.lieu.clone() }
    
    pub fn get_recompense(&self) -> HashMap<String, u32> { self.recompense.clone() }
    
    pub fn get_quete_suivantes(&self) -> Vec<String> { self.quetes_suivantes.clone() }

    pub fn get_ajout_quete(&self) -> String { self.ajout_quete.clone() }
    
    pub fn get_statut(&self) -> StatutQuete { self.statut.clone() }
    
    pub fn get_fin_de_quete(&self) -> HashMap<FinDeQuete, String> { self.fin_de_quete.clone() }

    fn str_recompense(&self) -> String {
        let mut recompense_str = String::new();
        for (key, value) in &self.recompense {
            recompense_str.push_str(&format!("{}: {}, ", key, value));
        }
        if !recompense_str.is_empty() {
            recompense_str.pop();
            recompense_str.pop();
        }
        recompense_str
    }

    fn str_quetes_suivantes(&self) -> String {
        let mut quetes_str = String::new();
        for i in 0..self.quetes_suivantes.len()-1 {
            quetes_str.push_str(&self.quetes_suivantes[i]);
            quetes_str.push_str(", ");
        }
        quetes_str.push_str(&self.quetes_suivantes[self.quetes_suivantes.len()-1]);
        quetes_str
    }

    fn str_fin_de_quete(&self) -> String {
        let mut fin_de_quete_str = String::new();
        for (key, value) in &self.fin_de_quete {
            fin_de_quete_str.push_str(&format!("{:?}: {}, ", key, value));
        }
        if !fin_de_quete_str.is_empty() {
            fin_de_quete_str.pop();
            fin_de_quete_str.pop();
        }
        fin_de_quete_str
    }
}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : entite = [{}], lieu = {}, recompense = {}, quetes_suivantes = {}, ajout_quete = {}, statut = {}, fin_de_quete = {}", 
               self.entite, self.lieu, self.str_recompense(), self.str_quetes_suivantes(), self.ajout_quete, self.statut, self.str_fin_de_quete())
    }
}