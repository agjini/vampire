# Réflexion Game Design - Projet Vampire Survivor-like

## 1. Introduction

### Présentation du projet
Développement d'un jeu dans le genre "Survivor-like" inspiré principalement de Vampire Survivors et des éléments de rogue-like.

### Objectifs du document
Ce document centralise toutes les réflexions, analyses et décisions de game design pour le projet. Il sert de référence pour le développement et l'itération du jeu.

### Public cible
Joueurs appréciant les jeux de type survivor-like, rogue-like avec progression permanente, accessibles, mais offrant de la profondeur stratégique.

## 2. Concept du jeu

### Vision générale
Créer un jeu survivor-like complet et partageable sur Steam, fonctionnant sur PC. Le jeu doit avoir une ambiance forte et une histoire simple, mais cohérente.

### Genre et inspirations
**Genre principal :** Survivor-like / Rogue-like avec progression permanente

**Inspirations principales :**
- **Vampire Survivors** (+++) - Référence principale
- **Balatro** - Système de progression
- **Doodle Jump** - Simplicité et addictivité
- **Spelunky** - Génération procédurale

### Proposition de valeur unique
Un survivor-like avec une thématique "Real Fantasy", évitant la surcharge de mécaniques tout en offrant suffisamment de profondeur pour maintenir l'engagement. Focus sur l'ambiance, la notion d'items/armes et une progression satisfaisante.

## 3. Mécaniques de jeu

### Gameplay principal
- **Déplacement :** Le personnage navigue sur une map infinie
- **Combat :** Le personnage possède une attaque de base pour se défendre
- **Monétisation in-game :** Le personnage gagne de l'argent durant une partie

### Systèmes de jeu

#### Système de progression en partie
- Le personnage peut monter de niveaux pour obtenir de nouvelles capacités (actives ou passives)
- Mode infini avec difficulté exponentielle

#### Système de progression permanente (Meta)
- **(Hors partie)** Le joueur peut dépenser l'argent pour débloquer des bonus permanents (passifs)
- **(Hors partie)** En remplissant certaines conditions, de nouvelles armes et accessoires sont débloqués et deviennent accessibles dans les parties futures

### Contrôles et interactions
- Contrôles adaptés pour PC : clavier et souris + controleur
- Attaque de base automatique ou manuelle (à définir selon le benchmark)
- Navigation fluide et responsive

### Boucles de gameplay

#### Boucle principale (en partie)
1. Explorer la map
2. Affronter des vagues d'ennemis
3. Monter de niveau
4. Choisir de nouvelles capacités
5. Collecter de l'argent
6. Survivre le plus longtemps possible

#### Boucle secondaire (méta-progression)
1. Terminer une partie avec de l'argent
2. Dépenser l'argent en améliorations permanentes
3. Débloquer de nouveaux contenus (armes, accessoires)
4. Recommencer une partie plus fort

## 4. Expérience joueur

### Courbe de difficulté
- **Difficulté progressive :** Augmentation exponentielle de la difficulté en mode infini
- **Modulable :** Possibilité de modifier la difficulté avant de lancer une partie (inspiré de Nomad Survivor et 20 Minutes Till Dawn)
- **Challenges optionnels :** Activation de défis spécifiques pour obtenir plus de récompenses

### Système de progression

#### Progression en partie
- Montée en niveaux régulière
- Choix de capacités à chaque niveau
- Système de spécialisation tous les X niveaux (inspiré de Nomad Survivor - tous les 30 niveaux ?)

#### Progression permanente
- Déblocage d'améliorations passives avec l'argent gagné
- Déblocage de nouveaux personnages, armes et accessoires via des conditions spécifiques
- Sentiment de progression constante même en cas d'échec

### Récompenses et motivation
- **Argent :** Collecté pendant la partie pour acheter des améliorations permanentes
- **Déblocages :** Nouveaux contenus accessibles en remplissant des conditions
- **Synergies :** Découverte de combinaisons de capacités puissantes
- **Records :** Temps de survie, vagues vaincues, etc.

### Feedback et tutoriel
- Introduction progressive des mécaniques
- Feedback visuel et sonore clair pour les actions du joueur
- Interface simple et intuitive
- Tutoriel intégré dans les premières parties

## 5. Univers et narration

