use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;

use crate::structs::Entite;
use crate::ennemie::Ennemie;
use crate::joueur::Joueur;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Meteo {
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
    destinations: Vec<String>,
    meteo: Meteo,
    contient_ressources: HashMap<String, u32>,
    contient_ennemies: HashMap<String, Vec<u16>>,
    contient_pnj: Vec<String>
}

impl Lieu {
    pub fn new(entite: Entite, destinations: Vec<String>, meteo: String, contient_ressources: HashMap<String, u32>, contient_ennemies: HashMap<String, Vec<u16>>, contient_pnj: Vec<String>) -> Self {
        Self {
            entite,
            destinations,
            meteo: match meteo.as_str() {
                "Soleil" => Meteo::Soleil,
                "Pluie" => Meteo::Pluie,
                "Neige" => Meteo::Neige,
                "Interieur" => Meteo::Interieur,
                _ => panic!("Météo inconnue : {}", meteo)
            },
            contient_ressources,
            contient_ennemies,
            contient_pnj
        }
    }

    pub fn get_id(&self) -> String { self.entite.id.clone() }

    pub fn get_description(&self) -> String { self.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.entite.nom.clone() }

    pub fn get_destinations(&self) -> Vec<String> { self.destinations.clone() }

    pub fn get_meteo(&self) -> Meteo { self.meteo.clone() }

    pub fn get_contient_ressources(&self) -> HashMap<String, u32> { self.contient_ressources.clone() }

    pub fn get_contient_ennemies(&self) -> HashMap<String, Vec<u16>> { self.contient_ennemies.clone() }

    pub fn get_contient_pnj(&self) -> Vec<String> { self.contient_pnj.clone() }

    fn str_destinations(&self) -> String {
        let mut str_destinations = String::new();
        for i in 0..self.destinations.len()-1 {
            str_destinations.push_str(self.destinations[i].as_str());
            str_destinations.push_str(", ");
        }
        str_destinations.push_str(self.destinations[self.destinations.len()-1].as_str());
        str_destinations
    }

    fn str_contient_ressources(&self) -> String {
        let mut str_contient_ressources = String::new();
        for (key, value) in &self.contient_ressources {
            str_contient_ressources.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_contient_ressources.is_empty() {
            str_contient_ressources.pop(); // Remove last space
            str_contient_ressources.pop(); // Remove last space
        }
        str_contient_ressources
    }

    fn str_contient_ennemies(&self) -> String {
        let mut str_contient_ennemies = String::new();
        for (key, value) in &self.contient_ennemies {
            str_contient_ennemies.push_str(&format!("{}: {:?}, ", key, value));
        }
        if !str_contient_ennemies.is_empty() {
            str_contient_ennemies.pop(); // Remove last space
            str_contient_ennemies.pop(); // Remove last space
        }
        str_contient_ennemies
    }

    fn str_contient_pnj(&self) -> String {
        let mut str_contient_pnj = String::new();
        for i in 0..self.contient_pnj.len()-1 {
            str_contient_pnj.push_str(self.contient_pnj[i].as_str());
            str_contient_pnj.push_str(", ");
        }
        str_contient_pnj.push_str(self.contient_pnj[self.contient_pnj.len()-1].as_str());
        str_contient_pnj
    }

    pub fn synchro_ennemie(&self, ennemie: &mut Ennemie, joueur: &Joueur) {

        let mut rng = rand::thread_rng();
        let ennemie_stats = &self.contient_ennemies[&ennemie.get_id()];
        let length_stats = ennemie_stats.len();
        let ennemie_base = ennemie.clone();
    
        // Niveau aléatoire
        let niveau = ennemie_stats[rng.gen_range(0..length_stats)];

        for i in 0..niveau {
            let stat_rand_pv:u16 = rng.gen_range(0..ennemie.get_mod_pv());
            ennemie.set_pv_actuel(ennemie.get_pv_actuel() + stat_rand_pv);
            ennemie.set_pv_max(ennemie.get_pv_actuel() + stat_rand_pv);

            let stat_rand_force:u16  = rng.gen_range(0..ennemie.get_mod_force());
            ennemie.set_force(ennemie.get_force() + stat_rand_force * niveau);
    

            let stat_rand_force:u16  = rng.gen_range(0..ennemie.get_mod_force());
            ennemie.set_force(ennemie.get_force() + stat_rand_force * niveau);
        
            let stat_rand_dexterite:u16  = rng.gen_range(0..ennemie.get_mod_dexterite());
            ennemie.set_dexterite(ennemie.get_dexterite() + stat_rand_dexterite * niveau);
        
            let stat_rand_intelligence:u16  = rng.gen_range(0..ennemie.get_mod_intelligence());
            ennemie.set_intelligence(ennemie.get_intelligence() + stat_rand_intelligence * niveau);
        
            let stat_rand_vitesse:u16  = rng.gen_range(0..ennemie.get_mod_vitesse());
            ennemie.set_vitesse(ennemie.get_vitesse() + stat_rand_vitesse * niveau);
        
            let stat_rand_esquive:u16  = rng.gen_range(0..ennemie.get_mod_esquive());
            ennemie.set_esquive(ennemie.get_esquive() + stat_rand_esquive * niveau);
        
            let stat_rand_chance:u16  = rng.gen_range(0..ennemie.get_mod_chance());
            ennemie.set_chance(ennemie.get_chance() + stat_rand_chance * niveau);

            let stat_rand_resistance_physique:u16  = rng.gen_range(0..ennemie.get_mod_resistance_physique());
            ennemie.set_resistance_physique(ennemie.get_resistance_physique() + stat_rand_resistance_physique * niveau);

            let stat_rand_resistance_magique:u16  = rng.gen_range(0..ennemie.get_mod_resistance_magique());
            ennemie.set_resistance_magique(ennemie.get_resistance_magique() + stat_rand_resistance_magique * niveau);
        }


        let mut somme_stats_random: u16 = 0;
        somme_stats_random += ennemie.get_force() - ennemie_base.get_force();
        somme_stats_random += ennemie.get_dexterite() - ennemie_base.get_dexterite();
        somme_stats_random += ennemie.get_intelligence() - ennemie_base.get_intelligence();
        somme_stats_random += ennemie.get_vitesse()- ennemie_base.get_vitesse();
        somme_stats_random += ennemie.get_esquive()- ennemie_base.get_esquive();
        somme_stats_random += ennemie.get_chance() - ennemie_base.get_chance();
        somme_stats_random += ennemie.get_resistance_physique() - ennemie_base.get_resistance_physique();
        somme_stats_random += ennemie.get_resistance_magique()- ennemie_base.get_resistance_magique();

        let xp = (niveau as u32 * (somme_stats_random as u32 +ennemie.get_xp())) * (niveau as u32 / joueur.get_niveau() as u32) * joueur.get_multiplicateur_xp() as u32;
        
        ennemie.set_xp(xp);
    }


}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lieu : entite = [{}], destinations = {}, meteo = {}, contient_ressources = [{}], contient_ennemies = [{}], contient_pnj [{}]",self.entite, self.str_destinations(), self.meteo, self.str_contient_ressources(), self.str_contient_ennemies(), self.str_contient_pnj())
    }
}