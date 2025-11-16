# Game Design Document

**Titre** : Colonie terminus
**Projet** : Survivor-like skill-based avec système magie/tech
**Équipe** : Indie collaborative (temps libre)
**Cible** : PC (Steam) → Mobile premium
**Moteur** : Bevy (Rust)
**Statut** : Pré-production

---

## Vision en 3 phrases

Votre vaisseau s'écrase sur une **planète hostile inconnue**. Vous devez **survivre jusqu'à l'exfiltration** en affrontant des hordes de créatures extraterrestres, équipé d'un mélange de technologie et de magie. Chaque run dure le temps que vous choisissez, mais plus vous restez, plus les récompenses sont grandes.

---

## Pourquoi ce jeu sera FUN

### 1. Système magie/tech avec jauge = Ressource management
- **Chaque action consomme de l'énergie** → Gestion tactique de la jauge
- Jauge se recharge avec le temps → Équilibre risque/récompense
- Lancer une grosse attaque au mauvais moment = danger de mort
- Skill rewarding : **timing > spam**

### 2. Choix tactiques constants
- **2 armes + 4 passifs** max → Chaque level-up est crucial
- "Prendre cette nouvelle arme ou améliorer l'existante ?"
- Synergies à découvrir entre tech et magie

### 3. "Une dernière run" addictif
- **Tu choisis la durée** (5-30 min) avant le départ
- Mort avant exfiltration = perte des gains de la run
- Events aléatoires (boss, vaisseau crashé) → Opportunités risquées
- Meta-progression conservée à chaque mort

### 4. Montée en puissance gratifiante
- Début : Équipement de base, tu survies à peine
- Milieu : Events maîtrisés, nouvelles armes, tu domines
- Fin : Tu chaînes les attaques, explosions partout
- **Chaque event = opportunité de loot**

### 5. Univers spatial dépouillé et désespéré
- **Planète hostile** : Environnement minimaliste et dangereux
- **Situation de survie** : Crashé, seul, attendre l'exfiltration
- **Carte sphérique** : Tu tournes à l'infini sur la planète
- **Minimap sphérique** : Point de position + vaisseau crashé

---

## Core Gameplay Loop

```
Crash sur planète → Choisir durée exfiltration → Survivre → Tuer créatures → Ramasser loot
    ↓
Level up (30-60s) → Choisir 1 amélioration parmi 3
    ↓
Upgrade arme OU nouveau passif OU nouvelle arme (si <2)
    ↓
Events aléatoires → Boss pop OU Vaisseau crashé (loot)
    ↓
Devenir plus puissant → Difficulté augmente exponentiellement
    ↓
Mourir (perte gains run) OU Exfiltration réussie (récompenses)
    ↓
Meta-progression → Améliorer personnage permanent → RETRY
```

---

## Décisions de Design FINALES

| Question | Décision | Raison |
|----------|----------|--------|
| **Thème** | Exploration spatiale | Style dépouillé, situation désespérée, faisable |
| **Combat** | Magie/Tech avec jauge | Resource management skill-based |
| **Durée run** | Choix joueur (5-30 min) | Flexibilité, reward scaling |
| **Mort** | Perte gains run, garde méta | Roguelite, encourage retry stratégique |
| **Limite build** | 2 armes + 4 passifs | Choix tactiques forcés |
| **Carte** | Planète sphérique infinie | Unique, pas d'obstacles, simple |
| **Fog of war** | Non | Pas justifié, simplifie le développement |
| **Minimap** | Sphérique (position + vaisseau) | Référence spatiale claire |
| **Loot** | Ennemis drop en mourant | Système simple et lisible |
| **Events** | Boss aléatoires, vaisseaux crashés | Opportunités risquées, variété |
| **Exfiltration** | Temps choisi au départ | Stratégie risk/reward |

---

## MVP (Level 1) - 2-3 mois

### Objectif unique
**Valider que le core loop magie/tech + events est FUN en 10 minutes de jeu.**

### Features OBLIGATOIRES
- [ ] **Personnage** : Déplacement WASD fluide, sprite 32x32
- [ ] **Système de jauge d'énergie** :
  - Jauge max 100 points
  - Régénération 20 points/seconde
  - Chaque action coûte X points
  - Visuel : Barre bleue/cyan lumineuse
- [ ] **Tir manuel** : Clic souris = tire arme dans direction visée (consomme jauge)
- [ ] **2 armes de départ** :
  - **Blaster** : Arme de base, projectile rapide, coût 10 énergie, dégâts 15
  - **Lance-plasma** : Déblocable niveau 3, projectile explosif, coût 30 énergie, dégâts 50 zone
