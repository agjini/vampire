- vampire survivor +++
- Gun Smoke ++
- rogue like
- balatro
- doodle jump
- spelunky
- gradius

# Objectifs ?
    - Avoir un produit finit
    - Jeu partagé sur steam
    - marche sur mobile (mode portrait/paysage)
    - une ambiance
    - une histoire simple

# Gameplay
    - progression (montée en niveau)

# Histoire

notion d'items, d'armes, ambiance


[https://itch.io/game-assets](assets)
[https://itch.io/jam/bevy-jam-6/entries](Bevy Gamejam #6)
[https://poki.com/fr](Poki)

OBJECTIFS:
    - Level 1 : Forme minimale du jeu (personnage / ennemis /déplacement / mécaniques / Argent? / Mode infini avec difficulté exponentielle?)
        -- Thématique : Real Fantasy? Eviter trop de mécaniques qui complexifient le jeu.
        -- Analyser différents Survivor-likes pour en extraire des mécaniques disinctives. (Void-S, Vampire-S, 10 minutes till dawn, Pixel-S, MegaBonk, etc...)
        --
    - Level 2 : Gestion de la carte générée procéduralement et gestion des ennemis
    - Level 3 : Gestion des objets interactibles et la progression du joueur.

RESSOURCES:
    - Afin d'effectuer des tests, nécessité de modifier rapidement les réglages du jeu. -> Fichier config?
    - Charas-project (Sprites) : http://charas-project.net/resources.php?wa=0&lang=fr&area=5&offset=1&howmany=10&fsearch=
    - Musiques avec licences (Dark Fantasy et Chiptunes)
    - 

GAMEPLAY:
    - Le personnage navigue sur une map infinie.
    - Le personnage possède une attaque de base pour se défendre.
    - Le personnage peut monter de niveaux pour obtenir de nouvelles capacités. (Actives ou passives)
    - Le personnage gagne de l'argent durant une partie.
    - (Hors partie) Le joueur peut dépenser l'argent pour débloquer des bonus permanents. (passifs)
    - (Hors partie) En remplissant certaines conditions, de nouvelles armes et accessoires sont débloqués et deviennent accessibles dans les parties futures.

BENCHMARK: Comment les concurrents de Vampire Survivors se démarquent dans leur mécanique ou leur DA (Direction Artistique)

- Nomad Survivor : https://store.steampowered.com/app/1929870/Nomad_Survival/
    -- Chaque personnage démarre une partie avec une arme qui lui est spécifique et à laquelle les autres personnages ne peuvent pas avoir accès
  
    -- Tous les 30 Niveaux, le joueur choisit une spécialisation parmi deux qui va modifier le fonctionnement de base de son arme initiale : <img width="1211" height="682" alt="image" src="https://github.com/user-attachments/assets/2cfd4afa-e621-46aa-a75b-dbb42a5cd274" />
    
    -- Le joueur a la possibilité de moduler la difficulté et d'activer des challenges spécifiques à niveau avant de lancer une partie. Cela lui permet d'obtenir davantage de récompenses au prix d'une difficulté bien plus accrues : 
        --- <img width="1788" height="997" alt="image" src="https://github.com/user-attachments/assets/f0da8c1f-365d-4c4b-ad51-781d98a02ac1" />
        --- <img width="1573" height="880" alt="image" src="https://github.com/user-attachments/assets/738f17fe-786b-4d40-9395-5e1d43a99332" />
        --- <img width="1580" height="890" alt="image" src="https://github.com/user-attachments/assets/653efecc-4a07-4b33-9f9b-df142f9cbea2" />
        
    -- Le joueur a la possibilité de débloquer et d'améliorer des animaux de compagnies qui l'aideront à vaincre les adversaires ou qui iront récupérer les bonus tombés au sol pour lui : <img width="1792" height="1007" alt="image" src="https://github.com/user-attachments/assets/36107989-8f3a-4abe-b6de-92f554b32aaa" />

    
- 20 Minutes till dawn: https://store.steampowered.com/app/1966900/20_Minutes_Till_Dawn/
    -- Identité visuelle unique exclusivement à base de Rouge/Blanc/Gris/Noir.
  
    -- Le personnage n'attaque pas automatiquement; Le joueur doit cliquer avec sa souris pour tirer.
        --- Les personnages utilisant exclusivement des armes à feu, le joueur doit régulièrement recharger son arme lorsqu'elle est vide. Cependant, les munitions sont illimitées
        --- Le joueur peut basculer rapidement entre une visée automatique (sur l'ennemi le plus proche) ou manuelle.
        --- De nombreux effets se déclenchent au moment du rechargement ou en fonction des balles utilisées, donc la mécanique en elle-même a une réelle justification en terme de game-design
    
    -- Quand un boss est vaincu, il dépose un objet. Quand le joueur le récupère, il a alors le choix entre 3 effets comportant tous un effet positif et un effet négatif. Il tient alors au joueur de décider s'il en prend un pour se spécialiser, mais il peut tout aussi bien ne rien choisir.

    -- Le jeu utilise un système d'élément (Feu, Glace et foudre) qui provoque des effets (brûlure, gel, foudroiement)
        --- De nombreux passifs se basent sur ces éléments pour déclencher des effets. (Exemples: Quand un ennemi qui brûle meurt, alors il explose et brûle les autres ennemis à proximité)
  
    -- Le joueur a la possibilité de moduler la difficulté avant de lancer une partie. Cela lui permet d'obtenir davantage de récompenses au prix d'une difficulté bien plus accrues
    
    -- Le jeu se déroulant dans des endroits très sombres et généralement la nuit, une mécanique de "brouillard de guerre" empêche le joueur de détecter les ennemis se trouvant trop loin de lui.
        --- Certains passifs augmentent la visibilité et fournissent des effets spécifiques lorsqu'un ennemi devient visible.

