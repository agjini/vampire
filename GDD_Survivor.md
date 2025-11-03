# Game Design Document
## Survivor-like Action Roguelite

---

## 1. Vue d'ensemble

### 1.1 Concept résumé
Un jeu d'action survivor-like en 2D pixel art où le joueur affronte des hordes d'ennemis infinies dans un environnement procédural. Le personnage progresse en niveau durant la partie pour débloquer des capacités, et gagne de l'argent pour des améliorations permanentes entre les parties. L'objectif est de survivre le plus longtemps possible face à une difficulté croissante exponentielle.

### 1.2 Informations générales
- **Genre** : Action / Survivor-like / Roguelite
- **Sous-genre** : Bullet heaven, Horde survival
- **Plateforme cible** : PC (Steam) / Mobile (Portrait et Paysage)
- **Public visé** : Joueurs casual à hardcore
- **Durée de partie estimée** : 15-30 minutes par run
- **Nombre de joueurs** : Solo (possibilité de co-op en extension)

### 1.3 Piliers du jeu
1. **Accessibilité** : Facile à prendre en main, difficile à maîtriser
2. **Progression satisfaisante** : Montée en puissance gratifiante en partie + méta-progression
3. **Build diversity** : Nombreuses combinaisons de capacités et d'armes
4. **Rejouabilité** : Chaque run est différent, incitation à "une dernière partie"

### 1.4 Références principales
- **Vampire Survivors** : Gameplay automatique, progression de niveau, builds
- **Nomad Survivor** : Système de spécialisation, modulateurs de difficulté
- **20 Minutes Till Dawn** : Contrôle manuel des tirs, système d'éléments
- **Brotato** : Économie d'armes et objets
- **Gun Smoke / Gradius** : Inspirations shoot'em up classiques

---

## 2. Gameplay

### 2.1 Core Loop
```
Spawn sur la map → Tuer des ennemis → Gagner de l'XP →
Monter de niveau → Choisir une amélioration → Répéter →
Mourir → Dépenser l'argent gagné → Débloquer des améliorations permanentes →
Recommencer avec de nouveaux avantages
```

### 2.2 Objectif principal
Survivre le plus longtemps possible face à des vagues d'ennemis de plus en plus difficiles, tout en accumulant de l'or pour débloquer des améliorations permanentes.

### 2.3 Conditions de victoire/défaite
- **Victoire** : Survivre jusqu'à un seuil de temps défini (15-30 minutes selon le mode)
- **Défaite** : Points de vie à 0
- **Score** : Basé sur le temps de survie, ennemis tués, argent collecté

### 2.4 Contrôles

#### PC
- **ZQSD / Flèches** : Déplacement du personnage
- **Souris** : Viser (si mode de tir manuel)
- **Clic gauche** : Tirer (si mode de tir manuel)
- **Espace** : Utiliser capacité active
- **Tab** : Pause / Menu
- **Touches 1-4** : Capacités actives rapides

#### Mobile
- **Joystick virtuel gauche** : Déplacement
- **Joystick virtuel droit** : Viser (si nécessaire) OU Auto-aim
- **Bouton capacité** : Déclencher capacité active
- **Mode automatique** : Attaques automatiques vers l'ennemi le plus proche

---

## 3. Mécaniques de jeu

### 3.1 Système de combat

#### Attaques de base
- **Type** : Automatique ou semi-automatique (selon personnage/arme)
- **Direction** : Vers l'ennemi le plus proche OU direction du joueur
- **Dégâts** : Évolutifs selon les améliorations
- **Cadence** : Dépend de l'arme équipée

#### Types d'armes
Catégories inspirées des survivor-likes :

**Armes à projectiles**
- Pistolet, fusil, arc : Tir direct en ligne droite
- Arbalète magique : Traverse plusieurs ennemis
- Bâton : Boules de feu qui explosent

**Armes orbitales**
- Couteaux/épées tournant autour du joueur
- Boucliers qui bloquent et renvoient les projectiles
- Orbes magiques en rotation

**Armes de zone (AoE)**
- Explosions périodiques
- Flaques de lave/poison au sol
- Tornades qui aspirent les ennemis

**Armes spéciales**
- Laser continu qui balaie la zone
- Invocations (créatures alliées)
- Auras de dégâts autour du joueur

#### Système d'éléments (inspiré de 20 Minutes Till Dawn)
**Éléments disponibles** :
- **Feu** : Inflige des dégâts sur la durée (brûlure)
- **Glace** : Ralentit/gèle les ennemis
- **Foudre** : Chaîne sur plusieurs ennemis, inflige un stun
- **Poison** : Dégâts sur la durée amplifiés

