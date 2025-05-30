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

}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Quete : id = [{}], nom = [{}], description = [{}], lieu = [{}] , recompense = [{:?}], quete suivante = [{}]",
        self.entite.id, self.entite.nom, self.entite.description, self.lieu, self.recompense, self.quete_suivante)
    }
}