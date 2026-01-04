# Mob Entity Module

## Overview

Mobs use the `define_entity!` macro for declarative definitions with direct spawning via `MobId::spawn()`.

## Key Files

| File | Purpose |
|------|---------|
| `src/mob/definitions.rs` | All mobs defined via `define_entity!` macro |
| `src/mob/enums.rs` | `MobQuality` enum |
| `src/mob/definition.rs` | `Mob` struct - spawned mob instances |
| `src/mob/combat.rs` | Combat trait implementations |

## Entity Macro System

Mobs are defined using the `define_entity!` macro which generates:
- `MobId` enum with all variants
- `MobId::spec(&self) -> &'static MobSpec` method
- `MobId::spawn(&self) -> Mob` convenience method
- `MobId::ALL: &[MobId]` for iteration

## Spawning Mobs

```rust
// Direct spawning (preferred)
let dragon = MobId::Dragon.spawn();

// Spec lookup
let spec = MobId::Dragon.spec();
println!("Name: {}", spec.name);
```

## Naming Convention

Entity identifier enums follow the pattern `{Entity}Id`:
- `ItemId` (items)
- `MobId` (mobs)
- `RockId` (mineable rocks)
- `LocationId` (locations)

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

Add to `src/mob/definitions.rs` inside the `define_entity!` block:

```rust
Orc => MobSpec {
    name: "Orc",
    max_health: 40..=60,
    attack: 12..=18,
    defense: 8..=12,
    dropped_gold: 15..=25,
    dropped_xp: 20..=30,
    quality: MobQuality::Normal,
    loot: LootTable::new()
        .with(ItemId::IronOre, 1, 4, 1..=2),
},
```

Then add to field spawn weights if spawnable:
```rust
// In src/location/field/definition.rs or field specs
mob_weights.insert(MobId::Orc, 30);
```

## Trait Implementations

| Trait | File | Notes |
|-------|------|-------|
| `HasLoot` | `src/mob/combat.rs` | Provides `loot()` and `roll_drops()` |
| `IsKillable` | `src/mob/combat.rs` | `on_death(magic_find)` returns `MobDeathResult` |
| `Combatant` | `src/mob/combat.rs` | Combat interface |
| `DealsDamage` | `src/mob/combat.rs` | Uses default ±25% attack variance |

## Death Processing Guard

The `Mob` struct has a `death_processed: bool` field:
- Initialized to `false` when spawned
- Set to `true` on first `on_death()` call
- Subsequent calls return `MobDeathResult::default()` (empty)

## Related Modules

- `src/combat/` - Combat system that uses mobs
- `src/location/field/` - Fields spawn mobs based on weighted probabilities
- `src/loot/` - `LootTable` and `HasLoot` trait
