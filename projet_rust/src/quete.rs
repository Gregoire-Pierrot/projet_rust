use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    Combat(String),
    Dialogue(String),
    Obtention(String),
    Interaction(String)
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
    quete_joueur: bool,
    dialogue_a_enlever: Option<String>,
    statut: StatutQuete,
    fin_de_quete: FinDeQuete
}

impl Quete {
    pub fn new(entite: Entite, lieu: String, recompense: HashMap<String, u32>, quetes_suivantes: Vec<String>, quete_joueur: bool,dialogue_a_enlever: Option<String>, statut: String, fin_de_quete: FinDeQuete) -> Self {
        Self {
            entite,
            lieu,
            recompense,
            quetes_suivantes,
            quete_joueur,
            dialogue_a_enlever,
            statut: match statut.as_str() {
                "NonCommencee" => StatutQuete::NonCommencee,
                "EnCours" => StatutQuete::EnCours,
                "Terminee" => StatutQuete::Terminee,
                _ => panic!("Statut de quête inconnu : {}", statut)
            },
            fin_de_quete
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }
    pub fn set_description(&mut self, description: String) { self.entite.description = description}

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }
    pub fn set_nom(&mut self, nom: String) {self.entite.nom = nom}

    pub fn get_lieu(&self) -> String { self.lieu.clone() }
    pub fn set_lieu(&mut self, lieu: String) {self.lieu = lieu}

    pub fn get_recompense(&self) -> HashMap<String, u32> { self.recompense.clone() }
    pub fn set_recompense(&mut self, recompense: HashMap<String, u32>) {self.recompense = recompense}

    pub fn get_quetes_suivantes(&self) -> Vec<String> { self.quetes_suivantes.clone() }
    pub fn set_quetes_suivantes(&mut self, quetes_suivantes: Vec<String>) {self.quetes_suivantes = quetes_suivantes}

    pub fn get_quete_joueur(&self) -> bool { self.quete_joueur.clone() }

    pub fn get_dialogue_a_enlever(&self) -> Option<String> {self.dialogue_a_enlever.clone()}

    pub fn get_statut(&self) -> StatutQuete { self.statut.clone() }
    pub fn set_statut(&mut self, statut: StatutQuete) {self.statut = statut;}

    pub fn get_fin_de_quete(&self) -> FinDeQuete { self.fin_de_quete.clone() }


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

    fn str_quetes_suivantes(&self) -> String {
        let mut quetes_str = String::new();
        for i in 0..self.quetes_suivantes.len()-1 {
            quetes_str.push_str(&self.quetes_suivantes[i]);
            quetes_str.push_str(", ");
        }
        quetes_str.push_str(&self.quetes_suivantes[self.quetes_suivantes.len()-1]);
        quetes_str
    }
    
    pub fn str_fin_de_quete(&self) -> String {
        match &self.fin_de_quete {
            FinDeQuete::Combat(enemy) => format!("Combat contre {}", enemy),
            FinDeQuete::Dialogue(dialogue) => format!("Dialogue avec {}", dialogue),
            FinDeQuete::Obtention(objet) => format!("Obtention de {}", objet),
            FinDeQuete::Interaction(interaction) => format!("Interaction avec {}", interaction),
        }
    }

    ///////////////
    ///Fonction pour vérifier si une quête du joueur est fini
    pub fn find_fin_de_quete(&self, id_fin_de_quete: String) -> bool {
        match &self.fin_de_quete {
            FinDeQuete::Obtention(objet) if objet == &id_fin_de_quete => {
                println!("Fin de quête par obtention de {}", objet);
                return true
            }
            FinDeQuete::Combat(enemy) if enemy == &id_fin_de_quete => {
                println!("Fin de quête par combat contre {}", enemy);
                return true
            }
            FinDeQuete::Dialogue(dialogue) if dialogue == &id_fin_de_quete => {
                println!("Fin de quête par dialogue avec {}", dialogue);
                return true
            }
            FinDeQuete::Interaction(inter) if inter == &id_fin_de_quete => {
                println!("Fin de quête par interaction avec {}", inter);
                return true
            }
            _ => {
                println!("Ce n'est pas la bonne condition de fin.");
                return false
            }
        }
    }
}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : entite = [{}], lieu = {}, recompense = {}, quetes_suivantes = {}, quete_joueur = {}, statut = {}, fin_de_quete = {}", 
               self.entite, self.lieu, self.str_recompense(), self.str_quetes_suivantes(), self.quete_joueur, self.statut, self.fin_de_quete)
    }
}