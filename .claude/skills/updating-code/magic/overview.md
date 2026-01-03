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

## Tome Equipment System

### EquipmentType::Tome
- Location: `src/item/enums.rs`
- Maps to `OffHand` slot (shared with shields)
- Check with `EquipmentType::is_tome()`

### Item.tome_data
- Location: `src/item/definition.rs`
- Field: `pub tome_data: Option<Tome>`
- Initialized automatically when spawning tome items (see `spawn_from_spec` in `src/item/spec/traits.rs`)

### Equipped Tome Access
- `Inventory::equipped_tome()` -> `Option<&Tome>`
- `Inventory::equipped_tome_mut()` -> `Option<&mut Tome>`
- `Player::equipped_tome()` -> `Option<&Tome>`
- `Player::equipped_tome_mut()` -> `Option<&mut Tome>`

## Passive Effect Integration

### Player Bonus Methods
- Location: `src/entities/player/definition.rs`
- `tome_passive_effects()` -> `Vec<&PassiveEffect>`
- `tome_attack_bonus()` -> `i32`
- `tome_defense_bonus()` -> `i32`
- `tome_goldfind_bonus()` -> `i32`
- `tome_magicfind_bonus()` -> `i32`

### Effective Stats (include tome bonuses)
- `effective_attack()` includes `tome_attack_bonus()`
- `effective_defense()` includes `tome_defense_bonus()`
- `effective_goldfind()` includes `tome_goldfind_bonus()`
- `effective_magicfind()` includes `tome_magicfind_bonus()`

## Combat Integration

### Spell Casting Functions
- Location: `src/combat/system.rs`
- `player_cast_spell_step(player, combat)` -> `SpellCastResult`
- `player_has_castable_spell(player)` -> `bool`
- `get_active_spell_name(player)` -> `Option<String>`

### SpellCastResult Enum
- Location: `src/combat/result.rs`
- Variants: `Damage`, `Heal`, `LifeDrain`, `NoSpell`, `Fizzle`

### Supported Active Effects
- `Damage { amount, element }` - Direct damage with defense reduction
- `Heal { amount }` - Restore caster HP
- `LifeDrain { damage, heal_percent }` - Damage + percentage healing
- `AreaDamage { amount, element }` - Treated as single-target in 1v1
- `DamageWithEffect` - Recursive effect application

## Future Work

1. **Mana System**: Resource cost for casting
2. **Defense/Slow Buffs**: Implement buff tracking in combat state
3. **World Effects**: Non-combat spell effects (dungeon bypass, etc.)
4. **Inscription UI**: Allow players to inscribe words onto pages
