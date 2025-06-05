use crate::json_manager::MasterFile;

use crate::joueur::Joueur;
use crate::quete::Quete;

pub fn ajout_quete_joueur(master_file: &mut MasterFile,joueur: &mut Joueur, quete: &mut Quete){
    let mut new_quete: Quete =  master_file.prendre_quete_id(&quete.get_ajout_quete()).expect("Quête introuvable");
    joueur.add_quete(new_quete.get_id());
    new_quete.set_statut(crate::quete::StatutQuete::EnCours);
    //manque juste la sauvegarde dans le fichier
}

pub fn suivi_quete(master_file: &mut MasterFile, joueur: &mut Joueur, quete: &mut Quete){ //fonction pour continuer une quête de joueur 
    let quete_suivante = quete.get_quete_suivantes();
    if quete_suivante.len() == 1 {
        joueur.remove_quete(quete.get_id());
        quete.set_statut(crate::quete::StatutQuete::Terminee);
        joueur.add_quete(quete_suivante[0].clone());
        println!("Quête terminée : [{:?}]", quete.get_statut());
    }
    //manque juste la sauvegarde dans le fichier
}

pub fn ajout_recompense_inventaire(master_file: &mut MasterFile,joueur: &mut Joueur,quete: &mut Quete){
    let recompense = quete.get_recompense();
    for (item, quantite) in recompense.iter() {
        joueur.add_inventaire(item.clone(), *quantite);
    }
    //manque juste la sauvegarde dans le fichier
}

pub fn fin_de_quete(master_file: &mut MasterFile,joueur: &mut Joueur, quete: &mut Quete){
    ajout_recompense_inventaire(master_file, joueur, quete);
    suivi_quete(master_file, joueur, quete);
    //manque juste la sauvegarde dans le fichier
}


pub fn completion_quete(master_file: &mut MasterFile,joueur: &mut Joueur, id_condition: String){
    let quetes: Vec<String> = joueur.get_quetes();
    for quete_id in quetes {
        let mut quete: Quete = master_file.prendre_quete_id(&quete_id).expect("Quête introuvable");
        if quete.find_fin_de_quete(id_condition.clone()) {
            println!("quete fini");
            fin_de_quete(master_file, joueur, &mut quete);
            break;
        }
    }
}