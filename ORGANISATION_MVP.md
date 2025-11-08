# Organisation MVP - Sorceleuse

**Objectif** : Finir le MVP en 2-3 mois. Simple, minimaliste, fun d'abord.

---

## 📋 RÉPARTITION DES RÔLES

**Inscrivez-vous** en ajoutant votre nom/pseudo dans la colonne "Qui ?"

| Rôle | Responsabilités MVP | Qui ? | Statut |
|------|---------------------|-------|--------|
| **Dev Bevy Core** | ECS, systèmes de base (mouvement, mana, input) | ??? | ⏳ À assigner |
| **Dev Gameplay** | Boss AI, spawn entités, level-up, collisions | ??? | ⏳ À assigner |
| **Dev UI** | Barres (vie, mana, XP), timer, menu level-up | ??? | ⏳ À assigner |
| **Assets 2D** | Sprites placeholder (voir liste ci-dessous) | ??? | ⏳ À assigner |
| **Sound Design** | Trouver musiques libres + SFX basiques | ??? | ⏳ À assigner |
| **Config/Balance** | Fichier config.ron, ajustements stats | ??? | ⏳ À assigner |
| **QA/Playtest** | Tester toutes les 2 semaines, feedback | TOUT LE MONDE | 🔄 Continu |

**Notes** :
- Une personne peut avoir **plusieurs rôles**
- **Priorité 1** : Dev Bevy Core + Assets 2D (pour démarrer)
- Si vous hésitez entre 2 rôles, choisissez celui qui **vous plaît le plus**

---

## 🎨 LISTE ASSETS MVP

**Règle d'or** : Placeholder simple d'abord. Polish = Level 2+.

### Sprites (Pixel Art)

| Asset | Taille | Description | Placeholder acceptable | Priorité |
|-------|--------|-------------|------------------------|----------|
| **Sorceleuse** | 32x32 | Fille, robe simple, vue de dessus | Rectangle violet + point blanc (tête) | ⭐⭐⭐ |
| **Ombre rampante** | 32x32 | Silhouette sombre, floue | Rectangle noir semi-transparent | ⭐⭐⭐ |
| **Spectre** | 32x32 | Fantôme gris, traversant obstacles | Rectangle gris transparent (50%) | ⭐⭐⭐ |
| **Démon mineur** | 32x32 | Créature rouge/orange, agressive | Rectangle rouge + 2 points jaunes (yeux) | ⭐⭐⭐ |
| **Boss : Gardien Ténèbres** | 64x64 | Grande ombre, yeux rouges lumineux | Rectangle noir 64x64 + 2 cercles rouges | ⭐⭐ |
| **Projectile Lueur** | 8x8 | Orbe bleu/blanc lumineux | Cercle bleu clair | ⭐⭐⭐ |
| **Projectile Boule de feu** | 12x12 | Boule orange/rouge | Cercle orange avec bordure rouge | ⭐⭐ |
| **Cristal XP** | 8x8 | Petit cristal violet qui flotte | Losange violet | ⭐⭐ |
| **Fragment de magie** | 16x16 | Cristal lumineux (monnaie) | Étoile bleue | ⭐ |

**Format** : PNG avec transparence
**Palette** : Sombre (cave) - Noir/Gris + accents lumineux (Bleu, Violet, Rouge, Orange)

### UI Elements

| Element | Specs | Placeholder | Priorité |
|---------|-------|-------------|----------|
| **Barre de vie** | Rectangle rouge, fond noir | Rect rouge basique | ⭐⭐⭐ |
| **Barre de mana** | Rectangle bleu lumineux, pulse | Rect bleu basique (pulse = Level 2) | ⭐⭐⭐ |
| **Barre XP** | Rectangle violet, bas d'écran | Rect violet basique | ⭐⭐⭐ |
| **Carte amélioration** | 200x300px, fond sombre | Rectangle gris + texte blanc | ⭐⭐ |
| **Icône sort** | 32x32 pour UI | Cercle coloré (bleu/rouge) | ⭐⭐ |

### Effets visuels (VFX)

| Effet | Description | Placeholder | Priorité |
|-------|-------------|-------------|----------|
| **Particules mort entité** | 5-10 particules blanches qui se dispersent | Cercles blancs simples | ⭐⭐ |
| **Explosion boss** | Burst de lumière blanche | Flash blanc fullscreen | ⭐ |
| **Lumière Sorceleuse** | Cercle lumineux 150px rayon | Cercle blanc semi-transparent (shader) | ⭐⭐⭐ |
| **Impact sort** | Petit flash au contact | Cercle blanc qui disparaît | ⭐⭐ |

### Audio

