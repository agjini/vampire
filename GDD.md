# Game Design Document

**Titre** : Eclipse of Magic
**Projet** : Survivor-like skill-based avec système de magie
**Équipe** : Indie collaborative (temps libre)
**Cible** : PC (Steam) → Mobile premium
**Moteur** : Bevy (Rust)
**Statut** : Pré-production

---

## Vision en 3 phrases

Dans un monde où **la magie a disparu**, vous incarnez une Sorceleuse piégée dans une cave sombre et hostile. Armée d'un simple bâton magique, vous affrontez des hordes d'entités pendant **15 minutes** en utilisant des sorts à jauge. Chaque boss vaincu libère un fragment de magie et révèle l'histoire oubliée de ce monde.

---

## Pourquoi ce jeu sera FUN

### 1. Système de magie à jauge = Ressource management intense
- **Chaque sort consomme de la magie** → Gestion tactique de la jauge
- Jauge se recharge avec le temps → Équilibre risque/récompense
- Lancer un gros sort au mauvais moment = danger de mort
- Skill rewarding : **timing > spam**

### 2. Choix tactiques constants
- **2 sorts + 4 passifs** max → Chaque level-up est crucial
- "Prendre ce nouveau sort ou améliorer l'existant ?"
- Synergies à découvrir entre sorts et passifs magiques

### 3. "Une dernière run" addictif
- Runs courtes (**15 min**) → Parfait pour "juste une partie"
- Mort permanente → "Si j'avais mieux géré ma jauge..."
- Boss tous les **5 niveaux** avec lore → Progression narrative

### 4. Montée en puissance gratifiante
- Niveau 1 : Bâton avec lumière faible, tu survis à peine dans le noir
- Niveau 10 : Premier boss vaincu, nouveau sort débloqué, tu domines
- Niveau 15 : Tu chaînes les sorts, explosions magiques partout
- **Chaque boss = nouveau pouvoir** → Récompense immédiate

### 5. Univers dark fantasy immersif
- **Ambiance sombre** : Cave hostile, entités menaçantes
- **Narration environnementale** : Chaque boss révèle un fragment d'histoire
- **Progression cosmétique** : La Sorceleuse s'illumine au fur et à mesure qu'elle récupère la magie
- Niveau 100 → Nouveau personnage avec sa propre histoire

---

## Core Gameplay Loop

```
Spawn cave sombre → Lancer sorts (viser souris) → Gérer jauge magie → Tuer entités → Ramasser XP
    ↓
Level up (toutes les 30-60s) → Choisir 1 amélioration parmi 3
    ↓
Upgrade sort OU nouveau passif OU nouveau sort (si <2)
    ↓
Tous les 5 niveaux → BOSS avec pattern unique
    ↓
Victoire boss → Fragment de magie + Lore + Sort aléatoire garanti
    ↓
Devenir plus puissante → Difficulté augmente exponentiellement
    ↓
Mourir OU Niveau 15 (victoire) OU Niveau 100 (unlock nouveau perso)
    ↓
Dépenser fragments de magie → Améliorer stats permanentes → RETRY
```

---

## Décisions de Design FINALES

