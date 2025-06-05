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

   /* println!("Avant modification :");
    println!();
    println!("{}", joueur);

    println!();
    println!("Après modification :");
    println!();

    let ressource = Ressource::new("epee1".to_string(), "Une épée en fer".to_string(),"epée de fer".to_string(), 100, 0.09, vec!["fer".to_string()]);
    let equipement = Equipement::new(ressource, 0, 0, 100, 0, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, "Arme".to_string());
    joueur.add_equipement(&Categorie::Arme, &equipement.get_id().clone());

    joueur.remove_equipement(&Categorie::Arme);
    joueur.remove_equipement(&Categorie::Arme);
    joueur.add_equipement(&Categorie::Arme, &equipement.get_id().clone());
    
    println!();
    println!("{}", joueur);
*/
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

    let mut ennemie = match master_file.prendre_ennemie_id("ennemie_1") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

/*    let lieu = match master_file.prendre_lieu_id("pièce1") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

    let id = ennemie.get_id();
    if let Some(stats) = lieu.get_stats_ennemie(&id) {
        lieu.synchro_ennemie(&mut ennemie);
        println!("Ennemie après attribution de lieu et de stats :");
        println!("{:?}", ennemie);
        println!();
        println!("{:?}", ennemie.lootable());
    } else {
        eprintln!("Aucune stats trouvée dans le lieu pour l'ennemi {}", id);
    }*/

    combat(&mut master_file, &mut ennemie, &mut joueur);

}