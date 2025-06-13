mod structs;
mod json_manager;
mod lieu;
mod quete;
mod joueur;
mod pnj;
mod ennemie;
mod consommable;
mod equipement;
mod parchemin;
mod attaque;
mod interface;

use joueur::Joueur;
use pnj::Pnj;
use ennemie::Ennemie;
use lieu::Lieu;
use quete::Quete;
use consommable::Consommable;
use equipement::{Equipement, Categorie, Arme, Armure};
use structs::{EquipementType, Ressource};
use parchemin::Parchemin;
use attaque::Attaque;
use json_manager::{MasterFile, Item};

use cursive::{Cursive, CursiveExt};
use interface::{main_menu_screen};

use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut siv = Cursive::new();
    
    siv.add_layer(main_menu_screen());
    siv.run();
}