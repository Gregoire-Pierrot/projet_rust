mod structs;
mod json_manager;
mod lieu;
mod joueur;
mod pnj;
mod ennemie;
mod consommable;
mod equipement;
mod deplacement;

use crate::deplacement::deplacement;
use lieu::Lieu;
use joueur::Joueur;
use pnj::Pnj;
use ennemie::Ennemie;
use consommable::Consommable;
use equipement::Equipement;
use json_manager::MasterFile;

fn main() {
    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();

    println!("{}",deplacement(&mut master_file,&mut joueur,"piece3"));
}