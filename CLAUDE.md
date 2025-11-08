# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Eclipse of Magic** - A magic-based survivor-like game where the world's magic has vanished.

- **Title**: Eclipse of Magic
- **Genre**: Action / Survivor-like / Roguelite with magic system and resource management
- **Core Mechanic**: Manual spellcasting with mana gauge management
- **Setting**: Dark cave, Sorceress trapped in hostile environment, recovering lost magic
- **Target Platform**: PC (Steam), with future mobile premium support
- **Tech Stack**: Bevy (Rust) - ECS architecture
- **Current Status**: Pre-production - concise GDD completed, ready to start MVP development

## Key Design Document

- **GDD.md** - Main Game Design Document
  - Concise, actionable, fun-focused design specs
  - Final design decisions with rationale
  - MVP feature list and scope boundaries (Level 1)
  - Detailed combat system (spells, mana, boss)
  - Technical architecture for Bevy (ECS, config.ron pattern)
  - Production philosophy and red flags
  - Post-MVP roadmap (Level 2-4)

## Game Design Pillars

1. **Mana Management** - Manual spellcasting with mana gauge creates tactical resource management
2. **Tactical Choices** - 2 spells + 4 passives max, every level-up is critical
3. **Addictive Loop** - 15 min runs, permadeath, boss every 5 levels with lore reveals
4. **Power Growth** - From barely surviving in darkness to dominating with magic
5. **Skill Ceiling** - Spell timing, mana management, positioning over spam

## Core Gameplay Loop

```
Spawn in dark cave → Cast spells (mouse aim) → Manage mana gauge → Kill entities → Collect XP crystals
    ↓
Level up (30-60s) → Choose 1 upgrade from 3 cards
    ↓
Upgrade spell OR new passive OR new spell (if <2)
    ↓
Every 5 levels → BOSS with unique pattern + lore
    ↓
Boss victory → Magic Fragment + Guaranteed spell drop
    ↓
Become more powerful → Sorceress glows brighter
    ↓
Die OR Level 15 (victory) OR Level 100 (unlock character 2)
    ↓
Spend Magic Fragments → Permanent upgrades → RETRY
```

## Key Systems (MVP - Level 1)

### Character
- **Sorceress** (girl): First playable character
- Becomes more luminous as she recovers magic (visual progression)
- Stats: 100 HP, 140 speed, 100 max mana, 20 mana regen/s

### Combat System
- **Manual spellcasting**: Aim with mouse, click to cast (consumes mana)
- **Mana gauge**: 100 max, regenerates 20/s (5s for full recharge)
- **2 spell slots max** + **4 passive slots max**
- No auto-aim, skill-based targeting

### Spells (MVP)
1. **Gleam (Lueur)**: Starter spell
   - Cost: 10 mana, Damage: 15, Fast projectile
   - Light projectile that illuminates area

2. **Fireball (Boule de feu)**: Unlockable level 3
   - Cost: 30 mana, Damage: 50 AoE
   - Explosive projectile, high risk/reward

### Entities (MVP)
- **Shadow Crawler (Ombre)**: Slow, weak HP, flees from light
- **Specter**: Medium speed, passes through walls
- **Lesser Demon (Démon mineur)**: Fast, aggressive, dangerous

### Boss (MVP)
- **Guardian of Darkness** (Gardien des Ténèbres) - Level 5
  - 500 HP, 2-phase combat
  - Phase 1: Slow charge + projectiles
  - Phase 2 (<50% HP): Faster, 3-projectile fan attacks
  - Drops: Magic Fragment + Random spell + Lore text

### Ambiance
- **Dark cave environment**: Black/dark gray background
- **Fog of war**: Entities emerge from darkness
- **Light source**: Circle around Sorceress (150px radius)
- **Visual feedback**: Mana bar glows/pulses

### Difficulty Scaling
- Levels 1-3: Tutorial phase
- Levels 3-5: Intensification, Fireball available
- Level 5: BOSS
- Levels 6-15: Exponential scaling (+15% HP per 30s)
- Level 15: VICTORY (~15 min)

