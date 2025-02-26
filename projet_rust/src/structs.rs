#[derive(Debug, Clone)]
pub struct Entite {
    description: String,
    nom: String,
}

impl Entite {
    pub fn new(description: String, nom: String) -> Entite {
        Entite { description, nom }
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_nom(&self) -> String {
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

#[derive(Debug, Clone)]
pub struct Lieu {
    description: String,
    nom: String,
    destination: Vec<Lieu>,
    entites: Vec<Entite>,
    meteo: Meteo,
}

impl Lieu {
    pub fn new(description: String, nom: String, entites: Vec<Entite>, meteo: Meteo) -> Lieu {
        Lieu {
            description,
            nom,
            destination: Vec::new(),
            entites,
            meteo,
        }
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_nom(&self) -> String {
        self.nom.clone()
    }

    fn get_destination(&self) -> Vec<Lieu> {
        self.destination.clone()
    }

    fn get_entites(&self) -> Vec<Entite> {
        self.entites.clone()
    }

    fn get_meteo(&self) -> Meteo {
        self.meteo.clone()
    }

    fn add_destination(&mut self, lieu: Lieu) {
        self.destination.push(lieu);
    }
}

#[derive(Debug, Clone)]
pub struct Joueur {
    description: String,
    nom: String,
    pronom: String,
    niveau: u16,
    position: Lieu
}

impl Joueur {
    pub fn new(description: String, nom: String, pronom: String, niveau: u16, position: Lieu) -> Joueur {
        Joueur {
            description,
            nom,
            pronom,
            niveau,
            position,
        }
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_nom(&self) -> String {
        self.nom.clone()
    }

    fn get_pronom(&self) -> String {
        self.pronom.clone()
    }

    fn get_niveau(&self) -> u16 {
        self.niveau.clone()
    }

    fn get_position(&self) -> Lieu {
        self.position.clone()
    }

    fn add_niveau(&mut self, niveau: u16) {
        self.niveau += niveau;
    }

    fn set_position(&mut self, lieu: Lieu) {
        self.position = lieu;
    }
}