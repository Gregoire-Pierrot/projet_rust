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

fn main() {
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    let mut quete = master_file.prendre_quete_id("principale");

    println!("{:?}", joueur);

    suivi_quete(&mut master_file,&mut joueur,&mut quete);
    println!("{:?}", joueur);

    let mut quete = master_file.prendre_quete_id("secondaire_1");
    ajout_quete_joueur(&mut master_file,&mut joueur,&mut quete);
    println!("{:?}", joueur);

    ajout_recompense_inventaire(&mut master_file, &mut joueur, &mut quete);
    println!("{:?}", joueur);
}