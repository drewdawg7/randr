# Loot System

## Overview

The loot system handles item drops from entities (mobs, rocks, chests). It uses a probability-based `LootTable` and the `HasLoot` trait with direct item spawning.

## Key Files

| File | Purpose |
|------|---------|
| `src/loot/definition.rs` | `LootTable`, `LootItem`, `LootDrop` structs |
| `src/loot/traits.rs` | `HasLoot` trait |
| `src/loot/mod.rs` | Re-exports |

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
| `with(item, num, denom, qty)` | `Self` | Builder: add item with `num/denom` chance |
| `add_loot_item(item)` | `Result<usize, LootError>` | Add item, returns index on success |
| `roll_drops(magic_find)` | `Vec<LootDrop>` | Roll and spawn items directly |

### Roll Logic

Each item rolls independently:
1. Roll `1..=denominator`
2. If roll `<= numerator`, item drops
3. Roll quantity from range
4. Spawn item via `ItemId::spawn()` directly

## HasLoot Trait

```rust
pub trait HasLoot {
    fn loot(&self) -> &LootTable;

    fn roll_drops(&self, magic_find: i32) -> Vec<LootDrop> {
        self.loot().roll_drops(magic_find)
    }
}
```

Implementors:
- `Mob` - drops loot on death (`src/mob/combat.rs`)
- `Chest` - chest loot table (`src/chest/traits.rs`)
- `Rock` - drops when mined (`src/location/mine/rock/traits.rs`)

## Loot Flow: Combat

```
Mob.on_death(magic_find)
  -> self.roll_drops(magic_find)   [HasLoot trait]
  -> MobDeathResult.loot_drops     [Vec<LootDrop>]
  -> Fight UI adds to inventory
```

## Loot Flow: Mining

```
Rock.on_death(magic_find)
  -> self.roll_drops(magic_find)   [HasLoot trait]
  -> RockDeathResult.drops         [Vec<LootDrop>]
  -> Mine UI adds to inventory
```

## Creating a Loot Table

```rust
let loot = LootTable::new()
    .with(ItemId::CopperOre, 1, 1, 1..=3)      // 100% chance, 1-3 items
    .with(ItemId::IronOre, 1, 4, 1..=2)        // 25% chance, 1-2 items
    .with(ItemId::GoldOre, 1, 20, 1..=1);      // 5% chance, 1 item
```

## Magic Find

The `magic_find` parameter increases drop chances:
- Passed to `roll_drops(magic_find)` and `on_death(magic_find)`
- Higher values increase probability of drops

## Related Types

| Type | File | Description |
|------|------|-------------|
| `MobDeathResult` | `src/combat/result.rs` | Contains `loot_drops: Vec<LootDrop>` |
| `RockDeathResult` | `src/combat/result.rs` | Contains `drops: Vec<LootDrop>` |
