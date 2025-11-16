# Tracker Hebdomadaire - Colonie terminus

**Objectif** : Vérifier chaque semaine qu'on respecte nos contraintes pour garantir qu'on termine.

---

## Comment utiliser ce tracker

1. **Chaque vendredi soir** : Remplir le rapport de la semaine
2. **Copier le template** en bas de ce fichier
3. **Remplir honnêtement** les checkboxes et métriques
4. **Si ≥2 ❌ rouges** → Réunion d'urgence équipe le weekend

---

## 🚨 Red Flags à surveiller

Si **2+ red flags** actifs → Danger critique de ne pas finir

- [ ] Aucun commit depuis >7 jours
- [ ] Feature en cours depuis >2 semaines
- [ ] Passé >2h à chercher/créer des assets
- [ ] Débat non résolu depuis >1 semaine
- [ ] Code refactorisé sans ajout de feature
- [ ] Feature "cool" ajoutée hors MVP
- [ ] Playtest négatif non adressé
- [ ] Retard >1 semaine sur planning

---

## Rapport Semaine [NUMÉRO] - [DATE DÉBUT] au [DATE FIN]

### 📊 Métriques de la semaine

**Temps investi** :
- Personne 1 : ___h
- Personne 2 : ___h
- Personne 3 : ___h
- Personne 4 : ___h
- **Total équipe** : ___h / objectif 15-40h

**Code** :
- Lignes ajoutées : ~___
- Lignes total projet : ~___ / objectif <5000
- Commits : ___

**Features** :
- Feature en cours : _______________
- Depuis combien de semaines : ___
- Avancement estimé : ___%

### ✅ Checklist respect des contraintes

