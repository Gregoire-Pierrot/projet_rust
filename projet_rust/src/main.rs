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

    println!();
    let mut degats: Vec<u16> =joueur.attaque(&"brise_glace".to_string());
    println!("L'attaque brise glace fait {} de dégats brutes et {} de dégats magiques ( {} dégats totaux )",degats[0],degats[1],degats[0]+degats[1]);
    println!("L'attaque fait {} de dégats au joueur",joueur.defense(&degats));
    println!();
    degats=joueur.attaque(&"tempête_feu".to_string());
    println!("L'attaque tempête de feu fait {} de dégats brutes et {} de dégats magiques ( {} dégats totaux )",degats[0],degats[1],degats[0]+degats[1]);
    println!("L'attaque fait {} de dégats au joueur",joueur.defense(&degats));



    /*println!();
    println!("Ennemie avant attribution de lieu :");
    let mut ennemie = master_file.prendre_ennemie_id("ennemie_1");
    println!("{:?}", ennemie);
    println!();
    let mut lieu = master_file.prendre_lieu_id("pièce1");
    println!("Lieu propre :");
    match &lieu {
        Ok(l) => println!("{}", l),
        Err(e) => eprintln!("Erreur lors de la récupération du lieu : {}", e),
    }
    println!();
    println!("Lieu debug :");
    println!("{:?}", lieu);
    println!();
    match (ennemie, lieu) {
        (Ok(mut en), Ok(l)) => {
            let id = en.get_id();
            if let Some(stats) = l.get_stats_ennemie(&id) {
                let personnage_modifie = Lieu::attribuer_stats_ennemie(en.get_personnage(), &stats);
                en.set_personnage(personnage_modifie);
                println!("Ennemie après attribution de lieu et de stats :");
                println!("{:?}", en);
            } else {
                eprintln!("Aucune stats trouvée dans le lieu pour l'ennemi {}", id);
            }
        }
        (Err(e), _) => eprintln!("Erreur lors de la récupération de l'ennemie : {}", e),
        (_, Err(e)) => eprintln!("Erreur lors de la récupération du lieu : {}", e),
    }*/
    
}