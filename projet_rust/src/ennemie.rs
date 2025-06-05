use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;


use crate::structs::Personnage;
use crate::json_manager::Item;
use crate::json_manager::MasterFile;
use crate::structs::Ressource;
use crate::joueur::Joueur;
use crate::equipement::Categorie;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ennemie {
    personnage: Personnage,
    dialogues: Vec<String>,
    droptable: HashMap<String, u32>,
}

impl Ennemie {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }

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

    pub fn get_attaques(&self) -> Vec<String> { self.personnage.attaques.clone() }

    pub fn get_equipement(&self) -> HashMap<Categorie, Option<String>> { self.personnage.equipement.clone()}

    pub fn get_inventaire(&self) -> HashMap<String, u32> { self.personnage.inventaire.clone() }

    pub fn get_dialogues(&self) -> Vec<String> { self.dialogues.clone() }

    pub fn get_droptable(&self) -> HashMap<String, u32> { self.droptable.clone() }
    
    pub fn get_personnage(&self) -> &Personnage {&self.personnage}//////////////////////////////////////à enlever ?

    pub fn set_personnage(&mut self, personnage: Personnage) {self.personnage = personnage;}//////////////////////////////////////à enlever ?

    fn str_dialogues(&self) -> String {
        let mut str_dialogues = String::new();
        for i in 0..self.dialogues.len()-1 {
            str_dialogues.push_str(&self.dialogues[i].to_string());
            str_dialogues.push_str(", ");
        }
        str_dialogues.push_str(&self.dialogues[self.dialogues.len()-1].to_string());
        str_dialogues
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

    ///////////////
    /// Fonction qui permet de calculer la chance de récupérer les récompenses de fin de combat
    pub fn lootable(&self) -> HashMap<String, u32>{ 
        let master_file = MasterFile::new();
        let mut loot: HashMap<String, u32> = HashMap::new();
        let mut rng = rand::thread_rng();
        for (objet, quantite) in self.personnage.inventaire.iter() {
            match master_file.prendre_item_id(objet) {
                Ok(item) => {
                    let chance_loot = match &item {
                        Item::Ressource(r) => r.get_value_rarete(),
                        Item::Consommable(c) => c.get_value_rarete(),
                        Item::Equipement(e) => e.get_value_rarete(),
                    };
                    println!("chance d'avoir l'item : {} - {}",objet,chance_loot);
                    for _ in 1..self.droptable[objet.as_str()] {
                        if rng.gen::<f32>() <= chance_loot {
                            loot.entry(objet.clone()).and_modify(|e| *e += 1).or_insert(1);
                        }
                    }
                }
                Err(e) => println!("Item indisponible: {}", e),
            }
        }
        loot
    }

    pub fn degats_recus_net(&mut self,degats_recus_brut: &Vec<u16>) -> u16{
        self.personnage.defense(degats_recus_brut)
    }

    pub fn application_degats(&mut self,degats_recus_net: &u16, joueur: &mut Joueur) -> bool{
                let new_pv = self.get_pv().saturating_sub(*degats_recus_net);

        self.set_pv(new_pv);
        /*
        println!("degats reçus {:?}",degats_recus);
        println!("get pv {}",self.get_pv());
        println!("new_pv {}",new_pv);
        println!("defense {}",defense);
        */
        if self.get_pv() == 0 {
            let mut loot = self.lootable();
            joueur.ajout_recompense_inventaire(loot.clone());
            println!("Vous avez gagnée le combat : voici vos récompenses : {:?}",loot);
            return true;
            //Fin de combat -> retour à l'interface
        }
        false
    }

    ///////////////////////////////////////Mettre la possiblilité de défense pour l'ennemi et l'attaque de base ?
    pub fn combat(&mut self,joueur: &mut Joueur) -> bool {
        let master_file = MasterFile::new();
        let mut rng = rand::thread_rng();
        let attaques = self.get_attaques();
        if !attaques.is_empty() {
            let index = rng.gen_range(0..attaques.len());
            let attaque_obj = match master_file.prendre_attaque_id(&attaques[index]) {
                Ok(a) => a,
                Err(a) => {
                    eprintln!("Erreur : {}", a);
                    return false;
                }
            };
            let attaque = self.personnage.attaque(&attaques[index]);
            let degats = self.degats_recus_net(&attaque);
            println!("{} lance l'attaque : {} - {} dégâts infligés", self.get_nom() , attaque_obj.get_nom(),degats);
            return joueur.application_degats(&degats);
        }
        false
    }

    
}

impl std::fmt::Display for Ennemie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ennemie : personnage = [{}], dialogues = [{}], droptable = [{}]", self.personnage, self.str_dialogues(), self.str_drop_table())
    }
}