## Development Phases

### Level 1: MVP (2-3 months) - CURRENT FOCUS
**Goal**: Validate that magic system + boss is FUN in 10 minutes

**Must-have features:**
- Sorceress character with WASD movement
- Mana gauge system (100 max, 20 regen/s)
- 2 spells: Gleam + Fireball
- 3 entity types + 1 boss (level 5)
- XP system with level-ups every 30-60s
- 6 upgrade types (new spell, +damage, +mana regen, +speed, +HP, +collect radius)
- Magic Fragment currency (kept on death)
- Dark cave ambiance with fog of war
- Basic UI: HP bar, Mana bar (glowing blue), XP bar, timer, fragments counter

**Forbidden scope creep:**
- Multiple bosses (1 only)
- Multiple biomes (cave only)
- Spendable meta-progression (Level 2)
- Complex elements/synergies (Level 3)
- Multiple characters (Sorceress only)
- Complex animations (static sprites OK)

**Success metrics:**
- 60 FPS with 100+ entities
- <50ms input lag
- Mana management feels tactical, not frustrating
- Boss at level 5 feels rewarding
- 3/3 external testers want to replay after boss

### Level 2: Meta-Progression & Bosses (1-2 mois)
- Meta-progression menu to spend Magic Fragments
- 2nd and 3rd bosses (levels 10, 15)
- 5 total spells, 12 passives
- Lore reveals after each boss
- Destructible obstacles
- Basic animations + particle effects

### Level 3: Depth & Content (2-3 mois)
- Elemental system (Fire/Ice/Lightning) with synergies
- Spell evolutions (e.g., Gleam → Laser Beam)
- 2nd unlockable character (level 100)
- 10 spells, 25 passives
- Additional biome
- 30+ achievements

### Level 4: Steam Release (1-2 mois)
- Final polish
- Complete music & sound design
- 5 biomes, 6 bosses with complete lore story
- Infinite mode
- Tutorial
- Leaderboards
- Steam page preparation

## Technical Architecture (Bevy/Rust)

### ECS Components Structure

```rust
// Core player component
struct Sorceress {
    hp: f32,
    max_hp: f32,
    speed: f32,
    mana: f32,
    max_mana: f32,
    mana_regen: f32,
}

// Mana system is critical differentiator
struct Spell {
    damage: f32,
    mana_cost: f32,
    cast_cooldown: f32,
    is_aoe: bool,
    aoe_radius: Option<f32>,
}

// Boss with phases
struct Boss {
    hp: f32,
    max_hp: f32,
    phase: BossPhase,
    attack_timer: f32,
}

// Fog of war lighting
struct LightSource {
    radius: f32,
    intensity: f32,
}
```

### System Execution Order
1. input_system
2. **mana_regen_system** (unique to this game)
3. movement_system
4. spell_system (mana cost check)
5. projectile_system
6. collision_system
7. boss_ai_system
8. spawn_system
9. level_up_system
10. **lighting_system** (fog of war)
11. ui_system

### Configuration File (config.ron)

All game balance in `config.ron` for rapid iteration:

```ron
(
    sorceress: (
        hp: 100.0,
        max_mana: 100.0,
        mana_regen: 20.0,
        speed: 140.0,
        light_radius: 150.0,
    ),
    spells: {
        "lueur": (damage: 15.0, mana_cost: 10.0, ...),
        "boule_de_feu": (damage: 50.0, mana_cost: 30.0, is_aoe: true, ...),
    },
    entities: {
        "ombre": (hp: 25.0, speed: 40.0, ...),
        "spectre": (hp: 45.0, can_pass_walls: true, ...),
        "demon": (hp: 60.0, speed: 110.0, ...),
    },
    boss: {
        "gardien_tenebres": (hp: 500.0, phase2_hp_threshold: 250.0, ...),
    },
)
```

## Key Differentiators vs Vampire Survivors

