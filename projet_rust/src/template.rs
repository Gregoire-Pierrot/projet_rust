use cursive::views::{Dialog, TextView, SelectView, LinearLayout, ScrollView, DummyView};
use cursive::view::{Resizable};
use cursive::{Cursive, CursiveExt};

fn get_lieux() -> Vec<Lieu> {
    let mut lieux: Vec<Lieu> = Vec::new();
    for i in 1..21 {
        lieux.push(Lieu {
            nom: "Lieu n°".to_string() + &i.to_string(),
            description: "Description du lieu n°".to_string() + &i.to_string(),
            id: "lieu_id_".to_string() + &i.to_string()
        })
    }
    lieux
}

fn get_attaques() -> Vec<Attaque> {
    let mut attaques: Vec<Attaque> = Vec::new();
    for i in 1..6 {
        attaques.push(Attaque {
            nom: "Attaque n°".to_string() + &i.to_string(),
            description: "Description de l'attaque n°".to_string() + &i.to_string(),
            categorie: "Categorie de l'attaque n°".to_string() + &i.to_string(),
            degats: i,
            pourcent_bonus_degats: i
        })
    }
    attaques
}

fn get_items() -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    for i in 1..43 {
        items.push(Item {
            nom: "Item n°".to_string() + &i.to_string(),
            description: "Description de l'item n°".to_string() + &i.to_string()
        })
    }
    items
}

fn get_equipement() -> Vec<Equipement> {
    let mut equipements: Vec<Equipement> = Vec::new();
    equipements.push(Equipement {
        nom: "Casque de feu".to_string(),
        description: "Description du casque de feu".to_string(),
        categorie: Categorie::Casque
    });
    equipements.push(Equipement {
        nom: "Plastron de feu".to_string(),
        description: "Description du plastron de feu".to_string(),
        categorie: Categorie::Plastron
    });
    equipements.push(Equipement {
        nom: "Gants de feu".to_string(),
        description: "Description des gants de feu".to_string(),
        categorie: Categorie::Gants
    });
    equipements.push(Equipement {
        nom: "Jambières de feu".to_string(),
        description: "Description des jambières de feu".to_string(),
        categorie: Categorie::Jambieres
    });
    equipements.push(Equipement {
        nom: "Bottes de feu".to_string(),
        description: "Description des bottes de feu".to_string(),
        categorie: Categorie::Bottes
    });
    equipements
}

fn get_quetes() -> Vec<Quete> {
    let mut quetes: Vec<Quete> = Vec::new();
    for i in 1..8 {
        quetes.push(Quete {
            nom: "Quete n°".to_string() + &i.to_string(),
            description: "Description de la quete n°".to_string() + &i.to_string(),
            recompense: "Recompense de la quete n°".to_string() + &i.to_string()
        })
    }
    quetes
}

