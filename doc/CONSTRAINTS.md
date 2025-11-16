# Contraintes du Projet - Colonie terminus

**Objectif** : Définir des contraintes strictes basées sur nos compétences pour **garantir qu'on termine le MVP**.

---

## Profil de l'équipe

- **Compétences Rust/Bevy** : Avancées ✅
- **Compétences graphiques** : Aucune ⚠️
- **Temps disponible** : 5-10h/semaine/personne ⚠️
- **Taille équipe** : 3-4 personnes
- **Budget total MVP** : ~180-480h sur 3 mois

---

## CONTRAINTES STRICTES (NON NÉGOCIABLES)

### 1. Assets graphiques : ZÉRO création custom

**Problème** : Personne ne sait faire des assets → risque de bloquer le projet

**Solutions autorisées UNIQUEMENT** :
- ✅ Formes géométriques simples (cercles, carrés, triangles)
- ✅ Assets gratuits de qualité (itch.io, OpenGameArt, Kenney.nl)
- ✅ Colored rectangles pour UI
- ✅ Particules = cercles colorés
- ❌ INTERDIT : Passer >2h à chercher des assets
- ❌ INTERDIT : Essayer de dessiner soi-même
- ❌ INTERDIT : Commander des assets custom

**Règle d'or** : Si tu ne trouves pas un asset en 30 min → utilise un carré coloré

### 2. Scope graphique : Minimalisme EXTRÊME

**MVP doit ressembler à un prototype fonctionnel, PAS à un jeu fini**

**Autorisé** :
- Player = carré blanc 32x32
- Creatures = cercles colorés (rouge/vert/bleu)
- Boss = grand cercle rouge 64x64
- Projectiles = petits cercles/traits colorés
- UI = rectangles colorés avec texte
- Map = texture simple répétée OU couleur unie
- Minimap = cercle avec 2 points

**Interdit** :
- Animations complexes
- Sprites détaillés
- Effets visuels élaborés
- Parallax, shaders complexes

**Validation** : Si un testeur dit "c'est moche" mais "c'est fun" → VICTOIRE

### 3. Temps : Règle du "2 semaines max par feature"

**Problème** : 5-10h/semaine = très peu de temps

**Règles temporelles** :
- Chaque feature MVP = max 2 semaines (20-40h équipe)
- Si feature dépasse 2 semaines → COUPE ou SIMPLIFIE
- Planning sprint 2 semaines :
  - Semaine 1 : Code + tests basiques
  - Semaine 2 : Polish minimal + playtest
- Playtest obligatoire toutes les 2 semaines

**Budget MVP (3 mois)** :
```
Semaine 1-2  : Player movement + energy system (CORE)
Semaine 3-4  : Weapons + projectiles (CORE)
Semaine 5-6  : Creatures + spawning
Semaine 7-8  : XP + level-up + upgrades
Semaine 9-10 : Spherical world wrapping
Semaine 11-12: Events (boss + crashed ship) - SIMPLIFIÉ
```

Si retard sur planning → COUPE features Level 2+

### 4. Code : KISS (Keep It Simple, Stupid)

**Problème** : Avancés en Rust → risque d'over-engineering

**Règles de code** :
- ✅ Code simple qui marche > Code élégant qui prend du temps
- ✅ Copier-coller acceptable si ça va plus vite
- ✅ Hardcode acceptable pour MVP (refacto = Level 2)
- ✅ config.ron SEULEMENT pour valeurs de balance
- ❌ INTERDIT : Créer des abstractions "pour plus tard"
- ❌ INTERDIT : Refactoriser du code qui marche pendant MVP
- ❌ INTERDIT : Ajouter des features "parce que c'est facile"

**Validation** : Si le code fait le job → NEXT feature

### 5. Features : Liste noire ABSOLUE (MVP)

**CES FEATURES SONT INTERDITES PENDANT LE MVP** :

**Graphics/Polish** :
- ❌ Animations (sprites statiques UNIQUEMENT)
- ❌ Shaders custom
- ❌ Particules complexes
- ❌ Screenshake, juice, polish
- ❌ Menus élaborés
- ❌ Transitions/cutscenes

**Gameplay** :
- ❌ Plus de 2 weapons pour MVP
- ❌ Plus de 3 creature types pour MVP
- ❌ Synergies weapon/passives (Level 3)
- ❌ Évolutions d'armes (Level 3)
- ❌ Obstacles sur planète
- ❌ Fog of war
- ❌ Dash/esquive (optionnel = NON pour MVP)

**Systèmes** :
- ❌ Méta-progression dépensable (Level 2)
- ❌ Multiple characters
- ❌ Save/load (1 run = 1 session pour MVP)
- ❌ Sound design complexe (SFX basiques SEULEMENT)
- ❌ Musique custom (tracks gratuits OU silence)

**Si quelqu'un propose une feature** → Question : "Est-ce CRITIQUE pour valider le fun ?"
→ Si non → Liste "Level 2+"

### 6. Décisions : Règle des 15 minutes

**Problème** : Débats sans fin tuent les projets

**Règle** :
- Débat max 15 min sur une décision
- Si pas de consensus après 15 min → VOTE à la majorité
- Si égalité → Solution LA PLUS SIMPLE gagne
- Pas de retour en arrière sauf si bloquant critique

**Exemples** :
- "Quelle couleur pour le player ?" → 5 min max, puis vote
- "Quel nom pour cette variable ?" → 0 min, prend le premier qui vient

### 7. Tests : Validation stricte du fun

**MVP doit être FUN, pas beau**

