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
    ///Fonction pour récupérer le premier dialogue qui sera jouer avec le statut EnCours et ajoute une quête à un joueur
    pub fn get_dialogue_a_jouer(&mut self, master_file: &MasterFile, dialogues: Vec<String>, joueur: &mut Joueur) -> Option<Quete> {
        for dialogue_id in dialogues {
            if let Ok(mut quete) = master_file.prendre_quete_id(&dialogue_id) {
                if quete.get_statut() == crate::quete::StatutQuete::EnCours {
                    return Some(quete);
                }
                else if quete.get_statut() == crate::quete::StatutQuete::NonCommencee && quete.get_quete_joueur() {
                    joueur.ajout_quete_joueur(&mut quete);
                    return None;  // Si c'est None alors faire un retour au menu
                }
            }
        }
        None // Si c'est None alors faire un retour au menu
    }
    
}

impl std::fmt::Display for Pnj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Pnj : personnage = [{}], dialogues = [{}], commerce_table = [{}]", self.personnage, self.str_dialogues(), self.str_commerce_table())
    }
}