fn main() {
    let mut siv = Cursive::default();

    // Créez un écran de menu principal
    fn main_menu_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Jouer", 1)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(Dialog::text("Chargement du jeu... Veuillez patienter").title("Chargement"));
                    // *chargement du jeu et vérification du json
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
        Dialog::around(SelectView::new()
            .item("Se déplacer", 1)
            .item("Intéragir", 2)
            .item("Personnage", 3)
            .item("Menu", 4)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(move_screen());
                }
                else if *choice == 2 {
                    s.pop_layer();
                    s.add_layer(interaction_screen());
                }
                else if *choice == 3 {
                    s.pop_layer();
                    s.add_layer(personnage_screen());
                }
                else if *choice == 4 {
                    s.pop_layer();
                    s.add_layer(menu_screen());
                }
            })
        )
        .title("Jeu")
    }

    // Créer un écran de déplacement
    fn move_screen() -> Dialog {
        let lieux: Vec<Lieu> = get_lieux();
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..lieux.len() + 1 {
            let lieu = &lieux[i - 1];
            select.add_item(lieu.nom.clone(), i.to_string());
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
        Dialog::around(LinearLayout::vertical()
            .child(TextView::new(lieu.description.clone()))
            .child(SelectView::new()
                .item("S'y rendre", 1)
                .on_submit(|s, choice| {
                    if *choice == 1 {
                        //TODO: déplacer le joueur dans le lieu
                        s.pop_layer();
                        s.pop_layer();
                        s.add_layer(game_screen());
                    }
                }))
        )
        .title(lieu.nom.clone())
        .button("Retour", |s| {
            s.pop_layer();
        })
    }

    // Créer un écran d'intéraction
    fn interaction_screen() -> Dialog {
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
                    //TODO: combattre
                }
            })
        )
        .title("Intéraction")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(game_screen());
        })
    }

    // Créer un écran d'analyse de zone
    fn analyse_screen() -> Dialog {
        //TODO: récupérer le lieu actuel
        let lieu: Lieu = get_lieux()[0].clone();
        Dialog::around(LinearLayout::vertical()
            .child(TextView::new(lieu.description.clone()))
            //TODO: afficher les ressources, pnj et ennemies présent dans la zone
        )
        .title("Analyse de zone")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(interaction_screen());
        })
    }

    // Créer un écran de récolte
    fn recolter_screen() -> Dialog {
        let ressources: Vec<Item> = get_items(); // TODO: récupérer la bonne liste
        let consommables: Vec<Item> = get_items(); // TODO: récupérer la bonne liste
        let equipements: Vec<Item> = get_items(); // TODO: récupérer la bonne liste
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..ressources.len() + 1 {
            let ressource = &ressources[i - 1];
            select.add_item(ressource.nom.clone(), i.to_string());
            if i % 10 == 0 {
                let ressources_clone = ressources.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let ressource = ressources_clone[index].clone();
                    s.add_layer(create_dialog_recolte(ressource));
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
                s.add_layer(create_dialog_recolte(ressource));
            });
            layout.add_child(select);
        }
        layout.add_child(DummyView::new().fixed_width(6));
        select = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..consommables.len() + 1 {
            let consommable = &consommables[i - 1];
            select.add_item(consommable.nom.clone(), i.to_string());
            if i % 10 == 0 {
                let consommables_clone = consommables.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let consommable = consommables_clone[index].clone();
                    s.add_layer(create_dialog_recolte(consommable));
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
                s.add_layer(create_dialog_recolte(consommable));
            });
            layout.add_child(select);
        }
        layout.add_child(DummyView::new().fixed_width(6));
        select = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..equipements.len() + 1 {
            let equipement = &equipements[i - 1];
            select.add_item(equipement.nom.clone(), i.to_string());
            if i % 10 == 0 {
                let equipements_clone = equipements.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let equipement = equipements_clone[index].clone();
                    s.add_layer(create_dialog_recolte(equipement));
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
                s.add_layer(create_dialog_recolte(equipement));
            });
            layout.add_child(select);
        }
        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Récolter")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(interaction_screen());
        })
    }

    fn create_dialog_recolte(item: Item) -> Dialog {
        let item_clone = item.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(item.description.clone()));
        layout.add_child(SelectView::new()
            .item("Récolter", 1)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &item.nom + " a bien été recolter")
                        .title("Récolter")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(recolter_screen());
                        }));
                }
            })
        );
        Dialog::around(layout)
        .title(&item_clone.nom)
        .button("Retour", |s| {
            s.pop_layer();
        })
    }

    // Créer un écran de pnj
    fn pnj_screen() -> Dialog {
        Dialog::around(TextView::new("TODO"))
        .title("PNJ")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(interaction_screen());
        })
    }

    // Créer un écran de personnage
    fn personnage_screen() -> Dialog {
        Dialog::around(SelectView::new()
            .item("Informations", 1)
            .item("Inventaire", 2)
            .item("Equipement", 3)
            .item("Quêtes", 4)
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
                    s.add_layer(equipement_screen());
                }
                else if *choice == 4 {
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
            .item("Attaques", 3)
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
        let points = 0;
        // points = get_player_points();
        layout.add_child(TextView::new("Points à attribuer : ".to_string() + &points.to_string()));
        layout.add_child(SelectView::new()
            .item("Choisir une statistique", 1)
            .on_submit(|s, choice| {
                if *choice == 1 {
                    s.pop_layer();
                    s.add_layer(stat_choice());
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
        let select_stats = SelectView::new()
            .item("pv", 1)
            .item("force", 2)
            .item("dexterite", 3)
            .item("intelligence", 4)
            .item("vitesse", 5)
            .item("esquive", 6)
            .item("chance", 7)
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
                match stat.as_str() {
                    /*
                    "pv" => {} //TODO: rajouter un point de pv
                    "force" => {} //TODO: rajouter un point de force
                    "dexterite" => {} //TODO: rajouter un point de dexterite
                    "intelligence" => {} //TODO: rajouter un point d'intelligence
                    "vitesse" => {} //TODO: rajouter un point de vitesse
                    "esquive" => {} //TODO: rajouter un point d'esquive
                    "chance" => {} //TODO: rajouter un point de chance
                    */
                    _ => {}
                }
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
        let pv = 0; // TODO: prendre les pv du joueur
        let force = 0; // TODO: prendre la force du joueur
        let dexterite = 0; // TODO: prendre la dexterite du joueur
        let intelligence = 0; // TODO: prendre l'intelligence du joueur
        let vitesse = 0; // TODO: prendre la vitesse du joueur
        let esquive = 0; // TODO: prendre l'esquive du joueur
        let chance = 0; // TODO: prendre la chance du joueur
        let resistance_physique = 0; // TODO: prendre la resistance physique du joueur
        let resistance_magique = 0; // TODO: prendre la resistance magique du joueur
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

    // Créer un écran d'attaques
    fn attaques_screen() -> Dialog {
        let attaques: Vec<Attaque> = get_attaques();
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..attaques.len() + 1 {
            let attaque = &attaques[i - 1];
            select.add_item(attaque.nom.clone(), i.to_string());
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
        Dialog::around(LinearLayout::vertical()
            .child(TextView::new(attaque.description.clone()))
            .child(TextView::new("Catégorie : ".to_string() + &attaque.categorie.clone()))
            .child(TextView::new("Dégats : ".to_string() + &attaque.degats.clone().to_string()))
            .child(TextView::new("Bonus : ".to_string() + &attaque.pourcent_bonus_degats.clone().to_string() + "%")))
        .title(attaque.nom.clone())
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
        let consommables: Vec<Item> = get_items();
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..consommables.len() + 1 {
            let consommable = &consommables[i - 1];
            select.add_item(consommable.nom.clone(), i.to_string());
            if i % 10 == 0 {
                let consommables_clone = consommables.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let consommable = consommables_clone[index].clone();
                    s.add_layer(create_dialog_consommable(consommable));
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

    fn create_dialog_consommable(consommable: Item) -> Dialog {
        let consommable_clone = consommable.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(consommable.description.clone()));
        layout.add_child(SelectView::new()
            .item("Utiliser", 1)
            .item("Décomposer", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    //TODO: utiliser le consommable
                    s.add_layer(Dialog::text("L'objet : ".to_string() + &consommable.nom + " a bien été utilisé")
                        .title("Equiper")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_consommable());
                        }));
                }
                else if *choice == 2 {
                    //TODO: décomposer l'equipement
                    s.add_layer(Dialog::text("L'objet : ".to_string() + &consommable.nom + " a bien été décomposer")
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
            .title(consommable_clone.nom.clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de ressources
    fn inventaire_ressources() -> Dialog {
        let ressources: Vec<Item> = get_items();
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..ressources.len() + 1 {
            let ressource = &ressources[i - 1];
            select.add_item(ressource.nom.clone(), i.to_string());
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

    fn create_dialog_ressource(ressource: Item) -> Dialog {
        let ressource_clone = ressource.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(ressource.description.clone()));
        layout.add_child(SelectView::new()
            .item("Décomposer", 1)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    //TODO: décomposer l'equipement
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &ressource.nom + " a bien été décomposer")
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
            .title(ressource_clone.nom.clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran d'équipement
    fn inventaire_equipements() -> Dialog {
        let equipements: Vec<Item> = get_items();
        let mut layout: LinearLayout = LinearLayout::horizontal();
        let mut select: SelectView = SelectView::new();
        select.set_inactive_highlight(false);
        for i in 1..equipements.len() + 1 {
            let equipement = &equipements[i - 1];
            select.add_item(equipement.nom.clone(), i.to_string());
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

    fn create_dialog_equipement(equipement: Item) -> Dialog {
        let equipement_clone = equipement.clone();
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(equipement.description.clone()));
        layout.add_child(SelectView::new()
            .item("Equiper", 1)
            .item("Décomposer", 2)
            .on_submit(move |s, choice| {
                if *choice == 1 {
                    //TODO: equiper l'equipement
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &equipement.nom + " a bien été équiper")
                        .title("Equiper")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                            s.pop_layer();
                            s.add_layer(inventaire_equipements());
                        }));
                }
                else if *choice == 2 {
                    //TODO: décomposer l'equipement
                    s.add_layer(Dialog::text("L'objet : ".to_owned() + &equipement.nom + " a bien été décomposer")
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
            .title(equipement_clone.nom.clone())
            .button("Retour", |s| {
                s.pop_layer();
            })
    }

    // Créer un écran de l'équipement du joueur
    fn equipement_screen() -> Dialog {
        let equipements: Vec<Equipement> = get_equipement();
        let mut layout: LinearLayout = LinearLayout::vertical();
        for equipement in equipements {
            let mut line_str = "";
            match equipement.categorie {
                Categorie::Casque => line_str = "Casque : ",
                Categorie::Plastron => line_str = "Plastron : ",
                Categorie::Gants => line_str = "Gants : ",
                Categorie::Jambieres => line_str = "Jambieres : ",
                Categorie::Bottes => line_str = "Bottes : ",
                Categorie::Arme => line_str = "Arme : ",
                _ => {}
            }
            layout.add_child(TextView::new(line_str.to_owned() + &equipement.nom.clone()));
        }
        Dialog::around(layout)
            .title("Equipement")
            .button("Retour", |s| {
                s.pop_layer();
                s.pop_layer();
                s.add_layer(personnage_screen());
            })
    }

    // Créer un écran de quêtes
    fn quetes_screen() -> Dialog {
        let quetes: Vec<Quete> = get_quetes();
        let mut layout: LinearLayout = LinearLayout::vertical();
        let mut select: SelectView = SelectView::new();
        for i in 1..quetes.len() + 1 {
            let quete = &quetes[i - 1];
            select.add_item(quete.nom.clone(), i.to_string());
            if i % 10 == 0 {
                let quetes_clone = quetes.clone();
                select.set_on_submit(move |s, choice: &String| {
                    let index = choice.parse::<usize>().unwrap() - 1;
                    let quete = quetes_clone[index].clone();
                    s.add_layer(create_dialog_quete(quete));
                });
                layout.add_child(select);
                select = SelectView::new();
            }
            if i % 10 == 1 && i != 1 {
                layout.add_child(DummyView::new().fixed_width(5));
            }
        }
        if quetes.len() % 10 != 0 {
            let quetes_clone = quetes.clone();
            select.set_on_submit(move |s, choice: &String| {
                let index = choice.parse::<usize>().unwrap() - 1;
                let quete = quetes_clone[index].clone();
                s.add_layer(create_dialog_quete(quete));
            });
            layout.add_child(select);
        }

        Dialog::around(ScrollView::new(layout)
            .scroll_x(true)
            .scroll_y(true)
        )
        .title("Quetes")
        .button("Retour", |s| {
            s.pop_layer();
            s.add_layer(inventaire_screen());
        })
    }

    fn create_dialog_quete(quete: Quete) -> Dialog {
        let mut layout: LinearLayout = LinearLayout::vertical();
        layout.add_child(TextView::new(quete.description.clone()));
        layout.add_child(TextView::new(quete.recompense.clone()));
        Dialog::around(layout)
            .title(quete.nom.clone())
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
                    //TODO: sauvegarder le jeu
                    s.add_layer(Dialog::text("Sauvegarde effectuée").title("Sauvegarde").button("Ok", |s| {
                        s.pop_layer();
                    }));
                }
                else if *choice == 2 {
                    //TODO: recharger le jeu
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
}

#[derive(Debug, Clone)]
pub struct Lieu {
    pub nom: String,
    pub description: String,
    pub id: String,
}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lieu : nom = {}, description = {}", self.nom, self.description)
    }
}

#[derive(Debug, Clone)]
pub struct Attaque {
    pub nom: String,
    pub description: String,
    pub categorie: String,
    pub degats: u32,
    pub pourcent_bonus_degats: u32
}

impl std::fmt::Display for Attaque {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attaque : nom = {}, description = {}, categorie = {}, degats = {}, pourcent_bonus_degats = {}",
            self.nom, self.description, self.categorie, self.degats, self.pourcent_bonus_degats)
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub nom: String,
    pub description: String
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item : nom = {}, description = {}", self.nom, self.description)
    }
}

#[derive(Debug, Clone)]
pub enum Categorie {
    Casque,
    Plastron,
    Gants,
    Jambieres,
    Bottes,
    Arme
}

impl std::fmt::Display for Categorie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Equipement {
    pub nom: String,
    pub description: String,
    pub categorie: Categorie
}

impl std::fmt::Display for Equipement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Equipement : nom = {}, description = {}, categorie = {}", self.nom, self.description, self.categorie)
    }
}

#[derive(Debug, Clone)]
pub struct Quete {
    pub nom: String,
    pub description: String,
    pub recompense: String
}

impl std::fmt::Display for Quete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Quete : nom = {}, description = {}, recompense = {}", self.nom, self.description, self.recompense)
    }
}