**Test obligatoire toutes les 2 semaines** :
- 1 personne de l'équipe qui n'a pas codé cette feature
- 10 min de gameplay
- Question : "Est-ce que tu t'es amusé ?"

**Test externe (fin MVP)** :
- 3 personnes externes (amis, famille, community)
- Questions :
  1. "Après 10 min, veux-tu rejouer ?" (>2/3 OUI obligatoire)
  2. "Energy management : intéressant ou frustrant ?"
  3. "Events : excitants ou ennuyeux ?"
  4. "Map sphérique : intuitive ou confusante ?"

**Si test échoue** :
- 1 semaine pour FIX
- Re-test
- Si 2e échec → PIVOT ou ARRÊT (ne pas s'acharner)

### 8. Assets : Liste blanche approuvée

**Pour éviter de perdre du temps, voici les sources approuvées** :

**Sprites** :
- Kenney.nl (geometric shapes, space shooter packs)
- itch.io/game-assets/free (filtrer 2D, top-down)
- OpenGameArt.org
- **Temps max** : 30 min de recherche par type d'asset

**Si rien trouvé → Utilise ça** :
```
Player      = Carré blanc 32x32
Crawler     = Cercle vert 24x24
Flyer       = Triangle bleu 24x24
Predator    = Losange rouge 32x32
Boss        = Cercle rouge 64x64 avec border noir
Projectiles = Traits/cercles colorés
```

**Musique/SFX** :
- Freesound.org (SFX)
- Incompetech.com (musique)
- OpenGameArt.org
- **OU** : Silence total acceptable pour MVP

### 9. Communication : Daily async updates

**Problème** : 3-4 personnes, temps limité → risque désynchronisation

**Règle** :
- Message court quotidien (Discord/Slack) :
  - "Hier : [ce que j'ai fait]"
  - "Aujourd'hui : [ce que je fais]"
  - "Bloqué sur : [problème ou rien]"
- Meeting sync 1h/semaine max
- Décisions importantes → doc/decisions.md

### 10. Abandon : Savoir quand arrêter

**Il vaut mieux arrêter qu'un projet zombie**

**Red flags critiques** :
- 3 semaines sans commit
- 2 playtests négatifs consécutifs
- Équipe passe plus de temps à débattre qu'à coder
- Feature MVP dépasse 4 semaines

**Si 2+ red flags** → Réunion d'équipe :
- On continue avec scope réduit ?
- On arrête proprement ?
- Jamais de "on verra plus tard" qui tue la motivation

---

## Checklist de validation MVP

**Avant de dire "MVP terminé", valider** :

### Technique
- [ ] 60 FPS avec 100+ creatures à l'écran
- [ ] Input lag <50ms
- [ ] Pas de crash pendant 15 min de jeu
- [ ] Spherical wrapping fonctionne sans bugs visuels

### Gameplay
- [ ] Energy management tactique (pas frustrant)
- [ ] 2 weapons implémentées avec différences notables
- [ ] 3 creature types avec comportements distincts
- [ ] XP + level-up + 6 upgrades fonctionnels
- [ ] Boss event spawn et est challenging
- [ ] Crashed ship event spawn avec loot

### Fun (CRITIQUE)
- [ ] 3/3 testeurs externes veulent rejouer après 10 min
- [ ] 3/3 testeurs trouvent energy management intéressant
- [ ] 2/3 testeurs trouvent events excitants

**Si 1 seul item "Fun" échoue → MVP pas validé**

---

## Planning réaliste 3 mois

**Avec 5-10h/semaine/personne (3-4 personnes) = 60-160h/mois**

### Mois 1 : Core systems (Sprint 1-4)
**Objectif** : Player bouge et tire avec energy

- S1-2 : Movement + energy system + orbital cursor
- S3-4 : 2 weapons + projectiles + collision
- **Livrable** : Peut tirer et toucher des targets statiques

### Mois 2 : Gameplay loop (Sprint 5-8)
**Objectif** : Loop de gameplay complet

- S5-6 : Creatures spawn + AI basique + XP drops
- S7-8 : Level-up + upgrade system + difficulty scaling
- **Livrable** : Peut jouer une run complète sans events

### Mois 3 : Events + polish minimal (Sprint 9-12)
**Objectif** : MVP testable

- S9-10 : Spherical world + minimap + exfiltration timer
- S11   : Boss event + crashed ship event (SIMPLIFIÉ)
- S12   : Bug fixes + UI polish minimal + playtests externes
- **Livrable** : MVP complet prêt pour validation

**Marge** : 0 semaine → Si retard → COUPE features secondaires

---

## Métriques de succès

**Objectif chiffré du MVP** :

1. **Temps de dev** : ≤3 mois (12 semaines)
2. **Budget temps** : ≤480h équipe total
3. **Code** : ≤5000 lignes Rust (si plus → over-engineering)
4. **Assets** : 100% gratuits ou géométriques
5. **Fun** : ≥2/3 testeurs veulent rejouer

**Si on respecte toutes ces contraintes → 80% chance de finir le MVP**

---

## Mantra à afficher partout

```
┌────────────────────────────────────────┐
│   FINI ET MOCHE > PARFAIT ET JAMAIS   │
│                                        │
│   Chaque feature ajoutée réduit        │
│   la probabilité de terminer           │
│                                        │
│   Si tu hésites → SIMPLIFIE            │
└────────────────────────────────────────┘
```

---

**Ce document est un contrat avec vous-même. Le respecter = finir le jeu. L'ignorer = projet abandonné dans 6 mois.**
