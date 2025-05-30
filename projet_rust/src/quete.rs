use serde::{Serialize, Deserialize};

use crate::structs::Entite;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quete {
    entite: Entite,
    lieu: String,
    recompense: Vec<String>,
    quete_suivante: String,
}

impl Quete {

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_lieu(&self) -> String {self.lieu.clone()}

    pub fn get_quete_suivante(&self) -> String {self.quete_suivante.clone()}

    pub fn get_recompense(&self) -> Vec<String> {self.recompense.clone()}


    pub fn set_description(&mut self, description: String) -> String { self.entite.description = description}
    pub fn set_nom(&mut self, nom: String) -> String {self.entite.nom = nom}
    pub fn set_lieu(&mut self, lieu: String) -> String {self.lieu = lieu}
    pub fn set_quete_suivante(&mut self, quete_suivante: String) -> String {self.quete_suivante = quete_suivante}
    pub fn set_recompense(&mut self, recompense: Vec<String>) -> Vec<String> {self.recompense = recompense}

    pub fn add_recompense(&mut self, recompense: String) {
        self.recompense.push(recompense);
    }

    pub fn remove_recompense(&mut self, recompense: String){
        if let Some(pos) = self.recompense.iter().position(|x| *x == recompense) {
            self.recompense.remove(pos);
        } else {
            panic!("Erreur : la recompense [{}] n'existe pas dans la quete [{}]", recompense, self.entite.id);
        }
    }

}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : id = [{}], nom = [{}], description = [{}], lieu = [{}] , recompense = [{:?}], quete suivante = [{}]",
        self.entite.id, self.entite.nom, self.entite.description, self.lieu, self.recompense, self.quete_suivante)
    }
}