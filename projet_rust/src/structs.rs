use serde::{Serialize, Deserialize};

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
    pub position: String, //id de Lieu
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
    pub resistance_magique: u16
}

impl std::fmt::Display for Personnage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Personnage : entite = [{}], pronom = {}, niveau = {}, position = {}, pv = {}, force = {}, dextérité = {}, intelligence = {}, vitesse = {}, esquive = {}, resistance physique = {}, resistance magique = {}",self.entite, self.pronom, self.niveau, self.position, self.pv, self.force, self.dexterite, self.intelligence, self.vitesse, self.esquive, self.resistance_physique, self.resistance_magique)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ressource {
    pub entite: Entite,
    pub prix: u32,
    pub ressource: Vec<String>
}

impl Ressource {
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