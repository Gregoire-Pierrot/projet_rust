use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::structs::{Personnage, EquipementType};
use crate::json_manager::MasterFile;
use crate::equipement::{Categorie, Arme};
use crate::structs::Ressource;
use crate::equipement::Equipement;
use crate::consommable::Consommable;
use crate::quete::Quete;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur {
    personnage: Personnage,
    position: String,
    pronom: String,
    niveau: u8,
    temps: u32,
    reputations: Vec<u16>,
    xp: u32,
    multiplicateur_xp: u16,
    points_competence: u8,
    quetes: Vec<String>
}

impl Joueur {
    pub fn get_id(&self) -> String { self.personnage.entite.id.clone() }

    pub fn get_description(&self) -> String { self.personnage.entite.description.clone() }

    pub fn get_personnage(&self) -> Personnage {self.personnage.clone()}

    pub fn get_nom(&self) -> String { self.personnage.entite.nom.clone() }
    pub fn set_nom(&mut self, nom: String) { self.personnage.entite.nom = nom; }

    pub fn get_pv_max(&self) -> u16 { self.personnage.pv_max.clone() }
    pub fn set_pv_max(&mut self, pv_max: u16) {self.personnage.pv_max = pv_max}

    pub fn get_pv_actuel(&self) -> u16 { self.personnage.pv_actuel.clone() }
    pub fn set_pv_actuel(&mut self, pv_actuel: u16) { self.personnage.pv_actuel = pv_actuel; }

    pub fn get_force(&self) -> u16 { self.personnage.force.clone() }
    pub fn set_force(&mut self, force: u16) { self.personnage.force = force; }

    pub fn get_dexterite(&self) -> u16 { self.personnage.dexterite.clone() }
    pub fn set_dexterite(&mut self, dexterite: u16) { self.personnage.dexterite = dexterite; }

    pub fn get_intelligence(&self) -> u16 { self.personnage.intelligence.clone() }
    pub fn set_intelligence(&mut self, intelligence: u16) { self.personnage.intelligence = intelligence; }

    pub fn get_vitesse(&self) -> u16 { self.personnage.vitesse.clone() }
    pub fn set_vitesse(&mut self, vitesse: u16) { self.personnage.vitesse = vitesse; }

    pub fn get_esquive(&self) -> u16 { self.personnage.esquive.clone() }
    pub fn set_esquive(&mut self, esquive: u16) { self.personnage.esquive = esquive; }

    pub fn get_chance(&self) -> u16 { self.personnage.chance.clone() }
    pub fn set_chance(&mut self, chance: u16) { self.personnage.chance = chance; }

    pub fn get_resistance_physique(&self) -> u16 { self.personnage.resistance_physique.clone() }
    pub fn set_resistance_physique(&mut self, resistance_physique: u16) { self.personnage.resistance_physique = resistance_physique; }

    pub fn get_resistance_magique(&self) -> u16 { self.personnage.resistance_magique.clone() }
    pub fn set_resistance_magique(&mut self, resistance_magique: u16) { self.personnage.resistance_magique = resistance_magique; }

    pub fn get_points_competence(&self) -> u8 { self.points_competence }
    pub fn set_points_competence(&mut self, points_competence: u8) { self.points_competence = points_competence; }

    pub fn get_attaques(&self) -> Vec<String> { self.personnage.attaques.clone() }
    pub fn add_attaque(&mut self, attaque: String) {
        if !self.personnage.attaques.contains(&attaque) {
            self.personnage.attaques.push(attaque);
        }
    }
    pub fn remove_attaque(&mut self, attaque: &String) {
        if let Some(pos) = self.personnage.attaques.iter().position(|x| x == attaque) {
            self.personnage.attaques.remove(pos);
        }
    }

    pub fn get_equipement(&self) -> HashMap<EquipementType, Option<String>> { self.personnage.equipement.clone() }
  
    pub fn add_equipement(&mut self, categorie: &EquipementType, equipement: &String) {
        let eq = self.personnage.equipement.entry(categorie.clone()).or_insert(None);
        if eq.is_some() {
            println!("Un équipement est déjà équipé dans la catégorie {:?}: {:?}", categorie, eq.as_ref().unwrap());
        } else {
            *eq = Some(equipement.clone());
            println!("Équipement équipé dans la catégorie {:?}", categorie);
            self.remove_inventaire(equipement, 1);
        }
    }