- [ ] **3 types de créatures** :
  - Grouilleur : Lent, faible HP, spawn en groupe
  - Voltigeur : Vitesse moyenne, déplacement erratique
  - Prédateur : Rapide, résistant, dangereux
- [ ] **Event Boss aléatoire** :
  - Gardien planétaire : 500 HP, pattern simple (charge + projectiles)
  - Drop garanti : Loot rare
  - Probabilité d'apparition : 20% tous les 3 niveaux
- [ ] **Event Vaisseau crashé** :
  - Zone de loot riche avec vague de créatures
  - Probabilité d'apparition : 15% tous les 2 niveaux
- [ ] **Système XP** : Cristaux drop, ramassage auto dans rayon, barre XP
- [ ] **Level-up** : Pause jeu, choix 3 cartes aléatoires
- [ ] **6 améliorations** :
  - Nouvelle arme (si <2)
  - +Dégâts arme actuelle (+20%)
  - +Vitesse recharge énergie (+15%)
  - +Vitesse déplacement (passif, +10%)
  - +HP max (passif, +20)
  - +Zone collecte cristaux (passif, +20px)
- [ ] **Carte sphérique** : Planète qui se répète à l'infini, pas d'obstacles
- [ ] **Minimap sphérique** : Point position joueur + vaisseau crashé
- [ ] **Timer exfiltration** : Choix durée au départ (5/10/15/20/30 min)
- [ ] **Spawn continu** : Créatures apparaissent aux bords, densité croît avec temps
- [ ] **Difficulté progressive** : +15% HP créatures toutes les 30s
- [ ] **UI minimale** :
  - Barre vie (haut gauche, rouge)
  - Barre énergie (au-dessus vie, cyan lumineuse)
  - Barre XP (bas, violette)
  - Timer exfiltration (haut centre, compte à rebours)
  - Minimap (bas droite, sphérique)

### Features INTERDITES (scope creep)
- ❌ Pas de multiples biomes (planète unique)
- ❌ Pas de menu méta-progression dépensable (Level 2)
- ❌ Pas d'éléments complexes avec synergies (Level 3)
- ❌ Pas d'évolutions d'armes complexes (Level 3)
- ❌ Pas de multiples personnages (1 seul pour MVP)
- ❌ Pas d'animations complexes (sprites statiques OK)
- ❌ Pas d'obstacles sur la planète (carte plate)
- ❌ Pas de fog of war (pas nécessaire)

### Test de validation FUN
À la fin du MVP, testez avec 3 personnes externes :

**Question 1** : "Après 10 min, as-tu envie de rejouer ?"
→ Si <2/3 disent oui → PROBLÈME CORE LOOP

**Question 2** : "La gestion de la jauge d'énergie est-elle intéressante ou frustrante ?"
→ Si frustrante → REBALANCER vitesse recharge ou coût armes

**Question 3** : "Les events aléatoires arrivent au bon moment ?"
→ Si timing off → AJUSTER probabilités ou fréquence

**Question 4** : "La carte sphérique est-elle agréable ou déroutante ?"
→ Si déroutante → AMÉLIORER minimap ou repères visuels

**Question 5** : "Sens-tu que tu deviens plus puissant progressivement ?"
→ Si non → PROBLÈME SCALING dégâts/améliorations

---

## Système de Combat (détails MVP)

### Contrôles
- **WASD** : Déplacement 8 directions
- **Souris** : Déplace curseur orbital (croix) sur cercle autour du personnage (rayon 150px)
- **Clic gauche** : Tirer arme slot 1 vers le curseur (si énergie suffisante)
- **Clic droit** : Tirer arme slot 2 vers le curseur (si équipée ET énergie suffisante)
- **Espace** : Dash/esquive (cooldown 5s) - *Optionnel MVP*

**Mécanique curseur orbital**
- Le curseur (croix) reste toujours sur un cercle de 150px autour du personnage
- Bouger la souris déplace le curseur sur ce cercle
- Les armes tirent vers la position du curseur
- **Avantages** : Focus constant sur personnage, mobile-friendly, unique

### Système d'énergie
- **Jauge max** : 100 points
- **Régénération** : 20 points/seconde (5 secondes pour full recharge)
- **Visuel** : Barre cyan lumineuse qui pulse légèrement
- **Son** : "Bip" électronique quand jauge pleine

### Blaster (arme de départ)
- **Coût énergie** : 10 points
- **Dégâts** : 15
- **Cadence** : ~2 coups/s (limité par énergie)
- **Portée** : 350px
- **Projectile** : Laser bleu, vitesse 280px/s
- **Feel** : Spammable, fiable, précis

