use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::equipement::Categorie;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entite {
    pub id: String,
    pub description: String,
    pub nom: String,
}

impl std::fmt::Display for Entite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Entite : id = {}, description = {}, nom = {}",self.id, self.description, self.nom)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Personnage {
    pub entite: Entite,
    pub position: String,
    pub pronom: String,
    pub niveau: u8,
    pub pv: u16,
    pub force: u16,
    pub dexterite: u16,
    pub intelligence: u16,
    pub vitesse: u16,
    pub esquive: u16,
    pub chance: u16,
    pub resistance_physique: u16,
    pub resistance_magique: u16,
    pub attaques: Vec<String>,
    pub equipement: HashMap<Categorie, Option<String>>,
    pub inventaire: HashMap<String, u32>,
}

impl Personnage {
    fn str_attaques(&self) -> String {
        let mut str_attaques = String::new();
        for i in 0..self.attaques.len()-1 {
            str_attaques.push_str(&self.attaques[i]);
            str_attaques.push_str(", ");
        }
        str_attaques.push_str(&self.attaques[self.attaques.len()-1]);
        str_attaques
    }

    fn str_equipement(&self) -> String {
        let mut str_equipement = String::new();
        for (key, value) in &self.equipement {
            let valeur_affichee = match value {
                Some(v) => v.as_str(),
                None => "aucun",
            };
            str_equipement.push_str(&format!("{}: {}, ", key, valeur_affichee));
        }
        if !str_equipement.is_empty() {
            str_equipement.pop(); 
            str_equipement.pop(); 
        }
        str_equipement
    }

    fn str_inventaire(&self) -> String {
        let mut str_inventaire = String::new();
        for (key, value) in &self.inventaire {
            str_inventaire.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_inventaire.is_empty() {
            str_inventaire.pop();
            str_inventaire.pop();
        }
        str_inventaire
    }
}

impl std::fmt::Display for Personnage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Personnage : entite = [{}], pronom = {}, niveau = {}, position = {}, pv = {}, force = {}, dextérité = {}, intelligence = {}, vitesse = {}, esquive = {}, resistance physique = {}, resistance magique = {}, attaques = {}, equipement = {}, inventaire = {}",self.entite, self.pronom, self.niveau, self.position, self.pv, self.force, self.dexterite, self.intelligence, self.vitesse, self.esquive, self.resistance_physique, self.resistance_magique, self.str_attaques(), self.str_equipement(), self.str_inventaire())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ressource {
    pub entite: Entite,
    pub prix: u32,
    pub ressource: Vec<String>
}

impl Ressource {
    pub fn new(id: String, description: String, nom: String, prix: u32, ressource: Vec<String>) -> Self {
        Self {
            entite: Entite {
                id,
                description,
                nom,
            },
            prix,
            ressource,
        }
    }

    pub fn get_id(&self) -> String {
        self.entite.id.clone()
    }
    pub fn get_description(&self) -> String {
        self.entite.description.clone()
    }
    pub fn get_nom(&self) -> String {
        self.entite.nom.clone()
    }

    pub fn get_prix(&self) -> u32 {
        self.prix
    }
    pub fn get_ressource(&self) -> Vec<String> {
        self.ressource.clone()
    }

    fn str_ressource(&self) -> String {
        let mut str_ressource = String::new();
        for i in 0..self.ressource.len()-1 {
            str_ressource.push_str(&self.ressource[i]);
            str_ressource.push_str(", ");
        }
        str_ressource.push_str(&self.ressource[self.ressource.len()-1]);
        str_ressource
    }
}

impl std::fmt::Display for Ressource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ressource : entite = [{}], prix = {}, ressource = {}",self.entite, self.prix, self.str_ressource())
    }
}