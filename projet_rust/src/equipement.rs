use serde::{Serialize, Deserialize};

use crate::structs::{Ressource, Rarete};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Categorie {
    Arme(Arme),
    Armure(Armure)
}

impl std::fmt::Display for Categorie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Arme {
    ArmeMelee,
    ArmeDistance,
    ArmeMagie
}

impl std::fmt::Display for Arme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Armure {
    Casque,
    Plastron,
    Gants,
    Jambieres,
    Bottes
}

impl std::fmt::Display for Armure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equipement {
    ressource: Ressource,
    bonus_pv: u16,
    bonus_force: u16,
    bonus_dexterite: u16,
    bonus_intelligence: u16,
    bonus_vitesse: u16,
    bonus_esquive: u16,
    bonus_chance: u16,
    bonus_resistance_physique: u16,
    bonus_resistance_magique: u16,
    bonus_multiplicateur_xp: u16,
    pourcent_bonus_pv: u16,
    pourcent_bonus_force: u16,
    pourcent_bonus_dexterite: u16,
    pourcent_bonus_intelligence: u16,
    pourcent_bonus_vitesse: u16,
    pourcent_bonus_esquive: u16,
    pourcent_bonus_chance: u16,
    pourcent_bonus_resistance_physique: u16,
    pourcent_bonus_resistance_magique: u16,
    categorie: Categorie
}

impl Equipement {
    pub fn get_id(&self) -> String { self.ressource.entite.id.clone() }

    pub fn get_description(&self) -> String { self.ressource.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.ressource.entite.nom.clone() }

    pub fn get_prix(&self) -> u32 { self.ressource.prix.clone() }

    pub fn get_ressource(&self) -> Ressource { self.ressource.clone() }

    pub fn get_bonus_pv(&self) -> u16 { self.bonus_pv.clone() }

    pub fn get_bonus_force(&self) -> u16 { self.bonus_force.clone() }

    pub fn get_bonus_dexterite(&self) -> u16 { self.bonus_dexterite.clone() }

    pub fn get_bonus_intelligence(&self) -> u16 { self.bonus_intelligence.clone() }

    pub fn get_bonus_vitesse(&self) -> u16 { self.bonus_vitesse.clone() }

    pub fn get_bonus_esquive(&self) -> u16 { self.bonus_esquive.clone() }

    pub fn get_bonus_chance(&self) -> u16 { self.bonus_chance.clone() }

    pub fn get_bonus_resistance_physique(&self) -> u16 { self.bonus_resistance_physique.clone() }

    pub fn get_bonus_resistance_magique(&self) -> u16 { self.bonus_resistance_magique.clone() }

    pub fn get_bonus_multiplicateur_xp(&self) -> u16 { self.bonus_multiplicateur_xp.clone() }

    pub fn get_pourcent_bonus_pv(&self) -> u16 { self.pourcent_bonus_pv.clone() }

    pub fn get_pourcent_bonus_force(&self) -> u16 { self.pourcent_bonus_force.clone() }

    pub fn get_pourcent_bonus_dexterite(&self) -> u16 { self.pourcent_bonus_dexterite.clone() }

    pub fn get_pourcent_bonus_intelligence(&self) -> u16 { self.pourcent_bonus_intelligence.clone() }

    pub fn get_pourcent_bonus_vitesse(&self) -> u16 { self.pourcent_bonus_vitesse.clone() }

    pub fn get_pourcent_bonus_esquive(&self) -> u16 { self.pourcent_bonus_esquive.clone() }

    pub fn get_pourcent_bonus_chance(&self) -> u16 { self.pourcent_bonus_chance.clone() }

    pub fn get_pourcent_bonus_resistance_physique(&self) -> u16 { self.pourcent_bonus_resistance_physique.clone() }

    pub fn get_pourcent_bonus_resistance_magique(&self) -> u16 { self.pourcent_bonus_resistance_magique.clone() }

    pub fn get_categorie(&self) -> Categorie { self.categorie.clone() }

    pub fn get_rarete(&self) -> Rarete { self.ressource.rarete.clone() }


    pub fn get_value_rarete(&self) -> f32{
        match self.get_rarete(){
            Rarete::Commun => 0.4,             
            Rarete::PeuCommun => 0.3,
            Rarete::Rare => 0.2,
            Rarete::TresRare => 0.1,
            Rarete::Epique => 0.01,
            Rarete::Legendaire => 0.005,
            Rarete::Mythique => 0.0001,
            Rarete::Divin => 0.00001
        }
    }
}

impl std::fmt::Display for Equipement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Equipement : ressource = [{}], bonus_pv : {}, bonus_force : {}, bonus_dexterite : {}, bonus_intelligence : {}, bonus_vitesse : {}, bonus_esquive : {}, bonus_chance : {}, bonus_resistance_physique : {}, bonus_resistance_magique : {}, bonus_multiplicateur_xp : {}, pourcent_bonus_pv : {}, pourcent_bonus_force : {}, pourcent_bonus_dexterite : {}, pourcent_bonus_intelligence : {}, pourcent_bonus_vitesse : {}, pourcent_bonus_esquive : {}, pourcent_bonus_chance : {}, pourcent_bonus_resistance_physique : {}, pourcent_bonus_resistance_magique : {}, categorie = {}",
            self.ressource, self.bonus_pv, self.bonus_force, self.bonus_dexterite, self.bonus_intelligence, self.bonus_vitesse, self.bonus_esquive, self.bonus_chance, self.bonus_resistance_physique, self.bonus_resistance_magique, self.bonus_multiplicateur_xp, self.pourcent_bonus_pv, self.pourcent_bonus_force, self.pourcent_bonus_dexterite, self.pourcent_bonus_intelligence, self.pourcent_bonus_vitesse, self.pourcent_bonus_esquive, self.pourcent_bonus_chance, self.pourcent_bonus_resistance_physique, self.pourcent_bonus_resistance_magique, self.categorie)
    }
}