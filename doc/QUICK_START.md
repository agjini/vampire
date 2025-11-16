# Quick Start - Colonie terminus

**Guide ultra-rapide pour démarrer le MVP sans se perdre**

---

## 🎯 L'essentiel en 30 secondes

**Jeu** : Survivor-like spatial, crash sur planète, survie jusqu'à exfiltration

**Contrainte #1** : ZÉRO assets custom → formes géométriques ou gratuits uniquement
**Contrainte #2** : 3 mois pour MVP, 2 semaines max par feature
**Contrainte #3** : Minimalisme EXTRÊME (player = carré, creatures = cercles)
**Contrainte #4** : Code simple > code élégant

**Mantra** : **FINI ET MOCHE > PARFAIT ET JAMAIS**

---

## 📚 Documents à lire (dans l'ordre)

### Pour démarrer AUJOURD'HUI
1. **doc/CONSTRAINTS.md** (15 min) - LIS ÇA EN PREMIER
   - Contraintes non-négociables basées sur vos skills
   - Liste noire des features interdites
   - Planning 3 mois réaliste

2. **doc/GDD.md - Section MVP** (10 min)
   - Features obligatoires pour MVP
   - Stats et valeurs de référence
   - Architecture ECS Bevy

### Pour référence
3. **doc/WEEKLY_TRACKER.md** - À remplir chaque vendredi
4. **doc/GDD.md** complet - Quand besoin de détails
5. **doc/decisions.md** - Historique des choix

---

## 🚀 Plan d'action immédiat (Semaine 0-1)

### Jour 1 : Setup
- [ ] Lire CONSTRAINTS.md et GDD.md MVP
- [ ] Setup repo Git et Bevy project
- [ ] Définir qui fait quoi dans l'équipe
- [ ] Premier update dans chat équipe

### Jour 2-3 : Premier code
- [ ] Player movement basique (WASD, carré blanc 32x32)
- [ ] Caméra qui suit player
- [ ] Map simple (couleur unie ou texture répétée)

### Jour 4-7 : Energy + Orbital cursor
- [ ] Energy gauge (barre cyan, 100 max, regen 20/s)
- [ ] Orbital cursor (croix blanche sur cercle 150px)
- [ ] Visuel debug du cercle orbital (optionnel)

**Livrable Semaine 1** : Player bouge avec carré blanc, energy gauge affichée, orbital cursor visible

---

## ⚡ Les 3 règles d'or

### Règle 1 : Quand tu hésites → SIMPLIFIE
```
Hésitation : "J'utilise quel sprite pour le player ?"
→ Carré blanc 32x32, NEXT!

Hésitation : "Comment architecturer ce système ?"
→ Version la plus simple qui marche, NEXT!

Hésitation : "On devrait peut-être ajouter X feature ?"
→ C'est dans le MVP ? NON → Level 2, NEXT!
```

### Règle 2 : Si bloqué >2h → Demande de l'aide
- Chat équipe
- Discord Bevy
- Stack Overflow
- **NE JAMAIS** rester bloqué seul >1 jour

### Règle 3 : Chaque vendredi → Remplis le tracker
- 10 min pour remplir WEEKLY_TRACKER.md
- Compte les red flags
- Si ≥2 → Réunion urgente weekend

---

## 📋 Checklist MVP (copie dans un fichier séparé)

**Mois 1** :
- [ ] Player movement + energy (S1-2)
- [ ] Blaster weapon + projectiles (S3-4)

**Mois 2** :
- [ ] 3 creature types + spawning (S5-6)
- [ ] XP + level-up + upgrades (S7-8)

**Mois 3** :
- [ ] Spherical world + minimap (S9-10)
- [ ] Events (boss + ship) (S11)
- [ ] Polish minimal + tests (S12)

**Validation finale** :
- [ ] 3/3 testeurs externes veulent rejouer
- [ ] 3/3 disent energy = intéressant
- [ ] 2/3 disent events = excitants

---

## 🎨 Assets approuvés (ne perds pas de temps ailleurs)

### Sprites
**Option 1 - Geometric (RECOMMANDÉ)** :
```
Player      = Carré blanc 32x32
Crawler     = Cercle vert 24x24
Flyer       = Triangle bleu 24x24
Predator    = Losange rouge 32x32
Boss        = Cercle rouge 64x64
Projectiles = Lignes/cercles colorés
```

