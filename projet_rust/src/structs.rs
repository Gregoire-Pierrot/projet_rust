use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entite {
    description: String,
    nom: String,
}

impl Entite {
    pub fn new(description: String, nom: String) -> Self {
        Self { description, nom }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_nom(&self) -> String {
        self.nom.clone()
    }

    pub fn set_nom(&mut self, nom: String) {
        self.nom = nom;
    }
}

#[derive(Debug, Clone)]
pub enum Meteo {
    Soleil,
    Pluie,
    Neige,
    Interieur,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lieu {
    entite: Entite,
    id: String,
    destinations_id: Vec<String>,
    //entites: Vec<Entite>,
    //meteo: Meteo,
}

impl Lieu {
    pub fn new(entite: Entite, id: String/*, entites: Vec<Entite>, meteo: Meteo*/) -> Self {
        Self {
            entite,
            id,
            destinations_id: Vec::new(),
            //entites,
            //meteo,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_description(&self) -> String {
        self.entite.get_description()
    }

    pub fn get_nom(&self) -> String {
        self.entite.get_nom()
    }

    pub fn get_destinations_id(&self) -> Vec<String> {
        self.destinations_id.clone()
    }

    //fn get_entites(&self) -> Vec<Entite> {
    //    self.entites.clone()
    //}

    //fn get_meteo(&self) -> Meteo {
    //    self.meteo.clone()
    //}

    pub fn add_destination_id(&mut self, id: String) {
        self.destinations_id.push(id);
    }
}

impl std::fmt::Display for Lieu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lieu : description = {}, nom = {}", self.get_description(), self.get_nom())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur {
    entite: Entite,
    pronom: String,
    niveau: u16,
    position: String
}

impl Joueur {
    pub fn new(entite: Entite,pronom: String, niveau: u16, position: String) -> Self {
        Self {
            entite,
            pronom,
            niveau,
            position,
        }
    }

    pub fn get_description(&self) -> String {
        self.entite.get_description()
    }

    pub fn get_nom(&self) -> String {
        self.entite.get_nom()
    }

    pub fn get_pronom(&self) -> String {
        self.pronom.clone()
    }

    pub fn get_niveau(&self) -> u16 {
        self.niveau.clone()
    }

    pub fn get_position(&self) -> String {
        self.position.clone()
    }

    pub fn add_niveau(&mut self, niveau: u16) {
        self.niveau += niveau;
    }

    pub fn set_nom(&mut self, nom: String) {
        self.entite.set_nom(nom)
    }

    pub fn set_pronom(&mut self, pronom: String) {
        self.pronom = pronom;
    }

    pub fn set_position(&mut self, lieu: String) {
        self.position = lieu;
    }
}

impl std::fmt::Display for Joueur {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Joueur : description = {}, nom = {}, pronom = {}, niveau = {}, position = {}", self.get_description(), self.get_nom(), self.get_pronom(), self.get_niveau(), self.get_position())
    }
}