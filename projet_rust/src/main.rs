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
use equipement::{Equipement, Arme};
use structs::{EquipementType, Ressource};
use attaque::Attaque;
use json_manager::{MasterFile, Item};
//use combat::combat;

use cursive::views::{Dialog, TextView, SelectView, LinearLayout, ScrollView, DummyView};
use cursive::view::{Resizable};
use cursive::{Cursive, CursiveExt};

use std::collections::HashMap;

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
                    //s.add_layer(interaction_screen());
                    s.add_layer(Dialog::text("TODO: Intéragir").title("Intéragir").button("Retour", |s| { s.pop_layer(); }));
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
        .title("Se déplacer")
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
                    //s.add_layer(quetes_screen());
                    s.add_layer(Dialog::text("TODO: Quêtes").title("Quêtes").button("Retour", |s| { s.pop_layer(); }));
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
        let pv: u16;
        let force: u16;
        let dexterite: u16;
        let intelligence: u16;
        let vitesse: u16;
        let esquive: u16;
        let chance: u16;
        let resistance_physique: u16;
        let resistance_magique: u16;
        { pv = MasterFile::get_instance().lock().unwrap().get_joueur().get_pv_max(); }
        { force = MasterFile::get_instance().lock().unwrap().get_joueur().get_force(); }
        { dexterite = MasterFile::get_instance().lock().unwrap().get_joueur().get_dexterite(); }
        { intelligence = MasterFile::get_instance().lock().unwrap().get_joueur().get_intelligence(); }
        { vitesse = MasterFile::get_instance().lock().unwrap().get_joueur().get_vitesse(); }
        { esquive = MasterFile::get_instance().lock().unwrap().get_joueur().get_esquive(); }
        { chance = MasterFile::get_instance().lock().unwrap().get_joueur().get_chance(); }
        { resistance_physique = MasterFile::get_instance().lock().unwrap().get_joueur().get_resistance_physique(); }
        { resistance_magique = MasterFile::get_instance().lock().unwrap().get_joueur().get_resistance_magique(); }
        layout.add_child(TextView::new("pv : ".to_string() + &pv.to_string()));
        layout.add_child(TextView::new("force : ".to_string() + &force.to_string()));
        layout.add_child(TextView::new("dexterite : ".to_string() + &dexterite.to_string()));
        layout.add_child(TextView::new("intelligence : ".to_string() + &intelligence.to_string()));
        layout.add_child(TextView::new("vitesse : ".to_string() + &vitesse.to_string()));
        layout.add_child(TextView::new("esquive : ".to_string() + &esquive.to_string()));
        layout.add_child(TextView::new("chance : ".to_string() + &chance.to_string()));
        layout.add_child(TextView::new("resistance physique : ".to_string() + &resistance_physique.to_string()));
        layout.add_child(TextView::new("resistance magique : ".to_string() + &resistance_magique.to_string()));
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
        let mut layout: LinearLayout = LinearLayout::vertical();
        for (equipement_type, equipement_id) in equipements {
            let line_str: &str;
            match equipement_type {
                EquipementType::Casque => line_str = "Casque : ",
                EquipementType::Plastron => line_str = "Plastron : ",
                EquipementType::Gants => line_str = "Gants : ",
                EquipementType::Jambieres => line_str = "Jambieres : ",
                EquipementType::Bottes => line_str = "Bottes : ",
                EquipementType::Arme => line_str = "Arme : "
            }
            match equipement_id {
                Some(id) => {
                    let equipement: Equipement = MasterFile::get_instance().lock().unwrap().prendre_equipement_id(&id).expect("Equipement introuvable");
                    layout.add_child(TextView::new(line_str.to_owned() + &equipement.get_nom().clone()));
                }
                None => {
                    layout.add_child(TextView::new(line_str.to_owned() + "Non equippé"));
                }
            }
        }
        Dialog::around(layout)
            .title("Equipement")
            .button("Retour", |s| {
                s.pop_layer();
                s.pop_layer();
                s.add_layer(informations_screen());
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
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(SelectView::new()
            .item("Utiliser", 1)
            .item("Décomposer", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    //TODO: utiliser le consommable
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
        layout.add_child(DummyView::new().fixed_height(1));
        layout.add_child(SelectView::new()
            .item("Equiper", 1)
            .item("Décomposer", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    //TODO: equiper l'equipement
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &equipement.get_nom() + " a bien été équiper")
                        .title("Equiper")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_equipements());
                        }));
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