### Contexte et setting
- **Thématique :** "Real Fantasy". Univers fantastique avec une touche de réalisme
- **Ambiance :** Forte identité visuelle et atmosphérique
- **Environnements :** Maps générées procéduralement
- **Tonalité :** À définir — sombre, épique, ou légèrement humoristique

### Personnages principaux
- **Personnages jouables :** Plusieurs personnages déblocables
- **Armes spécifiques :** Chaque personnage possède potentiellement une arme unique (inspiré de Nomad Survivor)
- **Identité visuelle :** Design de personnages cohérent avec la thématique Real Fantasy

### Histoire et quêtes
- **Histoire simple :** Narration minimaliste mais cohérente
- **Progression narrative :** Histoire se dévoilant au fur et à mesure des déblocages
- **Objectifs :** Conditions de déblocage servant de quêtes implicites

### Lore
- **Lore intégré :** Backstory des personnages et armes
- **Bestiaire :** Descriptions des ennemis et boss
- **Monde :** Contexte général expliquant la présence des vagues d'ennemis

## 6. Design technique

### Architecture du jeu
- **Modularité :** Architecture permettant l'ajout facile de nouvelles armes et capacités
- **Système de configuration :** Fichier config pour modifier rapidement les réglages du jeu lors des tests
- **Génération procédurale :** Système de génération de maps et de spawn d'ennemis

