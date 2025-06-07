use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::equipement::{Categorie, Arme};
use crate::json_manager::MasterFile;
use crate::attaque::Attaque;
use rand::Rng;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref CHANCE_CRITIQUE: Mutex<f32> = Mutex::new(1.0);
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EquipementType {
    Arme,
    Casque,
    Plastron,
    Gants,
    Jambieres,
    Bottes
}

impl std::fmt::Display for EquipementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Personnage {
    pub entite: Entite,
    pub pv_actuel: u16,
    pub pv_max: u16,
    pub force: u16,
    pub dexterite: u16,
    pub intelligence: u16,
    pub vitesse: u16,
    pub esquive: u16,
    pub chance: u16,
    pub resistance_physique: u16,
    pub resistance_magique: u16,
    pub attaques: Vec<String>,
    pub equipement: HashMap<EquipementType, Option<String>>,
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


    pub fn get_equipement(&self) -> HashMap<EquipementType, Option<String>> { self.equipement.clone() }

    pub fn get_inventaire(&self) -> std::collections::HashMap<String, u32> { self.inventaire.clone() }

    pub fn calcul_dexterite(&mut self) -> u16 {
        let mut dexterite: u16 = self.dexterite;
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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
        let master_file  = MasterFile::new();
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

    pub fn attaque(&mut self,attaque: &Attaque) -> Vec<u16> {// base + equipement + %base+equipement + attaque + %total
        let mut rng = rand::thread_rng();
        let mut degats: Vec<u16> = vec![0, 0]; // [dégâts physique, dégâts magique]
        let mut degats_brute: u16 = 0;
        let mut degats_magique: u16 = 0;
        match attaque.get_categorie() {
            Arme::ArmeMelee => {
                degats_brute= self.calcul_force()+attaque.get_degats();
                degats_brute += (degats_brute * attaque.get_pourcent_bonus_degats()) / 100;// base + attaque + %base+attaque
            }
            Arme::ArmeDistance => {
                degats_brute = self.calcul_dexterite()+attaque.get_degats();
                degats_brute += (degats_brute * attaque.get_pourcent_bonus_degats()) / 100;
            },
            Arme::ArmeMagie => {
                degats_magique = self.calcul_intelligence()+attaque.get_degats();
                degats_magique += (degats_magique * attaque.get_pourcent_bonus_degats()) / 100;// base + attaque + %base+attaque
            }
        }

        let chance_critique = *CHANCE_CRITIQUE.lock().unwrap()   * (0.1*self.calcul_chance() as f32)/100.0;
        if rng.gen::<f32>() <= chance_critique {
            degats_brute = (degats_brute as f32 * 1.5) as u16;
            degats_magique = (degats_magique as f32 * 1.5) as u16;
            println!("Coup critique !");
        }
        degats[0] = degats_brute;
        degats[1] = degats_magique;
        degats
    }

    ///////////////
    /// Fonction qui calcul les dégâts de l'attaque de base.
    pub fn attaque_base(&mut self,master_file: &MasterFile) -> Vec<u16> {  // à opti
        let mut rng = rand::thread_rng();
        let mut degats: Vec<u16> = vec![0, 0];
        let mut degats_brute: u16 = 0;
        let mut degats_magique: u16 = 0;
        if let Some(Some(id)) = self.equipement.get(&EquipementType::Arme) {
            if let Ok(equipement) = master_file.prendre_equipement_id(id) {
                match equipement.get_categorie() {
                    Categorie::Arme(Arme::ArmeMelee) => {
                        degats_brute = self.calcul_force()+equipement.get_bonus_force();
                        degats_brute += (degats_brute * equipement.get_pourcent_bonus_force()) / 100;
                    },
                    Categorie::Arme(Arme::ArmeDistance) => {
                        degats_brute = self.calcul_dexterite()+equipement.get_bonus_dexterite();
                        degats_brute += (degats_brute * equipement.get_pourcent_bonus_dexterite()) / 100;
                    },
                    Categorie::Arme(Arme::ArmeMagie) => {
                        degats_magique = self.calcul_intelligence()+equipement.get_bonus_intelligence();
                        degats_magique += (degats_magique * equipement.get_pourcent_bonus_intelligence()) / 100;
                    },
                    _ => {

                    }
                }

            }
            else {
                degats_brute = self.calcul_force();

            }
        }
        else {
            degats_brute = self.calcul_force();

        }

        let chance_critique = *CHANCE_CRITIQUE.lock().unwrap() * (0.1*self.calcul_chance() as f32)/100.0;
        if rng.gen::<f32>() <= chance_critique {
            degats_brute = (degats_brute as f32 * 1.5) as u16;
            degats_magique = (degats_magique as f32 * 1.5) as u16;
            println!("Coup critique !");
        }

        degats[0] = degats_brute;
        degats[1] = degats_magique;

        degats
    }

    pub fn defense(&mut self, degats_recus: &Vec<u16>) -> u16 {
        let mut rng = rand::thread_rng();
        let esquive = (0.5_f32).min(0.001 * self.calcul_esquive() as f32);
        if rng.gen::<f32>() <= esquive {
            println!("Dégâts esquiver !");
            return 0;
        }
        let degats_physiques = degats_recus[0].saturating_sub(self.calcul_resistance_physique());
        let degats_magiques = degats_recus[1].saturating_sub(self.calcul_resistance_magique());
        degats_physiques + degats_magiques
    }

}

impl std::fmt::Display for Personnage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Personnage : entite = [{}],pv_max={}, pv_actuel = {}, force = {}, dextérité = {}, intelligence = {}, vitesse = {}, esquive = {}, resistance physique = {}, resistance magique = {}, attaques = {}, equipement = [{}], inventaire = [{}]",self.entite, self.pv_actuel,self.pv_max, self.force, self.dexterite, self.intelligence, self.vitesse, self.esquive, self.resistance_physique, self.resistance_magique, self.str_attaques(), self.str_equipement(), self.str_inventaire())
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
    pub ressource: HashMap<String, u32>,
    pub rarete: Rarete
}

impl std::fmt::Display for Rarete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Ressource {
    pub fn new(entite: Entite, prix: u32, ressource: HashMap<String, u32>, rarete: String) -> Self {
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
    pub fn get_ressource(&self) -> HashMap<String, u32> { self.ressource.clone() }
    pub fn get_rarete(&self) -> Rarete { self.rarete.clone() }

    fn str_ressource(&self) -> String {
        let mut str_ressource = String::new();
        for (key, value) in &self.ressource {
            str_ressource.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_ressource.is_empty() {
            str_ressource.pop();
            str_ressource.pop();
        }
        str_ressource
    }

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

impl std::fmt::Display for Ressource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ressource : entite = [{}], prix = {}, ressource = {}",self.entite , self.prix, self.str_ressource())
    }
}