**Synergies élémentaires** :
- Feu + Glace = Explosion de vapeur
- Foudre + Eau = Zone électrifiée étendue
- Feu sur ennemi brûlé qui meurt = Explosion qui propage les flammes

### 3.2 Système de progression en partie

#### Gain d'expérience (XP)
- Obtenue en tuant des ennemis
- Quantité augmente avec le niveau de l'ennemi
- Barre d'XP visible en permanence

#### Montée de niveau
- **Fréquence** : Environ toutes les 30-60 secondes en début de partie
- **Effet** : Le temps se fige (ou ralentit), le joueur choisit une amélioration
- **Choix** : 3-4 options aléatoires parmi le pool disponible

#### Types d'améliorations
1. **Nouvelles armes** : Ajoute une arme supplémentaire à l'arsenal
2. **Amélioration d'arme existante** : +dégâts, +cadence, +projectiles, +portée
3. **Passifs défensifs** : +PV max, régénération, armure, vitesse de déplacement
4. **Passifs offensifs** : +dégâts globaux, +chances de critique, +zone d'effet
5. **Capacités actives** : Dash, bouclier temporaire, explosion

#### Spécialisations (inspiré de Nomad Survivor)
- Tous les **10-15 niveaux**, le joueur choisit une spécialisation majeure
- **Effet** : Transforme radicalement une arme ou capacité existante
- **Exemples** :
  - Pistolet → Fusil à pompe (moins de portée, plus de dégâts en zone)
  - Couteaux orbitaux → Lames géantes (moins nombreux, beaucoup plus puissants)
  - Boules de feu → Météores (plus lent, AoE massif)

### 3.3 Système d'ennemis

#### Types d'ennemis de base
- **Zombie** : Lent, faibles HP, apparaît en groupe
- **Squelette** : Vitesse moyenne, tire des projectiles à distance
- **Vampire** : Rapide, forts dégâts de mêlée
- **Fantôme** : Traverse les obstacles, vitesse variable
- **Golem** : Très lent, énormes HP, dégâts élevés

#### Vagues et scaling
- **Spawn continu** : Les ennemis apparaissent constamment aux bords de l'écran
- **Densité croissante** : Plus d'ennemis au fil du temps
- **Élites** : Ennemis renforcés avec aura de couleur (HP x3, dégâts x2)
- **Boss** : Apparaissent tous les 5-10 minutes
  - HP massifs
  - Patterns d'attaque spécifiques
  - Drops garantis (amélioration + or)

#### Comportements IA
- **Basique** : Poursuite directe du joueur
- **Tireur** : Maintient une distance, tire périodiquement
- **Téléporteur** : Se téléporte près du joueur
- **Invocateur** : Génère d'autres ennemis

### 3.4 Système de collectibles

#### Drops en partie
- **Gemmes d'XP** : Bleues (petites), vertes (moyennes), violettes (grosses)
- **Pièces d'or** : Monnaie permanente conservée après la mort
- **Coffres** : Contiennent des améliorations temporaires ou de l'or
- **Cœurs** : Restaurent des points de vie
- **Aimants** : Attirent automatiquement les collectibles

#### Collection automatique
- Les gemmes/pièces sont attirées vers le joueur quand il s'approche
- Rayon de collecte améliorable via passifs
- Option : Familier/animal de compagnie qui collecte automatiquement

### 3.5 Système de statistiques

#### Stats du personnage
- **Points de vie (HP)** : 100 de base
- **Armure** : Réduction de dégâts en %
- **Vitesse de déplacement** : Pixels/seconde
- **Portée de collecte** : Rayon en pixels
- **Régénération** : HP/seconde
- **Chances de critique** : % de chance, multiplicateur de dégâts
- **Vitesse d'attaque** : Multiplicateur global

---

## 4. Méta-progression

### 4.1 Système monétaire permanent

#### Or gagné en partie
- Tombe des ennemis tués
- Conservé même en cas de mort
- Bonus multiplicateur selon performance (boss tués, temps de survie)

#### Dépense hors partie
Menu "Améliorations permanentes" accessible depuis le menu principal :

**Catégories d'achats** :
1. **Stats de base** : +HP max, +vitesse, +dégâts (5-10 niveaux par stat)
2. **Déblocages d'armes** : Rendre disponibles de nouvelles armes en partie
3. **Déblocages de personnages** : Nouveaux personnages avec stats/capacités uniques
4. **Compagnons/Pets** : Animaux qui aident en combat ou collectent
5. **Cosmétiques** : Skins de personnage, effets visuels

#### Système de prix progressifs
- Première amélioration de HP : 100 or
- Deuxième : 250 or
- Troisième : 500 or, etc.
- Formule exponentielle pour encourager la diversification