### Lance-plasma (arme déblocable niveau 3)
- **Coût énergie** : 30 points
- **Dégâts** : 50 (zone 80px rayon)
- **Cadence** : ~0.6 coups/s (limité par énergie)
- **Portée** : 300px
- **Projectile** : Boule orange/rouge, vitesse 200px/s, explose à l'impact
- **Feel** : Puissant, explosif, vide la jauge rapidement

### Créatures (stats MVP)

| Type | HP | Speed | Damage | Comportement | Drop XP |
|------|-----|-------|--------|--------------|---------|
| Grouilleur | 25 | 40px/s | 8 | Poursuite lente en groupe | 5 |
| Voltigeur | 45 | 90px/s | 12 | Déplacement erratique, rapide | 10 |
| Prédateur | 60 | 110px/s | 18 | Sprint agressif vers joueur | 15 |

### Event Boss : Gardien planétaire

| Stat | Valeur |
|------|--------|
| HP | 500 |
| Speed | 60px/s |
| Damage contact | 25 |
| Damage projectiles | 15 |

**Pattern d'attaque** :
1. Phase 1 (500-250 HP) : Charge lente + tire 1 projectile toutes les 2s
2. Phase 2 (250-0 HP) : Vitesse +50%, tire 3 projectiles en éventail toutes les 1.5s
3. Mort : Explosion, drop garanti loot rare

**Visuel** : Grande créature (64x64), aspect alien menaçant

**Déclenchement** : 20% de chance tous les 3 niveaux

### Event Vaisseau crashé
- **Déclenchement** : 15% de chance tous les 2 niveaux
- **Zone** : Cercle 200px rayon avec loot au centre
- **Ennemis** : Vague de 15-20 créatures qui spawn autour
- **Loot** : 2-3 items rares garantis

### Scaling difficulté

| Temps/Niveau | HP créatures | Spawn rate | Événement |
|--------------|--------------|------------|-----------|
| Niveau 1-3 | 100% | 1 créature/s | Introduction, apprendre contrôles |
| Niveau 3-5 | 140% | 2 créatures/s | Intensification, Lance-plasma disponible |
| Niveau 6-8 | 180% | 3 créatures/s | Premiers events possibles |
| Niveau 9-12 | 230% | 5 créatures/s | Hordes denses, gestion énergie critique |
| Niveau 13-15 | 320% | 8 créatures/s | Survie extrême |
| Niveau 15+ | +15%/30s | +1/min | Scaling continu jusqu'à exfiltration |

---

## Configuration Technique (Bevy)

### Architecture ECS

```rust
struct Player {
    hp: f32,
    max_hp: f32,
    speed: f32,
    energy: f32,
    max_energy: f32,
    energy_regen: f32,
}

struct Creature {
    hp: f32,
    damage: f32,
    creature_type: CreatureType,
}

struct Boss {
    hp: f32,
    max_hp: f32,
    phase: BossPhase,
    attack_timer: f32,
}

struct Weapon {
    damage: f32,
    energy_cost: f32,
    cast_cooldown: f32,
    last_cast: f32,
}

struct Projectile {
    damage: f32,
    speed: f32,
    lifetime: f32,
    is_aoe: bool,
    aoe_radius: f32,
}

struct XpCrystal { value: u32 }

struct SphericalWorld {
    radius: f32,
    circumference: f32,
}

struct OrbitalCursor {
    angle: f32,
    radius: f32,
    target_angle: f32,
}
```

### System ordre d'exécution
1. input_system
2. orbital_cursor_system
3. energy_regen_system
4. movement_system
5. spherical_world_wrapping_system
6. weapon_system
7. projectile_system
8. collision_system
9. boss_ai_system
10. spawn_system
11. event_system
12. level_up_system
13. ui_system

### config.ron

```ron
(
    player: (
        hp: 100.0,
        max_energy: 100.0,
        energy_regen: 20.0,
        speed: 140.0,
        collect_radius: 70.0,
    ),
    weapons: {
        "blaster": (
            damage: 15.0,
            energy_cost: 10.0,
            cast_cooldown: 0.3,
            projectile_speed: 280.0,
            range: 350.0,
            is_aoe: false,
        ),
        "lance_plasma": (
            damage: 50.0,
            energy_cost: 30.0,
            cast_cooldown: 0.5,
            projectile_speed: 200.0,
            range: 300.0,
            is_aoe: true,
            aoe_radius: 80.0,
        ),
    },
    creatures: {
        "grouilleur": (hp: 25.0, speed: 40.0, damage: 8.0, xp: 5),
        "voltigeur": (hp: 45.0, speed: 90.0, damage: 12.0, xp: 10),
        "predateur": (hp: 60.0, speed: 110.0, damage: 18.0, xp: 15),
    },
    boss: (
        hp: 500.0,
        speed: 60.0,
        damage_contact: 25.0,
        damage_projectile: 15.0,
        phase2_hp_threshold: 250.0,
        phase2_speed_mult: 1.5,
        spawn_chance: 0.2,
        spawn_every_n_levels: 3,
    ),
    crashed_ship_event: (
        spawn_chance: 0.15,
        spawn_every_n_levels: 2,
        enemy_count: 18,
        loot_count: 3,
    ),
    spherical_world: (
        radius: 2000.0,
    ),
    difficulty_curve: [
        (level: 1, hp_mult: 1.0, spawn_rate: 1.0),
        (level: 3, hp_mult: 1.4, spawn_rate: 2.0),
        (level: 6, hp_mult: 1.8, spawn_rate: 3.0),
        (level: 9, hp_mult: 2.3, spawn_rate: 5.0),
        (level: 13, hp_mult: 3.2, spawn_rate: 8.0),
    ],
)
```

