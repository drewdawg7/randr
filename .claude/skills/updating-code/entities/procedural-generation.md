# Procedural Generation System

## Overview

The procedural generation system allows creating modified variants of entities at runtime. Both `MobSpec` and `ItemSpec` support:
- **Scaling**: Multiply stats by a factor for elite/dungeon variants
- **Naming**: Custom names for unique entities
- **Quality**: Fixed quality levels instead of random rolls

All spawned entities are tied to a base registry ID (`MobId`, `ItemId`).

## Key Concepts

| Term | Description |
|------|-------------|
| Spec | Template defining entity properties (ranges, base stats) |
| Instance | Spawned entity with rolled/computed values |
| ID | Required identifier linking entity to registry (`MobId`, `ItemId`) |

## MobId Spawning

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `spawn()` | `&self -> Mob` | Spawn a mob |
| `with_multiplier()` | `&self, f32 -> MobSpawner` | Start builder with scaled stats |
| `with_name()` | `&self, impl Into<String> -> MobSpawner` | Start builder with custom name |
| `with_quality()` | `&self, MobQuality -> MobSpawner` | Start builder with fixed quality |

### MobSpawner (Builder)

| Method | Signature | Description |
|--------|-----------|-------------|
| `spawn()` | `self -> Mob` | Spawn with all modifications |
| `with_multiplier()` | `self, f32 -> Self` | Chain: scale all stat ranges |
| `with_name()` | `self, impl Into<String> -> Self` | Chain: change display name |
| `with_quality()` | `self, MobQuality -> Self` | Chain: set quality |

### Examples

```rust
// Normal spawn
let slime = MobId::Slime.spawn();

// Elite variant: 1.5x stats with custom name
let elite_slime = MobId::Slime
    .with_multiplier(1.5)
    .with_name("Elite Slime")
    .spawn();

// Dungeon scaling: deeper floors = stronger mobs
let depth_multiplier = 1.0 + (floor as f32 * 0.2);
let scaled_goblin = MobId::Goblin
    .with_multiplier(depth_multiplier)
    .spawn();

// Boss variant of a normal mob
let mini_boss = MobId::Cow
    .with_multiplier(3.0)
    .with_name("Dire Cow")
    .with_quality(MobQuality::Boss)
    .spawn();
```

### What Gets Scaled

`with_multiplier(factor)` scales these ranges:
- `max_health`
- `attack`
- `defense`
- `dropped_gold`
- `dropped_xp`

**Not scaled**: `name`, `quality`, `loot` (loot tables are cloned as-is).

## ItemId Spawning

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `spawn()` | `&self -> Item` | Spawn an item |
| `with_multiplier()` | `&self, f32 -> ItemSpawner` | Start builder with scaled stats |
| `with_name()` | `&self, impl Into<String> -> ItemSpawner` | Start builder with custom name |
| `with_quality()` | `&self, ItemQuality -> ItemSpawner` | Start builder with fixed quality |

### ItemSpawner (Builder)

| Method | Signature | Description |
|--------|-----------|-------------|
| `spawn()` | `self -> Item` | Spawn with all modifications |
| `with_multiplier()` | `self, f32 -> Self` | Chain: scale stats and gold |
| `with_name()` | `self, impl Into<String> -> Self` | Chain: change display name |
| `with_quality()` | `self, ItemQuality -> Self` | Chain: set quality |

### Examples

```rust
// Normal spawn
let sword = ItemId::Sword.spawn();

// Enchanted version of a base item
let enchanted_sword = ItemId::Sword
    .with_multiplier(1.5)
    .with_name("Enchanted Sword")
    .with_quality(ItemQuality::Masterworked)
    .spawn();

// Dungeon loot scaling
let loot_multiplier = 1.0 + (floor as f32 * 0.1);
let dungeon_drop = ItemId::BronzeSword
    .with_multiplier(loot_multiplier)
    .spawn();
```

### What Gets Scaled

`with_multiplier(factor)` scales:
- All stats in `StatSheet` (Attack, Defense, etc.)
- `gold_value`

**Not scaled**: `name`, `item_type`, `quality`, `max_upgrades`, `max_stack_quantity`.

## Use Cases

### Dungeon Scaling

```rust
fn spawn_dungeon_mob(base: MobId, floor: u32) -> Mob {
    let multiplier = 1.0 + (floor as f32 * 0.15);
    base.with_multiplier(multiplier).spawn()
}
```

### Elite/Rare Variants

```rust
fn maybe_spawn_elite(base: MobId) -> Mob {
    if rand::random::<f32>() < 0.1 {
        let spec = base.spec();
        base.with_multiplier(2.0)
            .with_name(format!("Elite {}", spec.name))
            .spawn()
    } else {
        base.spawn()
    }
}
```

### Procedural Loot

```rust
fn generate_treasure(tier: u32) -> Item {
    let base = match tier {
        0..=2 => ItemId::Dagger,
        3..=5 => ItemId::Sword,
        _ => ItemId::BronzeSword,
    };

    base.with_multiplier(1.0 + tier as f32 * 0.1)
        .with_quality(ItemQuality::Improved)
        .spawn()
}
```

## Key Files

| File | Purpose |
|------|---------|
| `src/mob/definitions.rs` | `MobSpec` proc-gen methods |
| `src/item/definitions.rs` | `ItemSpec` proc-gen methods |
| `src/mob/definition.rs` | `Mob` struct |
| `src/item/definition.rs` | `Item` struct with `item_id: ItemId` |

## Related Documentation

- [entities/mob.md](mob.md) - Base mob system
- [entities/items.md](items.md) - Base item system
- [dungeon/overview.md](../dungeon/overview.md) - Dungeon system (uses proc-gen)