    pub fn remove_equipement(&mut self, categorie: &EquipementType) {
        match self.personnage.equipement.get_mut(categorie) {
            Some(equipement) => {
                if let Some(eq) = equipement.take() {
                    println!("Équipement retiré de la catégorie {:?}: {:?}", categorie, eq);
                    self.add_inventaire(eq, 1);
                } else {
                    println!("Aucun équipement de la catégorie {:?} à retirer.", categorie);
                }
            }
            None => {
                println!("Catégorie {:?} inconnue dans l'équipement.", categorie);
            }
        }
    }

    pub fn get_inventaire(&self) -> HashMap<String, u32> { self.personnage.inventaire.clone() }
    
    pub fn add_inventaire(&mut self, item: String, quantite: u32) {
        let entry = self.personnage.inventaire.entry(item).or_insert(0);
        *entry += quantite;
    }

    pub fn remove_inventaire(&mut self, item: &String, quantite: u32){
        if let Some(entry) = self.personnage.inventaire.get_mut(item) {
            if *entry >= quantite {
                *entry -= quantite;
                if *entry == 0 {
                    self.personnage.inventaire.remove(item);
                }
            } else {
                println!("Quantité insuffisante pour retirer {} de {}.", quantite, item);
            }
        } else {
            println!("L'item {} n'est pas dans l'inventaire.", item);
        }
    }

    pub fn recup_ressources(&mut self, master_file: &MasterFile) -> Vec<Ressource>{
        let inventaire = self.get_inventaire();
        let mut ressources = Vec::new();

        for (i, (id, _)) in inventaire.iter().enumerate() {
            match master_file.prendre_ressource_id(id) {
                Ok(item) => {
                    ressources.push(item);
                }
                Err(_) => {}
            }
        }
        ressources
    }

    pub fn recup_consommable(&mut self, master_file: &MasterFile) -> Vec<Consommable>{
        let inventaire = self.get_inventaire();
        let mut consommable = Vec::new();

        for (i, (id, _)) in inventaire.iter().enumerate() {
            match master_file.prendre_consommable_id(id) {
                Ok(item) => {
                    consommable.push(item);
                }
                Err(_) => {}
            }
        }
        consommable
    }

    pub fn recup_equipement(&mut self, master_file: &MasterFile) -> Vec<Equipement>{
        let inventaire = self.get_inventaire();
        let mut equipement = Vec::new();

        for (i, (id, _)) in inventaire.iter().enumerate() {
            match master_file.prendre_equipement_id(id) {
                Ok(item) => {
                    equipement.push(item);
                }
                Err(_) => {}
            }
        }
        equipement
    }

    pub fn demantelement(&mut self, item: &String, master_file: &MasterFile) {
        self.remove_inventaire(item, 1);
        if let Ok(item_obj) = master_file.prendre_item_id(item) {
            for (composant, quantite) in item_obj.get_ressources() {
                self.add_inventaire(composant, quantite);
            }
        } else {
            eprintln!("Item '{}' introuvable dans le master_file", item);
        }
    }


    pub fn get_position(&self) -> String { self.position.clone() }
    pub fn set_position(&mut self, lieu: String) { self.position = lieu; }

    pub fn get_pronom(&self) -> String { self.pronom.clone() }
    pub fn set_pronom(&mut self, pronom: String) { self.pronom = pronom; }

    pub fn get_niveau(&self) -> u8 { self.niveau.clone() }
    pub fn add_niveau(&mut self, niveau: u8) { 
        self.niveau += niveau; 
        self.points_competence+=5;
    }


    pub fn get_temps(&self) -> u32 { self.temps.clone() }
    pub fn set_temps(&mut self, temps: u32) { self.temps = temps; }

    pub fn get_reputations(&self) -> Vec<u16> { self.reputations.clone() }
    pub fn set_reputations(&mut self, reputation: Vec<u16>) { self.reputations = reputation; }

    pub fn get_xp(&self) -> u32 { self.xp.clone() }
    pub fn add_xp(&mut self, xp: u32) {
        self.xp += xp;
        while self.xp >= self.niveau as u32 * 150 {
            self.xp -= self.niveau as u32 * 150;
            self.add_niveau(1);
        }
    }
    pub fn set_xp(&mut self, xp: u32) { self.xp = xp; }

    pub fn get_multiplicateur_xp(&self) -> u16 { self.multiplicateur_xp.clone() }
    pub fn set_multiplicateur_xp(&mut self, multiplicateur_xp: u16) { self.multiplicateur_xp = multiplicateur_xp; }
    
    pub fn get_quetes(&self) -> Vec<String> { self.quetes.clone() }
    pub fn add_quete(&mut self, quete: String) {
        if !self.quetes.contains(&quete) {
            self.quetes.push(quete);
        }
    }
    pub fn set_quetes(&mut self, quetes: Vec<String>) { self.quetes = quetes}