### Technologies utilisées
- **Moteur :** À définir (possibilité d'utiliser Bevy - cf. Bevy Jam #6)
- **Langages :** À définir selon le moteur choisi
- **Outils :** Outils de développement et de test

### Contraintes techniques
- **plateforme :** PC (Steam)
- **Performance :** Optimisation pour gérer de nombreux ennemis à l'écran simultanément
- **Sauvegarde :** Système de sauvegarde pour la progression permanente

### Performance
- **Optimisation :** Gestion efficace des entités (ennemis, projectiles, effets)
- **Scalabilité :** Mode infini avec difficulté exponentielle nécessitant une bonne optimisation
- **Framerate :** Maintien d'un framerate stable même avec beaucoup d'éléments à l'écran

## 7. Aspects artistiques

### Direction artistique
- **Style visuel :** Real Fantasy - Mélange de fantastique et de réalisme
- **Palette de couleurs :** À définir — possibilité d'une palette limitée pour une identité forte (inspiré de 20 Minutes Till Dawn)
- **Ambiance :** Atmosphère immersive et cohérente

### Interface utilisateur (UI/UX)
- **Simplicité :** Interface claire et intuitive
- **Adaptabilité :** UI fonctionnant sur PC et différentes résolutions
- **Feedback visuel :** Indication claire des actions et de l'état du joueur
- **Menus :** Menu principal, écran de progression, boutique d'améliorations

### Design sonore et musique
- **Musique :** Musiques avec licences appropriées (Dark Fantasy et Chiptunes mentionnées)
- **Effets sonores :** Sons d'impacts, de collecte, de montée en niveau
- **Ambiance sonore :** Sons contribuant à l'atmosphère du jeu

### Animations
- **Personnages :** Animations de déplacement, d'attaque, de dégâts
- **Ennemis :** Animations variées selon les types d'ennemis
- **Effets :** Animations de capacités, d'explosions, de collectibles
- **Fluidité :** Animations fluides et réactives

### Ressources artistiques
- **Sprites :** Charas-project pour les sprites de personnages
- **Assets :** itch.io/game-assets pour les ressources graphiques
- **Cohérence :** Maintenir une cohérence visuelle entre tous les éléments

## 8. Économie du jeu

### Ressources et monnaies
- **Argent in-game :** Monnaie principale collectée pendant les parties
- **Utilisation :** Achat d'améliorations permanentes hors partie
- **Sources :** Ennemis vaincus, objectifs complétés, bonus temporaires

### Système de récompenses
- **Récompenses de partie :** Argent proportionnel à la performance
- **Déblocages :** Conditions spécifiques pour débloquer personnages, armes et accessoires
- **Bonus de difficulté :** Récompenses accrues en activant les challenges de difficulté
- **Progression visible :** Le joueur voit toujours sa progression même en échec

### Équilibrage
- **Courbe de prix :** Prix des améliorations équilibrés pour maintenir l'engagement
- **Puissance des améliorations :** Bonus significatifs, mais pas déséquilibrés
- **Synergies :** Combinaisons de capacités puissantes mais pas game-breaking
- **Scaling :** Difficulté qui s'adapte à la puissance du joueur

### Monétisation (si applicable)
- **Modèle de base :** Jeu premium sur Steam
- **Pas de pay-to-win :** Pas d'avantage de gameplay payant

## 9. Contenu et rejouabilité

### Durée de vie
- **Parties individuelles :** Durée variable selon performance (mode infini)
- **Progression complète :** Plusieurs heures pour débloquer tout le contenu
- **Objectif minimal :** Suffisamment de contenu pour justifier le prix sur Steam
- **Extension :** Architecture permettant l'ajout de contenu post-lancement

### Variété du contenu
- **Personnages multiples :** Plusieurs personnages jouables avec armes uniques
- **Capacités variées :** Large pool de capacités actives et passives
- **Ennemis diversifiés :** Différents types d'ennemis et boss
- **Environnements :** Maps générées procéduralement pour éviter la répétition

### Facteurs de rejouabilité
- **Synergies :** Découverte de nouvelles combinaisons de capacités
- **Challenges de difficulté :** Modulation de la difficulté pour plus de récompenses
- **Déblocages :** Objectifs à remplir pour débloquer nouveau contenu
- **Records personnels :** Amélioration constante de ses performances
- **Meta-progression :** Renforcement permanent encourageant les nouvelles parties

### Contenu post-lancement
- **Mises à jour :** Ajout de nouveaux personnages, armes et ennemis
- **Événements :** Possibilité d'événements saisonniers ou challenges temporaires
- **Communauté :** Écoute des retours pour améliorer et enrichir le jeu

## 10. Prochaines étapes

### Prototypage

#### Level 1 : Forme minimale du jeu
**Objectif :** Créer un prototype fonctionnel avec les mécaniques de base
- Personnage jouable avec déplacement
- Système d'ennemis et de spawn
- Attaque de base fonctionnelle
- Mécaniques de base du gameplay
- Système d'argent (optionnel à ce stade)
- Mode infini avec difficulté exponentielle (optionnel)

**Thématique :** Real Fantasy - Éviter trop de mécaniques qui complexifient le jeu

**Benchmark :** Analyser différents Survivor-likes pour en extraire des mécaniques distinctives
- Void Survivors
- Vampire Survivors
- 20 Minutes Till Dawn
- Pixel Survivors
- MegaBonk
- Et autres...

#### Level 2 : Génération procédurale
**Objectif :** Implémenter la carte et la gestion avancée des ennemis
- Génération procédurale de la carte
- Système de gestion des vagues d'ennemis
- Spawn intelligent des ennemis
- Optimisation des performances

#### Level 3 : Progression et objets
**Objectif :** Système de progression complet
- Objets interactibles (collectibles, power-ups)
- Système de progression du joueur (niveaux, capacités)
- Méta-progression (améliorations permanentes)
- Déblocages de contenu

### Milestones et planning
- **Phase 1 (Prototype) :** (durée ?) - Gameplay de base fonctionnel
- **Phase 2 (Alpha) :** (durée ?) - Systèmes avancés et contenu initial
- **Phase 3 (Beta) :** (durée ?) - Polish, équilibrage, contenu complet
- **Phase 4 (Release) :** (durée ?) - Tests finaux et lancement

### Ressources nécessaires

#### Techniques
- **Fichier de configuration :** Pour tests et ajustements rapides des paramètres
- **Moteur de jeu :** Bevy
- **Outils de développement :** IDE, versioning (Git), outils de debug

#### Artistiques
- **Sprites de personnages :** Charas-project (http://charas-project.net/resources.php)
- **Assets graphiques :** itch.io/game-assets
- **Musiques :** Bibliothèques de musiques avec licences (Dark Fantasy et Chiptunes)
- **Effets sonores :** Bibliothèques libres de droits

#### Références
- **Plateformes de jeux :** Poki, Steam pour benchmark
- **Game Jams :** Bevy Jam #6 pour inspiration

### Risques et défis

#### Techniques
- **Performance :** Optimisation nécessaire pour mode infini avec nombreux ennemis
- **plateforme :** Adaptation PC résolution variée
- **Génération procédurale :** Équilibrage de la difficulté et de la variété

#### Design
- **Équilibrage :** Éviter le power creep tout en maintenant la satisfaction
- **Complexité :** Trouver le bon équilibre entre simplicité et profondeur
- **Rejouabilité :** Maintenir l'intérêt sur la durée

#### Production
- **Scope :** Risque de sur-ambition, nécessité de prioriser les fonctionnalités
- **Temps :** Estimation réaliste du temps de développement
- **Ressources :** Disponibilité des assets et musiques avec licences appropriées

#### Marché
- **Concurrence :** Genre saturé, nécessité de se démarquer
- **Originalité :** Trouver la proposition de valeur unique
- **Audience :** Atteindre le public cible sur Steam

---

## Annexe : Benchmark des concurrents

### Comment les concurrents de Vampire Survivors se démarquent
Analyse des mécaniques et de la direction artistique des principaux concurrents.

### Nomad Survivor
**Lien :** [Steam - Nomad Survival](https://store.steampowered.com/app/1929870/Nomad_Survival/)

#### Armes uniques par personnage
- Chaque personnage démarre une partie avec une arme qui lui est spécifique
- Les autres personnages ne peuvent pas avoir accès aux armes des autres
- Crée une identité forte pour chaque personnage

#### Système de spécialisation
- **Tous les 30 niveaux** : Le joueur choisit une spécialisation parmi deux options
- La spécialisation modifie le fonctionnement de base de l'arme initiale
- Permet une personnalisation approfondie du build

#### Modulation de la difficulté
- Le joueur peut **moduler la difficulté avant de lancer une partie**
- Activation de **challenges spécifiques** pour plus de récompenses
- Trade-off : difficulté accrue contre meilleures récompenses
- Permet d'adapter l'expérience selon le niveau du joueur

#### Système de compagnons
- Déblocage et amélioration d'**animaux de compagnie**
- Les compagnons aident à vaincre les adversaires
- Certains récupèrent automatiquement les bonus tombés au sol
- Ajoute une couche stratégique supplémentaire

### 20 Minutes Till Dawn
**Lien :** [Steam - 20 Minutes Till Dawn](https://store.steampowered.com/app/1966900/20_Minutes_Till_Dawn/)

#### Identité visuelle unique
- **Palette de couleurs limitée** : Rouge/Blanc/Gris/Noir exclusivement
- Crée une identité visuelle forte et reconnaissable
- Atmosphère sombre et intense

#### Mécanique de tir manuelle
- Le personnage **n'attaque pas automatiquement**
- Le joueur doit **cliquer pour tirer**
- **Rechargement obligatoire** quand l'arme est vide (munitions illimitées)
- Visée automatique (ennemi le plus proche) ou manuelle au choix

#### Gameplay basé sur le rechargement
- De nombreux effets se déclenchent au rechargement
- Effets spéciaux selon les balles utilisées
- Le rechargement devient une mécanique de game design à part entière
- Ajoute de la profondeur stratégique

#### Système de choix risque/récompense
- Quand un boss est vaincu, il dépose un objet
- **Choix entre 3 effets** comportant chacun un effet positif ET un effet négatif
- Le joueur peut refuser tous les choix
- Encourage la prise de décision et la spécialisation du build

#### Système d'éléments
- **Trois éléments** : Feu, Glace et Foudre
- Effets élémentaires : brûlure, gel, foudroiement
- Nombreux passifs basés sur les éléments
- **Exemple de synergie** : Quand un ennemi brûlé meurt, il explose et brûle les ennemis à proximité

#### Modulation de la difficulté
- Possibilité de moduler la difficulté avant une partie
- Récompenses accrues en échange de difficulté accrue

#### Mécanique de visibilité
- **Brouillard de guerre** dans les environnements sombres/nocturnes
- Empêche de détecter les ennemis trop éloignés
- Certains passifs augmentent la visibilité
- Effets spéciaux quand un ennemi devient visible
- Ajoute tension et surprise au gameplay

---

## Liens et ressources utiles

### Assets et ressources
- [itch.io Game Assets](https://itch.io/game-assets) - Assets graphiques
- [Charas-project](http://charas-project.net/resources.php?wa=0&lang=fr&area=5&offset=1&howmany=10&fsearch=) - Sprites de personnages

### Game Jams et références
- [Bevy Jam #6](https://itch.io/jam/bevy-jam-6/entries) - Jeux créés avec Bevy
- [Poki](https://poki.com/fr) - Plateforme de jeux casual

### Musique et sons
- Bibliothèques de musiques Dark Fantasy
- Bibliothèques de musiques Chiptunes
- *(Liens spécifiques à ajouter)*

