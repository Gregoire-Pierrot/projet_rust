use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entite {
    pub description: String,
    pub nom: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Meteo {
    Soleil,
    Pluie,
    Neige,
    Interieur,
}

impl std::fmt::Display for Meteo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lieu {
    entite: Entite,
    id: String,
    destinations_id: Vec<String>,
    //entites: Vec<String>,
    meteo: Meteo,
}

impl Lieu {
    pub fn new(entite: Entite, id: String, destinations_id: Vec<String> /*entites: Vec<String>*/, meteo: String) -> Self {
        Self {
            entite,
            id,
            destinations_id,
            //entites,
            meteo: match meteo.as_str() {
                "Soleil" => Meteo::Soleil,
                "Pluie" => Meteo::Pluie,
                "Neige" => Meteo::Neige,
                "Interieur" => Meteo::Interieur,
                _ => Meteo::Interieur
            }
        }
    }

    pub fn get_id(&self) -> String { self.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_destinations_id(&self) -> Vec<String> { self.destinations_id.clone() }

    fn str_destination_id(&self) -> String {
        let mut res = String::new();
        for i in 0..self.destinations_id.len()-1 {
            res.push_str(self.destinations_id[i].as_str());
            res.push_str(", ");
        }
        res.push_str(self.destinations_id[self.destinations_id.len()-1].as_str());
        res
    }

    //fn get_entites(&self) -> Vec<Entite> { self.entites.clone() }

    fn get_meteo(&self) -> Meteo { self.meteo.clone() }

    pub fn add_destination_id(&mut self, id: String) { self.destinations_id.push(id); }
}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lieu : description = {}, nom = {}, destinations = {}, meteo = {}", self.entite.description, self.entite.nom, self.str_destination_id(), self.meteo)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Personnage {
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
    pub multiplicateur_xp: u16
}

impl std::fmt::Display for Personnage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Personnage : description = {}, nom = {}, pronom = {}, niveau = {}, position = {}, pv = {}, force = {}, dextérité = {}, intelligence = {}, vitesse = {}, esquive = {}, resistance physique = {}, resistance magique = {}, multiplicateur d'xp = {}", self.entite.description, self.entite.nom, self.pronom, self.niveau, self.position, self.pv, self.force, self.dexterite, self.intelligence, self.vitesse, self.esquive, self.resistance_physique, self.resistance_magique, self.multiplicateur_xp)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur {
    personnage: Personnage,
    temps: u32,
    reputations: Vec<u16>
}

impl Joueur {
    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }
    pub fn set_nom(&mut self, nom: String) { self.personnage.entite.nom = nom; }

    pub fn get_position(&self) -> String { self.personnage.position.clone() }
    pub fn set_position(&mut self, lieu: String) { self.personnage.position = lieu; }

    pub fn get_pronom(&self) -> String { self.personnage.pronom.clone() }
    pub fn set_pronom(&mut self, pronom: String) { self.personnage.pronom = pronom; }

    pub fn get_niveau(&self) -> u8 { self.personnage.niveau.clone() }
    pub fn add_niveau(&mut self, niveau: u8) { self.personnage.niveau += niveau; }

    pub fn get_pv(&self) -> u16 { self.personnage.pv.clone() }
    pub fn set_pv(&mut self, pv: u16) { self.personnage.pv = pv; }

    pub fn get_force(&self) -> u16 { self.personnage.force.clone() }
    pub fn set_force(&mut self, force: u16) { self.personnage.force = force; }

    pub fn get_dexterite(&self) -> u16 { self.personnage.dexterite.clone() }
    pub fn set_dexterite(&mut self, dexterite: u16) { self.personnage.dexterite = dexterite; }

    pub fn get_intelligence(&self) -> u16 { self.personnage.intelligence.clone() }
    pub fn set_intelligence(&mut self, intelligence: u16) { self.personnage.intelligence = intelligence; }

    pub fn get_vitesse(&self) -> u16 { self.personnage.vitesse.clone() }
    pub fn set_vitesse(&mut self, vitesse: u16) { self.personnage.vitesse = vitesse; }

    pub fn get_esquive(&self) -> u16 { self.personnage.esquive.clone() }
    pub fn set_esquive(&mut self, esquive: u16) { self.personnage.esquive = esquive; }

    pub fn get_chance(&self) -> u16 { self.personnage.chance.clone() }
    pub fn set_chance(&mut self, chance: u16) { self.personnage.chance = chance; }

    pub fn get_resistance_physique(&self) -> u16 { self.personnage.resistance_physique.clone() }
    pub fn set_resistance_physique(&mut self, resistance_physique: u16) { self.personnage.resistance_physique = resistance_physique; }

    pub fn get_resistance_magique(&self) -> u16 { self.personnage.resistance_magique.clone() }
    pub fn set_resistance_magique(&mut self, resistance_magique: u16) { self.personnage.resistance_magique = resistance_magique; }

    pub fn get_multiplicateur_xp(&self) -> u16 { self.personnage.multiplicateur_xp.clone() }
    pub fn set_multiplicateur_xp(&mut self, multiplicateur_xp: u16) { self.personnage.multiplicateur_xp = multiplicateur_xp; }

    pub fn get_temps(&self) -> u32 { self.temps.clone() }
    pub fn set_temps(&mut self, temps: u32) { self.temps = temps; }

    pub fn get_reputations(&self) -> Vec<u16> { self.reputations.clone() }
    pub fn set_reputations(&mut self, reputation: Vec<u16>) { self.reputations = reputation; }
    fn str_reputations(&self) -> String {
        let mut res = String::new();
        for i in 0..self.reputations.len()-1 {
            res.push_str(&self.reputations[i].to_string());
            res.push_str(", ");
        }
        res.push_str(&self.reputations[self.reputations.len()-1].to_string());
        res
    }
}

impl std::fmt::Display for Joueur {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Joueur : personnage = [{}], temps = {}, reputation = {}", self.personnage, self.temps, self.str_reputations())
    }
}