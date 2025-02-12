# Projet Rust
**Projet d'un jeu de rôle en mode texte, en langage Rust.**

---
---

## Description
Ce projet va être réaliser dans le cadre de l'UCE : `Algorithmique et modélisation avancée`.

L'objectif de ce projet est de programmer un petit jeu de rôle en mode texte orienté sur une simulation logique d'un monde physique. Il doit permettre de développer des compétences en modélisation et algorithmique tout en mettant à profit les atouts de Rust (sécurité, fiabilité, rapidité d'exécution, etc.).

Plus de détailles [ici](https://e-uapv2024.univ-avignon.fr/mod/page/view.php?id=60296).

---

## Détailles du contenu

#### Thème : RPG médiéval fantaisie

#### Fonctionnalitée principal :
 - **Gestion du temps** : Le joueur à un objectif mais dois gérérer le temps depuis sont départ.

#### Interface graphique : 
 - **Minimal** => objectif : jeu en mode texte donc pas de graphismes, centré le texte (description de ce qu'il se passe).
 - **Temps** : toujours affiché en haut (millieu).
 - **Lieu** : en haut a gauche (région : `ville` / `route` : `rue` ...).
 - **Intempérie** : en dessous du lieu (`beau temps` / `pluie` / `neige` ...).
 - **Caractéristiques** : liste des caractéristiques personnage en haut a droite.
 - **Boite de dialogue** : description de ce qu'il se passe (qui parle, les item intéractifs, mobs ...).
 - **Icone de personnage** : en bas a gauche (à côté) de la boite de dialogue.
 - **Boite de choix** : en dessous de la boite de dialogue, plus grande quelle.

#### Combats : 
 - **3 styles** :
   - corps à corps / distance / magie.
 - **Choix du style** en fonction des attributs de personnage (possibilité d'équipé un objet en fonction des stats du personnage).
 - **5 à 10 armes** différentes par style de combats (15 à 30 armes différentes).
 - **Chaque arme** peut avoir une spécialitée / attributs : `feu`, `terre`, ...

#### Temps : 
 - Le joueur a un temps de jeu limité.
 - Une fois l'objectif final atteint, la limite de temps saute (scénario du mob qui va tout détruire au bout d'un certain temps, une fois battue plus de menace).
 - But : rajouter du temps de jeu, le joueur va découvrir une première fois la carte / le jeu / les mécaniques dans la première run et pourra le finir dès la deuxième.
 - Temps calculé en fonction des actions réalisés (déplacement, combats, quête, nuit à l'auberge, ...).

#### Personnage :
 - Personnalisation :
   - Le joueur choisi sont nom en début de partie.
   - Son pronom (il, elle, iel).
   - Choisi où vont aller les points de stats et de compétences.
 - Caractéristiques :
   - En combat :
     - Force : augmente les dégâts brutes.
     - Dextérité : permet de pouvoir équipé un plus grand panelle d'armes.
     - Intélligence : augmente les dégâts de magie.
   - En et hors combat :
     - Vitesse : augmente la fréquence d'actions & diminue le temps consomé.
     - Esquive : diminue la précision de l'adversaire & diminue le nombre de monstre rencontré.
     - Chance : augmente le taux de coups critique & augmente les chances de drops.
   - Niveau :
     - Par niveau gagné : 2 points à dépenser dans chaque catégorie de caractéristiques (en combat & en / hors combat).
   - Certaines quêtes donnent des points en plus.
   - L'équipement influe sur les stats (mais ne modifie pas les stats de bases).

#### Quêtes :
  - Quête principal.
  - Quêtes secondaires :
    - Les quêtes secondaires peuvent être réalisé en 1 à 4 étapes.

#### Carte :
  - Divisé en 5 zones : (ref info)
    - début : `python`, `scratch`.
    - désert : `android`, `apk`.
    - jungle : `java`, `mangoDB`.
    - mer / océan : `docker`.
    - final / enfer : `assembleur`, `pascal` (monstre plus "optimisé").
  - La carte est soumis à des Intempérie par zones.

#### Villes :
  - Réputation :
    - On fonction des actions, la réputations du joueur évolue.
    - Augmentation : perte de temps mais récompenses.
    - Diminution : impossiblité de marchander.

#### Histoire :
  - L’histoire est linéaire mais pas la carte, le joueur est libre d'aller où bon lui semble.
  - du jour au lendemain il se casse.
  - Doit battre un certain mob avant qu'il ne détruise tout (limite de temps).

---

## Mobs

voir le fichier des [mobs](Mobs.md).

---

## NPCs

voir le fichier des [npcs](NPCs.md)

---

## Items

voir le fichier des [items](Items.md).

---

## Armes

voir le fichier des [armes](Weapons.md).

---

## Répartitions des tâches

