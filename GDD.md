# Game Design Document
## Dungeon Explorer Compétitif

---

## 1. Vue d'ensemble

### 1.1 Concept résumé
Un jeu d'exploration de donjon compétitif en 2D pixel art où plusieurs joueurs s'affrontent dans une course au trésor. Les joueurs doivent naviguer dans un labyrinthe généré aléatoirement, gérer le brouillard de guerre, collecter des armes et des pièges, et être le premier à atteindre le trésor final.

### 1.2 Informations générales
- **Genre** : Roguelike / Exploration de donjon / Multijoueur compétitif
- **Plateforme cible** : PC (Steam) / Mobile (Portrait et Paysage)
- **Public visé** : Joueurs casual à intermédiaires
- **Durée de partie estimée** : 10-15 minutes
- **Nombre de joueurs** : 2-4 joueurs (en ligne ou local)

### 1.3 Piliers du jeu
1. **Exploration stratégique** : Le brouillard de guerre crée une tension et nécessite des choix tactiques
2. **Compétition équilibrée** : Tous les joueurs commencent avec les mêmes chances
3. **Prise de risque** : Choisir entre explorer prudemment ou foncer vers le trésor
4. **Adaptation** : S'adapter à la génération procédurale et aux actions des adversaires

---

## 2. Gameplay

### 2.1 Core Loop
```
Spawn dans une salle → Explorer le donjon (révéler la carte) →
Collecter armes/objets/pièges → Éviter/Combattre les ennemis et autres joueurs →
Trouver un moyen de descendre (trappe/pelle) → Répéter jusqu'à trouver le trésor
```

### 2.2 Objectif principal
Être le premier joueur à atteindre le trésor situé au dernier étage du donjon.

### 2.3 Conditions de victoire/défaite
- **Victoire** : Atteindre le trésor en premier
- **Défaite** :
  - Mourir (points de vie à 0)
  - Un autre joueur trouve le trésor avant vous

### 2.4 Contrôles
#### PC
- **ZQSD / Flèches** : Déplacement
- **Espace** : Interaction (ramasser objet, utiliser trappe)
- **Clic gauche** : Attaque/Utiliser arme équipée
- **Clic droit** : Placer un piège
- **1-5** : Sélection rapide d'objets
- **Tab** : Ouvrir inventaire
- **M** : Ouvrir/fermer la mini-carte

#### Mobile
- **Joystick virtuel** : Déplacement
- **Boutons contextuels** : Actions disponibles selon la situation
- **Tap** : Sélection d'objet dans l'inventaire

---

## 3. Mécaniques de jeu

### 3.1 Système d'exploration

#### Génération procédurale
- Chaque étage est un labyrinthe généré aléatoirement
- Composition : salles interconnectées par des couloirs
- Types de salles :
  - Salle de spawn (1 par étage, aléatoire pour chaque joueur)
  - Salles de trésor (contiennent des objets/armes)
  - Salles de piège
  - Salles vides
  - Salle du boss (dernier étage uniquement)

#### Brouillard de guerre
- La carte est initialement masquée
- Révélation progressive en fonction du champ de vision du joueur
- Les zones déjà visitées restent visibles sur la carte (version grisée)
- Les autres joueurs ne sont visibles que s'ils sont dans le champ de vision

### 3.2 Système de progression verticale

#### Descente d'étage
Deux méthodes pour descendre :
1. **Trappe** : Objet trouvable dans certaines salles
   - Utilisation instantanée
   - Mène à un étage inférieur aléatoire

2. **Pelle** : Objet à ramasser
   - Utilisation : creuser depuis n'importe quelle position
   - Usage unique ou limité (à définir)
   - Permet de contrôler où on descend

#### Structure du donjon
- Nombre d'étages : 5-7 étages
- Difficulté croissante à chaque étage
- Dernier étage = Salle du trésor + Boss gardien

### 3.3 Système de combat

#### Armes
Types d'armes disponibles :
- **Armes de mêlée** : Épée, hache, dague
  - Portée courte
  - Dégâts élevés
  - Pas de munitions

- **Armes à distance** : Arc, arbalète, bâton magique
  - Portée longue
  - Dégâts modérés
  - Munitions limitées ou temps de recharge

- **Armes spéciales** : Bombes, potions offensives
  - Usage unique
  - Effets de zone ou spéciaux

#### Stats de combat
Chaque joueur possède :
- **Points de vie (HP)** : 100 de base
- **Vitesse de déplacement** : Standard (modifiable par objets)
- **Dégâts** : Dépendent de l'arme équipée
- **Défense** : Réduction de dégâts (armure trouvée)

