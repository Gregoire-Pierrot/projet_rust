mod structs;
mod JsonManager;

use structs::{Joueur, Lieu};
use JsonManager::{get_joueur, prendre_lieu_id, changer_position_joueur, changer_pronom_joueur,changer_niveau_joueur,changer_nom_joueur};

fn main() {
    let mut joueur = get_joueur();
    joueur.set_position("pièce1".to_string());
    joueur.set_pronom("il".to_string());
    println!("Joueur : description = {}, nom = {}, pronom = {}, niveau = {}, position = {}", joueur.get_description(), joueur.get_nom(), joueur.get_pronom(), joueur.get_niveau(), joueur.get_position());

    match changer_nom_joueur("Jane") {
        Ok(_) => println!("Nom changé"),
        Err(e) => println!("Erreur lors de la modification du nom joueur : {}", e)
    }

    match changer_pronom_joueur("elle") {
        Ok(_) => println!("Pronom changé"),
        Err(e) => println!("Erreur lors de la modification pronom du joueur : {}", e)
    }

    match changer_niveau_joueur(&9) {
        Ok(_) => println!("Niveau augmenté"),
        Err(e) => println!("Erreur lors de la modification du niveau du joueur : {}", e)
    }
    
    match changer_position_joueur("pièce2") {
        Ok(_) => println!("Position modifiée"),
        Err(e) => println!("Erreur lors de la modification de la position du joueur : {}", e)
    }

    let joueur = get_joueur();

    println!("Joueur : description = {}, nom = {}, pronom = {}, niveau = {}, position = {}", joueur.get_description(), joueur.get_nom(), joueur.get_pronom(), joueur.get_niveau(), joueur.get_position());
}