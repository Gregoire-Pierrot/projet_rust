use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::equipement::Categorie;
use crate::json_manager::MasterFile;


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


    pub fn get_equipement(&self) -> HashMap<Categorie, Option<String>> { self.equipement.clone() }

    pub fn get_inventaire(&self) -> std::collections::HashMap<String, u32> { self.inventaire.clone() }

    pub fn calcul_dexterite(&mut self) -> u16 {
        let mut dexterite: u16 = self.dexterite;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    dexterite += equipement_obj.get_bonus_dexterite();
                    dexterite += (dexterite * equipement_obj.get_pourcent_bonus_dexterite()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        dexterite
    }

    pub fn calcul_vitesse(&mut self) -> u16 {
        let mut vitesse: u16 = self.vitesse;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    vitesse += equipement_obj.get_bonus_vitesse();
                    vitesse += (vitesse * equipement_obj.get_pourcent_bonus_vitesse()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        vitesse
    }

    pub fn calcul_esquive(&mut self) -> u16 {
        let mut esquive: u16 = self.esquive;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    esquive += equipement_obj.get_bonus_esquive();
                    esquive += (esquive * equipement_obj.get_pourcent_bonus_esquive()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        esquive
    }

    pub fn calcul_chance(&mut self) -> u16 {
        let mut chance: u16 = self.chance;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    chance += equipement_obj.get_bonus_chance();
                    chance += (chance * equipement_obj.get_pourcent_bonus_chance()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        chance
    }

    pub fn calcul_resistance_physique(&mut self) -> u16 {
        let mut resistance_physique: u16 = self.resistance_physique;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    resistance_physique += equipement_obj.get_bonus_resistance_physique();
                    resistance_physique += (resistance_physique * equipement_obj.get_pourcent_bonus_resistance_physique()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        resistance_physique
    }

     pub fn calcul_resistance_magique(&mut self) -> u16 {
        let mut resistance_magique: u16 = self.resistance_magique;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    resistance_magique += equipement_obj.get_bonus_resistance_magique();
                    resistance_magique += (resistance_magique * equipement_obj.get_pourcent_bonus_resistance_magique()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        resistance_magique
    }

    pub fn calcul_force(&mut self) -> u16 { // Dégats physique
        let mut force: u16 = self.force;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    force += equipement_obj.get_bonus_force();
                    force += (force * equipement_obj.get_pourcent_bonus_force()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        force
    }

    pub fn calcul_intelligence(&mut self) -> u16 { // Dégats magique
        let mut intelligence: u16 = self.intelligence;
        let mut master_file  = MasterFile::new();
        for equipement in self.get_equipement() {
            if let Some(equipement_id) = equipement.1 {
                if let Ok(equipement_obj) = master_file.prendre_equipement_id(&equipement_id) {
                    intelligence += equipement_obj.get_bonus_intelligence();
                    intelligence += (intelligence * equipement_obj.get_pourcent_bonus_intelligence()) / 100; // base + equipement + %base+equipement
                }
            }
        }
        intelligence
    }

    pub fn attaque(&mut self,attaque_id: &String) -> Vec<u16> {// base + equipement + %base+equipement + attaque + %total
        let mut master_file  = MasterFile::new();
        let mut degats: Vec<u16> = vec![0, 0]; // [dégâts physique, dégâts magique]
        if let Ok(attaque) = master_file.prendre_attaque_id(&attaque_id) {
            let mut degats_brute: u16 = self.calcul_force()+attaque.get_force();
            degats_brute += (degats_brute * attaque.get_pourcent_bonus_force()) / 100;// base + attaque + %base+attaque

            let mut degats_magique: u16 = self.calcul_intelligence()+attaque.get_intelligence();
            degats_magique += (degats_magique * attaque.get_pourcent_bonus_intelligence()) / 100;// base + attaque + %base+attaque

            degats[0] = degats_brute;
            degats[1] = degats_magique;
        }
        degats
    }

    pub fn defense(&mut self, degats_recus: &Vec<u16>) -> u16 {
        let degats_physiques = degats_recus[0].saturating_sub(self.calcul_resistance_physique());
        let degats_magiques = degats_recus[1].saturating_sub(self.calcul_resistance_magique());
        /*
        println!("degats physiques - {}",degats_physiques);
        println!("degats magiques - {}",degats_magiques);
        println!("resistance physique - {}",self.calcul_resistance_physique());
        println!("resistance magiques - {}",self.calcul_resistance_magique());
        */
        degats_physiques + degats_magiques
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