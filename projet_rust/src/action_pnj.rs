use crate::json_manager::MasterFile;

use crate::joueur::Joueur;
use crate::quete::Quete;
use crate::structs::Ressource;
use crate::pnj::Pnj;
    
pub fn afficher_dialogue(dialogue: &mut Quete) -> String { // fonctions pour récupérer le texte d'un dialogue
    dialogue.get_description().clone()
}

pub fn get_dialogue_primaire(master_file: &mut MasterFile,pnj: &mut Pnj) -> Quete {// fonctions pour récupérer le dialogue primaire à afficher en premier
    let dialogue_primaire: Quete = master_file.prendre_quete_id(&pnj.get_dialogues()[0].clone()).expect("REASON");;
    dialogue_primaire
}

pub fn get_id_dialogues_joueur(master_file: &mut MasterFile,dialogue: &mut Quete) -> Vec<String> { //fonctions pour récupérer les id des dialogues de réponses du joueur à un dialogue du pnj
    dialogue.get_quete_suivantes().clone()
}

pub fn get_reponse_dialogue_pnj(master_file: &mut MasterFile,dialogue_joueur_id: String) -> String {// fonctions pour récupérer la réponse du pnj à un dialogue du joueur
    let dialogue_joueur: Quete = master_file.prendre_quete_id(&dialogue_joueur_id).expect("REASON");;
    let dialogues :Vec<String> = dialogue_joueur.get_quete_suivantes();
    if dialogues.len()>0 {
        return dialogue_joueur.get_quete_suivantes()[0].clone(); // à modifier plus tard mais pour l'instant un dialogue joueur n'a qu'une réponse possible
    } 
    return String::new(); // si pas de réponse possible
}