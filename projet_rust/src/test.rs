use serde::{Serialize, Deserialize};

#[derive(Deserialize)]

struct Joueur {
    niveau: char,
    pronom: char,
    position:char
}

fn main(){
    let data = fs::read_to_string("masterFile.json");
    let joueur: Joueur = serde_json::from_str(&data).unwrap();
    println!("joueur = {:?}",joueur);
}