#### Ennemis (PNJ)
- Présents dans certaines salles
- Types : Squelettes, vampires, crânes flottants (selon assets)
- Comportement : Patrouille ou garde statique
- Drop : Objets, armes, ou clés

### 3.4 Système de pièges

#### Placement de pièges
- Le joueur peut placer des pièges pour ralentir/blesser les adversaires
- Types de pièges :
  - Piège à pointes (dégâts)
  - Piège à filet (ralentissement)
  - Piège sonore (révèle la position)

#### Détection de pièges
- Les pièges des autres joueurs sont invisibles
- Peuvent être révélés par des objets spéciaux (lunettes de détection)

### 3.5 Système d'objets et inventaire

#### Inventaire
- Capacité limitée : 8-10 emplacements
- Types d'objets :
  - Armes
  - Consommables (potions de soin, de vitesse)
  - Outils (pelle, clé, torche)
  - Pièges
  - Équipement (armure, bottes, amulettes)

#### Objets interactifs
- **Coffres** : Contiennent des objets aléatoires, peuvent nécessiter une clé
- **Potions** : Placées dans l'environnement
- **Clés** : Ouvrent des portes ou coffres spécifiques

---

## 4. Systèmes

### 4.1 Système de vision et détection

#### Champ de vision
- Rayon de vision : Environ 5-7 cases autour du joueur
- Peut être augmenté avec des objets (torche, lanterne)
- Les murs bloquent la vision

#### Mini-carte
- Affichée dans un coin de l'écran
- Montre les zones explorées
- Indicateurs :
  - Position du joueur (flèche bleue)
  - Trappes/escaliers découverts (icône spéciale)
  - Zones non explorées (noir)

### 4.2 Système de son et signaux

#### Signaux audio
- Bruits de pas : Révèlent la proximité d'autres joueurs/ennemis
- Son d'ouverture de coffre : Indique qu'un joueur a trouvé un trésor
- Son de combat : Signale un affrontement proche

#### Signaux visuels
- Effets de particules lors de l'utilisation d'objets spéciaux
- Indicateur de dégâts reçus

