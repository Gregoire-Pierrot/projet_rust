mod structs;
mod json_manager;
mod lieu;
mod quete;
mod joueur;
mod pnj;
mod ennemie;
mod consommable;
mod equipement;
mod action_quetes;
mod attaque;
mod combat;
mod action_pnj;

use joueur::Joueur;
use pnj::Pnj;
use ennemie::Ennemie;
use lieu::Lieu;
use quete::Quete;
use consommable::Consommable;
use equipement::Equipement;
use structs::EquipementType;
use attaque::Attaque;
use json_manager::MasterFile;
use crate::action_quetes::{completion_quete};
use combat::combat;
use crate::action_pnj::{get_dialogue_primaire,afficher_dialogue,get_id_dialogues_joueur,get_reponse_dialogue_pnj};

fn main() {
    println!();
    println!("Affichage de base :");
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    let mut quete = master_file.prendre_quete_id(&String::from("principale")).expect("Quête introuvable");
    println!("{:?}", joueur);


    let mut ressource = master_file.prendre_ressource_id(&String::from("clé")).expect("Ressource introuvable");
    completion_quete(&mut master_file,&mut joueur, ressource.get_id());

    println!();
    println!("Affichage de la fin d'une quête :");
    println!("{:?}", joueur);    

    println!("Avant modification :");
    println!();
    println!("{}", joueur);

    println!();
    println!("Après modification :");
    println!();

    let equipement = master_file.prendre_equipement_id("baton").unwrap();

    joueur.add_equipement(&EquipementType::Arme, &equipement.get_id().clone());
    joueur.remove_equipement(&EquipementType::Arme);
    joueur.remove_equipement(&EquipementType::Arme);
    joueur.add_equipement(&EquipementType::Arme, &equipement.get_id().clone());
    
    println!();
    println!("{}", joueur);


    let mut pnj = master_file.prendre_pnj_id_string(String::from("pnj_1"));
    println!("{:?}", pnj);
    let mut dialogue_primaire: Quete = get_dialogue_primaire(&mut master_file,&mut pnj);
    println!("{:?}", afficher_dialogue(&mut dialogue_primaire));
    for vector in get_id_dialogues_joueur(&mut master_file, &mut dialogue_primaire) {
        let mut quete = master_file.prendre_quete_id(&vector.clone()).expect("Failed to get quete");
        println!("{:?}", afficher_dialogue(&mut quete));
        let reponse_id = get_reponse_dialogue_pnj(&mut master_file, vector.clone());
        if reponse_id != String::new(){
            let mut dialogue_reponse = master_file.prendre_quete_id(&reponse_id).expect("Failed to get dialogue response");
            println!("{:?}", afficher_dialogue(&mut dialogue_reponse));
        }
    }

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


    println!();
    println!("Avant ajout de stats");
    println!("{:?}", &joueur);
    joueur.ajout_point_stat("pv");
    println!();
    println!("Après ajout de stats");
    println!("{:?}", &joueur);
    println!();

    println!();
    println!("Affichage Ressources :");
    println!("{:?}", joueur.recup_ressources(&master_file));

    println!();
    println!("Affichage Equipement :");
    println!("{:?}", joueur.recup_equipement(&master_file));


    println!();
    println!("Affichage Consommable :");
    println!("{:?}", joueur.recup_consommable(&master_file));



    combat(&mut master_file, &mut ennemie, &mut joueur);
    
    println!("{}", joueur);

    //master_file.sauvegarder(&joueur);
}