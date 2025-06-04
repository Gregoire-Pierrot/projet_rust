use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;

use crate::structs::Personnage;
use crate::json_manager::MasterFile;
use crate::structs::Ressource;
use crate::joueur::Joueur;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ennemie {
    personnage: Personnage,
    dialogues: Vec<String>,
    droptable: HashMap<String, f32>,
}

impl Ennemie {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }

    pub fn get_position(&self) -> String { self.personnage.position.clone() }

    pub fn get_pronom(&self) -> String { self.personnage.pronom.clone() }

    pub fn get_niveau(&self) -> u8 { self.personnage.niveau.clone() }

    pub fn get_pv(&self) -> u16 { self.personnage.pv.clone() }
    pub fn set_pv(&mut self, pv: u16) {self.personnage.pv = pv}

    pub fn get_force(&self) -> u16 { self.personnage.force.clone() }

    pub fn get_dexterite(&self) -> u16 { self.personnage.dexterite.clone() }

    pub fn get_intelligence(&self) -> u16 { self.personnage.intelligence.clone() }

    pub fn get_vitesse(&self) -> u16 { self.personnage.vitesse.clone() }

    pub fn get_esquive(&self) -> u16 { self.personnage.esquive.clone() }

    pub fn get_chance(&self) -> u16 { self.personnage.chance.clone() }

    pub fn get_resistance_physique(&self) -> u16 { self.personnage.resistance_physique.clone() }

    pub fn get_resistance_magique(&self) -> u16 { self.personnage.resistance_magique.clone() }

    pub fn get_dialogues(&self) -> Vec<String> { self.dialogues.clone() }

    pub fn get_droptable(&self) -> HashMap<String, f32> { self.droptable.clone() }

    pub fn set_droptable(&mut self,droptable: HashMap<String, f32>) { self.droptable = droptable}

    fn str_dialogues(&self) -> String {
        let mut res = String::new();
        for i in 0..self.dialogues.len()-1 {
            res.push_str(&self.dialogues[i].to_string());
            res.push_str(", ");
        }
        res.push_str(&self.dialogues[self.dialogues.len()-1].to_string());
        res
    }

    fn str_drop_table(&self) -> String {
        let mut str_drop_table = String::new();
        for (key, value) in &self.droptable {
            str_drop_table.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_drop_table.is_empty() {
            str_drop_table.pop();
            str_drop_table.pop();
        }
        str_drop_table
    }

    pub fn get_personnage(&self) -> &Personnage {
        &self.personnage
    }
    pub fn set_personnage(&mut self, personnage: Personnage) {
        self.personnage = personnage;
    }

    pub fn lootable(&self) -> HashMap<String, u32>{
        let master_file = MasterFile::new();
        let mut loot: HashMap<String, u32> = HashMap::new();
        let mut rng = rand::thread_rng();
        for (item, quantite) in self.personnage.inventaire.iter() {
            if let Ok(ressource) = master_file.prendre_ressource_id(item) {
                let chance_loot: f32 = ressource.get_rarete()+self.droptable[item];
                println!("chance d'avoir l'item : {} - {}",item,chance_loot);
                if chance_loot>1.0 {
                    loot.insert(item.clone(), *quantite);
                }
                else{ 
                    for _ in 1..*quantite {
                        if rng.gen::<f32>() <= chance_loot {
                            loot.entry(item.clone()).and_modify(|e| *e += 1).or_insert(1);
                        }
                    }
                }

            }else {
                println!("Ressource indispo");
            }
        }
        loot
    }

    pub fn application_degats(&mut self,degats: u16, joueur: &mut Joueur){
        self.set_pv(self.get_pv().saturating_sub(degats));
        if self.get_pv() == 0 {
            joueur.ajout_recompense_inventaire(self.lootable());
            //Fin de combat -> retour à l'interface
        }
    }

    
}

impl std::fmt::Display for Ennemie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ennemie : personnage = [{}], dialogues = [{}], droptable = [{}]", self.personnage, self.str_dialogues(), self.str_drop_table())
    }
}