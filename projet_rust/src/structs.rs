use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    pub equipement: HashMap<String, Option<String>>,
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
            match value {
                Some(item) => str_equipement.push_str(&format!("{}: {}, ", key, item)),
                None => str_equipement.push_str(&format!("{}: None, ", key)),
            }
        }
        if !str_equipement.is_empty() {
            str_equipement.pop(); // Remove last comma
            str_equipement.pop(); // Remove last space
        }
        str_equipement
    }

    fn str_inventaire(&self) -> String {
        let mut str_inventaire = String::new();
        for (key, value) in &self.inventaire {
            str_inventaire.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_inventaire.is_empty() {
            str_inventaire.pop(); // Remove last comma
            str_inventaire.pop(); // Remove last space
        }
        str_inventaire
    }
}

impl std::fmt::Display for Personnage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Personnage : entite = [{}], pv = {}, force = {}, dextérité = {}, intelligence = {}, vitesse = {}, esquive = {}, resistance physique = {}, resistance magique = {}, attaques = {}, equipement = [{}], inventaire = [{}]",self.entite, self.pv, self.force, self.dexterite, self.intelligence, self.vitesse, self.esquive, self.resistance_physique, self.resistance_magique, self.str_attaques(), self.str_equipement(), self.str_inventaire())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Rarete {
    Commun,          // 0.4 => 40%
    PeuCommun,       // 0.3 => 30%
    Rare,            // 0.2 => 20%
    TresRare,        // 0.1 => 10%
    Epique,          // 0.01 => 1%
    Legendaire,      // 0.005 => 0.5%
    Mythique,        // 0.0001 => 0.01%
    Divin            // 0.00001 => 0.001%
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ressource {
    pub entite: Entite,
    pub prix: u32,
    pub ressource: Vec<String>,
    pub rarete: Rarete
}

impl std::fmt::Display for Rarete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Ressource {
    pub fn new(entite: Entite, prix: u32, ressource: Vec<String>, rarete: String) -> Self {
        Self {
            entite: entite.clone(),
            prix,
            ressource,
            rarete: match rarete.as_str() {
                "Commun" => Rarete::Commun,             
                "PeuCommun" => Rarete::PeuCommun,
                "Rare" => Rarete::Rare,
                "TresRare" => Rarete::TresRare,
                "Epique" => Rarete::Epique,
                "Legendaire" => Rarete::Legendaire,
                "Mythique" => Rarete::Mythique,
                "Divin" => Rarete::Divin,
                _ => panic!("Erreur sur la ressource : id={}, la rareté n'est pas reconnue.", entite.id)
            }
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }
    pub fn get_description(&self) -> String { self.entite.description.clone() }
    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_prix(&self) -> u32 { self.prix }
    pub fn get_ressource(&self) -> Vec<String> { self.ressource.clone() }
    pub fn get_rarete(&self) -> Rarete { self.rarete.clone() }

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
        write!(f, "ressource : entite = [{}], prix = {}, ressource = {}",self.entite , self.prix, self.str_ressource())
    }
}