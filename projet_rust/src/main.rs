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

    println!("\n--- AFFICHAGE DE BASE ---\n");

    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    let mut quete = master_file.prendre_quete_id(&String::from("principale")).expect("Quête introuvable");
    let mut ressource = master_file.prendre_ressource_id(&String::from("clé")).expect("Ressource introuvable");
    let mut pnj = master_file.prendre_pnj_id_string(String::from("pnj_1"));
    let equipement = master_file.prendre_equipement_id("baton").unwrap();
    println!("{:?}", joueur);


    println!("\n--- TEST DES QUETES ---\n");

    // Étape 1 : Dialogue primaire
    println!("\nÉtape 1 : Dialogue primaire");
    let dialogue_primaire_id = pnj.get_dialogue_a_jouer(&mut master_file, pnj.get_dialogues(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_primaire = master_file.prendre_quete_id(&dialogue_primaire_id).expect("Dialogue primaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));

    // Étape 2 : Dialogue secondaire
    println!("\nÉtape 2 : Dialogue secondaire :");
    let dialogue_secondaire_id = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_secondaire = master_file.prendre_quete_id(&dialogue_secondaire_id).expect("Dialogue secondaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));
    
    // Étape 3 : La quete à ajouter (ajout de quete)
    println!("\nÉtape 3 : La quete à ajouter (ajout de quete) :");
    if let Some(dq) = pnj.get_dialogue_a_jouer(&mut master_file,dialogue_secondaire.get_quetes_suivantes(),&mut joueur) {
        println!("{}", joueur);
    } else {
        println!("\nAprès avoir eu un dialogue qui donne une quête : ");
        println!("{}", joueur);
    }

    println!("\nÉtape 4 dialogue mi quete :");
    //Étape 4 dialogue mi quete
    let dialogue_secondaire_id = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_secondaire = master_file.prendre_quete_id(&dialogue_secondaire_id).expect("Dialogue secondaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));

    // Étape 5 : Fin de quête
    println!("\nÉtape 5 : Fin de quête :");
    joueur.completion_quete(&mut master_file, ressource.get_id());

    // Rechargement du dialogue primaire mis à jour
    // Étape 6 :Reparler au pnj après avoir fini la quête
    println!("\nÉtape 6 : Reparler au pnj :");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));

    if let Some(autres_dialogue_secondaire) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur){
        let autres_id = autres_dialogue_secondaire.get_id();
        let mut autres = master_file.prendre_quete_id(&autres_id).expect("Autre dialogue secondaire introuvable");
        println!("{:?}", pnj.afficher_dialogue(&mut autres));
    } else {
        println!("Aucun autre dialogue secondaire trouvé.");
    }
    println!("{}", joueur);

    // Étape 7 : Reparler au pnj pour avoir le dialogue par défaut 
    println!("\nÉtape 7 : Reparler au pnj pour avoir le dialogue par défaut  :");
    if let Some(autres_dialogue_secondaire) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur){
        let autres_id = autres_dialogue_secondaire.get_id();
        let mut autres = master_file.prendre_quete_id(&autres_id).expect("Autre dialogue secondaire introuvable");
        println!("{:?}", pnj.afficher_dialogue(&mut autres));
    } else {
        println!("Aucun autre dialogue secondaire trouvé.");
    }


    println!("\n--- TEST DES EQUIPEMENTS ---\n");
    
    joueur.add_equipement(&EquipementType::Arme, &equipement.get_id().clone());
    joueur.remove_equipement(&EquipementType::Arme);
    joueur.remove_equipement(&EquipementType::Arme);
    joueur.add_equipement(&EquipementType::Arme, &equipement.get_id().clone());
    println!("\n{}", joueur);


    println!("\n--- TEST DES STATS DES ENNEMIES EN FONCTION DU LIEU ---\n");
    
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

    println!("\nEnnemie avant attribution des stats de lieu  :");
    println!("{:?}", ennemie);
    let id = ennemie.get_id();
    lieu.synchro_ennemie(&mut ennemie, &mut joueur);
    println!("\nEnnemie après attribution des stats de lieu :");
    println!("{:?}\n", ennemie);
    println!("{:?}\n", ennemie.lootable());


    println!("\n--- TEST DE L'AJOUT DE STATS AU JOUEUR ---\n");

    println!("\nAvant ajout de stats");
    println!("{:?}", &joueur);
    joueur.ajout_point_stat("pv");
    println!("\nAprès ajout de stats");
    println!("{:?}\n", &joueur);


    println!("\n--- TEST DE L'AFFICHAGE DES ITEMS DE L'INVENTAIRE (équipement,ressources et consommable) ---\n");

    println!("\nAffichage Ressources :");
    println!("{:?}", joueur.recup_ressources(&master_file));
    println!("\nAffichage Equipement :");
    println!("{:?}", joueur.recup_equipement(&master_file));
    println!("\nAffichage Consommable :");
    println!("{:?}", joueur.recup_consommable(&master_file));


    println!("\n--- TEST DU DEMANTELEMENT D'UN ITEM ---\n");

    println!("\nAvant Demantelement");
    println!("{:?}", &joueur);
    joueur.demantelement(&"clé".to_string(),&master_file);
    joueur.demantelement(&"pomme".to_string(),&master_file);
    joueur.demantelement(&"arc".to_string(),&master_file);
    println!("\nAprès Demantelement :");
    println!("{:?}", &joueur);


    println!("\n--- TEST DE LA RECOLTE D'UN ITEM DANS UN LIEU ---\n");

    println!("\nAvant Récupération item");
    println!("{:?}", &joueur);
    println!("{:?}", &lieu);
    lieu.recolter_item(("pomme").to_string(), 3, &mut joueur);

    println!("\nAprès Récupération item :");
    println!("{:?}", &joueur);
    println!("{:?}", &lieu);


    println!("\n--- TEST DU DÉPLACEMENT DU JOUEUR ---\n");

    println!("\nAvant Déplacement");
    println!("{:?}", &joueur);
    joueur.deplacement(&lieu.get_id());

    println!("\nAprès Déplacement :");
    println!("{:?}", &joueur);


    println!("\n--- TEST DU COMBAT ---\n");

    combat(&mut master_file, &mut ennemie, &mut joueur);
    


    println!("\n--- AFFICHAGE DU JOUEUR APRÈS LES MODIFICATIONS ---\n");
    println!("{}", joueur);

    //master_file.sauvegarder(&joueur);
}