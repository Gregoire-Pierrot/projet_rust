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
use equipement::Categorie;
use attaque::Attaque;
use json_manager::MasterFile;
use crate::structs::Ressource;

fn main() {
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();

    println!("Avant modification :");
    println!();
    println!("{}", joueur);

    println!();
    println!("Après modification :");
    println!();

    let ressource = Ressource::new("epee1".to_string(), "Une épée en fer".to_string(),"epée de fer".to_string(), 100, vec!["fer".to_string()]);
    let equipement = Equipement::new(ressource, 0, 0, 100, 0, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "Arme".to_string());
    joueur.add_equipement(&Categorie::Arme, &equipement.get_id().clone());

    joueur.remove_equipement(&Categorie::Arme);
    joueur.remove_equipement(&Categorie::Arme);
    joueur.add_equipement(&Categorie::Arme, &equipement.get_id().clone());
    
    println!();
    println!("{}", joueur);
    //master_file.sauvegarder(&joueur);
}