    pub fn remove_quete(&mut self, quete: String){
        if self.quetes.contains(&quete) {
            self.quetes.retain(|q| q != &quete);
        }
        else { panic!("Erreur : la quete [{}] n'est pas dans la liste des quetes du joueur : [{}].", quete, self.str_quetes()); }
    }
    
    fn str_reputations(&self) -> String {
        let mut res = String::new();
        for i in 0..self.reputations.len()-1 {
            res.push_str(&self.reputations[i].to_string());
            res.push_str(", ");
        }
        res.push_str(&self.reputations[self.reputations.len()-1].to_string());
        res
    }

    fn str_quetes(&self) -> String {
        let mut quetes = String::new();
        for i in 0..self.quetes.len()-1 {
            quetes.push_str(&self.quetes[i].to_string());
            quetes.push_str(", ");
        }
        quetes.push_str(&self.quetes[self.quetes.len()-1].to_string());
        quetes
    }


    pub fn ajout_point_stat(&mut self, stat: &str) {
        match stat {
            "pv" => {
                self.set_pv_max(self.get_pv_max()+10);
                self.set_pv_actuel(self.get_pv_actuel()+10);
            }
            "force" => {
                self.set_force(self.get_force()+1);
            }
            "dexterite" => {
                self.set_dexterite(self.get_dexterite()+1);
            }
            "intelligence" => {
                self.set_intelligence(self.get_intelligence()+1);
            }
            "vitesse" => {
                self.set_vitesse(self.get_vitesse()+1);
            }
            "esquive" => {
                self.set_esquive(self.get_esquive()+1);
            }
            "chance" => {
                self.set_chance(self.get_chance()+1);
            }
            "resistance_physique" => {
                self.set_resistance_physique(self.get_resistance_physique()+1);
            }
            "resistance_magique" => {
                self.set_resistance_magique(self.get_resistance_magique()+1);
            }
            _ => panic!("Erreur : la stat [{}] n'est pas reconnue.", stat),
        }
        self.set_points_competence(self.get_points_competence()-1);

    }

    ///////////////
    /// Fonction qui applique les effets d'un consommable au joueur.
    pub fn appliquer_effets_items(&mut self, effets: Vec<u16>,combat: &bool) {
        self.personnage.pv_actuel = if self.personnage.pv_actuel + effets[0] > self.personnage.pv_max {
            self.personnage.pv_max
        } else {
            self.personnage.pv_actuel + effets[0]
        };
        if *combat{
            self.personnage.force += effets[1];
            self.personnage.dexterite += effets[2];
            self.personnage.intelligence += effets[3];
            self.personnage.vitesse += effets[4];
            self.personnage.esquive += effets[5];
            self.personnage.chance += effets[6];
            self.personnage.resistance_physique += effets[7];
            self.personnage.resistance_magique += effets[8];
        }
    }

    ///////////////
    /// Fonction qui permet d'utiliser un consommable.
    pub fn utiliser_item(&mut self, master_file: &MasterFile,item: &String,combat: &bool) {
        match master_file.prendre_consommable_id(item) {
            Ok(consommable) => {
                let effets = consommable.get_effets().clone();
                let should_apply = {
                    let inventaire = &mut self.personnage.inventaire;
                    if let Some(quantite) = inventaire.get_mut(item) {
                        if *quantite > 0 {
                            *quantite -= 1;
                            if *quantite == 0 {
                                self.personnage.inventaire.remove(item);
                            }
                            true
                        } else {
                            println!("Quantité de {} insuffisante pour l'utiliser.", item);
                            false
                        }
                    } else {
                        println!("L'item {} n'est pas dans l'inventaire.", item);
                        false
                    }
                };

                if should_apply {
                    self.appliquer_effets_items(effets,&combat);
                }
            }
            _ => {
                println!("L'item {} n'est pas utilisable", item);
            }
        }
    }

    ///////////////
    /// Fonction qui permet d'ajouter les récompenses d'un combat dans l'inventaire du joueur.
    pub fn ajout_recompense_inventaire(&mut self,recompense: HashMap<String, u32>){
        for (item, quantite) in recompense.iter() {
            self.add_inventaire(item.clone(), *quantite);
        }
    }
    
    ///////////////
    /// Fonction qui retourne les dégâts reçus après que la résistance physique/magique ait été prise en compte.
    pub fn degats_recus_net(&mut self,degats_recus_brut: &Vec<u16>) -> u16{
        self.personnage.defense(degats_recus_brut)
    }

