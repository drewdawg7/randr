# Mob Entity Module

## Overview

The mob module defines enemy entities that the player can fight. It follows the same registry pattern as `Item`, `Rock`, and `Field`.

## Key Files

| File | Purpose |
|------|---------|
| `src/entities/mob/enums.rs` | Defines `MobId` enum (variants: Slime, Goblin, Cow, Dragon) and `MobQuality` |
| `src/entities/mob/definition.rs` | Defines `Mob` struct with fields: `spec: MobId`, `quality`, `name`, `stats`, `gold`, `dropped_xp`, `loot_table` |
| `src/entities/mob/mod.rs` | Re-exports `Mob`, `MobId`, `MobRegistry` |
| `src/entities/mob/spec/definition.rs` | Defines `MobSpec` struct (with defense field) and `MobRegistry` type alias |
| `src/entities/mob/spec/traits.rs` | Implements `SpawnFromSpec<MobId>` and `RegistryDefaults<MobId>` for `MobSpec` |
| `src/entities/mob/spec/specs.rs` | Defines static specs: `SLIME`, `GOBLIN`, `COW`, `DRAGON` |

## Naming Convention

Entity identifier enums follow the pattern `{Entity}Id`:
- `ItemId` (items)
- `MobId` (mobs)
- `RockId` (mineable rocks)
- `FieldId` (combat areas)

## Registry Pattern

Mobs use the generic `Registry<K, V>` pattern from `src/registry.rs`:

```rust
pub type MobRegistry = Registry<MobId, MobSpec>;
```

Required trait implementations:
- `SpawnFromSpec<MobId>` - Creates `Mob` instance from `MobSpec`
- `RegistryDefaults<MobId>` - Provides default specs for each `MobId` variant

## Consumers

Files that depend on `MobId`:

| File | Usage |
|------|-------|
| `src/system.rs` | `spawn_mob(mob: MobId) -> Mob` method |
| `src/field/definition.rs` | `mob_weights: HashMap<MobId, i32>` field |
| `src/field/spec/definition.rs` | `mob_weights: HashMap<MobId, i32>` in `FieldSpec` |
| `src/field/spec/specs.rs` | Mob spawn weights per field |
| `src/combat/tests.rs` | Test helper creates mobs with `spec: MobId::Slime` |

## MobSpec Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | `&'static str` | Display name |
| `max_health` | `RangeInclusive<i32>` | HP range when spawned |
| `attack` | `RangeInclusive<i32>` | Attack stat range |
| `defense` | `RangeInclusive<i32>` | Defense stat range |
| `dropped_gold` | `RangeInclusive<i32>` | Gold drop range |
| `dropped_xp` | `RangeInclusive<i32>` | XP drop range |
| `quality` | `MobQuality` | Normal or Boss |
| `loot` | `LootTable` | Item drop table |

## Current Mob Defense Values

| Mob | Defense Range | Notes |
|-----|--------------|-------|
| COW | 0..=2 | Weak, no armor |
| SLIME | 1..=3 | Slightly squishy |
| GOBLIN | 5..=10 | Light armor |
| DRAGON | 30..=50 | Heavy natural scales |

## Adding a New Mob

1. Add variant to `MobId` enum in `src/entities/mob/enums.rs`
2. Create static spec in `src/entities/mob/spec/specs.rs` (include defense field)
3. Add to `RegistryDefaults` impl in `src/entities/mob/spec/traits.rs`
4. Add to field spawn weights in `src/field/spec/specs.rs` (if spawnable)

## Related Modules

- `src/combat/` - Combat system that uses mobs
- `src/field/` - Fields spawn mobs based on weighted probabilities
- `src/loot/` - `LootTable` defines mob drops