| Question | Décision | Raison |
|----------|----------|--------|
| **Système de combat** | Magie à jauge | Différenciation vs VS, resource management skill-based |
| **Durée run** | 15 min (niveau 15) | Sessions courtes, intense, temps libre friendly |
| **Mort** | Permanente stricte | Roguelite pur, encourage retry |
| **Limite build** | 2 sorts + 4 passifs | Choix tactiques forcés, gestion jauge magie |
| **Boss** | Tous les 5 niveaux | Rythme, récompenses (sorts), narration |
| **Personnage MVP** | Sorceleuse (fille) | Thème magie, progression cosmétique (s'illumine) |
| **Monnaie** | Fragments de magie | Cohérent avec univers, méta-progression |
| **Monétisation mobile** | Premium payant | Cohérent Steam, pas de P2W |
| **Multijoueur** | Solo MVP, coop future | Focus qualité single-player d'abord |

---

## MVP (Level 1) - 2-3 mois

### Objectif unique
**Valider que le core loop magie + boss est FUN en 10 minutes de jeu.**

### Features OBLIGATOIRES
- [ ] **Personnage : Sorceleuse** : Déplacement WASD fluide, sprite 32x32, fille
- [ ] **Système de jauge de magie** :
  - Jauge max 100 points
  - Régénération 20 points/seconde
  - Chaque sort coûte X points
  - Visuel : Barre bleue lumineuse
- [ ] **Lancer de sort manuel** : Clic souris = lance sort dans direction visée (consomme jauge)
- [ ] **2 sorts de départ** :
  - **Lueur (Lumière)** : Sort de base, projectile lumineux, coût 10 mana, cadence rapide, dégâts 15
  - **Boule de feu** : Sort déblocable niveau 3, projectile explosif, coût 30 mana, cadence lente, dégâts 50 zone
- [ ] **3 types d'entités** :
  - Ombre rampante : Lente, faible HP, spawn en groupe, sensible à la lumière
  - Spectre : Vitesse moyenne, traverse les obstacles, vulnérable à la magie
  - Démon mineur : Rapide, résistant, dangereux au corps-à-corps
- [ ] **Premier BOSS (niveau 5)** :
  - Gardien des Ténèbres : 500 HP, pattern d'attaque simple (charge + projectiles)
  - Drop garanti : Fragment de magie + Nouveau sort aléatoire
  - Cinématique courte avant/après (texte + illustration)
- [ ] **Système XP** : Cristaux magiques drop, ramassage auto dans rayon, barre XP
- [ ] **Level-up** : Pause jeu, choix 3 cartes aléatoires
- [ ] **6 améliorations** :
  - Nouveau sort (si <2)
  - +Dégâts sort actuel (+20%)
  - +Vitesse recharge mana (+15%)
  - +Vitesse déplacement (passif, +10%)
  - +HP max (passif, +20)
  - +Zone collecte cristaux (passif, +20px)
- [ ] **Système fragments de magie** : Drops boss + petits drops ennemis, compteur, conservés à la mort
- [ ] **Spawn continu** : Entités apparaissent aux bords, densité croît avec temps
- [ ] **Difficulté progressive** : +15% HP entités toutes les 30s
- [ ] **UI minimale** :
  - Barre vie (haut gauche, rouge)
  - Barre magie (au-dessus vie, bleue lumineuse)
  - Barre XP (bas, violette)
  - Timer + Niveau (haut centre)
  - Fragments de magie (haut droit, icône cristal)
- [ ] **Ambiance cave sombre** :
  - Fond noir/gris très sombre
  - Lumière autour de la Sorceleuse (cercle lumineux)
  - Entités apparaissent dans l'obscurité (fog of war)

### Features INTERDITES (scope creep)
- ❌ Pas de multiples boss (1 seul boss niveau 5 pour MVP)
- ❌ Pas de biomes multiples (cave sombre uniquement)
- ❌ Pas de méta-progression dépensable (Level 2 : menu upgrades permanents)
- ❌ Pas d'éléments complexes Feu/Glace/Foudre avec synergies (Level 3)
- ❌ Pas de spécialisations de sorts / évolutions (Level 3)
- ❌ Pas de multiples personnages (Sorceleuse uniquement pour MVP)
- ❌ Pas d'animations complexes (sprites statiques OK)
- ❌ Pas de cinématiques élaborées (texte simple suffit)

### Test de validation FUN
À la fin du MVP, testez avec 3 personnes externes :

**Question 1** : "Après 10 min (jusqu'au boss), as-tu envie de rejouer ?"
→ Si <2/3 disent oui → PROBLÈME CORE LOOP

**Question 2** : "La gestion de la jauge de magie est-elle intéressante ou frustrante ?"
→ Si frustrante → REBALANCER vitesse recharge ou coût sorts

**Question 3** : "Le boss niveau 5 semble arriver au bon moment ?"
→ Si trop tôt/tard → AJUSTER fréquence level-up ou timing boss

**Question 4** : "L'ambiance sombre + lumière te plaît ou c'est juste chiant ?"
→ Si chiant → AUGMENTER rayon lumineux ou ajouter plus de clarté

**Question 5** : "Sens-tu que tu deviens plus puissante progressivement ?"
→ Si non → PROBLÈME SCALING dégâts/améliorations

---

## Système de Combat (détails MVP)

### Contrôles
- **WASD** : Déplacement 8 directions
- **Souris** : Viser direction du sort
- **Clic gauche** : Lancer sort slot 1 (si mana suffisante)
- **Clic droit** : Lancer sort slot 2 (si équipé ET mana suffisante)
- **Espace** : Dash/esquive (cooldown 5s) - *Optionnel MVP*

### Système de mana
- **Jauge max** : 100 points
- **Régénération** : 20 points/seconde (5 secondes pour full recharge)
- **Visuel** : Barre bleue lumineuse qui pulse légèrement
- **Son** : "Ding" cristallin quand jauge pleine

### Lueur (sort de départ)
- **Coût mana** : 10 points
- **Dégâts** : 15
- **Cadence** : ~2 coups/s (limité par mana)
- **Portée** : 350px
- **Projectile** : Orbe lumineux blanc/bleu, vitesse 280px/s
- **Feel** : Spammable, fiable, éclaire la zone autour

### Boule de feu (sort déblocable niveau 3)
- **Coût mana** : 30 points
- **Dégâts** : 50 (zone 80px rayon)
- **Cadence** : ~0.6 coups/s (limité par mana)
- **Portée** : 300px
- **Projectile** : Boule orange/rouge, vitesse 200px/s, explose à l'impact
- **Feel** : Puissant, explosif, vide la jauge rapidement, high-risk/reward

### Entités (stats MVP)

| Type | HP | Speed | Damage | Comportement | Drop Cristal XP |
|------|-----|-------|--------|--------------|-----------------|
| Ombre rampante | 25 | 40px/s | 8 | Poursuite lente, fuit la lumière | 5 |
| Spectre | 45 | 90px/s | 12 | Traverse obstacles, poursuite directe | 10 |
| Démon mineur | 60 | 110px/s | 18 | Sprint agressif vers joueur | 15 |

### Boss MVP : Gardien des Ténèbres (niveau 5)

| Stat | Valeur |
|------|--------|
| HP | 500 |
| Speed | 60px/s |
| Damage contact | 25 |
| Damage projectiles | 15 |

**Pattern d'attaque** :
1. Phase 1 (500-250 HP) : Charge lente vers joueur + tire 1 projectile toutes les 2s
2. Phase 2 (250-0 HP) : Vitesse +50%, tire 3 projectiles en éventail toutes les 1.5s
3. Mort : Explosion de lumière, drop garanti Fragment de magie + Sort aléatoire

**Visuel** : Grande silhouette sombre (64x64), yeux rouges lumineux

**Musique** : Track épique s'enclenche à l'apparition

### Scaling difficulté (15 min pour niveau 15)

| Temps/Niveau | HP entités | Spawn rate | Événement |
|--------------|------------|------------|-----------|
| Niveau 1-3 | 100% | 1 entité/s | Introduction, apprendre contrôles |
| Niveau 3-5 | 140% | 2 entités/s | Intensification, Boule de feu disponible |
| Niveau 5 | - | PAUSE | **BOSS : Gardien des Ténèbres** |
| Niveau 6-8 | 180% | 3 entités/s | Post-boss, nouveau sort équipé |
| Niveau 9-12 | 230% | 5 entités/s | Hordes denses, gestion mana critique |
| Niveau 13-15 | 320% | 8 entités/s | Survie extrême |
| Niveau 15 | - | - | **VICTOIRE** (~15 min) |

---

## Configuration Technique (Bevy)

### Architecture ECS recommandée

```rust
// Components
struct Sorceress {
    hp: f32,
    max_hp: f32,
    speed: f32,
    mana: f32,
    max_mana: f32,
    mana_regen: f32,  // points/seconde
}

struct Entity {
    hp: f32,
    damage: f32,
    entity_type: EntityType  // Ombre, Spectre, Demon
}

struct Boss {
    hp: f32,
    max_hp: f32,
    phase: BossPhase,  // Phase1, Phase2
    attack_timer: f32,
}

struct Spell {
    damage: f32,
    mana_cost: f32,
    cast_cooldown: f32,  // temps entre 2 casts
    last_cast: f32,
}

struct Projectile {
    damage: f32,
    speed: f32,
    lifetime: f32,
    is_aoe: bool,  // pour Boule de feu
    aoe_radius: f32,
}

struct XpCrystal { value: u32 }
struct MagicFragment { value: u32 }

struct LightSource {
    radius: f32,  // pour fog of war
    intensity: f32,
}

// Systems (ordre d'exécution)
1. input_system (lecture inputs clavier/souris)
2. mana_regen_system (régénération mana joueur)
3. movement_system (déplacement Sorceleuse + entités)
4. spell_system (gestion cast sorts avec coût mana)
5. projectile_system (déplacement projectiles + détection AoE)
6. collision_system (dégâts, collecte cristaux)
7. boss_ai_system (IA du boss si spawné)
8. spawn_system (génération entités + boss)
9. level_up_system (gestion XP et upgrades)
10. lighting_system (fog of war + zones éclairées)
11. ui_system (affichage HUD : vie, mana, XP)
```

### Fichier de configuration (config.ron)

```ron
(
    sorceress: (
        hp: 100.0,
        max_mana: 100.0,
        mana_regen: 20.0,  // points/seconde
        speed: 140.0,
        collect_radius: 70.0,
        light_radius: 150.0,  // rayon lumière autour d'elle
    ),
    spells: {
        "lueur": (
            damage: 15.0,
            mana_cost: 10.0,
            cast_cooldown: 0.3,  // secondes
            projectile_speed: 280.0,
            range: 350.0,
            is_aoe: false,
        ),
        "boule_de_feu": (
            damage: 50.0,
            mana_cost: 30.0,
            cast_cooldown: 0.5,
            projectile_speed: 200.0,
            range: 300.0,
            is_aoe: true,
            aoe_radius: 80.0,
        ),
    },
    entities: {
        "ombre": (hp: 25.0, speed: 40.0, damage: 8.0, xp: 5),
        "spectre": (hp: 45.0, speed: 90.0, damage: 12.0, xp: 10, can_pass_walls: true),
        "demon": (hp: 60.0, speed: 110.0, damage: 18.0, xp: 15),
    },
    boss: (
        "gardien_tenebres": (
            hp: 500.0,
            speed: 60.0,
            damage_contact: 25.0,
            damage_projectile: 15.0,
            phase2_hp_threshold: 250.0,  // passe en phase 2 à 50% HP
            phase2_speed_mult: 1.5,
        ),
    ),
    difficulty_curve: [
        (level: 1, hp_mult: 1.0, spawn_rate: 1.0),
        (level: 3, hp_mult: 1.4, spawn_rate: 2.0),
        (level: 5, hp_mult: 1.0, spawn_rate: 0.0),  // BOSS
        (level: 6, hp_mult: 1.8, spawn_rate: 3.0),
        (level: 9, hp_mult: 2.3, spawn_rate: 5.0),
        (level: 13, hp_mult: 3.2, spawn_rate: 8.0),
    ],
)
```

---

## Différenciateurs vs Vampire Survivors

| Aspect | Vampire Survivors | Eclipse of Magic |
|--------|-------------------|------------------|
| **Système de combat** | Attaques auto | Magie manuelle avec jauge |
| **Ressource** | Aucune | Gestion mana (resource management) |
| **Durée** | 30 min | 15 min (intense) |
| **Build** | Illimité (6+ armes) | 2 sorts + 4 passifs (choix tactiques) |
| **Boss** | Absents (sauf DLC) | Boss tous les 5 niveaux avec lore |
| **Narration** | Minimale | Histoire révélée par les boss |
| **Progression** | Cosmétique neutre | Sorceleuse s'illumine (progression visuelle) |
| **Ambiance** | Coloré, cartoon | Dark Fantasy, cave sombre, fog of war |
| **Feel** | Chill, zen | Tendu, tactique, gestion ressource |
| **Difficulté** | Casual accessible | Skill ceiling élevé (timing sorts) |

---

## Checklist Pré-Production

### Avant de coder (semaine 1)
- [x] Moteur choisi : Bevy
- [x] Décisions design finalisées
- [x] Univers défini : Eclipse of Magic
- [ ] Repo Git créé avec structure Bevy
- [ ] Sprite Sorceleuse 32x32 (fille, placeholder OK)
- [ ] 3 sprites entités 32x32 (Ombre, Spectre, Démon)
- [ ] Sprite boss 64x64 (Gardien des Ténèbres)
- [ ] Sprites projectiles : Lueur (orbe bleu), Boule de feu (orbe rouge)
- [ ] config.ron initial créé
- [ ] Rôles équipe définis (qui code quoi, qui fait assets, etc.)

### Assets MVP (minimum viable)
**Ne passez PAS 3 semaines sur les assets. Placeholder d'abord, polish après.**

- **Sorceleuse** : 1 sprite statique fille 32x32, vêtements simples (animation = Level 2)
- **Entités** : 1 sprite par type (silhouettes sombres suffisent)
- **Boss** : 1 sprite 64x64 (grande silhouette + yeux rouges)
- **Sorts** : Projectiles = cercles colorés (bleu pour Lueur, rouge/orange pour Feu)
- **UI** : Rectangles de couleur (vie rouge, mana bleue lumineuse, XP violette)
- **Map** : Fond noir/gris très sombre (cave)
- **Lumière** : Cercle blanc semi-transparent autour Sorceleuse (shader simple)
- **VFX** : Particules blanches/bleues simples (mort entité, explosion boss)

**Musique/SFX** :
- 1 track ambiance sombre cave (loop)
- 1 track boss épique
- SFX basiques : cast sort, impact, mort, level-up

### Métriques de succès MVP
- Framerate stable 60 FPS avec 100+ entités à l'écran
- Input lag <50ms (cast sort instantané au clic)
- Gestion mana fluide (régénération visible, pas de bug)
- Fog of war fonctionnel (zone lumière visible)
- Boss niveau 5 fun et challenging
- 3/3 testeurs externes veulent rejouer après le boss

---

## Roadmap Post-MVP (si MVP validé)

### Level 2 : Méta-Progression & Boss (1-2 mois)
- **Menu méta-progression** : Dépenser fragments de magie pour upgrades permanents
- **2e et 3e boss** (niveaux 10 et 15) avec patterns uniques
- **5 sorts au total** (3 nouveaux à débloquer)
- **Pool de 12 passifs** pour variété builds
- Obstacles destructibles sur la map (tonneaux, cristaux)
- Animations Sorceleuse (marche, cast sort)
- Particules et feedbacks visuels (screenshake, flash, traînées sorts)
- **Lore** : Textes courts révélés après chaque boss

### Level 3 : Profondeur & Contenu (2-3 mois)
- **Système d'éléments** : Feu/Glace/Foudre avec synergies
- **Évolutions de sorts** : Spécialisations à niveau élevé (ex: Lueur → Rayon laser)
- **2e personnage déblocable** (niveau 100 ou achievement)
- **10 sorts différents**, 25 passifs
- **Biome additionnel** : Crypte ou Forêt Maudite
- Menu complet, achievements (30+)
- Cosmétiques déblocables (couleurs sorts, skins Sorceleuse)

### Level 4 : Release Steam (1-2 mois)
- Polish final (animations fluides, VFX élaborés)
- **Musique et sound design complet** (5+ tracks, SFX variés)
- **5 biomes** avec ambiances uniques
- **6 boss** avec lore complète formant une histoire
- Mode Infini (après niveau 15, continue jusqu'à la mort)
- Tutoriel intégré (premier niveau guidé)
- Leaderboards (temps survie, niveau max atteint)
- **Préparation page Steam** : Trailer, screenshots, description

---

## Notes de Production

### Philosophie de développement
1. **MVP d'abord, polish après** - Un jeu moche mais fun > un jeu beau mais chiant
2. **Playtester SOUVENT** - Toutes les 2 semaines minimum
3. **Couper sans pitié** - Si une feature ne rend pas le jeu plus fun, OUT
4. **Data-driven** - Tout dans config.ron, ajustable sans recompile
5. **Fun > Réalisme** - Exagérer les feedbacks, c'est un jeu vidéo pas une simulation

### Red flags à surveiller
🚩 "On pourrait ajouter..." sans finir l'existant
🚩 Passer >2 jours sur un asset pour le MVP
🚩 Coder une feature sans spec claire
🚩 Ne pas tester le jeu pendant 2+ semaines
🚩 Débattre >30min d'un détail sans trancher et avancer

### Mantras de l'équipe
- "Est-ce que ça rend le jeu plus FUN ?" (si non → poubelle)
- "MVP d'abord" (feature = Level 2+)
- "Teste avant de coder plus"
- "Config file > hardcode"
