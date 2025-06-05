mod structs;
mod json_manager;
mod lieu;
mod quete;
mod joueur;
mod pnj;
mod ennemie;
mod consommable;
mod equipement;
mod quete;
mod action_quetes;
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
use quete::Quete;
use action_quetes::suivi_quete;
use action_quetes::ajout_recompense_inventaire;
use action_quetes::ajout_quete_joueur;
use action_quetes::fin_de_quete;
use action_quetes::completion_quete;
use crate::structs::Ressource;
use combat::combat;

fn main() {
    println!();
    println!("Affichage de base :");
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    /*let mut quete = master_file.prendre_quete_id(String::from("principale"));
    println!("{:?}", joueur);


    let mut ressource = master_file.prendre_ressource_id(String::from("clé"));
    completion_quete(&mut master_file,&mut joueur, ressource.get_id());

    println!();
    println!("Affichage de la fin d'une quête :");
    println!("{:?}", joueur);

    
    fin_de_quete(&mut master_file,&mut joueur,&mut quete);
    println!();
    println!("Affichage de la fin d'une quête :");
    println!("{:?}", joueur);

    let mut quete = master_file.prendre_quete_id(String::from("secondaire_1"));
    ajout_quete_joueur(&mut master_file,&mut joueur,&mut quete);
    println!();
    println!("Affichage de l'ajout d'une quête :");
    println!("{:?}", joueur);
    */

    

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
    lieu.synchro_ennemie(&mut ennemie, &mut joueur);
    println!();
    println!("Ennemie après attribution des stats de lieu :");
    println!("{:?}", ennemie);
    println!();
    println!("{:?}", ennemie.lootable());
    println!();

    combat(&mut master_file, &mut ennemie, &mut joueur);
    
    println!("{}", joueur);

    //master_file.sauvegarder(&joueur);
}