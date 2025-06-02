mod structs;
mod json_manager;
mod lieu;
mod quete;
mod joueur;
mod pnj;
mod ennemie;
mod consommable;
mod equipement;
mod attaque;

use joueur::Joueur;
use pnj::Pnj;
use ennemie::Ennemie;
use lieu::Lieu;
use quete::Quete;
use consommable::Consommable;
use equipement::Equipement;
use attaque::Attaque;
use json_manager::MasterFile;

fn main() {
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    joueur.set_position("pièce1".to_string());
    joueur.set_pronom("il".to_string());
    println!("{}", joueur);

    /*match master_file.changer_nom_joueur("Jane") {
        Ok(_) => println!("Nom changé"),
        Err(e) => println!("Erreur lors de la modification du nom joueur : {}", e)
    }

    match master_file.changer_pronom_joueur("elle") {
        Ok(_) => println!("Pronom changé"),
        Err(e) => println!("Erreur lors de la modification pronom du joueur : {}", e)
    }

    match master_file.changer_niveau_joueur(&9) {
        Ok(_) => println!("Niveau augmenté"),
        Err(e) => println!("Erreur lors de la modification du niveau du joueur : {}", e)
    }
    
    match master_file.changer_position_joueur("pièce2") {
        Ok(_) => println!("Position modifiée"),
        Err(e) => println!("Erreur lors de la modification de la position du joueur : {}", e)
    }*/

    master_file.sauvegarder(&joueur);

    println!("{}", joueur);

    joueur.set_nom("Jane".to_string());
    joueur.set_pronom("elle".to_string());
    joueur.add_niveau(9);
    joueur.set_position("pièce2".to_string());

    master_file.sauvegarder(&joueur);

    println!("{}", joueur);
}