### 4.2 Système de déblocage par achievements

#### Conditions de déblocage
Certains éléments ne peuvent être débloqués qu'en remplissant des conditions spécifiques :

**Exemples** :
- Tuer 1000 ennemis au total → Débloquer l'arme "Épée démoniaque"
- Survivre 30 minutes → Débloquer le personnage "Survivant"
- Atteindre le niveau 50 en une run → Débloquer la capacité "Transcendance"
- Battre le boss final → Débloquer le mode "Cauchemar"
- Finir une run sans prendre de dégâts pendant 5 minutes → Débloquer "Mode Parfait"

#### Suivi des progrès
- Tableau des achievements dans le menu
- Barre de progression pour les achievements cumulatifs
- Récompenses visibles avant déblocage pour créer de l'anticipation

### 4.3 Système de personnages

#### Personnages jouables
Chaque personnage a des stats et capacités uniques :

**Exemples** :
1. **Knight** (par défaut)
   - Arme de départ : Épée orbitale
   - +20% HP, vitesse normale
   - Capacité : Bouclier temporaire

2. **Archer**
   - Arme de départ : Arc rapide
   - HP normale, +15% vitesse
   - Capacité : Flèche perforante

3. **Mage**
   - Arme de départ : Boules de feu
   - -10% HP, +50% dégâts magiques
   - Capacité : Téléportation courte

4. **Berserker** (à débloquer)
   - Arme de départ : Haches tournoyantes
   - +30% HP, -10% vitesse, +20% dégâts
   - Capacité : Rage (dégâts x2 pendant 10s)

5. **Nécromancien** (à débloquer)
   - Arme de départ : Invoque des squelettes
   - HP faible, convoque des alliés
   - Capacité : Armée des morts

---

## 5. Systèmes de difficulté

### 5.1 Difficulté dynamique

#### Scaling naturel
- Plus le temps passe, plus les ennemis sont nombreux et résistants
- **0-5 min** : Introduction, ennemis faibles
- **5-10 min** : Densité augmente, premiers élites
- **10-15 min** : Boss intermédiaire, spawn accéléré
- **15-20 min** : Difficulté élevée, ennemis élites fréquents
- **20-30 min** : Survie extrême, boss multiples
- **30+ min** : Mode infini, difficulté exponentielle

### 5.2 Modulateurs de difficulté (inspiré de Nomad Survivor)

#### Système de "Malédictions"
Avant de lancer une partie, le joueur peut activer des malédictions pour augmenter la difficulté :

**Exemples de malédictions** :
- **Horde déchaînée** : +50% ennemis, +25% or gagné
- **Géants** : +100% HP ennemis, +30% or gagné
- **Vitesse fulgurante** : +40% vitesse ennemis, +20% or gagné
- **Projectiles mortels** : Ennemis tirent plus de projectiles, +35% or gagné
- **Régénération faible** : -50% efficacité des soins, +15% or gagné
- **Boss rush** : Boss apparaissent 2x plus souvent, +50% or gagné

#### Système de paliers
- Les malédictions s'empilent
- Récompenses multipliées selon le nombre de malédictions actives
- Achievements spéciaux pour combinaisons extrêmes

### 5.3 Modes de jeu

#### Mode Normal
- Difficulté standard
- Objectif : Survivre 20 minutes
- Tous les déblocages disponibles

#### Mode Infini
- Pas de limite de temps
- Difficulté augmente infiniment
- Classement par temps de survie

#### Mode Cauchemar (à débloquer)
- Ennemis 2x plus résistants dès le départ
- Moins de drops de vie
- Récompenses doublées

#### Mode Boss Rush (à débloquer)
- Boss apparaissent toutes les 2 minutes
- Hordes réduites entre les boss
- Focus sur les combats épiques

---

## 6. Environnement et map

### 6.1 Génération de map

#### Carte infinie
- Génération procédurale à la volée
- Le joueur peut se déplacer librement dans toutes les directions
- Pas de bordures visibles

#### Biomes
Différents environnements avec ambiances variées :

1. **Forêt hantée** : Arbres, brouillard léger
2. **Cimetière maudit** : Pierres tombales, cryptes
3. **Donjon souterrain** : Murs de pierre, torches
4. **Plaine désolée** : Peu d'obstacles, ouvert
5. **Temple ancien** : Ruines, colonnes brisées

#### Objets sur la map
- **Obstacles destructibles** : Coffres, tonneaux, caisses
  - Contiennent or, XP, ou améliorations temporaires
- **Obstacles permanents** : Arbres, rochers, murs
  - Bloquent le passage, peuvent servir de couverture
- **Objets interactifs** : Autels, fontaines
  - Effets temporaires (soin, bonus de dégâts)

