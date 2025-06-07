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

    // Étape 1 : Dialogue primaire
    let dialogue_primaire_id = pnj.get_dialogue_a_jouer(&mut master_file, pnj.get_dialogues(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();

    let mut dialogue_primaire = master_file.prendre_quete_id(&dialogue_primaire_id).expect("Dialogue primaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));

    // Étape 2 : Dialogue secondaire
    let dialogue_secondaire_id = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_secondaire = master_file.prendre_quete_id(&dialogue_secondaire_id).expect("Dialogue secondaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));
    
    // Étape 3 : Dernier dialogue (optionnel)
    if let Some(dq) = pnj.get_dialogue_a_jouer(&mut master_file,dialogue_secondaire.get_quetes_suivantes(),&mut joueur) {
        println!("{}", joueur);
    } else {
        println!("\nAprès avoir eu un dialogue qui donne une quête : ");
        println!("{}", joueur);
    }

    //Étape 4 dialogue mi quete
    let dialogue_secondaire_id = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_secondaire = master_file.prendre_quete_id(&dialogue_secondaire_id).expect("Dialogue secondaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));

    println!("\n--- FIN DES DIALOGUES ---\n");

    // Étape 4 : Fin de quête
    joueur.completion_quete(&mut master_file, ressource.get_id());

    // Rechargement du dialogue primaire mis à jour
    //let mut dialogue_primaire = master_file.prendre_quete_id(&dialogue_primaire_id).expect("Dialogue primaire introuvable après completion");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));

    // Étape 5 : Dialogue alternatif sur les quêtes suivantes de primaire
    if let Some(autres_dialogue_secondaire) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur){
        let autres_id = autres_dialogue_secondaire.get_id();
        let mut autres = master_file.prendre_quete_id(&autres_id).expect("Autre dialogue secondaire introuvable");
        println!("{:?}", pnj.afficher_dialogue(&mut autres));
    } else {
        println!("Aucun autre dialogue secondaire trouvé.");
    }
    println!("{}", joueur);


    if let Some(autres_dialogue_secondaire) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur){
        let autres_id = autres_dialogue_secondaire.get_id();
        let mut autres = master_file.prendre_quete_id(&autres_id).expect("Autre dialogue secondaire introuvable");
        println!("{:?}", pnj.afficher_dialogue(&mut autres));
    } else {
        println!("Aucun autre dialogue secondaire trouvé.");
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


    println!();
    println!("Avant Déplacement");
    println!("{:?}", &joueur);
    joueur.deplacement(&lieu.get_id());

    println!();
    println!("Après Déplacement :");
    println!("{:?}", &joueur);


    combat(&mut master_file, &mut ennemie, &mut joueur);
    
    println!("{}", joueur);

    //master_file.sauvegarder(&joueur);
}