| Aspect | Vampire Survivors | Eclipse of Magic |
|--------|-------------------|------------------|
| Combat | Auto-attack | Manual spellcasting |
| Resource | None | Mana gauge management |
| Duration | 30 min | 15 min (intense) |
| Build | Unlimited weapons | 2 spells + 4 passives |
| Bosses | None (DLC only) | Every 5 levels + lore |
| Narration | Minimal | Story revealed via bosses |
| Ambiance | Colorful cartoon | Dark Fantasy cave, fog of war |
| Progression | Cosmetic neutral | Sorceress glows brighter |
| Feel | Chill, zen | Tense, tactical |

## Development Philosophy

### Mantras
1. "Does it make the game more FUN?" (if no → cut)
2. "MVP first" (features belong in Level 2+)
3. "Test before coding more"
4. "Config file > hardcode"

### Red Flags to Watch
🚩 "We could add..." without finishing existing features
🚩 Spending >2 days on MVP assets
🚩 Coding features without clear specs
🚩 Not playtesting for 2+ weeks
🚩 Debating >30min without deciding and moving forward

### Critical Success Factors
- **Mana management must feel tactical, not frustrating** → Core differentiator
- **Boss at level 5 must be rewarding** → Validates boss rhythm
- **Fog of war must enhance, not annoy** → Atmosphere vs frustration balance
- **Sorceress visual progression** → Satisfying power fantasy feedback

## Assets Needed (MVP)

**Keep it simple - placeholders first, polish later**

- Sorceress sprite: 32x32 girl (static, no animation)
- Entity sprites: 32x32 dark silhouettes (Ombre, Spectre, Demon)
- Boss sprite: 64x64 large shadow with red glowing eyes
- Spell projectiles: Colored circles (blue for Gleam, red/orange for Fireball)
- UI: Colored rectangles (red HP, glowing blue mana, purple XP)
- Map: Solid dark background
- Light: Semi-transparent white circle shader
- VFX: Simple white/blue particles

**Music/SFX:**
- 1 dark ambient cave track (loop)
- 1 epic boss track
- Basic SFX: spell cast, impact, death, level-up

## Testing & Validation

After MVP completion, test with 3 external players:

1. "After 10 min (including boss), do you want to replay?"
2. "Is mana management interesting or frustrating?"
3. "Does boss at level 5 arrive at the right time?"
4. "Is dark ambiance + light enjoyable or annoying?"
5. "Do you feel progressively more powerful?"

If <2/3 say yes to Q1 → Core loop problem
If mana feels frustrating → Rebalance regen speed or spell costs
If boss timing off → Adjust level-up frequency
If darkness annoying → Increase light radius

## Important Files to Reference

- **GDD.md** - Complete game design, all systems detailed
- **GDD.md MVP section** - Features checklist and forbidden scope creep
- **GDD.md Combat System** - Detailed spell/mana/boss mechanics
- **GDD.md config.ron** - Balance values reference

## When Starting Development

1. Setup Bevy project with ECS architecture
2. Create `config.ron` with all balance values
3. Implement mana system FIRST (core differentiator)
4. Test mana feel before adding complexity
5. Add fog of war lighting early (impacts entire gameplay)
6. Keep boss simple - validate pattern fun before adding complexity
7. Playtest every 2 weeks minimum

## Universe & Lore (Brief)

- **Setting**: World where magic has vanished
- **Protagonist**: Sorceress trapped in hostile cave
- **Mission**: Recover lost magic by defeating guardians
- **Progression**: Each boss drops Magic Fragment + reveals lore
- **Endgame**: Level 100 unlocks new character with own story
- **Tone**: Dark Fantasy, environmental storytelling, minimal text

---

Remember: **Fun > Beautiful** for MVP. A ugly but fun game beats a beautiful but boring one.
- Je voudrais qu'en tant qu'assistant de l'équipe tu gardes tout le temps à l'esprit de nous aider à terminer ce jeu et éviter de perdre du temps et de s'égarer et de ne jamais pouvoir finir le jeu. Je pense important de faire simple et minimaliste. Si possible de definir un maximum de contraintes