### 6.2 Système de caméra
- **Vue top-down** : Le joueur au centre de l'écran
- **Zoom adaptatif** : Peut se dézoomer légèrement quand beaucoup d'ennemis
- **Indicateurs hors-écran** : Flèches indiquant boss ou collectibles importants

---

## 7. Interface utilisateur (UI/UX)

### 7.1 HUD en jeu

**Éléments permanents** :
- **Barre de vie** : Coin supérieur gauche, rouge, avec valeur numérique
- **Barre d'XP** : Bas de l'écran, large, montre progression vers niveau suivant
- **Niveau actuel** : À côté de la barre XP
- **Timer** : Coin supérieur centre, compte le temps de survie
- **Mini-carte** : Coin supérieur droit (optionnel, peut être masquée)
- **Compteur de kills** : Coin inférieur gauche
- **Or collecté** : Icône pièce + nombre

**Éléments contextuels** :
- **Indicateur de niveau up** : Animation flash quand disponible
- **Barre de vie du boss** : Apparaît en haut quand boss actif
- **Notification de déblocage** : Pop-up centrale lors d'achievements

### 7.2 Écran de level-up

**Design** :
- Le jeu se met en pause (ou ralentit fortement)
- Affichage de 3-4 cartes représentant les choix
- Chaque carte montre :
  - Icône de l'arme/capacité
  - Nom
  - Description courte
  - Niveau actuel (si amélioration)
- Navigation au clavier/souris/tactile

### 7.3 Menus

#### Menu principal
- **Jouer** : Lancer une partie
- **Personnages** : Sélection et déblocages
- **Améliorations** : Dépenser l'or permanent
- **Achievements** : Liste des succès
- **Options** : Audio, graphismes, contrôles
- **Quitter**

#### Écran de fin de run
- **Statistiques** :
  - Temps de survie
  - Ennemis tués
  - Dégâts infligés
  - Niveau atteint
  - Or gagné durant cette run
- **Bouton "Recommencer"** : Rapide, encourager "une dernière"
- **Bouton "Améliorer"** : Aller au menu des améliorations permanentes

---

## 8. Direction artistique

### 8.1 Style visuel
- **Pixel Art rétro** : Inspiré des jeux 16-bit
- **Style épuré** : Lisibilité prioritaire pour le gameplay
- **Thématique** : Real Fantasy / Dark Fantasy
- **Palette** : Tons sombres avec accents colorés pour les éléments importants

### 8.2 Exemples visuels
- **Personnages** : Style chibi/super deformed, 32x32 ou 64x64 pixels
- **Ennemis** : Designs variés mais reconnaissables instantanément
- **Effets** : Particules simples mais impactantes
- **UI** : Interface propre avec bordures pixelisées

