use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::Personnage;
use crate::Quete;
use crate::Joueur;
use crate::json_manager::MasterFile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pnj {
    personnage: Personnage,
    dialogues: Vec<String>,
    commerce_table: HashMap<String, u32>,
}

impl Pnj {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }

    pub fn get_inventaire(&self) -> HashMap<String, u32> { self.personnage.inventaire.clone() }

    pub fn remove_inventaire(&mut self, item: &String, quantite: u32){
        if let Some(entry) = self.personnage.inventaire.get_mut(item) {
            if *entry >= quantite {
                *entry -= quantite;
                if *entry == 0 {
                    self.personnage.inventaire.remove(item);
                }
            } else {
                println!("Quantité insuffisante pour retirer {} de {}.", quantite, item);
            }
        } else {
            println!("L'item {} n'est pas dans l'inventaire.", item);
        }
    }

    pub fn get_dialogues(&self) -> Vec<String> { self.dialogues.clone() }

    pub fn get_commerce_table(&self) -> HashMap<String, u32> { self.commerce_table.clone() }

    fn str_dialogues(&self) -> String {
        let mut str_dialogues = String::new();
        for i in 0..self.dialogues.len()-1 {
            str_dialogues.push_str(&self.dialogues[i].to_string());
            str_dialogues.push_str(", ");
        }
        str_dialogues.push_str(&self.dialogues[self.dialogues.len()-1].to_string());
        str_dialogues
    }

    fn str_commerce_table(&self) -> String {
        let mut str_commerce_table = String::new();
        for (key, value) in &self.commerce_table {
            str_commerce_table.push_str(&format!("{}: {}, ", key, value));
        }
        if !str_commerce_table.is_empty() {
            str_commerce_table.pop(); // Remove last space
            str_commerce_table.pop(); // Remove last space
        }
        str_commerce_table
    }

    ///////////////
    ///Fonction pour récupérer le texte d'un dialogue 
    pub fn afficher_dialogue(&self,dialogue: &mut Quete) -> String {
        dialogue.get_description().clone()
    }

    ///////////////
    /// Fonction pour mettre à jour le statut de la quête à enlever (si présente)
    fn terminer_quete_a_enlever(&self, quete: &mut Quete) {
        if let Some(dialogue_id) = quete.get_dialogue_a_enlever() {
            if let Ok(quete_a_enlever) = MasterFile::get_instance().lock().unwrap().prendre_quete_mut(&dialogue_id) {
                quete_a_enlever.set_statut(crate::quete::StatutQuete::Terminee);
            }
        }
    }

    ///////////////
    ///Fonction pour récupérer le premier dialogue qui sera jouer avec le statut EnCours et ajoute une quête à un joueur
    pub fn get_dialogue_a_jouer(&mut self, dialogues: Vec<String>) -> Option<Quete> {
        for dialogue_id in dialogues {
            let result: Result<Quete, String>;
            { result = MasterFile::get_instance().lock().unwrap().prendre_quete_id(&dialogue_id); } 
            if let Ok(mut quete) = result {
                match quete.get_statut() {
                    crate::quete::StatutQuete::EnCours => {
                        self.terminer_quete_a_enlever(&mut quete);
                        return Some(quete);
                    }
                    crate::quete::StatutQuete::NonCommencee if quete.get_quete_joueur() => {
                        { 
                            let master_file: &mut MasterFile = &mut MasterFile::get_instance().lock().unwrap();
                            let joueur = master_file.get_joueur_mut();
                            joueur.ajout_quete_joueur(&mut quete);
                        } 
                        return Some(quete);
                    }
                    _ => continue, // On continue avec le prochain dialogue
                }
            }
        }
        None // Si aucune quête n'a été trouvée, retour au menu
    }

    ////Fonction pour retirer les items acheté du pnj
    pub fn remove_item_commerce_table(&mut self, item_id: String, quantite: u32) {
        if let Some(item) = self.commerce_table.get_mut(&item_id) {
            if *item >= quantite {
                *item -= quantite;
            } else {
                panic!("Pas assez d'items");
            }
        } else {
            panic!("Pas d'items");
        }
    }
}

impl std::fmt::Display for Pnj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pnj : personnage = [{}], dialogues = [{}], commerce_table = [{}]", self.personnage, self.str_dialogues(), self.str_commerce_table())
    }
}