**Assets (Contrainte #1)** :
- [ ] Aucun asset custom créé cette semaine
- [ ] Assets utilisés = gratuits OU formes géométriques
- [ ] Temps recherche assets <30 min par type
- [ ] Si asset non trouvé → utilisé placeholder géométrique

**Scope (Contrainte #2 & #5)** :
- [ ] Aucune feature hors liste MVP ajoutée
- [ ] Aucune animation complexe ajoutée
- [ ] UI reste minimaliste (rectangles + texte)
- [ ] Pas de temps sur polish/juice

**Temps (Contrainte #3)** :
- [ ] Feature actuelle <2 semaines
- [ ] Aucune tâche bloquée >3 jours
- [ ] Planning respecté cette semaine

**Code (Contrainte #4)** :
- [ ] Code simple qui marche (pas d'over-engineering)
- [ ] Pas de refacto "pour faire beau"
- [ ] Pas d'abstractions "pour plus tard"
- [ ] config.ron utilisé pour balance values

**Décisions (Contrainte #6)** :
- [ ] Aucun débat >15 min cette semaine
- [ ] Toutes décisions documentées (si importantes)
- [ ] Pas de remise en question de décisions passées

**Tests (Contrainte #7)** :
- [ ] Si semaine paire → Playtest fait par quelqu'un qui n'a pas codé
- [ ] Feedback playtest documenté
- [ ] Actions correctives définies si problème

**Communication (Contrainte #9)** :
- [ ] Updates async quotidiens dans chat
- [ ] Tout le monde sait ce que les autres font
- [ ] Bloquages communiqués rapidement

### 🎯 Objectifs semaine suivante

**Feature principale** :
- [ ] ______________________________

**Tâches secondaires** :
- [ ] ______________________________
- [ ] ______________________________

**Risques identifiés** :
- ______________________________
- ______________________________

### 💚 Moral de l'équipe

**Confiance qu'on va finir le MVP** : ☆☆☆☆☆ (_/5)

**Commentaire** :
_____________________________________________
_____________________________________________

### 🔴 Red Flags actifs cette semaine

Cocher si applicable :
- [ ] Aucun commit depuis >7 jours
- [ ] Feature en cours depuis >2 semaines
- [ ] Passé >2h à chercher/créer des assets
- [ ] Débat non résolu depuis >1 semaine
- [ ] Code refactorisé sans ajout de feature
- [ ] Feature "cool" ajoutée hors MVP
- [ ] Playtest négatif non adressé
- [ ] Retard >1 semaine sur planning

**Nombre de red flags** : ___ / 8

**Actions correctives (si ≥2 red flags)** :
_____________________________________________
_____________________________________________

---

## 📈 Historique des semaines

### Semaine 0 - [DATE] au [DATE] - Setup projet

**Métriques** :
- Heures équipe : 0h (setup initial)
- Lignes code : 0
- Features : Setup Bevy + repo Git

**Contraintes respectées** : ✅ Toutes (pas encore de dev)

**Red flags** : 0/8

**Moral** : 5/5 - Motivation haute, début du projet!

---

### Semaine 1 - [DATE] au [DATE]

**Métriques** :
- Heures équipe : ___h / 15-40h
- Lignes code : ~___
- Features : _______________________

**Contraintes respectées** :
- Assets : ☐ ✅ ❌
- Scope : ☐ ✅ ❌
- Temps : ☐ ✅ ❌
- Code : ☐ ✅ ❌
- Décisions : ☐ ✅ ❌
- Tests : ☐ ✅ ❌ (N/A si semaine impaire)
- Communication : ☐ ✅ ❌

**Red flags** : ___/8

**Moral** : ___/5

**Commentaire** :
_____________________________________________

---

### Semaine 2 - [DATE] au [DATE]

**Métriques** :
- Heures équipe : ___h / 15-40h
- Lignes code : ~___
- Features : _______________________

**Contraintes respectées** :
- Assets : ☐ ✅ ❌
- Scope : ☐ ✅ ❌
- Temps : ☐ ✅ ❌
- Code : ☐ ✅ ❌
- Décisions : ☐ ✅ ❌
- Tests : ☐ ✅ ❌ **← PLAYTEST OBLIGATOIRE**
- Communication : ☐ ✅ ❌

**Red flags** : ___/8

**Moral** : ___/5

**Playtest résultat** :
- Testeur : _______________________
- Fun (Oui/Non) : ___
- Feedback : _______________________

**Commentaire** :
_____________________________________________

---

## 📊 Dashboard global MVP

**Mise à jour après chaque semaine**

### Planning

| Semaine | Sprint | Objectif | Status |
|---------|--------|----------|--------|
| 1-2 | Movement + Energy | Player bouge, energy regen, orbital cursor | ☐ |
| 3-4 | Weapons | Blaster + Plasma + projectiles + collision | ☐ |
| 5-6 | Creatures | 3 types spawn + AI + XP drops | ☐ |
| 7-8 | Progression | Level-up + 6 upgrades + difficulty scaling | ☐ |
| 9-10 | World | Spherical wrapping + minimap + timer | ☐ |
| 11 | Events | Boss + Crashed ship (simplifié) | ☐ |
| 12 | Polish | Bug fixes + UI minimal + playtests externes | ☐ |

**Semaine actuelle** : ___/12

**Retard** : ___ semaines (si négatif = en avance!)

### Budget

**Temps** :
- Investi : ___h / 480h budget
- Restant : ___h
- Rythme actuel : ___h/semaine

**Code** :
- Lignes actuelles : ~___
- Objectif : <5000 lignes

### Validation MVP

**Checklist technique** :
- [ ] 60 FPS avec 100+ creatures
- [ ] Input lag <50ms
- [ ] Pas de crash 15 min
- [ ] Spherical wrapping sans bugs

**Checklist gameplay** :
- [ ] Energy management implémenté
- [ ] 2 weapons fonctionnelles
- [ ] 3 creature types
- [ ] XP + level-up + 6 upgrades
- [ ] Boss event
- [ ] Crashed ship event

**Checklist fun (CRITIQUE)** :
- [ ] 3/3 testeurs veulent rejouer
- [ ] 3/3 energy = intéressant
- [ ] 2/3 events = excitants

**Avancement MVP** : ___/16 items (___%)

---

## 🎯 Objectif de fin

**Date cible MVP** : [DATE DANS 3 MOIS]

**Jours restants** : ___

**Probabilité de finir** :
- ☐ 🟢 Haute (>70%) - On suit le plan, bon moral, contraintes respectées
- ☐ 🟡 Moyenne (40-70%) - Léger retard ou quelques contraintes ignorées
- ☐ 🔴 Faible (<40%) - Retard sérieux, contraintes non respectées, red flags

**Action si probabilité faible** :
1. Réunion d'urgence équipe
2. Couper features non critiques
3. Réduire scope MVP
4. OU accepter d'arrêter proprement

---

## Template à copier chaque semaine

```markdown
---

### Semaine X - [DATE] au [DATE]

**Métriques** :
- Heures équipe : ___h / 15-40h
- Lignes code : ~___
- Features : _______________________

**Contraintes respectées** :
- Assets : ☐ ✅ ❌
- Scope : ☐ ✅ ❌
- Temps : ☐ ✅ ❌
- Code : ☐ ✅ ❌
- Décisions : ☐ ✅ ❌
- Tests : ☐ ✅ ❌
- Communication : ☐ ✅ ❌

**Red flags** : ___/8

**Moral** : ___/5

**Commentaire** :
_____________________________________________
```

---

**Rappel** : Ce tracker n'est utile que si rempli **honnêtement**. Mentir = se mentir = projet abandonné.
