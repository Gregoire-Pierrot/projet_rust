use crate::json_manager::MasterFile;
use crate::joueur::Joueur;

pub fn deplacement(master_file: &mut MasterFile,joueur: &mut Joueur,destination: &str) -> String {
    joueur.set_position(destination.to_string());
    master_file.sauvegarder(&joueur);
    let texte = "Vous vous êtes déplacé vers ".to_owned()+&destination.to_string();
    texte
}