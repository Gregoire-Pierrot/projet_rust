mod structs;

use structs::{Joueur, Lieu, Meteo};

fn main() {
    println!("Début du test");

    let joueur = Joueur::new(
        "Le joueur".to_string(),
        "C'est vous !".to_string(),
        "je suis un pronom".to_string(),
        1,
        Lieu::new(
            "Salle c137".to_string(),
            "c137".to_string(),
            Vec::new(),
            Meteo::Interieur,
        ),
    );

    println!("Joueur : {:#?}", joueur);

    println!("Fin du test");
}
