use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quete {
    entite: Entite,
    lieu: String,
    recompense: HashMap<String, u32>,
    quetes_suivantes: Vec<String>,
    ajout_quete: String,
    statut: String,
    fin_de_quete: HashMap<String, String>
}

impl Quete {
    pub fn get_id(&self) -> String { self.entite.id.clone() }
    pub fn get_description(&self) -> String { self.entite.description.clone() }
    pub fn get_nom(&self) -> String { self.entite.nom.clone() }
    
    pub fn get_lieu(&self) -> String { self.lieu.clone() }
    
    pub fn get_recompense(&self) -> HashMap<String, u32> { self.recompense.clone() }
    
    pub fn get_quete_suivantes(&self) -> Vec<String> { self.quetes_suivantes.clone() }

    pub fn get_ajout_quete(&self) -> String { self.ajout_quete.clone() }
    
    pub fn get_statut(&self) -> String { self.statut.clone() }
    
    pub fn get_fin_de_quete(&self) -> HashMap<String, String> { self.fin_de_quete.clone() }

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
        let mut fin_str = String::new();
        for (key, value) in &self.fin_de_quete {
            fin_str.push_str(&format!("{}: {}, ", key, value));
        }
        if !fin_str.is_empty() {
            fin_str.pop();
            fin_str.pop();
        }
        fin_str
    }
}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : entite = [{}], lieu = {}, recompense = {}, quetes_suivantes = {}, ajout_quete = {}, statut = {}, fin_de_quete = {}", 
               self.entite, self.lieu, self.str_recompense(), self.str_quetes_suivantes(), self.ajout_quete, self.statut, self.str_fin_de_quete())
    }
}