**Option 2 - Free assets** (max 30 min de recherche) :
- Kenney.nl (space shooter pack, geometric shapes)
- itch.io/game-assets/free (top-down, space)
- OpenGameArt.org

**Si rien trouvé en 30 min** → Utilise Option 1

### UI
- Rectangles colorés : Rouge (HP), Cyan (Energy), Violet (XP)
- Font : Bevy default
- Minimap : Cercle blanc avec 2 points

### Audio
- Freesound.org (SFX basiques)
- Incompetech.com (musique)
- **OU** silence total = OK pour MVP

---

## 🚨 Red Flags - Arrête immédiatement si tu fais ça

- ❌ Créer des sprites custom
- ❌ Chercher assets parfaits >30 min
- ❌ Ajouter animations
- ❌ Refactoriser code qui marche
- ❌ Débattre >15 min d'un détail
- ❌ Coder une feature "cool" hors MVP
- ❌ Optimiser prématurément

**Si tu fais 1 de ces trucs** → STOP, relis CONSTRAINTS.md

---

## 💬 Communication quotidienne (template)

**Chaque jour dans chat équipe** :
```
[TON NOM] - [DATE]
Hier : [Feature codée / problème résolu]
Aujourd'hui : [Ce que tu vas faire]
Bloqué : [Problème ou "RAS"]
```

Exemple :
```
Alice - 16/11
Hier : Player movement WASD fonctionnel
Aujourd'hui : Energy gauge UI
Bloqué : RAS
```

---

## 🎯 Questions fréquentes

**Q: "Je ne trouve pas de sprite qui me plaît"**
A: Carré coloré. NEXT!

**Q: "Cette feature va prendre 3 semaines"**
A: Coupe-la en 2 ou simplifie. Max 2 semaines.

**Q: "On pourrait ajouter X, c'est facile"**
A: C'est dans MVP ? Non → doc/futures_features.md, Level 2+

**Q: "Le code est moche, je devrais refacto"**
A: Ça marche ? Oui → TOUCHE À RIEN jusqu'à Level 2

**Q: "Je suis bloqué depuis 2 jours"**
A: DEMANDE DE L'AIDE maintenant! Chat équipe ou Discord Bevy

**Q: "On a du retard sur le planning"**
A: Coupe features secondaires. Niveau 15 victoire → Niveau 10 victoire, etc.

---

## 📊 Définition de "Done" pour chaque feature

Une feature est **DONE** si :
1. ✅ Code compile sans warnings
2. ✅ Pas de crash évident en <5 min de test
3. ✅ Fait le job minimum (pas besoin de perfection)
4. ✅ Valeurs dans config.ron (si applicable)
5. ✅ Commit + push

**PAS besoin de** :
- Tests unitaires (MVP)
- Documentation complète
- Code "propre" parfait
- Optimisations

---

## 🎮 Premier playtest (Semaine 2)

**Qui** : Quelqu'un qui n'a PAS codé cette feature

**Quoi** : Jouer 5-10 min

**Questions** :
1. "Ça marche ?" (Oui/Non)
2. "C'est fun / intéressant ?" (Oui/Moyen/Non)
3. "C'est bloquant si non ?" (Oui/Non)

**Action** :
- Si bloquant → Fix cette semaine
- Si pas fun mais pas bloquant → Note pour Level 2
- Si fun → NEXT feature!

---

## 🏁 Tu es prêt à démarrer si...

- [ ] Tu as lu CONSTRAINTS.md
- [ ] Tu sais quelle feature tu codes en premier
- [ ] Tu as les assets de base (carrés/cercles colorés)
- [ ] Tu sais où demander de l'aide si bloqué
- [ ] Tu sais que "moche mais fini" > "beau mais jamais"

**Si toutes les cases cochées** → GO CODE! 🚀

---

## 📞 Besoin d'aide ?

**Code/Bevy** :
- Discord Bevy officiel
- Stack Overflow
- Chat équipe

**Design/Décisions** :
- Relis GDD.md et CONSTRAINTS.md
- Chat équipe
- Règle des 15 min : débat max 15 min puis VOTE

**Moral/Motivation** :
- Chat équipe
- Relis le mantra : "FINI ET MOCHE > PARFAIT ET JAMAIS"
- Regarde WEEKLY_TRACKER.md : t'as déjà fait combien de features? 💪

---

**Bon courage! Vous allez finir ce MVP! 🎯**
