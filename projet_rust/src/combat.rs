use crate::joueur::Joueur;
use crate::ennemie::Ennemie;
use crate::consommable::Consommable;
use crate::json_manager::MasterFile;

use std::io;
use std::io::Write;

pub fn saisir_choix(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    choix.trim().to_string()
}

/*
pub fn combat(master_file: &mut MasterFile,ennemie: &mut Ennemie, joueur: &mut Joueur){
    let reset = joueur.clone();
    let mut combat_en_cours = true;
    while combat_en_cours {
        println!("\n--- État du combat ---");
        println!("Joueur  : {} pv", joueur.get_pv_actuel());
        println!("{} : {} pv", ennemie.get_nom(), ennemie.get_pv_actuel());

        println!("\n=== Menu de Combat ===");
        println!("1. Attaque");
        println!("2. Item");
        println!("3. Fuir");

        match saisir_choix(">").as_str() {
            "1" => {
                println!("\n--- Choisissez une attaque ---");
                let attaques_ids = joueur.get_attaques();
                let mut attaques_valides = Vec::new();

                println!("{}. {}",0,"Attaque basique");
                for (i, id) in attaques_ids.iter().enumerate() {
                    match master_file.prendre_attaque_id(id) {
                        Ok(attaque) => {
                            println!("{}. {}", i + 1, attaque.get_nom());
                            attaques_valides.push(attaque);
                        }
                        Err(e) => println!("{}. Erreur: {}", i + 1, e),
                    }
                }

                let choix = saisir_choix(">");

                match choix.parse::<usize>() {
                    Ok(0) => {
                        let degats = ennemie.degats_recus_net(&joueur.get_personnage().attaque_base(master_file));
                        println!("\n--- Actions ---");
                        println!("Vous lancez l'attaque basique - {} dégâts infligés", degats);
                        
                        if !ennemie.application_degats(&degats, joueur) {
                            combat_en_cours = !ennemie.combat();
                        } else {
                            combat_en_cours = false;
                        }
                    }
                    Ok(num) if num >= 1 && num <= attaques_valides.len() => {
                        let attaque = &attaques_valides[num - 1];
                        let attaque_obj = master_file.prendre_attaque_id(&attaque.get_id()).expect("Erreur lors de la récupération de l'attaque");
                       if let Some(categorie_joueur) = joueur.get_categorie_Arme() {
                            if attaque_obj.get_categorie() != categorie_joueur {
                                println!("Vous ne possédez pas d'arme nécessaire au déclenchement de cette attaque.");
                                continue;
                            }else{
                                let degats = ennemie.degats_recus_net(&joueur.get_personnage().attaque(&attaque_obj));
                                println!("\n--- Actions ---");
                                println!("Vous lancez l'attaque : {} - {} dégâts infligés", attaque.get_nom(), degats);
                                
                                if !ennemie.application_degats(&degats, joueur) {
                                    combat_en_cours = !ennemie.combat();
                                } else {
                                    combat_en_cours = false;
                                }
                            }
                        } 
                        else {
                            println!("Vous ne possédez pas d'arme nécessaire au déclenchement de cette attaque.");
                            continue;
                        }
                    }
                    Ok(_) => continue,
                    _ => println!("Choix d'attaque invalide."),
                }
            }

            "2" => {
                println!("\n--- Choisissez un objet ---");
                let inventaire = joueur.get_inventaire();
                let mut items_valides = Vec::new();

                for (i, (id, _)) in inventaire.iter().enumerate() {
                    match master_file.prendre_consommable_id(id) {
                        Ok(item) => {
                            println!("{}. {}", i + 1, item.get_nom());
                            items_valides.push(item);
                        }
                        Err(_) => {}
                    }
                }

                let choix = saisir_choix(">");

                match choix.parse::<usize>() {
                    Ok(num) if num >= 1 && num <= items_valides.len() => {
                        let item = &items_valides[num - 1];
                        let item: Consommable = master_file.prendre_consommable_id(&item.get_id()).expect("Erreur lors de la récupération de l'objet");
                        joueur.utiliser_item(&item,&true);
                        println!("\n--- Actions ---");
                        println!("Vous utilisez l'objet : {}", item.get_nom());
                        combat_en_cours = !ennemie.combat();
                    }
                    Ok(_) => continue,
                    _ => println!("Choix d'objet invalide."),
                }
            }

            "3" => {
                println!("Vous fuyez le combat !");
                combat_en_cours = false;
            }

            _ => println!("Choix invalide, réessayez."),
        }
    }
    joueur.reset_stats(reset);
}*/