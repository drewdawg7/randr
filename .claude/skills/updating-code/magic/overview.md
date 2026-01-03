# Magic System Overview

## Architecture

The magic system is located in `src/magic/` with the following structure:

```
src/magic/
├── mod.rs              # Module exports
├── effect/
│   ├── mod.rs
│   ├── active.rs       # ActiveEffect enum (damage, heal, buff)
│   └── passive.rs      # PassiveEffect enum (stat bonuses)
├── page/
│   ├── mod.rs
│   └── definition.rs   # Page struct (holds 1-5 words)
├── spell/
│   ├── mod.rs
│   ├── computation.rs  # compute_spell() - emergent effect calculation
│   ├── definition.rs   # ComputedSpell, BackfireEffect, SpellResult
│   └── recipes.rs      # Hardcoded recipes and invalid combos
├── tome/
│   ├── mod.rs
│   └── definition.rs   # Tome struct (holds 3 pages)
└── word/
    ├── mod.rs
    ├── definition.rs   # WordId enum, WordSpec, WordProperties
    └── specs.rs        # RegistryDefaults for all words
```

## Core Concepts

### Word System
- `WordId`: Enum of all available magic words (Fire, Ice, Bolt, etc.)
- `WordSpec`: Static definition with name, description, and properties
- `WordProperties`: Numeric properties (damage, defense, healing, etc.) and flags
- `WordRegistry`: Registry pattern for word lookup, stored in `GameState`

### Page System
- `Page`: Holds 1-5 `WordId` values
- `inscribe()`: Computes spell effect from words
- Caches `ComputedSpell` result for performance

### Tome System
- `Tome`: Holds 3 pages (spell slots)
- `active_page_index`: Which page is selected for casting
- `passive_effects()`: Aggregates passives from all pages

### Spell Computation
The `compute_spell()` function uses a hybrid approach:

1. **Check hardcoded recipes** (`recipes.rs:RECIPES`)
   - Exact word set matches produce designed spells
   - E.g., Fire + Bolt = "Firebolt" (15 fire damage)

2. **Check invalid combos** (`recipes.rs:INVALID_COMBOS`)
   - Specific word pairs cause backfire
   - E.g., Fire + Ice = damages self

3. **Compute emergent effect** (`computation.rs`)
   - Aggregate word properties (additive)
   - Determine spell type from combined properties
   - Generate name from word names

### Effect Types
- `ActiveEffect`: Castable combat effects (Damage, Heal, LifeDrain, DefenseBuff, etc.)
- `PassiveEffect`: Always-on bonuses (BonusAttack, BonusDefense, BonusGoldFind, Reveal)
- `ComputedSpell`: Result of word combination (Active, Passive, Hybrid, Backfire, Fizzle)

## Key Patterns

### Registry Pattern
Words use the standard registry pattern:
```rust
// In GameState
word_registry: WordRegistry,  // Registry<WordId, WordSpec>

// Access
game_state().word_registry().get(&WordId::Fire)
```

### Property Builder
WordProperties uses builder pattern:
```rust
WordProperties::new()
    .damage(5)
    .element(Element::Fire)
    .projectile()
```

### Property Combination
Properties combine additively when words are inscribed:
```rust
WordProperties::combine(&[&fire_props, &bolt_props])
// damage = fire.damage + bolt.damage
// elements = fire.elements ∪ bolt.elements
```

## UI Integration

### Spell Test Modal
- Location: `src/ui/components/magic/spell_test_modal.rs`
- Hotkey: 'T' (god mode testing)
- Allows text input to test any word combination
- Shows resulting spell or error

### Modal Registration
- `ModalType::SpellTest` in `src/ui/state.rs`
- `SpellTestModal` instance in `GameState`
- Handled in `modal_wrapper.rs`

## Available Words

| Category | Words |
|----------|-------|
| Elements | fire, ice, lightning |
| Actions | bolt, shield, burst, drain |
| Modifiers | power, swift, stable, chaos |
| Utility | sight, gold, mend |

## Example Recipes

- fire + bolt = "Firebolt" (15 fire damage)
- ice + bolt = "Frostbolt" (10 ice damage + slow)
- lightning + burst = "Thunder Nova" (12 lightning AoE)
- drain + power = "Life Siphon" (10 damage, 50% lifesteal)
- mend + power = "Greater Heal" (20 HP)

## Invalid Combos (Backfire)

- fire + ice = Elemental conflict (10 self-damage)
- lightning + drain = Energy feedback (2 turn stun)

## Future Work

1. **Combat Integration**: Add spell casting as combat action
2. **Tome Equipment**: Add EquipmentType::Tome to item system
3. **Player Integration**: Add tome field to Player struct
4. **Mana System**: Resource cost for casting
5. **World Effects**: Non-combat spell effects