    ///////////////
    /// Fonction qui applique les dégâts infligés au joueur et peut amener à des conséquences en cas de PV tombant à 0.
    pub fn application_degats(&mut self,degats_recus_net: &u16) -> bool {
        let new_pv_actuel = self.get_pv_actuel().saturating_sub(*degats_recus_net);
        self.set_pv_actuel(new_pv_actuel);
        if self.get_pv_actuel() == 0 {//game over si 0
            println!("Vous avez perdu !");
            return true;
            //Retour à l'interface
        }
        false
    }

    ///////////////
    ///Fonction qui permet de reset les stats du joueur à la fin d'un combat
    pub fn reset_stats(&mut self,joueur: Joueur){
        self.set_force(joueur.get_force());
        self.set_dexterite(joueur.get_dexterite());
        self.set_intelligence(joueur.get_intelligence());
        self.set_vitesse(joueur.get_vitesse());
        self.set_esquive(joueur.get_esquive());
        self.set_chance(joueur.get_chance());
        self.set_resistance_physique(joueur.get_resistance_physique());
        self.set_resistance_magique(joueur.get_resistance_magique());
    }

    pub fn get_categorie_Arme(&self) -> Option<Arme> {
        let categorie = self.personnage.equipement.get(&EquipementType::Arme)
            .and_then(|eq| eq.as_ref().and_then(|id| MasterFile::new().prendre_equipement_id(id).ok()))
            .and_then(|e| Some(e.get_categorie()));
        match categorie {
            Some(Categorie::Arme(Arme::ArmeMelee)) => Some(Arme::ArmeMelee),
            Some(Categorie::Arme(Arme::ArmeDistance)) => Some(Arme::ArmeDistance),
            Some(Categorie::Arme(Arme::ArmeMagie)) => Some(Arme::ArmeMagie),
            _ => None
        }
    }

    ///////////////
    ///Fonction pour ajouter une quête dans la liste des quêtes du joueur
    pub fn ajout_quete_joueur(&mut self, quete: &mut Quete){
        self.add_quete(quete.get_id());
        quete.set_statut(crate::quete::StatutQuete::EnCours);
    }

    ///////////////
    ///Fonction pour mettre la suite d'une quête dans la liste des quêtes du joueur 
    pub fn suivi_quete(&mut self, master_file: &mut MasterFile, quete: &mut Quete) {
        let quetes_suivantes = quete.get_quetes_suivantes();
        self.remove_quete(quete.get_id());
        quete.set_statut(crate::quete::StatutQuete::Terminee);
        self.ajout_recompense_inventaire(quete.get_recompense());
        println!("Quête terminée : [{}]", quete.get_statut());

        if let Some(suivante_id) = quetes_suivantes.get(0) {
            let mut quete_suivante = master_file.prendre_quete_id(suivante_id).expect("Quête suivante introuvable");
            if quete_suivante.get_quete_joueur() { //si la quête suivante n'est pas un dialogue alors on l'ajoute
                self.add_quete(suivante_id.clone());
            } else {
                quete_suivante.set_statut(crate::quete::StatutQuete::Terminee);
            }
        }
    }

    ///////////////
    ///Fonction qui permet de vérifier si l'une des quêtes du joueur est fini
    pub fn completion_quete(&mut self, master_file: &mut MasterFile, id_condition: String){
        let quetes = self.get_quetes();
        for quete_id in quetes {
            let mut quete: Quete = master_file.prendre_quete_id(&quete_id).expect("Quête introuvable");
            if quete.find_fin_de_quete(id_condition.clone()) {
                self.suivi_quete(master_file, &mut quete);

                if let Some(dialogue_id) = quete.get_dialogue_a_enlever() {
                    if let Ok(mut quete_a_enlever) = master_file.prendre_quete_mut(&dialogue_id) {
                        quete_a_enlever.set_statut(crate::quete::StatutQuete::Terminee);
                    }
                }

                break;
            }
        }
    }

    ///////////////
    ///Fonction qui permet de se déplacer
    pub fn deplacement(&mut self,destination: &str) {
        self.set_position(destination.to_string());
        println!("Vous vous êtes déplacé vers {} ",destination);
    }

}

impl std::fmt::Display for Joueur {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Joueur : personnage = [{}], position = {}, pronom = {}, niveau = {},  points de compétences = {}, temps = {}, reputation = {}, xp = {}, multiplicateur_xp = {}, quetes = {}", self.personnage, self.position, self.pronom, self.niveau, self.points_competence, self.temps, self.str_reputations(), self.xp, self.multiplicateur_xp, self.str_quetes())
    }
}