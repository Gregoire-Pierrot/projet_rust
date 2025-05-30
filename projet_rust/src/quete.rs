use serde::{Serialize, Deserialize};

use crate::structs::Entite;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quete {
    entite: Entite,
    lieu: String,
    recompense: HashMap<String, u32>,
    quete_suivante: String,
}

impl Quete {

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_lieu(&self) -> String {self.lieu.clone()}

    pub fn get_quete_suivante(&self) -> String {self.quete_suivante.clone()}

    pub fn get_recompense(&self) -> HashMap<String, u32> {self.recompense.clone()}


    pub fn set_description(&mut self, description: String) { self.entite.description = description}
    pub fn set_nom(&mut self, nom: String) {self.entite.nom = nom}
    pub fn set_lieu(&mut self, lieu: String) {self.lieu = lieu}
    pub fn set_quete_suivante(&mut self, quete_suivante: String) {self.quete_suivante = quete_suivante}
    pub fn set_recompense(&mut self, recompense: HashMap<String, u32>) {self.recompense = recompense}

 
    pub fn add_recompense(&mut self, recompense: String, quantite: u32){
        if let Some(q) = self.recompense.get_mut(&recompense) {
            *q += quantite;
        } else {
            self.recompense.insert(recompense, quantite);
        }
    }

    pub fn remove_recompense(&mut self, recompense: &String) {
        self.recompense.remove(recompense);
    }



}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : id = [{}], nom = [{}], description = [{}], lieu = [{}] , recompense = [{:?}], quete suivante = [{}]",
        self.entite.id, self.entite.nom, self.entite.description, self.lieu, self.recompense, self.quete_suivante)
    }
}