### 8.3 Assets identifiés
- **2D Pixel Dungeon Asset Pack** : Déjà présent, utilisable pour environnements
- **Charas-project** : Sprites de personnages (http://charas-project.net/resources.php)
- **Dungeon Tileset II** : 0x72 (gratuit)
- **Tiny RPG packs** : Assets payants de qualité

### 8.4 Animations

#### Personnages
- **Idle** : 2-4 frames, léger mouvement
- **Marche** : 4-8 frames selon directions
- **Attaque** : 3-4 frames (si visible)
- **Mort** : 4-6 frames, fade out

#### Ennemis
- **Idle/Marche** : 2-4 frames
- **Attaque** : 2-3 frames
- **Mort** : Explosion/dissipation de particules

#### Effets
- **Projectiles** : Rotation ou animation simple
- **Explosions** : 4-6 frames
- **Collectibles** : Rotation/flottement continu

### 8.5 Effets visuels

**Feedbacks importants** :
- **Hit feedback** : Flash blanc sur l'ennemi touché
- **Critique** : Particules dorées + shake de caméra
- **Level up** : Onde lumineuse émane du joueur
- **Mort ennemi** : Explosion de particules + drop d'objets
- **Dégâts reçus** : Vignette rouge sur les bords de l'écran

---

## 9. Audio

### 9.1 Musique

#### Ambiances musicales
- **Menu principal** : Thème épique orchestral ou chiptune
- **En jeu (début)** : Musique d'ambiance sombre, tempo modéré
- **En jeu (intensification)** : Augmentation du tempo à partir de 10-15 min
- **Boss** : Track épique avec percussions marquées
- **Victoire** : Fanfare triomphante courte
- **Défaite** : Musique sombre et mélancolique

#### Sources identifiées
- **Musiques avec licences** : Dark Fantasy et Chiptunes (mentionné dans reflexion.md)
- **Banques libres** : OpenGameArt, FreeMusicArchive
- **Compositeurs indie** : itch.io music assets

### 9.2 Effets sonores

#### Sons de gameplay
- **Armes** :
  - Épée : Swoosh métallique
  - Pistolet : Bang sec
  - Magie : Whoosh mystique
- **Impacts** :
  - Sur ennemi : Thunk sourd
  - Critique : Impact amplifié + tintement
- **Ennemis** :
  - Grognements à l'apparition
  - Cris à la mort
- **Collecte** :
  - XP : Tinkle cristallin
  - Or : Clink métallique
  - Objet : Cloche douce
- **UI** :
  - Navigation menu : Bip court
  - Sélection : Confirmation sonore
  - Level up : Jingle ascendant

#### Mixage audio
- Priorité aux sons de gameplay critiques
- Limitation du nombre de sons simultanés (pooling)
- Effets de fade pour éviter la cacophonie

---

## 10. Aspects techniques

### 10.1 Moteur de jeu

#### Options recommandées
1. **Godot 4** :
   - Excellent pour le 2D
   - Open-source, gratuit
   - Export facile PC + Mobile
   - GDScript accessible

2. **Unity** :
   - Très documenté
   - Asset Store fournie
   - Bon pour débutants
   - Export multi-plateformes

3. **Bevy (Rust)** :
   - Performance excellente
   - ECS moderne
   - Courbe d'apprentissage élevée
   - Communauté croissante
   - Mentionné dans les références du projet

### 10.2 Architecture technique

#### Systèmes principaux
- **Entity Component System (ECS)** : Gestion des entités (joueur, ennemis, projectiles)
- **Pool d'objets** : Réutilisation des ennemis/projectiles pour performance
- **Quad-tree / Spatial hash** : Optimisation des collisions
- **Data-driven design** : Fichiers config pour armes, ennemis, équilibrage

#### Fichier de configuration (mentionné dans reflexion.md)
```json
{
  "player": {
    "base_hp": 100,
    "base_speed": 150,
    "base_damage": 10
  },
  "enemies": {
    "zombie": {
      "hp": 50,
      "speed": 60,
      "damage": 10
    }
  },
  "weapons": {
    "pistol": {
      "damage": 15,
      "fire_rate": 2.0,
      "projectile_speed": 300
    }
  }
}
```
- Permet ajustements rapides sans recompilation
- Facilite le balancing et les tests

### 10.3 Performance

#### Optimisations nécessaires
- **Object pooling** : Pas d'instanciation/destruction continue
- **Culling** : Ne pas calculer/afficher hors écran
- **Batching** : Regrouper draw calls similaires
- **LOD** : Simplifier ennemis éloignés

#### Cibles de performance
- **PC** : 60 FPS stable, 1000+ ennemis à l'écran
- **Mobile** : 30-60 FPS, 300-500 ennemis
- **Mémoire** : <500 MB RAM sur mobile

### 10.4 Sauvegarde

#### Données à sauvegarder
- **Progression permanente** :
  - Or total
  - Améliorations permanentes achetées
  - Personnages débloqués
  - Achievements complétés
  - Statistiques globales (ennemis tués, temps total joué)

- **Paramètres** :
  - Volume audio (musique, SFX)
  - Résolution / Fullscreen
  - Keybindings personnalisés

#### Format
- **JSON** : Lisible, facile à parser
- **Chiffrement léger** : Éviter triche triviale
- **Cloud save** (Steam) : Sync entre appareils

### 10.5 Portage mobile

#### Adaptations nécessaires
- **Contrôles tactiles** :
  - Joystick virtuel pour déplacement
  - Auto-aim pour attaques
  - Boutons capacités accessibles
- **UI redimensionnée** :
  - Éléments plus gros
  - Textes lisibles sur petits écrans
- **Performance** :
  - Réduction nombre particules
  - Résolution adaptative
- **Orientations** :
  - **Portrait** : UI verticale, champ de vision adapté
  - **Paysage** : UI horizontale, plus d'espace visible

---

## 11. Plan de développement

### 11.1 Objectifs du projet
- Avoir un produit fini et partageable
- Publication sur Steam
- Version mobile fonctionnelle (portrait/paysage)
- Créer une ambiance immersive
- Histoire/lore simple mais présent

### 11.2 Phases de développement

#### Level 1 : Forme minimale du jeu (MVP)
**Durée estimée** : 2-3 mois

**Fonctionnalités** :
- Personnage jouable avec déplacement fluide
- 1-2 armes fonctionnelles (automatiques)
- Ennemis basiques (2-3 types)
- Spawn d'ennemis continu
- Système de dégâts/mort
- Système d'XP et montée de niveau
- 5-10 améliorations disponibles au level-up
- Collecte d'or en partie
- Difficulté progressive simple (scaling linéaire)
- Menu minimal (démarrer, quitter)

**Objectif** : Valider que le core loop est fun

**Tests** :
- Le jeu est-il addictif sur 5 minutes ?
- La montée en puissance est-elle satisfaisante ?
- Les contrôles sont-ils fluides ?

#### Level 2 : Génération procédurale et gestion avancée des ennemis
**Durée estimée** : 1-2 mois

**Fonctionnalités** :
- Génération procédurale de map infinie
- Obstacles et objets interactifs sur la map
- 5-7 types d'ennemis avec comportements variés
- Système d'ennemis élites
- Premier boss fonctionnel
- Amélioration du spawn (vagues, densité dynamique)
- Système de particules et effets visuels

**Objectif** : Créer de la variété et du challenge

#### Level 3 : Progression du joueur et objets
**Durée estimée** : 2-3 mois

**Fonctionnalités** :
- 10-15 armes différentes
- Système de spécialisations (évolutions d'armes)
- 30+ améliorations passives/actives
- Système d'éléments (Feu, Glace, Foudre)
- Synergies entre capacités
- Méta-progression hors partie :
  - Menu d'améliorations permanentes
  - Dépense de l'or
  - 5-10 niveaux d'améliorations par stat
- Déblocages par achievements
- 3-5 personnages jouables
- UI complète et polie

**Objectif** : Profondeur et rejouabilité maximales

#### Level 4 : Polish et contenu
**Durée estimée** : 1-2 mois

**Fonctionnalités** :
- Modulateurs de difficulté (malédictions)
- Modes de jeu additionnels (Infini, Cauchemar, Boss Rush)
- Système de compagnons/pets
- Animations complètes pour tous les éléments
- Musiques et SFX finalisés
- Biomes supplémentaires (3-5 biomes)
- Boss supplémentaires (4-6 boss)
- Achievements (30-50)
- Leaderboards locaux
- Tutoriel intégré
- Localisation (anglais minimum)

**Objectif** : Expérience complète et immersive

#### Level 5 : Portage et optimisation
**Durée estimée** : 1-2 mois

**Fonctionnalités** :
- Adaptation mobile (contrôles tactiles)
- Support portrait/paysage
- Optimisation performance (mobile et PC low-end)
- Tests utilisateurs et corrections de bugs
- Équilibrage final basé sur données
- Préparation Steam :
  - Achievements Steam
  - Steam Cloud
  - Leaderboards en ligne
  - Trading cards (optionnel)
- Préparation stores mobiles (App Store, Google Play)

**Objectif** : Prêt pour le release

#### Level 6 : Release et support
**Durée estimée** : En cours

**Activités** :
- Publication Steam (Early Access ou release complet)
- Publication mobile
- Communication (trailer, screenshots, description)
- Suivi des reviews et feedback
- Patches de bugs critiques
- Équilibrage basé sur données réelles
- Éventuels DLC/extensions :
  - Nouveaux personnages
  - Nouveaux biomes
  - Mode co-op local/en ligne

---

## 12. Métriques et équilibrage

### 12.1 Métriques à suivre

#### En développement
- **Playtest interne** :
  - Temps de survie moyen par testeur
  - Armes/builds les plus/moins utilisés
  - Moments où les joueurs meurent (timeframe)
  - Feeling de difficulté (trop facile/dur ?)

#### Post-launch
- **Analytics** :
  - Temps de jeu moyen par session
  - Taux de rétention (J1, J7, J30)
  - Armes/personnages les plus populaires
  - Taux de complétion des achievements
  - Distribution du temps de survie

### 12.2 Équilibrage des armes

#### Philosophie
- Chaque arme doit avoir une utilité
- Pas d'arme "objectivement meilleure"
- Synergies encouragées entre armes

#### Méthode
- **DPS normalisé** : Toutes les armes de même rareté ont un DPS similaire
- **Trade-offs** : Dégâts élevés = cadence faible, et vice-versa
- **Niches** : Certaines armes excellent contre certains types d'ennemis

#### Exemple de balance
```
Pistolet : DPS 50, cadence haute, dégâts faibles, portée moyenne
Fusil de chasse : DPS 50, cadence basse, dégâts élevés, portée courte
Arc : DPS 50, cadence moyenne, traverse ennemis, portée longue
```

### 12.3 Courbe de difficulté

#### Progression idéale
```
Minutes 0-5   : Facile, apprentissage, power fantasy naissant
Minutes 5-10  : Modéré, premières vraies menaces
Minutes 10-15 : Difficile, nécessite bonne build
Minutes 15-20 : Très difficile, gestion précise requise
Minutes 20-25 : Extrême, survival pur
Minutes 25-30 : Victoire ou défaite imminente
Minutes 30+   : Exponentiel, pour les masters
```

#### Ajustements dynamiques
- Si le joueur est trop faible : Augmenter drops de vie/xp
- Si le joueur est trop fort : Spawn d'élites/boss anticipé

### 12.4 Économie de l'or

#### Gains en partie
- **Ennemi de base** : 1-3 pièces
- **Ennemi élite** : 10-20 pièces
- **Boss** : 50-100 pièces
- **Run complète (20 min)** : 300-600 pièces en moyenne

#### Coûts d'améliorations
- **Amélioration stat niveau 1** : 100 pièces
- **Amélioration stat niveau 5** : 1000 pièces
- **Amélioration stat niveau 10** : 5000 pièces
- **Débloquer un personnage** : 1000-3000 pièces
- **Débloquer une arme** : 500-1500 pièces

#### Progression
- Permettre 1-2 achats significatifs par run
- Éviter le grind excessif
- Malédictions doivent accélérer la progression

---

## 13. Benchmarks et analyses

### 13.1 Vampire Survivors

**Points forts à intégrer** :
- Gameplay simple mais addictif
- Scaling de puissance très satisfaisant
- Énorme variété de builds possibles
- Prix accessible, contenu généreux

**Points faibles à éviter** :
- Peut devenir répétitif après de nombreuses heures
- Interface parfois peu claire
- Certaines combinaisons totalement OP

**Ce qu'on reprend** :
- Core loop : tuer → XP → améliorer → devenir plus fort
- Armes automatiques (accessible)
- Méta-progression généreuse

### 13.2 Nomad Survivor

**Mécaniques intéressantes** :
- **Armes uniques par personnage** : Donne une forte identité
- **Spécialisations tous les 30 niveaux** : Moments forts, transformations majeures
- **Modulateurs de difficulté** : Rejouabilité accrue, récompenses proportionnelles
- **Système de pets** : Aide automatique, collecte facilitée

**À intégrer** :
- Spécialisations d'armes (adaptation de la mécanique)
- Système de malédictions pré-run
- Pets/compagnons

### 13.3 20 Minutes Till Dawn

**Mécaniques distinctives** :
- **Tir manuel** : Engagement actif du joueur
- **Recharge d'armes** : Rythme de gameplay, synergies
- **Système d'éléments** : Profondeur tactique
- **Choix risque/récompense après boss** : Décisions significatives
- **Brouillard de guerre** : Tension, peur de l'inconnu

**À considérer** :
- Option de mode de tir manuel (en plus de l'auto)
- Système d'éléments avec synergies
- Objets avec trade-offs positifs/négatifs

### 13.4 Notre différenciation

**Comment se démarquer** :
- **Fusion auto/manuel** : Possibilité de choisir le mode de jeu
- **Thème Real Fantasy** : Mélange médiéval et créatures mythologiques
- **Évolution d'armes visuelle** : Les armes changent d'apparence en se spécialisant
- **Narration environnementale** : Petite histoire racontée via la map et les boss
- **Mode portrait mobile** : Optimisé pour jeu mobile une main

---

## 14. Histoire et univers

### 14.1 Contexte narratif (simple)

**Pitch** :
*"Dans un monde où la frontière entre réalité et fantaisie s'est effondrée, des hordes de créatures émergent du néant. Vous êtes un survivant doté d'un pouvoir mystérieux : l'absorption d'essence. Chaque ennemi vaincu vous rend plus fort. Combien de temps tiendrez-vous ?"*

### 14.2 Lore minimaliste

**Pas d'exposition lourde**, mais des indices :
- Descriptions des armes/objets donnent des infos sur le monde
- Design des ennemis suggère leur origine
- Biomes racontent une histoire visuelle
- Boss ont un nom et une phrase d'introduction

**Exemple** :
- **Épée du Premier Héros** : *"Forgée par celui qui affronta les ténèbres en premier. Il échoua, mais son héritage perdure."*

### 14.3 Objectif narratif

**Optionnel, déblocable** :
- Après avoir survécu 30 min (victoire), un écran révèle un fragment d'histoire
- Battre tous les boss débloque un "vrai ending"
- Crée une motivation secondaire au-delà du gameplay

---

## 15. Risques et solutions

### 15.1 Risques identifiés

| Risque | Impact | Probabilité | Solution |
|--------|--------|-------------|----------|
| Genre saturé (nombreux clones) | Élevé | Élevé | Se différencier par thème/DA/mécaniques uniques |
| Équilibrage complexe (nombreuses armes) | Élevé | Élevé | Fichier config, tests itératifs, analytics |
| Performance (milliers d'entités) | Moyen | Moyen | Object pooling, ECS, culling agressif |
| Scope trop large | Élevé | Élevé | Développement par phases strictes, MVP d'abord |
| Rejouabilité insuffisante | Élevé | Moyen | Variété de builds, méta-progression généreuse |

### 15.2 Plan de contingence

**Si le développement prend du retard** :
- Réduire le nombre d'armes/ennemis initialement
- Sortir en Early Access sur Steam
- Ajouter du contenu progressivement via updates

**Si l'équilibrage est problématique** :
- Phases de beta avec joueurs externes
- Système de balancing data-driven facile à modifier
- Patches fréquents post-launch

**Si le portage mobile pose problème** :
- Prioriser la version PC
- Mobile en version ultérieure

---

## 16. Annexes

### 16.1 Glossaire

- **Survivor-like** : Genre popularisé par Vampire Survivors, focus survie face à hordes
- **Bullet heaven** : Inverse du bullet hell, le joueur émet des projectiles
- **Roguelite** : Roguelike avec méta-progression permanente
- **Build** : Combinaison d'armes et capacités choisies durant une run
- **Run** : Une partie complète du début à la mort/victoire
- **Scaling** : Augmentation progressive de la difficulté/puissance
- **DPS** : Damage Per Second, dégâts par seconde

### 16.2 Ressources identifiées

**Assets** :
- 2D Pixel Dungeon Asset Pack (dans le projet)
- Charas-project : http://charas-project.net/resources.php
- Dungeon Tileset II : https://0x72.itch.io/dungeontileset-ii
- itch.io game assets : https://itch.io/game-assets

**Musiques** :
- Dark Fantasy et Chiptunes avec licences (mentionné)
- OpenGameArt.org
- FreeMusicArchive

**Références techniques** :
- Bevy Jam #6 : https://itch.io/jam/bevy-jam-6/entries
- Poki (portail de jeux casual) : https://poki.com/fr

### 16.3 Jeux à analyser (mentionnés dans reflexion.md)

- **Vampire Survivors** : Le référent du genre
- **Nomad Survivor** : Spécialisations et modulateurs
- **20 Minutes Till Dawn** : Tir manuel et éléments
- **Void Survivors** : (à analyser)
- **Pixel Survivors** : (à analyser)
- **MegaBonk** : (à analyser)
- **Brotato** : Économie d'items unique
- **Halls of Torment** : DA sombre et ambiance

### 16.4 Questions ouvertes à décider

- [ ] Mode de tir : Automatique uniquement, manuel uniquement, ou choix par personnage ?
- [ ] Durée cible d'une run : 15, 20 ou 30 minutes ?
- [ ] Mort permanente stricte ou possibilité de revive (1 fois, objet rare) ?
- [ ] Mode coopératif : prévu dès le début ou extension future ?
- [ ] Narration : minimale (actuel) ou plus développée ?
- [ ] Monétisation mobile : Premium (payant) ou F2P avec ads/IAP ?
- [ ] Armes équipables en simultané : limité à 6-8 ou illimité ?
- [ ] XP perdue à la mort ou conservée partiellement ?

---

## 17. Checklist pré-production

Avant de commencer le développement, vérifier :

**Équipe** :
- [ ] Rôles définis (programmeur, artiste, game designer, sound designer)
- [ ] Outils choisis (moteur, logiciels graphiques, versioning)
- [ ] Planning établi avec jalons clairs

**Technique** :
- [ ] Moteur de jeu choisi et maîtrisé par au moins 1 membre
- [ ] Architecture technique validée (ECS, pooling, etc.)
- [ ] Fichier de configuration template créé

**Assets** :
- [ ] Style artistique validé (mockups, tests)
- [ ] Assets de base disponibles ou créés (sprite joueur, 2-3 ennemis)
- [ ] Palette de couleurs définie

**Game Design** :
- [ ] Questions ouvertes (16.4) discutées et résolues
- [ ] Liste des armes/ennemis priorisée pour MVP
- [ ] Document d'équilibrage initial rédigé

**Marketing** :
- [ ] Nom du jeu choisi
- [ ] Tagline/pitch court rédigé
- [ ] Plateformes cibles confirmées

---

## 18. Historique des versions

| Version | Date | Auteur | Modifications |
|---------|------|--------|---------------|
| 1.0 | 2025-11-01 | Équipe | Création du GDD Survivor-like |

---

**Document vivant** : Ce GDD évoluera tout au long du développement. Mettre à jour régulièrement selon les décisions d'équipe, retours de playtests, et contraintes techniques rencontrées.

**Prochain document recommandé** : Créer un "Game Design Pillars" d'une page résumant les 3-5 piliers fondamentaux du jeu pour garder une vision claire durant tout le dev.
