mod structs;
mod JsonManager;

use structs::{Joueur, Lieu};
use JsonManager::{get_joueur, prendre_lieu_id, changer_position_joueur};

fn main() {
    let joueur = get_joueur();
    println!("Joueur : description = {}, nom = {}, pronom = {}, niveau = {}, position = {}", joueur.get_description(), joueur.get_nom(), joueur.get_pronom(), joueur.get_niveau(), joueur.get_position());

    match changer_position_joueur("pièce2") {
        Ok(_) => println!("Position modifiée"),
        Err(e) => println!("Erreur lors de la modification de la position du joueur : {}", e)
    }
}