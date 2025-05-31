use crate::json_manager::MasterFile;

use crate::joueur::Joueur;
use crate::quete::Quete;
use crate::structs::Ressource;

pub fn ajout_quete_joueur(master_file: &mut MasterFile,joueur: &mut Joueur, quete: &mut Quete){
    joueur.add_quete(quete.get_id());
}

pub fn suivi_quete(master_file: &mut MasterFile,joueur: &mut Joueur,quete: &mut Quete){
    let quete_suivante = quete.get_quete_suivante();
    joueur.remove_quete(quete.get_id());
    joueur.add_quete(quete_suivante);
    //manque juste la sauvegarde dans le fichier
}


pub fn ajout_recompense_inventaire(master_file: &mut MasterFile,joueur: &mut Joueur,quete: &mut Quete){
    let recompense = quete.get_recompense();
    for (item, quantite) in recompense.iter() {
        joueur.add_item_inventaire(item.clone(), *quantite);
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
        let mut quete: Quete = master_file.prendre_quete_id(quete_id);
        if quete.find_fin_de_quete(id_condition.clone()) {
            println!("quete fini");
            fin_de_quete(master_file, joueur, &mut quete);
            break;
        }
    }
}