| Type | Contenu | Source | Priorité |
|------|---------|--------|----------|
| **Musique ambiance cave** | Loop sombre, 2-3 min, mystérieux | [OpenGameArt](https://opengameart.org/) ou [FreePD](https://freepd.com/) | ⭐⭐ |
| **Musique boss** | Track épique, percussions, 2 min | OpenGameArt | ⭐⭐ |
| **SFX : Cast sort** | Whoosh magique court (0.2s) | [Freesound](https://freesound.org/) | ⭐⭐ |
| **SFX : Impact** | Thud sourd (0.1s) | Freesound | ⭐⭐ |
| **SFX : Mort entité** | Dissipation (0.3s) | Freesound | ⭐ |
| **SFX : Level up** | Jingle ascendant (0.5s) | Freesound | ⭐ |
| **SFX : Boss apparition** | Grondement grave (1s) | Freesound | ⭐ |

**Licences acceptables** : CC0, CC-BY, OGA-BY (créditer dans le jeu)

---

## ✅ TÂCHES SEMAINE 1 (Setup)

**À faire AVANT de coder** :

- [ ] **Tout le monde** : S'inscrire dans le tableau des rôles ci-dessus
- [ ] **Dev Core** : Installer Rust + Bevy, vérifier que ça compile
- [ ] **Assets 2D** : Créer les 7 sprites prioritaires ⭐⭐⭐ (même placeholder rectangles)
- [ ] **Dev Core** : Setup repo Git avec structure Bevy de base
- [ ] **Config** : Créer fichier `config.ron` initial (voir GDD.md)
- [ ] **Tout le monde** : Lire GDD.md section MVP entièrement

**Deadline** : Fin de semaine 1

---

## 🚨 RÈGLES POUR FINIR LE JEU

Ces règles sont **NON-NÉGOCIABLES** pour éviter de s'égarer :

### 1. Scope sacré
- ✅ **Faire** : Ce qui est dans la liste MVP du GDD.md
- ❌ **Ne PAS faire** : Tout le reste (même si "ça serait cool")
- Si vous voulez ajouter quelque chose → Demander AVANT à toute l'équipe

### 2. Temps limite par tâche
- Asset simple : **Max 2h**
- Système de code : **Max 1 journée**
- Si ça dépasse → **Simplifier** ou demander de l'aide

### 3. Placeholder > Perfection
- Rectangles de couleur = **OK pour tester**
- On polish en Level 2+ **UNIQUEMENT**
- Mantra : "Moche mais jouable > Beau mais incomplet"

### 4. Playtests obligatoires
- **Toutes les 2 semaines** minimum
- Tester nous-mêmes d'abord
- Puis 2-3 personnes externes

### 5. Décisions rapides
- Débat > 30 min sur un détail → **VOTER et passer à autre chose**
- Pas de décision parfaite, juste **avancer**

### 6. Communication
- Bloquer > 2h sur un problème → **Demander de l'aide** (Discord/Slack/etc.)
- Feature terminée → **Commit Git + message clair**
- Changement de plan → **Prévenir l'équipe AVANT**

---

## 📅 PLANNING MVP (2-3 mois)

### Mois 1 : Core Systems
- ✅ Semaine 1-2 : Setup + Sorceleuse qui bouge (WASD) + Mana system
- ✅ Semaine 3-4 : Cast sorts (Lueur) + Spawn entités basiques + Collisions

### Mois 2 : Gameplay Loop
- ✅ Semaine 5-6 : Système XP + Level-up + Upgrades
- ✅ Semaine 7-8 : Boss niveau 5 + AI basique + Drops

### Mois 3 : Polish MVP
- ✅ Semaine 9-10 : UI complète + Fog of war + Balance
- ✅ Semaine 11-12 : Playtest externe + Corrections + MVP final

**Milestone de validation** : Fin mois 3 → Tester avec 3 personnes externes (voir GDD.md section Tests)

---

## 🛠️ OUTILS RECOMMANDÉS

### Développement
- **Bevy** : https://bevyengine.org/
- **VSCode** : Avec extension `rust-analyzer`
- **Git** : Pour versioning (obligatoire)

### Assets 2D
- **Aseprite** (payant ~20€, excellent) : https://www.aseprite.org/
- **LibreSprite** (gratuit, fork Aseprite) : https://libresprite.github.io/
- **Piskel** (gratuit, web) : https://www.piskelapp.com/

### Audio
- **Audacity** (gratuit) : Pour éditer/mixer sons
- **Freesound** : https://freesound.org/ (SFX gratuits)
- **OpenGameArt** : https://opengameart.org/ (Musiques gratuites)

### Communication équipe
- **Discord** / **Slack** : Chat quotidien
- **GitHub Issues** : Tracker bugs et tâches
- **Google Docs** : Partage documents design

---

## 📞 QUESTIONS ?

Si quelque chose n'est pas clair dans ce document :

1. Relire **GDD.md** section MVP
2. Poser la question dans le chat équipe
3. Mettre à jour ce document avec la réponse

**Ce document est vivant** : Si vous trouvez une info manquante, ajoutez-la !

---

## 🎯 PROCHAINE ÉTAPE

**Maintenant** : Chaque membre s'inscrit dans le tableau des rôles ci-dessus.

**Ensuite** : La personne "Dev Bevy Core" lance le setup du projet Git/Bevy.

**Puis** : La personne "Assets 2D" commence les sprites ⭐⭐⭐.

**GO GO GO !** 🚀
