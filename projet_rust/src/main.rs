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
mod combat;

use joueur::Joueur;
use pnj::Pnj;
use ennemie::Ennemie;
use lieu::Lieu;
use quete::Quete;
use consommable::Consommable;
use equipement::{Equipement, Categorie, Arme, Armure};
use structs::{EquipementType, Ressource};
use attaque::Attaque;
use json_manager::{MasterFile, Item};
//use combat::combat;

use cursive::views::{Dialog, TextView, SelectView, LinearLayout, ScrollView, DummyView};
use cursive::view::{Resizable};
use cursive::{Cursive, CursiveExt};

use std::collections::HashMap;
use rand::Rng;

fn main() {

    /*
    println!("\n--- AFFICHAGE DE BASE ---\n");

    let mut master_file = MasterFile::new();
    let mut joueur = master_file.get_joueur();
    let mut quete = master_file.prendre_quete_id(&String::from("principale")).expect("Quête introuvable");
    let mut ressource = master_file.prendre_ressource_id(&String::from("clé")).expect("Ressource introuvable");
    let mut pnj = master_file.prendre_pnj_id_string(String::from("pnj_1"));
    let equipement = master_file.prendre_equipement_id("baton").unwrap();
    println!("{}", joueur);


    println!("\n--- TEST DES QUETES ---\n");

    // Étape 1 : Dialogue primaire
    println!("\nÉtape 1 : Dialogue primaire");
    let dialogue_primaire_id = pnj.get_dialogue_a_jouer(&mut master_file, pnj.get_dialogues(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_primaire = master_file.prendre_quete_id(&dialogue_primaire_id).expect("Dialogue primaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));

    // Étape 2 : Dialogue secondaire
    println!("\nÉtape 2 : Dialogue secondaire :");
    let dialogue_secondaire_id = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_secondaire = master_file.prendre_quete_id(&dialogue_secondaire_id).expect("Dialogue secondaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));
    
    // Étape 3 : La quete à ajouter (ajout de quete)
    println!("\nÉtape 3 : La quete à ajouter (ajout de quete) :");
    if let Some(dq) = pnj.get_dialogue_a_jouer(&mut master_file,dialogue_secondaire.get_quetes_suivantes(),&mut joueur) {
        println!("{}", joueur);
    } else {
        println!("\nAprès avoir eu un dialogue qui donne une quête : ");
        println!("{}", joueur);
    }

    println!("\nÉtape 4 dialogue mi quete :");
    //Étape 4 dialogue mi quete
    let dialogue_secondaire_id = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur).expect("Aucun dialogue trouvé").get_id().to_string();
    let mut dialogue_secondaire = master_file.prendre_quete_id(&dialogue_secondaire_id).expect("Dialogue secondaire introuvable");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_secondaire));

    // Étape 5 : Fin de quête
    println!("\nÉtape 5 : Fin de quête :");
    joueur.completion_quete(&mut master_file, ressource.get_id());
    println!("{}", joueur);

    // Rechargement du dialogue primaire mis à jour
    // Étape 6 :Reparler au pnj après avoir fini la quête
    println!("\nÉtape 6 : Reparler au pnj :");
    println!("{:?}", pnj.afficher_dialogue(&mut dialogue_primaire));

    if let Some(autres_dialogue_secondaire) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur){
        let autres_id = autres_dialogue_secondaire.get_id();
        let mut autres = master_file.prendre_quete_id(&autres_id).expect("Autre dialogue secondaire introuvable");
        println!("{:?}", pnj.afficher_dialogue(&mut autres));
    } else {
        println!("Aucun autre dialogue secondaire trouvé.");
    }
    println!("{}", joueur);

    // Étape 7 : Reparler au pnj pour avoir le dialogue par défaut 
    println!("\nÉtape 7 : Reparler au pnj pour avoir le dialogue par défaut  :");
    if let Some(autres_dialogue_secondaire) = pnj.get_dialogue_a_jouer(&mut master_file, dialogue_primaire.get_quetes_suivantes(), &mut joueur){
        let autres_id = autres_dialogue_secondaire.get_id();
        let mut autres = master_file.prendre_quete_id(&autres_id).expect("Autre dialogue secondaire introuvable");
        println!("{:?}", pnj.afficher_dialogue(&mut autres));
    } else {
        println!("Aucun autre dialogue secondaire trouvé.");
    }


    println!("\n--- TEST DES EQUIPEMENTS ---\n");
    
    joueur.add_equipement(&EquipementType::Arme, &equipement.get_id().clone());
    joueur.remove_equipement(&EquipementType::Arme);
    joueur.remove_equipement(&EquipementType::Arme);
    joueur.add_equipement(&EquipementType::Arme, &equipement.get_id().clone());
    println!("\n{}", joueur);


    println!("\n--- TEST DES STATS DES ENNEMIES EN FONCTION DU LIEU ---\n");
    
    let mut ennemie = match master_file.prendre_ennemie_id("ennemie_1").clone() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

    let mut lieu = match master_file.prendre_lieu_id("pièce1") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };

    println!("\nEnnemie avant attribution des stats de lieu  :");
    println!("{:?}", ennemie);
    let id = ennemie.get_id();
    lieu.synchro_ennemie(&mut ennemie, &mut joueur);
    println!("\nEnnemie après attribution des stats de lieu :");
    println!("{:?}\n", ennemie);
    println!("{:?}\n", ennemie.lootable());


    println!("\n--- TEST DE L'AJOUT DE STATS AU JOUEUR ---\n");

    println!("\nAvant ajout de stats");
    println!("{:?}", &joueur);
    joueur.ajout_point_stat("pv");
    println!("\nAprès ajout de stats");
    println!("{:?}\n", &joueur);


    println!("\n--- TEST DE L'AFFICHAGE DES ITEMS DE L'INVENTAIRE (équipement,ressources et consommable) ---\n");

    println!("\nAffichage Ressources :");
    println!("{:?}", joueur.recup_ressources(&master_file));
    println!("\nAffichage Equipement :");
    println!("{:?}", joueur.recup_equipement(&master_file));
    println!("\nAffichage Consommable :");
    println!("{:?}", joueur.recup_consommable(&master_file));


    println!("\n--- TEST DU DEMANTELEMENT D'UN ITEM ---\n");

    println!("\nAvant Demantelement");
    println!("{:?}", &joueur);
    joueur.demantelement(&"clé".to_string(),&master_file);
    joueur.demantelement(&"pomme".to_string(),&master_file);
    joueur.demantelement(&"arc".to_string(),&master_file);
    println!("\nAprès Demantelement :");
    println!("{:?}", &joueur);


    println!("\n--- TEST DE LA RECOLTE D'UN ITEM DANS UN LIEU ---\n");

    println!("\nAvant Récupération item");
    println!("{:?}", &joueur);
    println!("{:?}", &lieu);
    lieu.recolter_item(("pomme").to_string(), 3, &mut joueur);

    println!("\nAprès Récupération item :");
    println!("{:?}", &joueur);
    println!("{:?}", &lieu);


    println!("\n--- TEST DU DÉPLACEMENT DU JOUEUR ---\n");

    println!("\nAvant Déplacement");
    println!("{:?}", &joueur);
    joueur.deplacement(&lieu.get_id());

    println!("\nAprès Déplacement :");
    println!("{:?}", &joueur);


    println!("\n--- TEST DU COMBAT ---\n");

    combat(&mut master_file, &mut ennemie, &mut joueur);
    


    println!("\n--- AFFICHAGE DU JOUEUR APRÈS LES MODIFICATIONS ---\n");
    println!("{}", joueur);

    //master_file.sauvegarder(&joueur);

    */

    let mut siv = Cursive::new();

    // Créez un écran de menu principal
    fn main_menu_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Jouer", 1)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(Dialog::text("Chargement du jeu... Veuillez patienter").title("Chargement"));
                    // TODO: *chargement du jeu et vérification du json
                    s.pop_layer();
                    s.add_layer(game_screen());
                }
            })
        )
        .title("Menu principal")
        .button("Quitter", |s| s.quit())
    }

    // Créez un écran de jeu
    fn game_screen() -> Dialog {
        let mut layout = LinearLayout::vertical();
        let mut layout_player_infos = LinearLayout::horizontal();
        { layout_player_infos.add_child(TextView::new(MasterFile::get_instance().lock().unwrap().get_joueur().get_nom().to_string() + ",")) }
        layout_player_infos.add_child(DummyView::new().fixed_width(1));
        { layout_player_infos.add_child(TextView::new("nv:".to_string() + &MasterFile::get_instance().lock().unwrap().get_joueur().get_niveau().to_string() + ",")) }
        layout_player_infos.add_child(DummyView::new().fixed_width(1));
        {
            let position_id: String;
            { position_id = MasterFile::get_instance().lock().unwrap().get_joueur().get_position(); }
            let position_nom = MasterFile::get_instance().lock().unwrap().prendre_lieu_id(&position_id).expect("Lieu introuvable").get_nom();
            layout_player_infos.add_child(TextView::new("lieu:".to_string() + &position_nom + ","));
        }
        layout_player_infos.add_child(DummyView::new().fixed_width(1));
        {
            let pv_actuel: String;
            let pv_max: String;
            { pv_actuel = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_actuel().to_string(); }
            { pv_max = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_max().to_string(); }
            layout_player_infos.add_child(TextView::new("pv:".to_string() + &pv_actuel + "/" + &pv_max));
        }
        let select = SelectView::new()
            .item("Carte", 1)
            .item("Actions", 2)
            .item("Personnage", 3)
            .item("Menu", 4)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(move_screen());
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(actions_screen());
                }
                else if *choice == 3 {
                    s.pop_layer();
                    s.add_layer(personnage_screen());
                }
                else if *choice == 4 {
                    s.pop_layer();
                    s.add_layer(menu_screen());
                }
            });
        layout.add_child(layout_player_infos);
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(select);
        Dialog::around(layout)
            .title("Jeu")
    }

    // Créer un écran de déplacement
    fn move_screen() -> Dialog {
        let mut lieux: Vec<Lieu> = vec![];
        let position_id: String;
        { position_id = MasterFile::get_instance().lock().unwrap().get_joueur_mut().get_position(); }
        let mut destinations: Vec<String> = Vec::new();
        { destinations = MasterFile::get_instance().lock().unwrap().prendre_lieu_id(&position_id).expect("Lieu introuvable").get_destinations(); }
        {
            for lieu_id in destinations {
                lieux.push(MasterFile::get_instance().lock().unwrap().prendre_lieu_id(&lieu_id).expect("Lieu introuvable"));
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..lieux.len() + 1 {
            let lieu = &lieux[i - 1];
            select.add_item(lieu.get_nom(), i.to_string());
            if i % 10 == 0 {
                let lieux_clone = lieux.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let lieu = lieux_clone[index].clone();
                    s.add_layer(create_dialog_lieu(lieu));
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if lieux.len() % 10 != 0 {
            let lieux_clone = lieux.clone();
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let lieu = lieux_clone[index].clone();
                s.add_layer(create_dialog_lieu(lieu));
            });
            layout.add_child(select);
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Carte")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(game_screen());
        })
    }

    fn create_dialog_lieu(lieu: Lieu) -> Dialog {
        let lieu_clone = lieu.clone();
        Dialog::around(LinearLayout::vertical()
            .child(TextView::new(lieu.get_description()))
            .child(DummyView::new().fixed_height(1))
            .child(SelectView::new()
                .item("S'y rendre", 1)
                .on_submit(move |s, choice| {
                    if *choice == 1 {
                        MasterFile::get_instance().lock().unwrap().get_joueur_mut().set_position(lieu.get_id());
                        s.pop_layer();
                        s.pop_layer();
                        s.add_layer(game_screen());
                    }
                }))
        )
        .title(lieu_clone.get_nom())
        .button("Retour", |s| {
            s.pop_layer();
        })
    }

    // Créer un écran d'intéraction
    fn actions_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Analyser la zone", 1)
            .item("Récolter", 2)
            .item("PNJ", 3)
            .item("Combattre", 4)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(analyse_screen());
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(recolter_screen());
                }
                else if *choice == 3 {
                    s.pop_layer();
                    s.add_layer(pnj_screen());
                }
                else if *choice == 4 {
                    s.pop_layer();
                    //s.add_layer(combat_screen());
                    s.add_layer(Dialog::text("TODO: Combat").title("Combat").button("Retour", |s| { s.pop_layer(); }));
                }
            })
        )
        .title("Actions")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(game_screen());
        })
    }

    // Créer un écran d'analyse de zone
    fn analyse_screen() -> Dialog {
        let lieu: Lieu;
        {
            let lieu_id;
            { lieu_id = MasterFile::get_instance().lock().unwrap().get_joueur().get_position(); }
            lieu = MasterFile::get_instance().lock().unwrap().prendre_lieu_id(&lieu_id).expect("Lieu introuvable");
        }
        let mut layout = LinearLayout::vertical();
        layout.add_child(TextView::new(lieu.get_nom()));
        layout.add_child(TextView::new(lieu.get_description().clone()));
        let mut layout_ressources = LinearLayout::vertical();
        for (ressource_id, quantite) in lieu.get_contient_ressources() {
            let consommable: Result<Consommable, String>;
            { consommable = MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&ressource_id); }
            if consommable.is_ok() {
                layout_ressources.add_child(TextView::new(consommable.unwrap().get_nom().clone() + " : " + &quantite.to_string()));
            }
            let ressource: Result<Ressource, String>;
            { ressource = MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&ressource_id); }
            if ressource.is_ok() {
                layout_ressources.add_child(TextView::new(ressource.unwrap().get_nom().clone() + " : " + &quantite.to_string()));
            }
            let equipement: Result<Equipement, String>;
            { equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&ressource_id); }
            if equipement.is_ok() {
                layout_ressources.add_child(TextView::new(equipement.unwrap().get_nom().clone() + " : " + &quantite.to_string()));
            }
        }
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(LinearLayout::vertical()
            .child(TextView::new("Ressources :"))
            .child(LinearLayout::horizontal()
                .child(DummyView::new().fixed_width(2))
                .child(layout_ressources)
            )
        );
        let mut select_pnj = SelectView::new();
        for pnj_id in lieu.get_contient_pnj() {
            let pnj: Result<Pnj, String>;
            { pnj = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_id); }
            if pnj.is_ok() {
                select_pnj.add_item(pnj.clone().unwrap().get_nom().clone(), pnj.unwrap().get_id().clone());
            }
        }
        select_pnj.set_inactive_highlight(false);
        select_pnj.set_on_submit(|s, choice: &String| {
            let pnj: Pnj;
            { pnj = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(choice).unwrap(); }
            s.add_layer(create_dialog_pnj(pnj))
        });
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(LinearLayout::vertical()
            .child(TextView::new("PNJs :"))
            .child(LinearLayout::horizontal()
                .child(DummyView::new().fixed_width(2))
                .child(select_pnj)
            )
        );
        let mut layout_ennemies = LinearLayout::vertical();
        for (ennemie_id, nvs) in lieu.get_contient_ennemies() {
            let ennemie: Result<Ennemie, String>;
            { ennemie = MasterFile::get_instance().lock().unwrap().prendre_ennemie_id(&ennemie_id); }
            if ennemie.is_ok() {
                let mut nvs_string: String = "niveaux: [".to_string();
                for i in 0..nvs.len() - 1 {
                    nvs_string += &(nvs[i].to_string() + ", ");
                }
                nvs_string += &(nvs[nvs.len() - 1].to_string() + "]");
                layout_ennemies.add_child(TextView::new(ennemie.unwrap().get_nom().clone() + " : " + &nvs_string));
            }
        }
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(LinearLayout::vertical()
            .child(TextView::new("Ennemis :"))
            .child(LinearLayout::horizontal()
                .child(DummyView::new().fixed_width(2))
                .child(layout_ennemies)
            )
        );
        Dialog::around(layout)
        .title("Analyse de zone")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(actions_screen());
        })
    }

    fn create_dialog_pnj(pnj: Pnj) -> Dialog {
        let is_commercant: bool = pnj.get_commerce_table().len() != 0;
        let commercant_string: String = if is_commercant { "Oui".to_string() } else { "Non".to_string() };
        let mut layout = LinearLayout::vertical();
        layout.add_child(TextView::new(pnj.get_description().clone()));
        layout.add_child(TextView::new("Commercant : ".to_string() + &commercant_string));
        Dialog::around(layout)
            .title(pnj.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de récolte
    fn recolter_screen() -> Dialog {
        let ressources: HashMap<String, u32>;
        {
             let lieu_id: String;
             { lieu_id = MasterFile::get_instance().lock().unwrap().get_joueur_mut().get_position(); }
             ressources = MasterFile::get_instance().lock().unwrap().prendre_lieu_id(&lieu_id).expect("Lieu introuvable").get_contient_ressources();
        }
        let ressources_clone = ressources.clone();
        let mut layout = LinearLayout::vertical();
        let mut select = SelectView::new();
        for (ressource_id, quantite) in &ressources {
            let consommable: Result<Consommable, String>;
            { consommable = MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&ressource_id); }
            if consommable.is_ok() {
                select.add_item(consommable.unwrap().get_nom().clone() + " : " + &quantite.to_string(), ressource_id.clone());
            }
            else {
                let ressource: Result<Ressource, String>;
                { ressource = MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&ressource_id); }
                if ressource.is_ok() {
                    select.add_item(ressource.unwrap().get_nom().clone() + " : " + &quantite.to_string(), ressource_id.clone());
                }
                else {
                    let equipement: Result<Equipement, String>;
                    { equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&ressource_id); }
                    if equipement.is_ok() {
                        select.add_item(equipement.unwrap().get_nom().clone() + " : " + &quantite.to_string(), ressource_id.clone());
                    }
                }
            }
        }
        select.set_on_submit(move |s, choice: &String| {
            let ressource_id: String = choice.clone();
            let value = ressources.clone();
            s.add_layer(Dialog::text("Voulez vous récolter cette ressource ?")
                .title("Informations")
                .button("Oui", move |s| {
                    let quantite: u32;
                    {
                        quantite = value.get(&ressource_id).unwrap().clone();
                        MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_inventaire(ressource_id.to_string(), quantite);
                    }
                    let lieu_id: String;
                    { lieu_id = MasterFile::get_instance().lock().unwrap().get_joueur_mut().get_position(); }                        
                    {
                        let mut master_file = MasterFile::get_instance().lock().unwrap();
                        master_file.prendre_lieu_mut_id(&lieu_id).remove_contient_ressources(&ressource_id, quantite);
                    }

                    let mut joueur  = { MasterFile::get_instance().lock().unwrap().get_joueur_mut().clone() };
                    joueur.completion_quete(ressource_id.clone());
                    { *MasterFile::get_instance().lock().unwrap().get_joueur_mut()=joueur; }
                    
                    s.pop_layer();
                    s.pop_layer();
                    s.add_layer(recolter_screen());
                })
                .button("Non", |s| {
                    s.pop_layer();
                })
            );
        });
        if ressources_clone.len() == 0 {
            layout.add_child(TextView::new("Aucune ressource à récolter dans cette zone."));
        }
        else { layout.add_child(select); }
        Dialog::around(layout)
            .title("Récolter")
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(actions_screen());
            })
    }

  
    // Créer un écran de pnj
    fn pnj_screen() -> Dialog {
        let lieu_id: String;
        { lieu_id = MasterFile::get_instance().lock().unwrap().get_joueur_mut().get_position(); }
        let pnjs: Vec<String>;
        { pnjs = MasterFile::get_instance().lock().unwrap().prendre_lieu_id(&lieu_id).expect("Lieu introuvable").get_contient_pnj(); }
        let mut select = SelectView::new();
        for pnj_id in &pnjs {
            let pnj: Result<Pnj, String>;
            { pnj = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_id); }
            if pnj.is_ok() {
                select.add_item(pnj.unwrap().get_nom().clone(), pnj_id.clone());
            }
        }
        select.set_on_submit(move |s, choice: &String| {
            let pnj_id: String = choice.clone();
            let pnj: Result<Pnj, String>;
            { pnj = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_id); }
            if pnj.is_ok() {
                s.pop_layer();
                s.add_layer(create_dialog_action_pnj(pnj.unwrap()));
            }
        });
        Dialog::around(select)
            .title("PNJ")
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(actions_screen());
            })
    }

    fn create_dialog_action_pnj(pnj: Pnj) -> Dialog {
        let pnj_clone = pnj.clone();
        let is_commercant: bool = pnj.get_commerce_table().len() != 0;
        let mut select = SelectView::new();
        select.add_item("Parler", 1);
        if is_commercant { select.add_item("Commercer", 2); }
        select.add_item("Voler", 3);
        select.set_on_submit(move |s, choice| {
            if *choice == 1 {
                s.pop_layer();
                s.add_layer(create_dialogue_parler_pnj(pnj_clone.clone(),pnj_clone.clone().get_dialogues()));
            }
            else if *choice == 2 {
                s.pop_layer();
                s.add_layer(create_dialog_commerce_pnj(pnj_clone.clone()));
            }
            else if *choice == 3 {
                let player_pv: u16;
                { player_pv = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_actuel(); }
                if player_pv < 11 {
                    s.add_layer(Dialog::around(TextView::new("Vous n'avez pas assez de vie, vous risquez d'être mis KO."))
                        .title("Voler")
                        .button("Retour", |s| {
                            s.pop_layer();
                        })
                    );
                } else {
                    let chance_joueur: f32;
                    { chance_joueur = MasterFile::get_instance().lock().unwrap().get_joueur().get_chance() as f32 / 100 as f32; }
                    let mut rng = rand::thread_rng();
                    if chance_joueur.min(0.75) >= rng.gen_range(0.0..1.0) {
                        let inventaire: HashMap<String, u32> = pnj_clone.get_inventaire().clone();
                        let mut quantite_items = 0;
                        for (_, quantite) in inventaire {
                            quantite_items += quantite;
                        }
                        if quantite_items == 0 {
                            s.add_layer(Dialog::around(TextView::new(pnj_clone.get_nom() + " n'a rien à voler."))
                                .title("Voler")
                                .button("Retour", |s| {
                                    s.pop_layer();
                                })
                            );
                            return;
                        } else {
                            s.add_layer(create_dialog_voler_pnj(pnj_clone.clone()));
                        }
                    } else {
                        let joueur_pv: u16;
                        { joueur_pv = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_actuel().clone(); }
                        { MasterFile::get_instance().lock().unwrap().get_joueur_mut().set_pv_actuel(joueur_pv - 10); }
                        s.add_layer(Dialog::around(TextView::new("Vous vous êtes fait prendre !\nVous perdez 10pv.".to_string()))
                            .title("Vol raté")
                            .button("Ok", |s| {
                                s.pop_layer();
                            }));
                    }
                }
            }
        });
        Dialog::around(select)
            .title(pnj.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(pnj_screen());
            })
    }

    fn create_dialogue_parler_pnj(mut pnj: Pnj, dialogues: Vec<String>) -> Dialog {
        let pnj_clone = pnj.clone();
        let mut layout = LinearLayout::vertical();
        
        let prochains_dialogues: Vec<String>;

        if let Some(mut dialogue) = pnj.get_dialogue_a_jouer(dialogues) {
            if(dialogue.get_quete_joueur()){
                layout.add_child(TextView::new("Nouvelle quête obtenu : ".to_owned()+&dialogue.get_nom()));
                prochains_dialogues = Vec::new(); 
            }
            else{
                layout.add_child(TextView::new(pnj_clone.afficher_dialogue(&mut dialogue)));
                prochains_dialogues = dialogue.get_quetes_suivantes();
            } 
        } else {
            prochains_dialogues = Vec::new(); 
        }

        let mut dialog = Dialog::around(layout).title(pnj_clone.get_nom());


        if !prochains_dialogues.is_empty() {
            let dialogues_suivants = prochains_dialogues.clone();
            
            dialog = dialog.button("...", move |s| {
                s.pop_layer();
                s.add_layer(create_dialogue_parler_pnj(pnj.clone(),prochains_dialogues.clone()));
            });
        }
        else{
              dialog = dialog.button("...", move |s| {
                s.pop_layer();
                s.add_layer(create_dialog_action_pnj(pnj.clone()));
            });
        }
        dialog
    }



    fn create_dialog_commerce_pnj(pnj: Pnj) -> Dialog {
        let pnj_clone = pnj.clone();
        let mut layout = LinearLayout::vertical();
        let monnaie: u32;
        {
            let monnaie_option: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get("monnaie").copied();
            if monnaie_option.is_some() {
                monnaie = monnaie_option.unwrap();
            }
            else {
                monnaie = 0;
            }
        }
        layout.add_child(TextView::new("Monnaie : ".to_string() + &monnaie.to_string()));
        layout.add_child(SelectView::new()
            .item("Acheter", 1)
            .item("Vendre", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(create_dialog_acheter_pnj(pnj_clone.clone()));
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(create_dialog_vendre_pnj(pnj_clone.clone()));
                }
            }));
        Dialog::around(layout)
        .title(pnj.get_nom().clone() + " - Commercer")
        .button("Retour", move |s| {
            s.pop_layer();
            s.add_layer(create_dialog_action_pnj(pnj.clone()));
        })
    }

    fn create_dialog_acheter_pnj(pnj: Pnj) -> Dialog {
        let pnj_clone = pnj.clone();
        let mut layout = LinearLayout::vertical();
        let mut select = SelectView::new();
        let mut counter = 0;
        for (ressource_id, quantite) in pnj.get_commerce_table() {
            if quantite > 0 {
                let consommable: Result<Consommable, String>;
                { consommable = MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&ressource_id); }
                if consommable.is_ok() {
                    select.add_item(consommable.clone().unwrap().get_nom().clone() + " x " + &quantite.to_string() + " : " + &consommable.clone().unwrap().get_prix().clone().to_string(), ressource_id.clone());
                }
                else {
                    let ressource: Result<Ressource, String>;
                    { ressource = MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&ressource_id); }
                    if ressource.is_ok() {
                        select.add_item(ressource.clone().unwrap().get_nom().clone() + " x " + &quantite.to_string() + " : " + &consommable.clone().unwrap().get_prix().clone().to_string(), ressource_id.clone());
                    }
                    else {
                        let equipement: Result<Equipement, String>;
                        { equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&ressource_id); }
                        if equipement.is_ok() {
                            select.add_item(equipement.clone().unwrap().get_nom().clone() + " x " + &quantite.to_string() + " : " + &consommable.clone().unwrap().get_prix().clone().to_string(), ressource_id.clone());
                        }
                    }
                }
                counter += 1;
            }
        }
        select.set_on_submit(move |s, choice: &String| {
            let ressource_id: String = choice.clone();
            let consommable: Result<Consommable, String>;
            { consommable = MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&ressource_id); }
            if consommable.is_ok() {
                let mut layout: LinearLayout = LinearLayout::vertical();
                layout.add_child(TextView::new(consommable.clone().expect("Consommable non trouvé").get_description().clone()));
                let mut decomposer_string: String = "Décomposer => {".to_string();
                for (id, quantite) in consommable.clone().expect("Consommable non trouvé").get_ressources() {
                    decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
                }
                decomposer_string += "}";
                layout.add_child(TextView::new(decomposer_string));
                layout.add_child(TextView::new("Prix : ".to_owned() + &consommable.clone().expect("Consommable non trouvé").get_prix().clone().to_string()));
                let effets = consommable.clone().expect("Consommable non trouvé").get_effets().clone();
                layout.add_child(DummyView::new().fixed_height(1));
                layout.add_child(TextView::new("Régenération pv : ".to_string() + &effets[0].to_string()));
                layout.add_child(TextView::new("Force : +".to_string() + &effets[1].to_string() + " en combat"));
                layout.add_child(TextView::new("Dexterité : +".to_string() + &effets[2].to_string() + " en combat"));
                layout.add_child(TextView::new("Intelligence : +".to_string() + &effets[3].to_string() + " en combat"));
                layout.add_child(TextView::new("Vitesse : +".to_string() + &effets[4].to_string() + " en combat"));
                layout.add_child(TextView::new("Esquive : +".to_string() + &effets[5].to_string() + " en combat"));
                layout.add_child(TextView::new("Chance : +".to_string() + &effets[6].to_string() + " en combat"));
                layout.add_child(TextView::new("Résistance physique : +".to_string() + &effets[7].to_string() + " en combat"));
                layout.add_child(TextView::new("Résistance magique : +".to_string() + &effets[8].to_string() + " en combat"));
                let pnj_clone_clone = pnj_clone.clone();
                s.add_layer(Dialog::around(layout)
                    .title("Acheter : ".to_owned() + &consommable.clone().unwrap().get_nom().clone())
                    .button("Acheter", move |s| {
                        { MasterFile::get_instance().lock().unwrap().acheter(pnj_clone_clone.clone().get_id(), consommable.clone().expect("Consommable non trouvé").get_id(), 1, consommable.clone().expect("Consommable non trouvé").get_prix().clone()) }
                        s.pop_layer();
                        s.pop_layer();
                        let pnj_mod: Pnj;
                        {
                            let pnj_result: Result<Pnj, String>;
                            pnj_result = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_clone_clone.get_id());
                            pnj_mod = pnj_result.expect("Pnj introuvable");
                        }
                        s.add_layer(create_dialog_commerce_pnj(pnj_mod.clone()));
                    })
                    .button("Retour", move |s| {
                        s.pop_layer();
                    })
                );
            }
            else {
                let ressource: Result<Ressource, String>;
                { ressource = MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&ressource_id); }
                if ressource.is_ok() {
                    let mut layout: LinearLayout = LinearLayout::vertical();
                    layout.add_child(TextView::new(ressource.clone().expect("Ressource non trouvée").get_description().clone()));
                    let mut decomposer_string: String = "Décomposer => {".to_string();
                    for (id, quantite) in ressource.clone().expect("Ressource non trouvée").get_ressource() {
                        decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
                    }
                    decomposer_string += "}";
                    layout.add_child(TextView::new(decomposer_string));
                    layout.add_child(TextView::new("Prix : ".to_owned() + &ressource.clone().expect("Ressource non trouvée").get_prix().clone().to_string()));
                    let pnj_clone_clone = pnj_clone.clone();
                    s.add_layer(Dialog::around(layout)
                        .title("Acheter : ".to_owned() + &ressource.clone().expect("Ressource non trouvée").get_nom().clone())
                        .button("Acheter", move |s| {
                            { MasterFile::get_instance().lock().unwrap().acheter(pnj_clone_clone.clone().get_id(), ressource.clone().expect("Ressource non trouvée").get_id(), 1, ressource.clone().expect("Ressource non trouvée").get_prix().clone()) }
                            s.pop_layer();
                            s.pop_layer();
                            let pnj_mod: Pnj;
                            {
                                let pnj_result: Result<Pnj, String>;
                                pnj_result = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_clone_clone.get_id());
                                pnj_mod = pnj_result.expect("Pnj introuvable");
                            }
                            s.add_layer(create_dialog_commerce_pnj(pnj_mod.clone()));
                        })
                        .button("Retour", move |s| {
                            s.pop_layer();
                        })
                    );
                }
                else {
                    let equipement: Result<Equipement, String>;
                    { equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&ressource_id); }
                    if equipement.is_ok() {
                        let mut decomposer_string: String = "Décomposer => {".to_string();
                        for (id, quantite) in equipement.clone().expect("Equipement non trouvée").get_ressources() {
                            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
                        }
                        decomposer_string += "}";
                        let layout: LinearLayout = LinearLayout::vertical()
                            .child(TextView::new(equipement.clone().expect("Equipement non trouvée").get_description().clone()))
                            .child(TextView::new(&decomposer_string))
                            .child(TextView::new("Prix : ".to_owned() + &equipement.clone().expect("Equipement non trouvée").get_prix().to_string()))
                            .child(DummyView::new().fixed_height(1))
                            .child(TextView::new("Pv : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_pv().to_string()))
                            .child(TextView::new("Force : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_force().to_string()))
                            .child(TextView::new("Dexterité : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_dexterite().to_string()))
                            .child(TextView::new("Intelligence : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_intelligence().to_string()))
                            .child(TextView::new("Vitesse : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_vitesse().to_string()))
                            .child(TextView::new("Esquive : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_esquive().to_string()))
                            .child(TextView::new("Chance : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_chance().to_string()))
                            .child(TextView::new("Résistance physique : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_resistance_physique().to_string()))
                            .child(TextView::new("Résistance magique : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_resistance_magique().to_string()))
                            .child(TextView::new("Mult xp : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_bonus_multiplicateur_xp().to_string()))
                            .child(TextView::new("% Pv : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_pv().to_string()))
                            .child(TextView::new("% Force : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_force().to_string()))
                            .child(TextView::new("% Dexterité : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_dexterite().to_string()))
                            .child(TextView::new("% Intelligence : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_intelligence().to_string()))
                            .child(TextView::new("% Vitesse : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_vitesse().to_string()))
                            .child(TextView::new("% Esquive : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_esquive().to_string()))
                            .child(TextView::new("% Chance : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_chance().to_string()))
                            .child(TextView::new("% Résistance physique : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_resistance_physique().to_string()))
                            .child(TextView::new("% Résistance magique : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_pourcent_bonus_resistance_magique().to_string()))
                            .child(TextView::new("Catégorie : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_categorie().to_string()))
                            .child(TextView::new("Rareté : ".to_string() + &equipement.clone().expect("Equipement non trouvée").get_rarete().to_string()))
                        ;
                        let pnj_clone_clone = pnj_clone.clone();
                        s.add_layer(Dialog::around(layout)
                            .title("Acheter : ".to_owned() + &equipement.clone().expect("Equipement non trouvée").get_nom().clone())
                            .button("Acheter", move |s| {
                                { MasterFile::get_instance().lock().unwrap().acheter(pnj_clone_clone.clone().get_id(), equipement.clone().expect("Equipement non trouvée").get_id(), 1, equipement.clone().expect("Equipement non trouvée").get_prix().clone()) }
                                s.pop_layer();
                                s.pop_layer();
                                let pnj_mod: Pnj;
                                {
                                    let pnj_result: Result<Pnj, String>;
                                    pnj_result = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_clone_clone.get_id());
                                    pnj_mod = pnj_result.expect("Pnj introuvable");
                                }
                                s.add_layer(create_dialog_commerce_pnj(pnj_mod.clone()));
                            })
                            .button("Retour", move |s| {
                                s.pop_layer();
                            })
                        );
                    }
                }
            }
        });
        if counter > 0 { layout.add_child(select); }
        else { layout.add_child(TextView::new(pnj.get_nom().clone() + " n'a rien n'a vendre")); }
        Dialog::around(layout)
            .title(pnj.get_nom().clone() + " - Acheter")
            .button("Retour", move |s| {
                s.pop_layer();
                s.add_layer(create_dialog_commerce_pnj(pnj.clone()));
            })
    }

    fn create_dialog_vendre_pnj(pnj: Pnj) -> Dialog {
        let pnj_id: String = pnj.get_id();
        Dialog::around(SelectView::new()
            .item("Consommable", 1)
            .item("Ressource", 2)
            .item("Equipement", 3)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    let pnj_: Pnj;
                    { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_id).expect("Pnj introuvable"); }
                    s.add_layer(create_dialog_vendre_consommable_pnj(pnj_.clone()));
                }
                else if *choice == 2 {
                    s.pop_layer();
                    let pnj_: Pnj;
                    { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_id).expect("Pnj introuvable"); }
                    s.add_layer(create_dialog_vendre_ressource_pnj(pnj_.clone()));
                }
                else if *choice == 3 {
                    s.pop_layer();
                    let pnj_: Pnj;
                    { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj_id).expect("Pnj introuvable"); }
                    s.add_layer(create_dialog_vendre_equipement_pnj(pnj_.clone()));
                }
            })
        )
        .title("Vendre")
        .button("Retour", move |s| {
            s.pop_layer();
            s.add_layer(create_dialog_commerce_pnj(pnj.clone()));
        })
    }

    fn create_dialog_vendre_consommable_pnj(pnj: Pnj) -> Dialog {
        let inventaire: HashMap<String, u32>;
        { inventaire = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().clone() }
        let mut consommables: Vec<Consommable> = vec![];
        for (id, quantite) in inventaire {
            if quantite > 0 {
                match MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&id) {
                    Ok(consommable) => consommables.push(consommable),
                    Err(_) => {}
                }
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);

        let mut consommable: Consommable;
        for i in 1..consommables.len() + 1 {
            consommable = consommables[i - 1].clone();
            let quantite: u32;
            {
                let nb: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get(&consommable.get_id().clone()).copied();
                match nb {
                    Some(nb) => quantite = nb,
                    None => quantite = 0
                }
            }
            select.add_item(consommable.get_nom().clone().to_string() + " : " + &quantite.to_string(), i.to_string());
            if i % 10 == 0 {
                let consommables_clone = consommables.clone();
                let pnj_: Pnj;
                { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id()).expect("Pnj introuvable") }
                select.set_on_submit(move |s, choice: &String| {
                    if quantite == 0 {
                        s.add_layer(Dialog::text("Vous n'avez plus de ".to_string() + &consommable.get_nom().clone())
                            .title("Consommable")
                            .button("Retour", |s| { s.pop_layer(); })
                        );
                    }
                    else {
                        let index = choice.parse::<usize>().unwrap() - 1;
                        let consommable = consommables_clone[index].clone();
                        s.add_layer(create_dialog_vendre_consommable(consommable, pnj_.clone()));
                    }
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if consommables.len() % 10 != 0 {
            let consommables_clone = consommables.clone();
            let pnj_: Pnj;
            { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id()).expect("Pnj introuvable") }
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let consommable = consommables_clone[index].clone();
                s.pop_layer();
                s.add_layer(create_dialog_vendre_consommable(consommable, pnj_.clone()));
            });
            layout.add_child(select);
        }
        if consommables.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucun consommable."));
        }
        Dialog::around(layout)
            .title("Vendre consommable")
            .button("Retour", move |s| {
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj.clone()));
            })
    }

    fn create_dialog_vendre_consommable(consommable: Consommable, pnj: Pnj) -> Dialog {
        let pnj_: Pnj;
        { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id()).expect("Pnj introuvable") }
        let consommable_clone = consommable.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(consommable.get_description().clone()));
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in consommable.get_ressources() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        layout.add_child(TextView::new(decomposer_string));
        layout.add_child(TextView::new("Prix : ".to_owned() + &consommable.clone().get_prix().clone().to_string()));
        let effets = consommable.get_effets().clone();
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(TextView::new("Régenération pv : ".to_string() + &effets[0].to_string()));
        layout.add_child(TextView::new("Force : +".to_string() + &effets[1].to_string() + " en combat"));
        layout.add_child(TextView::new("Dexterité : +".to_string() + &effets[2].to_string() + " en combat"));
        layout.add_child(TextView::new("Intelligence : +".to_string() + &effets[3].to_string() + " en combat"));
        layout.add_child(TextView::new("Vitesse : +".to_string() + &effets[4].to_string() + " en combat"));
        layout.add_child(TextView::new("Esquive : +".to_string() + &effets[5].to_string() + " en combat"));
        layout.add_child(TextView::new("Chance : +".to_string() + &effets[6].to_string() + " en combat"));
        layout.add_child(TextView::new("Résistance physique : +".to_string() + &effets[7].to_string() + " en combat"));
        layout.add_child(TextView::new("Résistance magique : +".to_string() + &effets[8].to_string() + " en combat"));
        layout.add_child(DummyView::new().fixed_height(2));
        Dialog::around(layout)
            .title(consommable_clone.get_nom().clone())
            .button("Vendre", move |s| {
                { MasterFile::get_instance().lock().unwrap().vendre(consommable_clone.get_id(), 1); }
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj_.clone()));
            })
            .button("Retour", move |s| {
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj.clone()));
            })
    }

    fn create_dialog_vendre_ressource_pnj(pnj: Pnj) -> Dialog {
        let inventaire: HashMap<String, u32>;
        { inventaire = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().clone() }
        let mut ressources: Vec<Ressource> = vec![];
        for (id, quantite) in inventaire {
            if quantite > 0 {
                match MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id) {
                    Ok(ressource) => ressources.push(ressource),
                    Err(_) => {}
                }
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);

        let mut ressource: Ressource;
        for i in 1..ressources.len() + 1 {
            ressource = ressources[i - 1].clone();
            let quantite: u32;
            {
                let nb: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get(&ressource.get_id().clone()).copied();
                match nb {
                    Some(nb) => quantite = nb,
                    None => quantite = 0
                }
            }
            select.add_item(ressource.get_nom().clone().to_string() + " : " + &quantite.to_string(), i.to_string());
            if i % 10 == 0 {
                let ressources_clone = ressources.clone();
                let pnj_: Pnj;
                { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id().clone()).expect("Pnj introuvable"); }
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let ressource = ressources_clone[index].clone();
                    s.add_layer(create_dialog_vendre_ressource(ressource, pnj_.clone()));
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if ressources.len() % 10 != 0 {
            let ressources_clone = ressources.clone();
            let pnj_: Pnj;
            { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id().clone()).expect("Pnj introuvable"); }
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let ressource = ressources_clone[index].clone();
                s.pop_layer();
                s.add_layer(create_dialog_vendre_ressource(ressource, pnj_.clone()));
            });
            layout.add_child(select);
        }
        if ressources.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucune ressource."));
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Ressources")
        .button("Retour", move |s| {
            s.pop_layer();
            s.add_layer(create_dialog_vendre_pnj(pnj.clone()));
        })
    }

    fn create_dialog_vendre_ressource(ressource: Ressource, pnj: Pnj) -> Dialog {
        let pnj_: Pnj;
        { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id()).expect("Pnj introuvable") }
        let ressource_clone = ressource.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(ressource.get_description().clone()));
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in ressource.get_ressource() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        layout.add_child(TextView::new(decomposer_string));
        layout.add_child(TextView::new("Prix : ".to_owned() + &ressource.clone().get_prix().clone().to_string()));
        layout.add_child(DummyView::new().fixed_height(1));
        Dialog::around(layout)
            .title(ressource_clone.get_nom().clone())
            .button("Vendre", move |s| {
                { MasterFile::get_instance().lock().unwrap().vendre(ressource_clone.get_id(), 1); }
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj_.clone()));
            })
            .button("Retour", move |s| {
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj.clone()));
            })
    }

    fn create_dialog_vendre_equipement_pnj(pnj: Pnj) -> Dialog {
        let inventaire: HashMap<String, u32>;
        { inventaire = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().clone() }
        let mut equipements: Vec<Equipement> = vec![];
        for (id, quantite) in inventaire {
            if quantite > 0 {
                match MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id) {
                    Ok(equipement) => equipements.push(equipement),
                    Err(_) => {}
                }
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);

        let mut equipement: Equipement;
        for i in 1..equipements.len() + 1 {
            equipement = equipements[i - 1].clone();
            let quantite: u32;
            {
                let nb: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get(&equipement.get_id().clone()).copied();
                match nb {
                    Some(nb) => quantite = nb,
                    None => quantite = 0
                }
            }
            select.add_item(equipement.get_nom().clone() + " : " + &quantite.to_string(), i.to_string());
            if i % 10 == 0 {
                let equipements_clone = equipements.clone();
                let pnj_: Pnj;
                { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id().clone()).expect("Pnj introuvable"); }
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let equipement = equipements_clone[index].clone();
                    s.pop_layer();
                    s.add_layer(create_dialog_vendre_equipement(equipement, pnj_.clone()));
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if equipements.len() % 10 != 0 {
            let equipements_clone = equipements.clone();
            let pnj_: Pnj;
            { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id().clone()).expect("Pnj introuvable"); }
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let equipement = equipements_clone[index].clone();
                s.pop_layer();
                s.add_layer(create_dialog_vendre_equipement(equipement, pnj_.clone()));
            });
            layout.add_child(select);
        }
        if equipements.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucun equipement."));
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Equipements")
        .button("Retour", move |s| {
            s.pop_layer();
            s.add_layer(create_dialog_vendre_pnj(pnj.clone()));
        })
    }

    fn create_dialog_vendre_equipement(equipement: Equipement, pnj: Pnj) -> Dialog {
        let pnj_: Pnj;
        { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id()).expect("Pnj introuvable") }
        let equipement_clone = equipement.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(equipement.get_description().clone()));
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in equipement.get_ressources() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        layout.add_child(TextView::new(decomposer_string));
        layout.add_child(TextView::new("Prix : ".to_owned() + &equipement.clone().get_prix().clone().to_string()));
        let mut layout_caracteristiques: LinearLayout = LinearLayout::horizontal();
        let layout_equipement: LinearLayout = LinearLayout::vertical()
            .child(TextView::new(equipement.get_nom().clone() + " :"))
            .child(LinearLayout::horizontal()
                .child(DummyView::new().fixed_width(2))
                .child(LinearLayout::vertical()
                    .child(TextView::new("Pv : ".to_string() + &equipement.get_bonus_pv().to_string()))
                    .child(TextView::new("Force : ".to_string() + &equipement.get_bonus_force().to_string()))
                    .child(TextView::new("Dexterité : ".to_string() + &equipement.get_bonus_dexterite().to_string()))
                    .child(TextView::new("Intelligence : ".to_string() + &equipement.get_bonus_intelligence().to_string()))
                    .child(TextView::new("Vitesse : ".to_string() + &equipement.get_bonus_vitesse().to_string()))
                    .child(TextView::new("Esquive : ".to_string() + &equipement.get_bonus_esquive().to_string()))
                    .child(TextView::new("Chance : ".to_string() + &equipement.get_bonus_chance().to_string()))
                    .child(TextView::new("Résistance physique : ".to_string() + &equipement.get_bonus_resistance_physique().to_string()))
                    .child(TextView::new("Résistance magique : ".to_string() + &equipement.get_bonus_resistance_magique().to_string()))
                    .child(TextView::new("Mult xp : ".to_string() + &equipement.get_bonus_multiplicateur_xp().to_string()))
                    .child(TextView::new("% Pv : ".to_string() + &equipement.get_pourcent_bonus_pv().to_string()))
                    .child(TextView::new("% Force : ".to_string() + &equipement.get_pourcent_bonus_force().to_string()))
                    .child(TextView::new("% Dexterité : ".to_string() + &equipement.get_pourcent_bonus_dexterite().to_string()))
                    .child(TextView::new("% Intelligence : ".to_string() + &equipement.get_pourcent_bonus_intelligence().to_string()))
                    .child(TextView::new("% Vitesse : ".to_string() + &equipement.get_pourcent_bonus_vitesse().to_string()))
                    .child(TextView::new("% Esquive : ".to_string() + &equipement.get_pourcent_bonus_esquive().to_string()))
                    .child(TextView::new("% Chance : ".to_string() + &equipement.get_pourcent_bonus_chance().to_string()))
                    .child(TextView::new("% Résistance physique : ".to_string() + &equipement.get_pourcent_bonus_resistance_physique().to_string()))
                    .child(TextView::new("% Résistance magique : ".to_string() + &equipement.get_pourcent_bonus_resistance_magique().to_string()))
                    .child(TextView::new("Catégorie : ".to_string() + &equipement.get_categorie().to_string()))
                    .child(TextView::new("Rareté : ".to_string() + &equipement.get_rarete().to_string()))
                )
            )
        ;
        layout_caracteristiques.add_child(layout_equipement);
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(ScrollView::new(layout_caracteristiques).scroll_x(true).scroll_y(true));
        layout.add_child(DummyView::new().fixed_height(1));
        Dialog::around(layout)
            .title(equipement_clone.get_nom().clone())
            .button("Vendre", move |s| {
                { MasterFile::get_instance().lock().unwrap().vendre(equipement_clone.get_id(), 1); }
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj_.clone()));
            })
            .button("Retour", move |s| {
                s.pop_layer();
                s.add_layer(create_dialog_vendre_pnj(pnj.clone()));
            })
    }

    fn create_dialog_voler_pnj(pnj: Pnj) -> Dialog {
        let mut rng = rand::thread_rng();
        let inventaire: HashMap<String, u32> = pnj.get_inventaire();
        let mut keys: Vec<String> = vec![];
        for (key, _) in &inventaire {
            keys.push(key.clone());
        }
        let item_index = rng.gen_range(0..keys.len());
        if let Some(&ref key) = keys.get(item_index) {
            let quantite: u32 = inventaire.get(key).unwrap().clone();
            let quantite_vol: u32 = rng.gen_range(1..quantite + 1);
            let item_nom: String;
            { item_nom = MasterFile::get_instance().lock().unwrap().prendre_item_id(&key.clone()).expect("Item introuvable").get_nom().clone(); }
            { MasterFile::get_instance().lock().unwrap().voler(pnj.get_id(), key.clone(), quantite_vol); }
            let pnj_: Pnj;
            { pnj_ = MasterFile::get_instance().lock().unwrap().prendre_pnj_id(&pnj.get_id()).expect("Pnj introuvable"); }
            Dialog::around(TextView::new("Vous avez volé ".to_string() + &item_nom.to_string() + " x " + &quantite_vol.to_string() + "."))
                .title("Vol")
                .button("Ok", move |s| {
                    s.pop_layer();
                    s.pop_layer();
                    s.add_layer(create_dialog_action_pnj(pnj_.clone()));
                })
        } else {
            panic!("Item introuvable");
        }
    }

    // Créer un écran de personnage
    fn personnage_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Informations", 1)
            .item("Inventaire", 2)
            .item("Quêtes", 3)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(informations_screen());
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(inventaire_screen());
                }
                else if *choice == 3 {
                    s.pop_layer();
                    s.add_layer(quetes_screen());
                }
            })
        )
        .title("Personnage")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(game_screen());
        })
    }

    // Créer un écran d'information
    fn informations_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Attribuer des points", 1)
            .item("Statistiques", 2)
            .item("Equipement", 3)
            .item("Attaques", 4)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(points_attribution());
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(statistiques_screen());
                }
                else if *choice == 3 {
                    s.pop_layer();
                    s.add_layer(equipement_screen());
                }
                else if *choice == 4 {
                    s.pop_layer();
                    s.add_layer(attaques_screen());
                }
            })
        )
        .title("Informations")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(personnage_screen());
        })
    }

    // Créer un écran d'attribution des points
    fn points_attribution() -> Dialog {
        let mut layout = LinearLayout::vertical();
        let points: u8;
        { points = MasterFile::get_instance().lock().unwrap().get_joueur().get_points_competence(); }
        layout.add_child(TextView::new("Points à attribuer : ".to_string() + &points.to_string()));
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(SelectView::new()
            .item("Choisir une statistique", 1)
            .on_submit(move |s, choice| {
                if points > 0 {
                    if *choice == 1 {
                        s.pop_layer();
                        s.add_layer(stat_choice());
                    }
                }
                else {
                    s.add_layer(Dialog::text("Vous n'avez aucun de points à attribuer")
                        .title("Attribution des points")
                        .button("Retour", |s| { s.pop_layer(); })
                    );
                }
            })
        );

        Dialog::around(layout)
            .title("Attribuer des points")
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(informations_screen());
            })
    }

    // Créer un écran de choix de stats
    fn stat_choice() -> Dialog {
        let pv_max: u16;
        let force: u16;
        let dexterite: u16;
        let intelligence: u16;
        let vitesse: u16;
        let esquive: u16;
        let chance: u16;
        { pv_max = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_max(); }
        { force = MasterFile::get_instance().lock().unwrap().get_joueur().get_force(); }
        { dexterite = MasterFile::get_instance().lock().unwrap().get_joueur().get_dexterite(); }
        { intelligence = MasterFile::get_instance().lock().unwrap().get_joueur().get_intelligence(); }
        { vitesse = MasterFile::get_instance().lock().unwrap().get_joueur().get_vitesse(); }
        { esquive = MasterFile::get_instance().lock().unwrap().get_joueur().get_esquive(); }
        { chance = MasterFile::get_instance().lock().unwrap().get_joueur().get_chance(); }
        let select_stats = SelectView::new()
            .item("pv max : ".to_string() + &pv_max.to_string() + " (+10)", 1)
            .item("force : ".to_string() + &force.to_string() + " (+1)", 2)
            .item("dexterite : ".to_string() + &dexterite.to_string() + " (+1)", 3)
            .item("intelligence : ".to_string() + &intelligence.to_string() + " (+1)", 4)
            .item("vitesse : ".to_string() + &vitesse.to_string() + " (+1)", 5)
            .item("esquive : ".to_string() + &esquive.to_string() + " (+1)", 6)
            .item("chance : ".to_string() + &chance.to_string() + " (+1)", 7)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.add_layer(create_dialog_add_stat("pv".to_string()));
                }
                else if *choice == 2 {
                    s.add_layer(create_dialog_add_stat("force".to_string()));
                }
                else if *choice == 3 {
                    s.add_layer(create_dialog_add_stat("dexterite".to_string()));
                }
                else if *choice == 4 {
                    s.add_layer(create_dialog_add_stat("intelligence".to_string()));
                }
                else if *choice == 5 {
                    s.add_layer(create_dialog_add_stat("vitesse".to_string()));
                }
                else if *choice == 6 {
                    s.add_layer(create_dialog_add_stat("esquive".to_string()));
                }
                else if *choice == 7 {
                    s.add_layer(create_dialog_add_stat("chance".to_string()));
                }
            });
        Dialog::around(select_stats)
            .title("Choisir une statistique")
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(points_attribution());
            })
    }

    fn create_dialog_add_stat(stat: String) -> Dialog {
        Dialog::text("Voulez-vous rajouter un point de ".to_owned() + &stat + " ?")
            .title("Rajouter un point de : ".to_owned() + &stat)
            .button("Oui", move |s| {
                { MasterFile::get_instance().lock().unwrap().get_joueur_mut().ajout_point_stat(&stat); }
                s.pop_layer();
                s.pop_layer();
                s.add_layer(points_attribution());
            })
            .button("Non", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de statistiques
    fn statistiques_screen() -> Dialog {
        let mut layout = LinearLayout::vertical();
        let nv: u8;
        let xp: u32;
        let pv: u16;
        let force: u16;
        let dexterite: u16;
        let intelligence: u16;
        let vitesse: u16;
        let esquive: u16;
        let chance: u16;
        let resistance_physique: u16;
        let resistance_magique: u16;
        let multiplicateur_xp: u16;
        { nv = MasterFile::get_instance().lock().unwrap().get_joueur().get_niveau(); }
        { xp = MasterFile::get_instance().lock().unwrap().get_joueur().get_xp(); }
        { pv = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_max(); }
        { force = MasterFile::get_instance().lock().unwrap().get_joueur().get_force(); }
        { dexterite = MasterFile::get_instance().lock().unwrap().get_joueur().get_dexterite(); }
        { intelligence = MasterFile::get_instance().lock().unwrap().get_joueur().get_intelligence(); }
        { vitesse = MasterFile::get_instance().lock().unwrap().get_joueur().get_vitesse(); }
        { esquive = MasterFile::get_instance().lock().unwrap().get_joueur().get_esquive(); }
        { chance = MasterFile::get_instance().lock().unwrap().get_joueur().get_chance(); }
        { resistance_physique = MasterFile::get_instance().lock().unwrap().get_joueur().get_resistance_physique(); }
        { resistance_magique = MasterFile::get_instance().lock().unwrap().get_joueur().get_resistance_magique(); }
        { multiplicateur_xp = MasterFile::get_instance().lock().unwrap().get_joueur().get_multiplicateur_xp(); }
        layout.add_child(TextView::new("nv : ".to_string() + &nv.to_string()));
        layout.add_child(TextView::new("xp : ".to_string() + &xp.to_string() + "/" + &(nv * 150).to_string()));
        layout.add_child(TextView::new("pv : ".to_string() + &pv.to_string()));
        layout.add_child(TextView::new("force : ".to_string() + &force.to_string()));
        layout.add_child(TextView::new("dexterite : ".to_string() + &dexterite.to_string()));
        layout.add_child(TextView::new("intelligence : ".to_string() + &intelligence.to_string()));
        layout.add_child(TextView::new("vitesse : ".to_string() + &vitesse.to_string()));
        layout.add_child(TextView::new("esquive : ".to_string() + &esquive.to_string()));
        layout.add_child(TextView::new("chance : ".to_string() + &chance.to_string()));
        layout.add_child(TextView::new("resistance physique : ".to_string() + &resistance_physique.to_string()));
        layout.add_child(TextView::new("resistance magique : ".to_string() + &resistance_magique.to_string()));
        layout.add_child(TextView::new("multiplicateur xp : +".to_string() + &((multiplicateur_xp * 100) - 100).to_string() + "%"));
        Dialog::around(layout)
            .title("Statistiques")
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(informations_screen());
            })
    }

    // Créer un écran de l'équipement du joueur
    fn equipement_screen() -> Dialog {
        let mut equipements: HashMap<EquipementType, Option<String>> = HashMap::new();
        { equipements = MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement(); }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let arme: Option<String> = equipements.get(&EquipementType::Arme).unwrap().clone();
        let casque: Option<String> = equipements.get(&EquipementType::Casque).unwrap().clone();
        let plastron: Option<String> = equipements.get(&EquipementType::Plastron).unwrap().clone();
        let gants: Option<String> = equipements.get(&EquipementType::Gants).unwrap().clone();
        let jambieres: Option<String> = equipements.get(&EquipementType::Jambieres).unwrap().clone();
        let bottes: Option<String> = equipements.get(&EquipementType::Bottes).unwrap().clone();
        let mut select = SelectView::new();
        match arme {
            Some(ref id) => {
                let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                select.add_item("Arme : ".to_string() + &equipement.get_nom().clone(), 1);
            }
            None => {
                select.add_item("Arme : Non equippé", 1);
            }
        }
        match casque {
            Some(ref id) => {
                let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                select.add_item("Casque : ".to_string() + &equipement.get_nom().clone(), 2);
            }
            None => {
                select.add_item("Casque : Non equippé", 2);
            }
        }
        match plastron {
            Some(ref id) => {
                let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                select.add_item("Plastron : ".to_string() + &equipement.get_nom().clone(), 3);
            }
            None => {
                select.add_item("Plastron : Non equippé", 3);
            }
        }
        match gants {
            Some(ref id) => {
                let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                select.add_item("Gants : ".to_string() + &equipement.get_nom().clone(), 4);
            }
            None => {
                select.add_item("Gants : Non equippé", 4);
            }
        }
        match jambieres {
            Some(ref id) => {
                let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                select.add_item("Jambieres : ".to_string() + &equipement.get_nom().clone(), 5);
            }
            None => {
                select.add_item("Jambieres : Non equippé", 5);
            }
        }
        match bottes {
            Some(ref id) => {
                let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                select.add_item("Bottes : ".to_string() + &equipement.get_nom().clone(), 6);
            }
            None => {
                select.add_item("Bottes : Non equippé", 6);
            }
        }
        select.set_on_submit(move |s, choice| {
            if *choice == 1 {
                match arme {
                    Some(ref id) => {
                        let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                        s.add_layer(create_equipement_dialog(equipement.clone()));
                    }
                    None => {}
                }
            }
            if *choice == 2 {
                match casque {
                    Some(ref id) => {
                        let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                        s.add_layer(create_equipement_dialog(equipement.clone()));
                    }
                    None => {}
                }
            }
            if *choice == 3 {
                match plastron {
                    Some(ref id) => {
                        let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                        s.add_layer(create_equipement_dialog(equipement.clone()));
                    }
                    None => {}
                }
            }
            if *choice == 4 {
                match gants {
                    Some(ref id) => {
                        let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                        s.add_layer(create_equipement_dialog(equipement.clone()));
                    }
                    None => {}
                }
            }
            if *choice == 5 {
                match jambieres {
                    Some(ref id) => {
                        let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                        s.add_layer(create_equipement_dialog(equipement.clone()));
                    }
                    None => {}
                }
            }
            if *choice == 6 {
                match bottes {
                    Some(ref id) => {
                        let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                        s.add_layer(create_equipement_dialog(equipement.clone()));
                    }
                    None => {}
                }
            }
        });
        layout.add_child(select);
        Dialog::around(layout)
            .title("Equipement")
            .button("Retour", |s| {
                s.pop_layer();
                s.pop_layer();
                s.add_layer(informations_screen());
            })
    }

    fn create_equipement_dialog(equipement: Equipement) -> Dialog {
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in equipement.clone().get_ressources() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        let layout: LinearLayout = LinearLayout::vertical()
            .child(TextView::new(equipement.get_description().clone()))
            .child(TextView::new(&decomposer_string))
            .child(TextView::new("Prix : ".to_owned() + &equipement.clone().get_prix().to_string()))
            .child(DummyView::new().fixed_height(1))
            .child(TextView::new("Pv : ".to_string() + &equipement.get_bonus_pv().to_string()))
            .child(TextView::new("Force : ".to_string() + &equipement.get_bonus_force().to_string()))
            .child(TextView::new("Dexterité : ".to_string() + &equipement.get_bonus_dexterite().to_string()))
            .child(TextView::new("Intelligence : ".to_string() + &equipement.get_bonus_intelligence().to_string()))
            .child(TextView::new("Vitesse : ".to_string() + &equipement.get_bonus_vitesse().to_string()))
            .child(TextView::new("Esquive : ".to_string() + &equipement.get_bonus_esquive().to_string()))
            .child(TextView::new("Chance : ".to_string() + &equipement.get_bonus_chance().to_string()))
            .child(TextView::new("Résistance physique : ".to_string() + &equipement.get_bonus_resistance_physique().to_string()))
            .child(TextView::new("Résistance magique : ".to_string() + &equipement.get_bonus_resistance_magique().to_string()))
            .child(TextView::new("Mult xp : ".to_string() + &equipement.get_bonus_multiplicateur_xp().to_string()))
            .child(TextView::new("% Pv : ".to_string() + &equipement.get_pourcent_bonus_pv().to_string()))
            .child(TextView::new("% Force : ".to_string() + &equipement.get_pourcent_bonus_force().to_string()))
            .child(TextView::new("% Dexterité : ".to_string() + &equipement.get_pourcent_bonus_dexterite().to_string()))
            .child(TextView::new("% Intelligence : ".to_string() + &equipement.get_pourcent_bonus_intelligence().to_string()))
            .child(TextView::new("% Vitesse : ".to_string() + &equipement.get_pourcent_bonus_vitesse().to_string()))
            .child(TextView::new("% Esquive : ".to_string() + &equipement.get_pourcent_bonus_esquive().to_string()))
            .child(TextView::new("% Chance : ".to_string() + &equipement.get_pourcent_bonus_chance().to_string()))
            .child(TextView::new("% Résistance physique : ".to_string() + &equipement.get_pourcent_bonus_resistance_physique().to_string()))
            .child(TextView::new("% Résistance magique : ".to_string() + &equipement.get_pourcent_bonus_resistance_magique().to_string()))
            .child(TextView::new("Catégorie : ".to_string() + &equipement.get_categorie().to_string()))
            .child(TextView::new("Rareté : ".to_string() + &equipement.get_rarete().to_string()))
        ;
        Dialog::around(layout)
            .title(equipement.get_nom())
            .button("Retirer", move |s| {
                let equipement_type: EquipementType = match equipement.get_categorie() {
                    Categorie::Arme(_) => EquipementType::Arme,
                    Categorie::Armure(Armure::Casque) => EquipementType::Casque,
                    Categorie::Armure(Armure::Plastron) => EquipementType::Plastron,
                    Categorie::Armure(Armure::Gants) => EquipementType::Gants,
                    Categorie::Armure(Armure::Jambieres) => EquipementType::Jambieres,
                    Categorie::Armure(Armure::Bottes) => EquipementType::Bottes
                };
                { MasterFile::get_instance().lock().unwrap().get_joueur_mut().remove_equipement(&equipement_type); }
                s.pop_layer();
                s.pop_layer();
                s.add_layer(equipement_screen());
            })
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran d'attaques
    fn attaques_screen() -> Dialog {
        let attaques_id: Vec<String>;
        { attaques_id = MasterFile::get_instance().lock().unwrap().get_joueur().get_attaques(); };
        let mut attaques: Vec<Attaque> = vec![];
        for attaque_id in attaques_id {
            { attaques.push(MasterFile::get_instance().lock().unwrap().prendre_attaque_id(&attaque_id).expect("Attaque introuvable")); }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..attaques.len() + 1 {
            let attaque = &attaques[i - 1];
            select.add_item(attaque.get_nom().clone(), i.to_string());
            if i % 10 == 0 {
                let attaques_clone = attaques.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let attaque = attaques_clone[index].clone();
                    s.add_layer(create_dialog_attaque(attaque));
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if attaques.len() % 10 != 0 {
            let attaques_clone = attaques.clone();
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let attaque = attaques_clone[index].clone();
                s.add_layer(create_dialog_attaque(attaque));
            });
            layout.add_child(select);
        }
        if attaques.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucune attaque"));
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Attaques")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(informations_screen());
        })
    }

    fn create_dialog_attaque(attaque: Attaque) -> Dialog {
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(attaque.get_description().clone()));
        layout.add_child(DummyView::new().fixed_height(1));
        let string: String;
        match attaque.get_categorie() {
            Arme::ArmeMelee => string = "Melée".to_string(),
            Arme::ArmeDistance => string = "Distance".to_string(),
            Arme::ArmeMagie => string = "Magique".to_string()
        }
        layout.add_child(TextView::new("Catégorie : ".to_string() + &string));
        layout.add_child(TextView::new("Dégats : ".to_string() + &attaque.get_degats().clone().to_string()));
        layout.add_child(TextView::new("Bonus : ".to_string() + &attaque.get_pourcent_bonus_degats().clone().to_string() + "%"));
        Dialog::around(layout)
            .title(attaque.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de l'inventaire
    fn inventaire_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Consommable", 1)
            .item("Ressources", 2)
            .item("Equipements", 3)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(inventaire_consommable());
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(inventaire_ressources());
                }
                else if *choice == 3 {
                    s.pop_layer();
                    s.add_layer(inventaire_equipements());
                }
            })
        )
        .title("Inventaire")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(personnage_screen());
        })
    }

    // Créer un écran de consommables
    fn inventaire_consommable() -> Dialog {
        let inventaire: HashMap<String, u32>;
        { inventaire = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().clone() }
        let mut consommables: Vec<Consommable> = vec![];
        for (id, quantite) in inventaire {
            if quantite > 0 {
                match MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&id) {
                    Ok(consommable) => consommables.push(consommable),
                    Err(_) => {}
                }
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);

        let mut consommable: Consommable;
        for i in 1..consommables.len() + 1 {
            consommable = consommables[i - 1].clone();
            let quantite: u32;
            {
                let nb: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get(&consommable.get_id().clone()).copied();
                match nb {
                    Some(nb) => quantite = nb,
                    None => quantite = 0
                }
            }
            select.add_item(consommable.get_nom().clone().to_string() + " : " + &quantite.to_string(), i.to_string());
            if i % 10 == 0 {
                let consommables_clone = consommables.clone();
                select.set_on_submit(move |s, choice: &String| {
                    if quantite == 0 {
                        s.add_layer(Dialog::text("Vous n'avez plus de ".to_string() + &consommable.get_nom().clone())
                            .title("Consommable")
                            .button("Retour", |s| { s.pop_layer(); })
                        );
                    }
                    else {
                        let index = choice.parse::<usize>().unwrap() - 1;
                        let consommable = consommables_clone[index].clone();
                        s.add_layer(create_dialog_consommable(consommable));
                    }
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if consommables.len() % 10 != 0 {
            let consommables_clone = consommables.clone();
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let consommable = consommables_clone[index].clone();
                s.add_layer(create_dialog_consommable(consommable));
            });
            layout.add_child(select);
        }
        if consommables.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucun consommable."));
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Consommables")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(inventaire_screen());
        })
    }

    fn create_dialog_consommable(consommable: Consommable) -> Dialog {
        let consommable_clone = consommable.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(consommable.get_description().clone()));
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in consommable.get_ressources() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        layout.add_child(TextView::new(decomposer_string));
        layout.add_child(TextView::new("Prix : ".to_owned() + &consommable.clone().get_prix().clone().to_string()));
        let effets = consommable.get_effets().clone();
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(TextView::new("Régenération pv : ".to_string() + &effets[0].to_string()));
        layout.add_child(TextView::new("Force : +".to_string() + &effets[1].to_string() + " en combat"));
        layout.add_child(TextView::new("Dexterité : +".to_string() + &effets[2].to_string() + " en combat"));
        layout.add_child(TextView::new("Intelligence : +".to_string() + &effets[3].to_string() + " en combat"));
        layout.add_child(TextView::new("Vitesse : +".to_string() + &effets[4].to_string() + " en combat"));
        layout.add_child(TextView::new("Esquive : +".to_string() + &effets[5].to_string() + " en combat"));
        layout.add_child(TextView::new("Chance : +".to_string() + &effets[6].to_string() + " en combat"));
        layout.add_child(TextView::new("Résistance physique : +".to_string() + &effets[7].to_string() + " en combat"));
        layout.add_child(TextView::new("Résistance magique : +".to_string() + &effets[8].to_string() + " en combat"));
        layout.add_child(DummyView::new().fixed_height(2));
        layout.add_child(SelectView::new()
            .item("Utiliser", 1)
            .item("Décomposer", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    { MasterFile::get_instance().lock().unwrap().get_joueur_mut().utiliser_item(&consommable, &false) }
                    s.add_layer(Dialog::text("L'objet : ".to_string() + &consommable.get_nom() + " a bien été utilisé")
                        .title("Equiper")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_consommable());
                        }));
                }
                else if *choice == 2 {
                    {
                        let item: Item;
                        { item = match MasterFile::get_instance().lock().unwrap().prendre_item_id(&consommable.get_id().to_string()) {
                            Ok(item) => item,
                            Err(_) => panic!("Item introuvable")
                        }}
                        let mut ressources: HashMap<String, u32> = item.get_ressources().clone();
                        { MasterFile::get_instance().lock().unwrap().get_joueur_mut().remove_inventaire(&consommable.get_id().to_string(), 1); }
                        for (ressource_id, quantite) in ressources {
                            MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_inventaire(ressource_id, quantite);
                        }
                    }
                    s.add_layer(Dialog::text("L'objet : ".to_string() + &consommable.get_nom() + " a bien été décomposer")
                        .title("Décomposer")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_consommable());
                        }));
                }
            }));
        Dialog::around(layout)
            .title(consommable_clone.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de ressources
    fn inventaire_ressources() -> Dialog {
        let inventaire: HashMap<String, u32>;
        { inventaire = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().clone() }
        let mut ressources: Vec<Ressource> = vec![];
        for (id, quantite) in inventaire {
            if quantite > 0 {
                match MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id) {
                    Ok(ressource) => ressources.push(ressource),
                    Err(_) => {}
                }
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);

        let mut ressource: Ressource;
        for i in 1..ressources.len() + 1 {
            ressource = ressources[i - 1].clone();
            let quantite: u32;
            {
                let nb: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get(&ressource.get_id().clone()).copied();
                match nb {
                    Some(nb) => quantite = nb,
                    None => quantite = 0
                }
            }
            select.add_item(ressource.get_nom().clone().to_string() + " : " + &quantite.to_string(), i.to_string());
            if i % 10 == 0 {
                let ressources_clone = ressources.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let ressource = ressources_clone[index].clone();
                    s.add_layer(create_dialog_ressource(ressource));
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if ressources.len() % 10 != 0 {
            let ressources_clone = ressources.clone();
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let ressource = ressources_clone[index].clone();
                s.add_layer(create_dialog_ressource(ressource));
            });
            layout.add_child(select);
        }
        if ressources.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucune ressource."));
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Ressources")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(inventaire_screen());
        })
    }

    fn create_dialog_ressource(ressource: Ressource) -> Dialog {
        let ressource_clone = ressource.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(ressource.get_description().clone()));
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in ressource.get_ressource() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        layout.add_child(TextView::new(decomposer_string));
        layout.add_child(TextView::new("Prix : ".to_owned() + &ressource.clone().get_prix().clone().to_string()));
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(SelectView::new()
            .item("Décomposer", 1)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    {
                        let item: Item;
                        { item = match MasterFile::get_instance().lock().unwrap().prendre_item_id(&ressource.get_id().to_string()) {
                            Ok(item) => item,
                            Err(_) => panic!("Item introuvable")
                        }}
                        let mut ressources: HashMap<String, u32> = item.get_ressources().clone();
                        { MasterFile::get_instance().lock().unwrap().get_joueur_mut().remove_inventaire(&ressource.get_id().to_string(), 1); }
                        for (ressource_id, quantite) in ressources {
                            MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_inventaire(ressource_id, quantite);
                        }
                    }
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &ressource.get_nom() + " a bien été décomposer")
                        .title("Décomposer")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_ressources());
                        }));
                }
            }));
        Dialog::around(layout)
            .title(ressource_clone.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran d'équipement
    fn inventaire_equipements() -> Dialog {
        let inventaire: HashMap<String, u32>;
        { inventaire = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().clone() }
        let mut equipements: Vec<Equipement> = vec![];
        for (id, quantite) in inventaire {
            if quantite > 0 {
                match MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id) {
                    Ok(equipement) => equipements.push(equipement),
                    Err(_) => {}
                }
            }
        }
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);

        let mut equipement: Equipement;
        for i in 1..equipements.len() + 1 {
            equipement = equipements[i - 1].clone();
            let quantite: u32;
            {
                let nb: Option<u32> = MasterFile::get_instance().lock().unwrap().get_joueur().get_inventaire().get(&equipement.get_id().clone()).copied();
                match nb {
                    Some(nb) => quantite = nb,
                    None => quantite = 0
                }
            }
            select.add_item(equipement.get_nom().clone() + " : " + &quantite.to_string(), i.to_string());
            if i % 10 == 0 {
                let equipements_clone = equipements.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let equipement = equipements_clone[index].clone();
                    s.add_layer(create_dialog_equipement(equipement));
                });
                layout.add_child(select);
                select = SelectView::new();
                select.set_inactive_highlight(false);
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if equipements.len() % 10 != 0 {
            let equipements_clone = equipements.clone();
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let equipement = equipements_clone[index].clone();
                s.add_layer(create_dialog_equipement(equipement));
            });
            layout.add_child(select);
        }
        if equipements.len() == 0 {
            layout.add_child(TextView::new("Vous n'avez aucun equipement."));
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Equipements")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(inventaire_screen());
        })
    }

    fn create_dialog_equipement(equipement: Equipement) -> Dialog {
        let equipement_clone = equipement.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(equipement.get_description().clone()));
        let mut decomposer_string: String = "Décomposer => {".to_string();
        for (id, quantite) in equipement.get_ressources() {
            decomposer_string += &(MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&id).unwrap().get_nom().clone().to_string() + " : " + &quantite.to_string() + ", ");
        }
        decomposer_string += "}";
        layout.add_child(TextView::new(decomposer_string));
        layout.add_child(TextView::new("Prix : ".to_owned() + &equipement.clone().get_prix().clone().to_string()));
        let mut layout_caracteristiques: LinearLayout = LinearLayout::horizontal();
        let layout_equipement: LinearLayout = LinearLayout::vertical()
            .child(TextView::new(equipement.get_nom().clone() + " :"))
            .child(LinearLayout::horizontal()
                .child(DummyView::new().fixed_width(2))
                .child(LinearLayout::vertical()
                    .child(TextView::new("Pv : ".to_string() + &equipement.get_bonus_pv().to_string()))
                    .child(TextView::new("Force : ".to_string() + &equipement.get_bonus_force().to_string()))
                    .child(TextView::new("Dexterité : ".to_string() + &equipement.get_bonus_dexterite().to_string()))
                    .child(TextView::new("Intelligence : ".to_string() + &equipement.get_bonus_intelligence().to_string()))
                    .child(TextView::new("Vitesse : ".to_string() + &equipement.get_bonus_vitesse().to_string()))
                    .child(TextView::new("Esquive : ".to_string() + &equipement.get_bonus_esquive().to_string()))
                    .child(TextView::new("Chance : ".to_string() + &equipement.get_bonus_chance().to_string()))
                    .child(TextView::new("Résistance physique : ".to_string() + &equipement.get_bonus_resistance_physique().to_string()))
                    .child(TextView::new("Résistance magique : ".to_string() + &equipement.get_bonus_resistance_magique().to_string()))
                    .child(TextView::new("Mult xp : ".to_string() + &equipement.get_bonus_multiplicateur_xp().to_string()))
                    .child(TextView::new("% Pv : ".to_string() + &equipement.get_pourcent_bonus_pv().to_string()))
                    .child(TextView::new("% Force : ".to_string() + &equipement.get_pourcent_bonus_force().to_string()))
                    .child(TextView::new("% Dexterité : ".to_string() + &equipement.get_pourcent_bonus_dexterite().to_string()))
                    .child(TextView::new("% Intelligence : ".to_string() + &equipement.get_pourcent_bonus_intelligence().to_string()))
                    .child(TextView::new("% Vitesse : ".to_string() + &equipement.get_pourcent_bonus_vitesse().to_string()))
                    .child(TextView::new("% Esquive : ".to_string() + &equipement.get_pourcent_bonus_esquive().to_string()))
                    .child(TextView::new("% Chance : ".to_string() + &equipement.get_pourcent_bonus_chance().to_string()))
                    .child(TextView::new("% Résistance physique : ".to_string() + &equipement.get_pourcent_bonus_resistance_physique().to_string()))
                    .child(TextView::new("% Résistance magique : ".to_string() + &equipement.get_pourcent_bonus_resistance_magique().to_string()))
                    .child(TextView::new("Catégorie : ".to_string() + &equipement.get_categorie().to_string()))
                    .child(TextView::new("Rareté : ".to_string() + &equipement.get_rarete().to_string()))
                )
            )
        ;
        let maybe: Option<Option<String>> = match &equipement.get_categorie() {
            Categorie::Arme(Arme::ArmeMelee) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Arme).cloned(),
            Categorie::Arme(Arme::ArmeDistance) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Arme).cloned(),
            Categorie::Arme(Arme::ArmeMagie) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Arme).cloned(),
            Categorie::Armure(Armure::Casque) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Casque).cloned(),
            Categorie::Armure(Armure::Plastron) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Plastron).cloned(),
            Categorie::Armure(Armure::Gants) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Gants).cloned(),
            Categorie::Armure(Armure::Jambieres) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Jambieres).cloned(),
            Categorie::Armure(Armure::Bottes) => MasterFile::get_instance().lock().unwrap().get_joueur().get_equipement().get_mut(&EquipementType::Bottes).cloned(),
        };
        let equipement_actuel_id: Option<String> = match maybe {
            Some(id) => id,
            None => None
        };
        let mut layout_equipement_actuel: LinearLayout = LinearLayout::vertical();
        match equipement_actuel_id {
            Some(ref id) => {
                let equipement_actuel: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).unwrap();
                layout_equipement_actuel.add_child(TextView::new("Equipement actuel : ".to_string() + &equipement_actuel.get_nom().clone() + " :"));
                layout_equipement_actuel.add_child(LinearLayout::horizontal()
                    .child(DummyView::new().fixed_width(2))
                    .child(LinearLayout::vertical()
                        .child(TextView::new("Pv : ".to_string() + &equipement_actuel.get_bonus_pv().to_string()))
                        .child(TextView::new("Force : ".to_string() + &equipement_actuel.get_bonus_force().to_string()))
                        .child(TextView::new("Dexterité : ".to_string() + &equipement_actuel.get_bonus_dexterite().to_string()))
                        .child(TextView::new("Intelligence : ".to_string() + &equipement_actuel.get_bonus_intelligence().to_string()))
                        .child(TextView::new("Vitesse : ".to_string() + &equipement_actuel.get_bonus_vitesse().to_string()))
                        .child(TextView::new("Esquive : ".to_string() + &equipement_actuel.get_bonus_esquive().to_string()))
                        .child(TextView::new("Chance : ".to_string() + &equipement_actuel.get_bonus_chance().to_string()))
                        .child(TextView::new("Résistance physique : ".to_string() + &equipement_actuel.get_bonus_resistance_physique().to_string()))
                        .child(TextView::new("Résistance magique : ".to_string() + &equipement_actuel.get_bonus_resistance_magique().to_string()))
                        .child(TextView::new("Mult xp : ".to_string() + &equipement_actuel.get_bonus_multiplicateur_xp().to_string()))
                        .child(TextView::new("% Pv : ".to_string() + &equipement_actuel.get_pourcent_bonus_pv().to_string()))
                        .child(TextView::new("% Force : ".to_string() + &equipement_actuel.get_pourcent_bonus_force().to_string()))
                        .child(TextView::new("% Dexterité : ".to_string() + &equipement_actuel.get_pourcent_bonus_dexterite().to_string()))
                        .child(TextView::new("% Intelligence : ".to_string() + &equipement_actuel.get_pourcent_bonus_intelligence().to_string()))
                        .child(TextView::new("% Vitesse : ".to_string() + &equipement_actuel.get_pourcent_bonus_vitesse().to_string()))
                        .child(TextView::new("% Esquive : ".to_string() + &equipement_actuel.get_pourcent_bonus_esquive().to_string()))
                        .child(TextView::new("% Chance : ".to_string() + &equipement_actuel.get_pourcent_bonus_chance().to_string()))
                        .child(TextView::new("% Résistance physique : ".to_string() + &equipement_actuel.get_pourcent_bonus_resistance_physique().to_string()))
                        .child(TextView::new("% Résistance magique : ".to_string() + &equipement_actuel.get_pourcent_bonus_resistance_magique().to_string()))
                        .child(TextView::new("Catégorie : ".to_string() + &equipement_actuel.get_categorie().to_string()))
                        .child(TextView::new("Rareté : ".to_string() + &equipement_actuel.get_rarete().to_string()))
                    )
                );
            },
            None => layout_equipement_actuel.add_child(TextView::new("Aucun equipement actuellement"))
        }
        layout_caracteristiques.add_child(layout_equipement);
        layout_caracteristiques.add_child(layout_equipement_actuel);
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(ScrollView::new(layout_caracteristiques).scroll_x(true).scroll_y(true));
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(SelectView::new()
            .item("Equiper", 1)
            .item("Décomposer", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    match &equipement_actuel_id {
                        Some(id) => {
                            let value = equipement.clone();
                            s.add_layer(Dialog::text("Un objet est déjà équipé dans cette emplacement. Voulez vous le remplacer ?".to_string())
                                .title("Confirmation")
                                .button("Oui", move |s| {
                                    let equipement_type: EquipementType = match value.get_categorie() {
                                        Categorie::Arme(Arme::ArmeMelee) => EquipementType::Arme,
                                        Categorie::Arme(Arme::ArmeDistance) => EquipementType::Arme,
                                        Categorie::Arme(Arme::ArmeMagie) => EquipementType::Arme,
                                        Categorie::Armure(Armure::Casque) => EquipementType::Casque,
                                        Categorie::Armure(Armure::Plastron) => EquipementType::Plastron,
                                        Categorie::Armure(Armure::Gants) => EquipementType::Gants,
                                        Categorie::Armure(Armure::Jambieres) => EquipementType::Jambieres,
                                        Categorie::Armure(Armure::Bottes) => EquipementType::Bottes
                                    };
                                    { MasterFile::get_instance().lock().unwrap().get_joueur_mut().remove_equipement(&equipement_type); }
                                    { MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_equipement(&equipement_type, &value.get_id().clone()); }
                                    s.pop_layer();
                                    s.pop_layer();
                                    s.pop_layer();
                                    s.add_layer(inventaire_equipements());
                                })
                                .button("Non", |s| {
                                    s.pop_layer();
                                }));
                        }
                        None => {
                            let equipement_type: EquipementType = match equipement.get_categorie() {
                                Categorie::Arme(Arme::ArmeMelee) => EquipementType::Arme,
                                Categorie::Arme(Arme::ArmeDistance) => EquipementType::Arme,
                                Categorie::Arme(Arme::ArmeMagie) => EquipementType::Arme,
                                Categorie::Armure(Armure::Casque) => EquipementType::Casque,
                                Categorie::Armure(Armure::Plastron) => EquipementType::Plastron,
                                Categorie::Armure(Armure::Gants) => EquipementType::Gants,
                                Categorie::Armure(Armure::Jambieres) => EquipementType::Jambieres,
                                Categorie::Armure(Armure::Bottes) => EquipementType::Bottes
                            };
                            { MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_equipement(&equipement_type, &equipement.get_id().clone()); }
                            s.add_layer(Dialog::text("L'équipement : ".to_owned() + &equipement.get_nom() + " a bien été équipé")
                                .title("Equiper")
                                .button("Ok", |s| {
                                    s.pop_layer();
                                    s.pop_layer();
                                    s.pop_layer();
                                    s.add_layer(inventaire_equipements());
                                })
                            )
                        }
                    }
                }
                else if *choice == 2 {
                    {
                        let ressources: HashMap<String, u32>;
                        { ressources = MasterFile::get_instance().lock().unwrap().prendre_item_id(&equipement.get_id().clone()).expect("Ressource introuvable").get_ressources().clone(); }
                        { MasterFile::get_instance().lock().unwrap().get_joueur_mut().remove_inventaire(&equipement.get_id().clone(), 1); }
                        for (ressource_id, quantite) in ressources {
                            MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_inventaire(ressource_id, quantite);
                        }
                    }
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &equipement.get_nom() + " a bien été décomposer")
                        .title("Décomposer")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_equipements());
                        }));
                }
            }));
        Dialog::around(layout)
            .title(equipement_clone.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de quête
    fn quetes_screen() -> Dialog {
        let mut quetes: Vec<Quete> = vec![];
        {
            let quetes_id: Vec<String>;
            { quetes_id = MasterFile::get_instance().lock().unwrap().get_joueur().get_quetes(); }
            for id in quetes_id {
                quetes.push(MasterFile::get_instance().lock().unwrap().prendre_quete_id(&id).expect("Quête introuvable"));
            }
        }
        let mut select = SelectView::new();
        for quete in quetes {
            select.add_item(quete.get_nom().clone(), quete.get_id().clone());
        }
        select.set_on_submit(|s, choice: &String| {
            let quete: Quete;
            { quete = MasterFile::get_instance().lock().unwrap().prendre_quete_id(&choice).expect("Quête introuvable"); }
            s.add_layer(create_quete_dialog(quete))
        });
        Dialog::around(ScrollView::new(select).scroll_x(true).scroll_y(true))
            .title("Quête")
            .button("Retour", |s| {
                s.pop_layer();
                s.add_layer(personnage_screen());
            })
    }

    fn create_quete_dialog(quete: Quete) -> Dialog {
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(quete.get_description().clone()));
        layout.add_child(TextView::new("Récompenses :"));
        let mut layout_recompenses: LinearLayout = LinearLayout::vertical();
        for (ressource_id, quantite) in quete.get_recompense() {
            let consommable: Result<Consommable, String>;
            { consommable = MasterFile::get_instance().lock().unwrap().prendre_consommable_id(&ressource_id); }
            if consommable.is_ok() {
                layout_recompenses.add_child(TextView::new(consommable.unwrap().get_nom().clone() + " : " + &quantite.to_string()));
            }
            let ressource: Result<Ressource, String>;
            { ressource = MasterFile::get_instance().lock().unwrap().prendre_ressource_id(&ressource_id); }
            if ressource.is_ok() {
                layout_recompenses.add_child(TextView::new(ressource.unwrap().get_nom().clone() + " : " + &quantite.to_string()));
            }
            let equipement: Result<Equipement, String>;
            { equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&ressource_id); }
            if equipement.is_ok() {
                layout_recompenses.add_child(TextView::new(equipement.unwrap().get_nom().clone() + " : " + &quantite.to_string()));
            }
        }
        layout.add_child(LinearLayout::horizontal()
            .child(DummyView::new().fixed_width(2))
            .child(layout_recompenses)
        );
        Dialog::around(layout)
            .title(quete.get_nom().clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de menu
    fn menu_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Sauvegarder", 1)
            .item("Recharger", 2)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    { MasterFile::get_instance().lock().unwrap().sauvegarder(); }
                    s.add_layer(Dialog::text("Sauvegarde effectuée").title("Sauvegarde").button("Ok", |s| {
                        s.pop_layer();
                    }));
                }
                else if *choice == 2 {
                    { MasterFile::get_instance().lock().unwrap().recharger(); }
                    s.add_layer(Dialog::text("Rechargement de la dernière sauvegarde effectuée").title("Rechargement").button("Ok", |s| {
                        s.pop_layer();
                    }));
                }
            })
        )
        .title("Menu")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(game_screen());
        })
        .button("Quitter", |s| s.quit())
    }


    siv.add_layer(main_menu_screen());
    siv.run();

    /*
    let mut position = "".to_string();
    {
        println!("===== Joueur initial =====");
        println!("{}", MasterFile::get_instance().lock().unwrap().get_joueur());
    }

    {
        let master_file: &mut MasterFile = &mut MasterFile::get_instance().lock().unwrap();
        let joueur = master_file.get_joueur_mut();
        joueur.set_position("piece1".to_string());
        println!("===== Joueur modifie piece1 =====");
        println!("{}", joueur);
    }

    {
        println!("===== Joueur modifie vérification =====");
        println!("{}", MasterFile::get_instance().lock().unwrap().get_joueur());
        position = MasterFile::get_instance().lock().unwrap().get_joueur().get_position();
    }

    {
        let item_id = "arc";
        let ressources: HashMap<String, u32>;
        { ressources = MasterFile::get_instance().lock().unwrap().prendre_item_id(item_id).expect("Ressource introuvable").get_ressources().clone(); }
        { MasterFile::get_instance().lock().unwrap().get_joueur_mut().remove_inventaire(&item_id.to_string(), 1); }
        for (ressource_id, quantite) in ressources {
            MasterFile::get_instance().lock().unwrap().get_joueur_mut().add_inventaire(ressource_id, quantite);
        }
        println!("===== Joueur modifie dementelement arc =====");
        println!("{}", MasterFile::get_instance().lock().unwrap().get_joueur());
    }

    {
        println!("===== Joueur modifie vérification =====");
        println!("{}", MasterFile::get_instance().lock().unwrap().get_joueur());
    }

    println!("{}", position);

    //{ MasterFile::get_instance().lock().unwrap().sauvegarder(); }
    */
}