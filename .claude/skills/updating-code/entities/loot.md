# Loot System

## Overview

The loot system handles item drops from entities (mobs, rocks, chests). It uses a probability-based `LootTable` and the `HasLoot` trait to provide a unified interface for rolling drops.

## Key Files

| File | Purpose |
|------|---------|
| `src/loot/definition.rs` | Defines `LootTable`, `LootItem`, and `LootDrop` structs |
| `src/loot/traits.rs` | Defines `HasLoot` trait with `loot()` and `roll_drops()` methods |
| `src/loot/mod.rs` | Re-exports `LootDrop`, `LootTable`, `HasLoot` |

## LootDrop Struct

Represents a single loot drop with a spawned item instance:

```rust
pub struct LootDrop {
    pub item: Item,      // Spawned item instance
    pub quantity: i32,   // Quantity dropped
}
```

## LootTable

Container for loot items with drop probabilities.

### Key Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `LootTable` | Create empty loot table |
| `with(item, num, denom, qty)` | `Self` | Builder: add item with `num/denom` chance, `qty` range |
| `roll_drops()` | `Vec<LootDrop>` | Roll all items, spawn dropped items |
| `ore_proportions()` | `Iterator<(ItemId, f32)>` | Get drop chances as floats (for UI display) |

### Roll Logic

Each item rolls independently:
1. Roll `1..=denominator`
2. If roll `<= numerator`, item drops
3. Roll quantity from range
4. Spawn item via `game_state().spawn_item()`

## HasLoot Trait

```rust
pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    fn roll_drops(&self) -> Vec<LootDrop> {
        self.loot().roll_drops()
    }
}
```

Implementors:
- `Mob` - drops loot on death in combat (`src/entities/mob/traits.rs`)
- `Chest` - has loot table (unused currently) (`src/chest/traits.rs`)
- `Rock` - drops items when mined (`src/location/mine/rock/traits.rs`)

## Loot Flow: Combat

```
Mob.on_death()
  -> self.roll_drops()         [HasLoot trait method]
  -> MobDeathResult.loot_drops [Vec<LootDrop>]
  -> ActiveCombat.loot_drops   [Vec<LootDrop>]
  -> Fight UI adds to inventory
```

## Loot Flow: Mining

```
Rock.on_death()
  -> self.roll_drops()         [HasLoot trait method]
  -> RockDeathResult.drops     [Vec<LootDrop>]
  -> Mine UI adds to inventory
```

## Creating a Loot Table

```rust
let loot = LootTable::new()
    .with(ItemId::CopperOre, 1, 1, 1..=3)      // 100% chance, 1-3 items
    .with(ItemId::IronOre, 1, 4, 1..=2)        // 25% chance, 1-2 items
    .with(ItemId::GoldOre, 1, 20, 1..=1);      // 5% chance, 1 item
```

## Related Types

| Type | File | Description |
|------|------|-------------|
| `MobDeathResult` | `src/combat/result.rs` | Contains `loot_drops: Vec<LootDrop>` |
| `RockDeathResult` | `src/combat/result.rs` | Contains `drops: Vec<LootDrop>` |
| `ActiveCombat` | `src/combat/state.rs` | Contains `loot_drops: Vec<LootDrop>` |
| `CombatRounds` | `src/combat/system.rs` | Contains `loot_drops: Vec<LootDrop>` |
