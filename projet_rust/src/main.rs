mod structs;
mod json_manager;
mod lieu;
mod joueur;
mod pnj;
mod ennemie;
mod consommable;
mod equipement;
mod quete;
mod action_quetes;
mod action_pnj;

use lieu::Lieu;
use joueur::Joueur;
use pnj::Pnj;
use ennemie::Ennemie;
use consommable::Consommable;
use equipement::Equipement;
use json_manager::MasterFile;
use quete::Quete;
use action_quetes::suivi_quete;
use action_quetes::ajout_recompense_inventaire;
use action_quetes::ajout_quete_joueur;
use action_quetes::fin_de_quete;
use action_quetes::completion_quete;

use action_pnj::get_dialogue_primaire;
use action_pnj::afficher_dialogue;
use action_pnj::get_id_dialogues_joueur;
use action_pnj::get_reponse_dialogue_pnj;

fn main() {
    println!();
    println!("Affichage de base :");
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();

    let mut pnj = master_file.prendre_pnj_id_string(String::from("pnj_1"));

    println!("{:?}", pnj);

    let mut dialogue_primaire: Quete = get_dialogue_primaire(&mut master_file,&mut pnj);

    println!("{:?}", afficher_dialogue(&mut dialogue_primaire));

    for vector in get_id_dialogues_joueur(&mut master_file, &mut dialogue_primaire) {
        let quete = master_file.prendre_quete_id(vector.clone());
        println!("{:?}", afficher_dialogue(&mut quete.clone()));

        let reponse_id = get_reponse_dialogue_pnj(&mut master_file, vector.clone());
        
        if(reponse_id != String::new()){
            let mut dialogue_reponse = master_file.prendre_quete_id(reponse_id);
            println!("{:?}", afficher_dialogue(&mut dialogue_reponse));
        }
        
    }


    
    

    /*let mut quete = master_file.prendre_quete_id(String::from("principale"));
    println!("{:?}", joueur);


    let mut ressource = master_file.prendre_ressource_id(String::from("clé"));
    completion_quete(&mut master_file,&mut joueur, ressource.get_id());

    println!();
    println!("Affichage de la fin d'une quête :");
    println!("{:?}", joueur);*/

    /*
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

    
}