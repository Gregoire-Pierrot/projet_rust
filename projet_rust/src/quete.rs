use serde::{Serialize, Deserialize};

use crate::structs::Entite;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum FinDeQuete {
    Combat(String),
    Dialogue(String),
    Obtention(String),
    Interaction(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StatutQuete {
    EnCours,
    Terminee,
    NonCommencee,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quete {
    entite: Entite,
    lieu: String,
    recompense: HashMap<String, u32>,
    ajout_quete: String,
    quetes_suivantes: Vec<String>,
    statut: StatutQuete,
    fin_de_quete: FinDeQuete,
}

impl Quete {

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_lieu(&self) -> String {self.lieu.clone()}

    pub fn get_quete_suivante(&self) -> Vec<String> {self.quetes_suivantes.clone()}

    pub fn get_recompense(&self) -> HashMap<String, u32> {self.recompense.clone()}

    pub fn get_ajout_quete(&self) -> String {self.ajout_quete.clone()}

    pub fn get_statut(&self) -> StatutQuete {self.statut.clone()}

    pub fn get_fin_de_quete(&self) -> FinDeQuete {self.fin_de_quete.clone()}

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

    pub fn set_description(&mut self, description: String) { self.entite.description = description}
    pub fn set_nom(&mut self, nom: String) {self.entite.nom = nom}
    pub fn set_lieu(&mut self, lieu: String) {self.lieu = lieu}
    pub fn set_quete_suivante(&mut self, quetes_suivantes: Vec<String>) {self.quetes_suivantes = quetes_suivantes}
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

    pub fn set_statut(&mut self, statut: StatutQuete) {self.statut = statut;}



    pub fn get_str_recompense(&self) -> String {
        let mut str_recompense = String::new();
        for (item, quantite) in &self.recompense {
            str_recompense.push_str(&format!("{}: {}, ", item, quantite));
        }
        if !str_recompense.is_empty() {
            str_recompense.pop();
            str_recompense.pop();
        }
        str_recompense
    }

    pub fn get_str_fin_de_quete(&self) -> String {
        match &self.fin_de_quete {
            FinDeQuete::Combat(enemy) => format!("Combat contre {}", enemy),
            FinDeQuete::Dialogue(dialogue) => format!("Dialogue avec {}", dialogue),
            FinDeQuete::Obtention(objet) => format!("Obtention de {}", objet),
            FinDeQuete::Interaction(interaction) => format!("Interaction avec {}", interaction),
        }
    }

    pub fn get_details_quete(&self) -> Vec<String> { //Fonctions pour récupérer les détails à afficher dans l'interface quand on clique sur une quête
        let details:Vec<String> = vec![self.entite.nom.clone(), self.entite.description.clone(), self.lieu.clone(), self.get_str_recompense(),self.get_str_fin_de_quete(),];
        details
    }
}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : id = [{}], nom = [{}], description = [{}], lieu = [{}] , recompense = [{:?}], quete(s) suivante(s) = [{:?}], statut = [{:?}], fin de quête = [{:?}], ajout de quête = [{}]",
        self.entite.id, self.entite.nom, self.entite.description, self.lieu, self.recompense, self.quetes_suivantes, self.statut, self.fin_de_quete, self.ajout_quete)
    }
}