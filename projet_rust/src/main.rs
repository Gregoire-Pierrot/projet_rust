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
use structs::EquipementType;
use attaque::Attaque;
use json_manager::MasterFile;
use combat::combat;

fn main() {
    println!();
    println!("Affichage de base :");
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    let mut quete = master_file.prendre_quete_id(&String::from("principale")).expect("Quête introuvable");
    println!("{:?}", joueur);


    let mut ressource = master_file.prendre_ressource_id(&String::from("clé")).expect("Ressource introuvable");
    joueur.completion_quete(&mut master_file,ressource.get_id());

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
    let mut dialogue_primaire: Quete = pnj.get_dialogue_a_jouer(&mut master_file, pnj.get_dialogues(), &mut joueur).expect("Aucun dialogue trouvé");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));
    let mut dialogue_secondaire: Quete = pnj.get_dialogue_a_jouer(&mut master_file,dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));
    
    
    if let Some(mut dialogue_donne_quete) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_secondaire.get_quetes_suivantes(), &mut joueur) {
        println!("{}", joueur);
    } else {  // IMPORTANT le dialogue sera ajouter à NONE donc la suite sera dans le else si le dialogue est le dernier du pnj
        println!();
        println!("Après avoir eu un dialogue qui donne une quête : ");
        println!("{}", joueur);
    }


    let mut ennemie = match master_file.prendre_ennemie_id("ennemie_1").clone() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

    let mut lieu = match master_file.prendre_lieu_id("pièce1") {
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


    println!();
    println!("Avant Demantelement");
    println!("{:?}", &joueur);
    joueur.demantelement(&"clé".to_string(),&master_file);
    joueur.demantelement(&"pomme".to_string(),&master_file);
    joueur.demantelement(&"arc".to_string(),&master_file);
    println!();
    println!("Après Demantelement :");
    println!("{:?}", &joueur);


    println!();
    println!("Avant Récupération item");
    println!("{:?}", &joueur);
    println!("{:?}", &lieu);
    lieu.recolter_item(("pomme").to_string(), 3, &mut joueur);

    println!();
    println!("Après Récupération item :");
    println!("{:?}", &joueur);
    println!("{:?}", &lieu);


    combat(&mut master_file, &mut ennemie, &mut joueur);
    
    println!("{}", joueur);

    //master_file.sauvegarder(&joueur);
}