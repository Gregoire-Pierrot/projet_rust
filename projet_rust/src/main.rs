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
mod combat;

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
use combat::combat;

fn main() {
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();

    println!("Avant modification :");
    println!();
    println!("{}", joueur);

    println!();
    println!("Après modification :");
    println!();

    let equipement = master_file.prendre_equipement_id("baton").unwrap();

    joueur.add_equipement(&Categorie::Arme, &equipement.get_id().clone());
    joueur.remove_equipement(&Categorie::Arme);
    joueur.remove_equipement(&Categorie::Arme);
    joueur.add_equipement(&Categorie::Arme, &equipement.get_id().clone());
    
    println!();
    println!("{}", joueur);



    //master_file.sauvegarder(&joueur);

    /*
    println!();
    let mut degats: Vec<u16> =joueur.get_personnage().attaque(&"brise_glace".to_string());
    println!("L'attaque brise glace fait {} de dégats brutes et {} de dégats magiques ( {} dégats totaux )",degats[0],degats[1],degats[0]+degats[1]);
    println!("L'attaque fait {} de dégats au joueur",joueur.get_personnage().defense(&degats));
    
    degats=joueur.get_personnage().attaque(&"tempête_feu".to_string());
    println!("L'attaque tempête de feu fait {} de dégats brutes et {} de dégats magiques ( {} dégats totaux )",degats[0],degats[1],degats[0]+degats[1]);
    println!("L'attaque fait {} de dégats au joueur",joueur.get_personnage().defense(&degats));

    let hit = joueur.get_personnage().defense(&degats);
    joueur.application_degats(hit);
    println!();
    println!("Attaque reçu :");
    println!("{}", joueur);
*/

    let mut ennemie = match master_file.prendre_ennemie_id("ennemie_1").clone() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

    let lieu = match master_file.prendre_lieu_id("pièce1") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

    println!();
    println!("Ennemie avant attribution des stats de lieu  :");
    println!("{:?}", ennemie);
    let id = ennemie.get_id();
    lieu.synchro_ennemie(&mut ennemie);
    println!();
    println!("Ennemie après attribution des stats de lieu :");
    println!("{:?}", ennemie);
    println!();
    println!("{:?}", ennemie.lootable());
    println!();

    combat(&mut master_file, &mut ennemie, &mut joueur);
    
    master_file.sauvegarder(&joueur);
}