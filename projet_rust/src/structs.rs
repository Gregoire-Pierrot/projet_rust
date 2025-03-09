use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Entite {
    description: String,
    nom: String,
}

impl Entite {
    pub fn new(description: String, nom: String) -> Entite {
        Entite { description, nom }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_nom(&self) -> String {
        self.nom.clone()
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
    id: String,
    description: String,
    nom: String,
    destinations_id: Vec<String>,
    //entites: Vec<Entite>,
    //meteo: Meteo,
}

impl Lieu {
    pub fn new(id: String, description: String, nom: String/*, entites: Vec<Entite>, meteo: Meteo*/) -> Lieu {
        Lieu {
            id,
            description,
            nom,
            destinations_id: Vec::new(),
            //entites,
            //meteo,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_nom(&self) -> String {
        self.nom.clone()
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Joueur {
    description: String,
    nom: String,
    pronom: String,
    niveau: u16,
    position: String
}

impl Joueur {
    pub fn new(description: String, nom: String, pronom: String, niveau: u16, position: String) -> Joueur {
        Joueur {
            description,
            nom,
            pronom,
            niveau,
            position,
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_nom(&self) -> String {
        self.nom.clone()
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
        self.nom = nom;
    }

    pub fn set_pronom(&mut self, pronom: String) {
        self.pronom = pronom;
    }

    pub fn set_position(&mut self, lieu: String) {
        self.position = lieu;
    }
}