use crate::json_manager::MasterFile;

use crate::joueur::Joueur;
use crate::quete::Quete;


pub fn ajout_quetes(master_file: &mut MasterFile,joueur: &mut Joueur){
    
}

pub fn suivi_quete(master_file: &mut MasterFile,joueur: &mut Joueur,quete: &mut Quete){
    let quete_suivante = quete.get_quete_suivante();
    joueur.remove_quete(quete.get_id());
    joueur.add_quete(quete_suivante);
    //manque juste la sauvegarde dans le fichier
}
