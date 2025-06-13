# Manuel d'utilisation

## Sommaire
- [Menu Principal](#menu-principal)
- [Menu du Jeu](#menu-du-jeu)
- [Carte](#carte)
- [Actions](#actions)
  - [Analyser la zone](#analyser-la-zone)
  - [Récolter](#récolter)
  - [PNJ](#pnj)
    - [Parler](#pnj)
    - [Commerce](#pnj)
    - [Voler](#pnj)
  - [Combattre](#combattre)
- [Personnage](#personnage)
  - [Informations](#informations)
    - [Attribuer des points](#attribuer-des-points)
    - [Statistiques](#statistiques)
    - [Équipement](#equipement)
    - [Attaques](#attaques)
  - [Inventaire](#inventaire)
  - [Quêtes](#quêtes)
- [Menu](#menu)


Pour démarrer le jeu, il suffit de se déplacer dans le dossier `projet_rust/projet_rust`,  
puis d’exécuter la commande `cargo run` dans `projet_rust`.

## Menu Principal

Une fois cela fait, l’interface s’ouvre sur le menu du jeu.

![Image du Menu principal](./modélisation_rust/manuel_rust/MenuPrincipal.png)

Pour démarrer le jeu, il faut appuyer sur **Jouer**.

## Menu du Jeu

Une fois le jeu lancé, plusieurs informations sont disponibles telles que le nom du joueur, son niveau, sa position et ses points de vie.

De là, 4 choix sont offerts :  
- [**Carte**](#carte) → pour se déplacer  
- [**Actions**](#actions) → pour effectuer diverses actions dans la zone actuelle  
- [**Personnage**](#personnage) → pour voir les informations sur le personnage du joueur (telles que ses stats, son inventaire ou ses quêtes)  
- [**Menu**](#menu) → les paramètres du jeu, tels que la sauvegarde, le chargement de sauvegarde ou encore quitter le jeu  
![Image du Menu du Jeu](./modélisation_rust/manuel_rust/Jeu.png)

## Carte

Depuis la carte, il est possible de choisir de se rendre dans une destination reliée à celle où se trouve actuellement le joueur.

Ici, la Chambre Bleue est reliée aux chambres Rouge, Jaune et Verte.

Le joueur peut donc choisir de s’y rendre.

![Image de la Carte](./modélisation_rust/manuel_rust/Carte.png)

## Actions

Le menu des **Actions** permet d’effectuer diverses actions dans la zone actuelle.

![image d'Actions](./modélisation_rust/manuel_rust/Actions.png)

### Analyser la zone

L’option **Analyser la zone** permet de donner des détails sur la zone actuelle tels que :  
- Le nom du lieu actuel et sa description  
- Les ressources disponibles dans la zone  
- Les PNJ présents dans la zone  
- Les ennemis présents et leurs niveaux possibles dans cette zone

![image d'analyse de la zone](./modélisation_rust/manuel_rust/AnalyseZone.png)

### Récolter

L’option **Récolter** permet de récupérer les items disponibles dans la zone pour les ajouter à l’inventaire du joueur.

![image récolte](./modélisation_rust/manuel_rust/Récolter.png)

### PNJ

L’option **PNJ** permet d’interagir avec les différents PNJ dans la zone.

![image pnj](./modélisation_rust/manuel_rust/PNJ.png)

Types d’interactions possibles :  
- **Parler**  

- **Commerce**  

- **Voler**  

![image pnj_interactions](./modélisation_rust/manuel_rust/PNJ_Interactions.png)


### Combattre

L’option **Combattre** permet d’affronter un ennemi dans la zone.

Durant un combat, le joueur a 3 choix : **Attaquer**, **Utiliser ses items** ou **Fuir**.

![image Combat](./modélisation_rust/manuel_rust/Combattre.png)

## Personnage

Le menu du **Personnage** permet de voir les informations sur le personnage du joueur :

![image Personnage](./modélisation_rust/manuel_rust/Personnage.png)

### Informations

L’option **Informations** permet de consulter diverses données liées au personnage.

![image Informations](./modélisation_rust/manuel_rust/Informations.png)

#### Attribuer des points

L’option **Attribuer des points** permet d’améliorer les stats du personnage.

![image Points](./modélisation_rust/manuel_rust/points.png)

Le joueur peut choisir les stats à améliorer et voir les changements.

![image améliorations](./modélisation_rust/manuel_rust/pointsAmélioration.png)

#### Statistiques

L’option **Statistiques** permet de consulter les statistiques actuelles du joueur.

![image stats](./modélisation_rust/manuel_rust/statistiques.png)

#### Équipement

L’option **Équipement** permet de voir les équipements actuellement équipés sur le personnage.

![image equipements](./modélisation_rust/manuel_rust/equipements.png)

Depuis cet écran, le joueur peut également voir les détails de l’équipement et choisir de le déséquiper.

![image equipements](./modélisation_rust/manuel_rust/equipements_stats.png)


#### Attaques

L’option **Attaques** permet de consulter la liste des attaques que le joueur peut utiliser pendant un combat.

![image Attaques](./modélisation_rust/manuel_rust/attaques.png)

Elle permet également de voir les détails d’une attaque, tels que sa description, l’arme nécessaire pour l’utiliser et ses dégâts.

![image Attaques_stats](./modélisation_rust/manuel_rust/attaques_stats.png)



### Inventaire

L’option **Inventaire** permet de consulter les objets consommables, les ressources ou encore les équipements que le joueur possède.

![image Inventaire](./modélisation_rust/manuel_rust/Inventaire.png)

Depuis l’inventaire, le joueur peut utiliser ou décomposer un objet.

![image ItemUtiliser](./modélisation_rust/manuel_rust/Consommable.png)

Il peut également équiper un équipement de son inventaire.

![image EquipementInvetaire](./modélisation_rust/manuel_rust/InventaireEquipement.png)

### Quêtes

L’option **Quêtes** permet de consulter la liste des quêtes actives du joueur.

![image Quete](./modélisation_rust/manuel_rust/Quete.png)

Elle permet également de consulter les détails d’une quête, tels que sa description et ses récompenses.

![image QueteInfo](./modélisation_rust/manuel_rust/QueteDetails.png)


## Menu
Le **Menu** permet de recharger la sauvegarde, de sauvegarder la partie actuelle ou de démarrer une nouvelle partie. Le joueur a également l'option de quitter le jeu :

![image Menu](./modélisation_rust/manuel_rust/Menu.png)