---

## Différenciateurs vs Vampire Survivors

| Aspect | Vampire Survivors | Colonie terminus |
|--------|-------------------|------------------|
| **Combat** | Auto-attaque | Tir manuel avec curseur orbital |
| **Visée** | Auto-aim | Curseur sur cercle (unique) |
| **Ressource** | Aucune | Gestion énergie |
| **Durée** | 30 min fixe | Choix joueur (5-30 min) |
| **Build** | Illimité | 2 armes + 4 passifs |
| **Events** | Aucun | Boss + vaisseaux crashés aléatoires |
| **Carte** | Rectangle fixe | Planète sphérique infinie |
| **Narration** | Minimale | Situation survie spatiale |
| **Ambiance** | Coloré cartoon | Spatial dépouillé, désespéré |
| **Feel** | Chill zen | Tendu tactique |

---

## Checklist Pré-Production

### Avant de coder (semaine 1)
- [x] Moteur choisi : Bevy
- [x] Décisions design finalisées
- [x] Univers défini : Colonie terminus (spatial)
- [ ] Repo Git avec structure Bevy
- [ ] Sprite personnage 32x32 (placeholder OK)
- [ ] 3 sprites créatures 32x32
- [ ] Sprite boss 64x64
- [ ] Sprites projectiles : Blaster (laser bleu), Plasma (orbe rouge)
- [ ] config.ron initial créé
- [ ] Rôles équipe définis

### Assets MVP (minimum viable)
- **Personnage** : 1 sprite statique 32x32
- **Créatures** : 1 sprite par type (formes aliens simples)
- **Boss** : 1 sprite 64x64 (grande créature menaçante)
- **Armes** : Projectiles = traits/cercles colorés
- **UI** : Rectangles colorés (vie rouge, énergie cyan, XP violette)
- **Map** : Sol planète (texture simple répétitive)
- **Minimap** : Cercle avec points (position + vaisseau)
- **VFX** : Particules simples (mort créature, explosion)

**Musique/SFX** :
- 1 track ambient spatial (loop)
- 1 track event boss
- SFX basiques : tir, impact, mort, level-up

### Métriques succès MVP
- 60 FPS avec 100+ créatures
- Input lag <50ms
- Gestion énergie fluide
- Carte sphérique wrapping fonctionnel
- Events amusants et valorisants
- 3/3 testeurs veulent rejouer

---

## Roadmap Post-MVP

### Level 2 : Méta-Progression (1-2 mois)
- Menu méta-progression (dépenser gains)
- 5 armes au total
- Pool de 12 passifs
- Animations personnage
- Plus d'events variés
- Particules et feedbacks visuels

### Level 3 : Profondeur (2-3 mois)
- Système synergies magie/tech
- Évolutions d'armes
- 2e personnage déblocable
- 10 armes, 25 passifs
- Biome alternatif
- Achievements

### Level 4 : Release Steam (1-2 mois)
- Polish final
- Musique et sound design complet
- 3-4 biomes planétaires
- Mode Infini
- Tutoriel
- Leaderboards

---

## Notes de Production

### Mantras
1. "Est-ce que ça rend le jeu plus FUN ?" (si non → cut)
2. "MVP d'abord" (features = Level 2+)
3. "Tester souvent" (toutes les 2 semaines)
4. "Config file > hardcode"

### Red flags
🚩 "On pourrait ajouter..." sans finir l'existant
🚩 Passer >2 jours sur assets MVP
🚩 Coder sans spec claire
🚩 Ne pas tester pendant 2+ semaines
🚩 Débattre >30min sans décider

### Facteurs critiques
- **Curseur orbital doit être fluide** → Mécanique unique
- **Gestion énergie doit être tactique** → Core différenciateur
- **Carte sphérique doit être intuitive** → Feature unique
- **Events doivent être excitants** → Variété gameplay