### 4.3 Système de respawn (optionnel)
Deux options de design :
- **Option A** : Mort permanente (spectateur jusqu'à la fin de la partie)
- **Option B** : Respawn unique avec pénalité (perte d'objets, retour à l'étage 1)

*À décider selon le gameplay souhaité*

---

## 5. Modes de jeu

### 5.1 Mode principal : Course au trésor
- 2-4 joueurs
- Premier à trouver le trésor gagne
- Génération procédurale complète

### 5.2 Mode entraînement (Solo)
- Permet de s'entraîner contre des IA
- Génération identique au mode multijoueur
- Sert à apprendre les mécaniques

### 5.3 Modes futurs (extensions possibles)
- **Mode coopératif** : Les joueurs collaborent contre le donjon
- **Mode équipe** : 2v2, première équipe au trésor
- **Mode arène** : Combat PvP dans une salle fermée

---

## 6. Progression et méta-jeu

### 6.1 Système de déblocage
Entre les parties, les joueurs peuvent débloquer :
- **Nouveaux personnages** : Avec des stats/capacités différentes
- **Skins/cosmétiques** : Personnalisation visuelle
- **Modificateurs de partie** : Donjons plus grands, ennemis plus difficiles, etc.

### 6.2 Système de récompense
- **Pièces d'or** : Collectées durant les parties (même en cas de défaite)
- **Succès/Achievements** : Débloquer des conditions spécifiques
- **Classement** : Système de rang (Bronze, Argent, Or, Platine)

### 6.3 Conditions de déblocage (exemples)
- Terminer 5 parties → Débloquer personnage Voleur (vitesse +10%)
- Descendre 20 étages au total → Débloquer skin "Explorateur"
- Gagner 3 parties → Débloquer modificateur "Donjon géant"

---

## 7. Direction artistique

### 7.1 Style visuel
- **Pixel Art rétro** : Style épuré et lisible
- **Palette de couleurs** : Tons sombres pour l'ambiance de donjon, avec des accents de couleurs vives pour les objets importants
- **Inspiration** : Dungeon Tileset (assets disponibles), style 16-bit

### 7.2 Assets
- **Base** : 2D Pixel Dungeon Asset Pack (déjà présent dans le projet)
- **Personnages** : Sprites animés (idle, marche, attaque)
- **Tilesets** : Murs, sols, portes, escaliers
- **Objets** : Coffres, armes, potions, pièges
- **Effets** : Particules de magie, sang, fumée

### 7.3 Interface utilisateur
- **Style minimaliste** : Ne pas surcharger l'écran
- **HUD** :
  - Barre de vie (coin supérieur gauche)
  - Inventaire rapide (bas de l'écran)
  - Mini-carte (coin supérieur droit)
  - Indicateur d'étage actuel
- **Menus** : Style pixel art cohérent avec le jeu

### 7.4 Animations
- **Personnages** : Idle (4 frames), marche (4 frames), attaque (3-4 frames)
- **Ennemis** : Idle, attaque
- **Objets** : Coffres qui s'ouvrent, torches animées, pièges qui se déclenchent

### 7.5 Ambiance
- **Atmosphère** : Sombre, mystérieuse, légèrement oppressante
- **Éclairage** : Zones sombres avec sources de lumière limitées (torches, sort de lumière)
- **Particules** : Poussière, lueurs magiques, étincelles

---

## 8. Audio

### 8.1 Musique
- **Menu principal** : Thème épique mais calme
- **En partie** : Musique d'ambiance sombre et rythmée
- **Combat** : Intensification de la musique
- **Victoire/Défaite** : Jingles courts

### 8.2 Effets sonores
- **Déplacements** : Bruits de pas (différents selon le sol)
- **Combat** : Sons d'épée, magie, coups reçus
- **Interactions** : Ouverture de coffre, ramassage d'objet, placement de piège
- **Ambiance** : Gouttes d'eau, vent, chaînes

---

## 9. Aspects techniques

### 9.1 Moteur de jeu
À définir selon l'équipe :
- **Godot** : Open-source, bon pour le 2D, exports multi-plateformes
- **Unity** : Populaire, nombreux assets et tutoriels
- **Bevy** : Pour Rust, moderne (vu la mention du Bevy Jam #6)

### 9.2 Réseau/Multijoueur
- **Architecture** : Client-Server ou Peer-to-Peer
- **Synchronisation** : Position des joueurs, état du donjon, objets ramassés
- **Latence** : Prédiction côté client pour la fluidité

### 9.3 Génération procédurale
- **Algorithme** : BSP (Binary Space Partitioning) ou Cellular Automata
- **Seed** : Permettre de partager des seeds pour rejouer la même map
- **Contraintes** : Garantir un chemin accessible entre spawn et trésor

### 9.4 Sauvegarde
- **Progression du joueur** : Déblocages, pièces, stats
- **Paramètres** : Options audio/vidéo, contrôles
- **Format** : JSON ou base de données locale

### 9.5 Optimisation mobile
- **Interface tactile** : Adaptée pour écrans small
- **Performance** : 60 FPS stable sur mobiles moyens
- **Orientations** : Support portrait ET paysage

---

## 10. Plan de développement

### 10.1 Objectifs du projet
- Avoir un produit fini et partageable
- Publication sur Steam
- Version mobile fonctionnelle
- Créer une ambiance immersive
- Histoire/lore simple mais cohérent

### 10.2 Phases de développement

#### Phase 1 : Prototype jouable (MVP)
**Durée estimée** : 2-3 mois

**Fonctionnalités** :
- Génération basique de donjon (1 étage)
- Personnage jouable avec déplacement
- 1-2 types d'armes fonctionnelles
- Système de combat de base
- Brouillard de guerre
- Collecte d'objets
- Menu principal minimal

**Objectif** : Valider le gameplay core loop

#### Phase 2 : Multijoueur et progression
**Durée estimée** : 2-3 mois

**Fonctionnalités** :
- Implémentation du multijoueur (2 joueurs minimum)
- Système de plusieurs étages (3-5 étages)
- Trappes et pelles pour descendre
- Système de pièges
- Ennemis PNJ avec IA basique
- Interface utilisateur complète

**Objectif** : Avoir une boucle de jeu complète et testable

#### Phase 3 : Contenu et polish
**Durée estimée** : 2-3 mois

**Fonctionnalités** :
- Variété d'armes et objets (10+ items)
- Plusieurs types d'ennemis (5-7 types)
- Système de progression/déblocages
- Animations et effets visuels
- Musique et effets sonores
- Support jusqu'à 4 joueurs
- Mode solo contre IA

**Objectif** : Enrichir l'expérience de jeu

#### Phase 4 : Portage et optimisation
**Durée estimée** : 1-2 mois

**Fonctionnalités** :
- Adaptation mobile (contrôles tactiles)
- Optimisation performance
- Tests utilisateurs et corrections de bugs
- Équilibrage du gameplay
- Préparation pour Steam (achievements, leaderboards)

**Objectif** : Préparer le release

#### Phase 5 : Release et support
**Fonctionnalités** :
- Publication Steam
- Publication sur stores mobiles
- Support post-launch
- Corrections de bugs basées sur feedback
- Éventuels DLC/extensions

---

## 11. Métriques et équilibrage

### 11.1 Durée de partie cible
- **Minimum** : 5 minutes (speedrun, chance extrême)
- **Moyenne** : 10-15 minutes
- **Maximum** : 25 minutes (exploration complète)

### 11.2 Équilibrage des armes
- Armes de mêlée : Dégâts élevés mais risque (proximité)
- Armes à distance : Dégâts modérés, sécurité, munitions limitées
- Chaque arme doit avoir une utilité situationnelle

### 11.3 Équilibrage des personnages (si classes différentes)
- Différences de stats : ±15% maximum
- Chaque personnage doit avoir une stratégie viable
- Pas de personnage dominant dans tous les scénarios

### 11.4 Métriques à suivre
- Taux de victoire par joueur/personnage
- Temps moyen de partie
- Objets les plus/moins utilisés
- Causes de mort les plus fréquentes
- Satisfaction joueur (via feedback)

---

## 12. Risques et solutions

### 12.1 Risques identifiés

| Risque | Impact | Probabilité | Solution |
|--------|--------|-------------|----------|
| Génération procédurale injouable | Élevé | Moyen | Tests automatisés pour vérifier la jouabilité des maps |
| Déséquilibre multijoueur | Élevé | Élevé | Phases de playtest régulières, ajustements itératifs |
| Problèmes de latence réseau | Moyen | Moyen | Prédiction côté client, choix d'architecture adaptée |
| Scope trop large | Élevé | Élevé | Développement par phases, MVP d'abord |
| Manque d'assets | Faible | Faible | Assets déjà identifiés, backup sur itch.io |

### 12.2 Plan de contingence
- Si le multijoueur prend trop de temps : Focus sur le solo vs IA d'abord
- Si la génération procédurale est trop complexe : Maps prédéfinies avec variation
- Si le portage mobile pose problème : Release PC d'abord, mobile ensuite

---

## 13. Références et inspirations

### 13.1 Jeux de référence
- **Spelunky** : Génération procédurale, exploration verticale
- **Enter the Gungeon** : Combat, objets variés, roguelike
- **TowerFall** : Multijoueur local compétitif
- **Crypt of the NecroDancer** : Ambiance donjon, pixel art
- **Monaco** : Brouillard de guerre, multijoueur compétitif

### 13.2 Analyses mécaniques
- Étudier comment ces jeux gèrent :
  - Le brouillard de guerre (Monaco, Spelunky)
  - La génération procédurale équitable (Spelunky, Binding of Isaac)
  - L'équilibrage multijoueur compétitif (TowerFall, Duck Game)

---

## 14. Annexes

### 14.1 Glossaire
- **Brouillard de guerre** : Zone non explorée invisible sur la carte
- **Roguelike** : Genre avec génération procédurale et mort permanente
- **Seed** : Valeur initiale pour la génération procédurale
- **BSP** : Binary Space Partitioning, algorithme de génération de donjon

### 14.2 Ressources assets identifiées
- https://0x72.itch.io/dungeontileset-ii (gratuit)
- https://zerie.itch.io/tiny-rpg-character-asset-pack (payant)
- https://ansimuz.itch.io/tinyrpg-collection (payant)
- 2D Pixel Dungeon Asset Pack (déjà dans le projet)

### 14.3 Questions ouvertes à décider
- [ ] Mort permanente ou respawn ?
- [ ] Nombre exact d'étages ?
- [ ] Pelle usage unique ou multiple ?
- [ ] Classes de personnages différentes ou tous identiques ?
- [ ] Peut-on s'attaquer entre joueurs directement ?
- [ ] Le boss final est-il obligatoire à battre pour récupérer le trésor ?

---

## 15. Historique des versions

| Version | Date | Auteur | Modifications |
|---------|------|--------|---------------|
| 1.0 | 2025-11-01 | Équipe | Création du GDD initial |

---

**Document vivant** : Ce GDD est destiné à évoluer tout au long du développement. Les sections doivent être mises à jour régulièrement selon les décisions de l'équipe et les retours de playtest.
