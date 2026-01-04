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
| `spawn_modified()` | `&self, FnOnce(&MobSpec) -> MobSpec -> Mob` | Spawn with modifications |

### Spec Modifiers

| Method | Signature | Description |
|--------|-----------|-------------|
| `with_multiplier()` | `&self, f32 -> MobSpec` | Scale all stat ranges |
| `with_name()` | `&self, impl Into<String> -> MobSpec` | Change display name |
| `with_quality()` | `&self, MobQuality -> MobSpec` | Set quality (Normal/Boss) |

### Examples

```rust
// Normal spawn
let slime = MobId::Slime.spawn();

// Elite variant: 1.5x stats with custom name
let elite_slime = MobId::Slime.spawn_modified(|spec| {
    spec.with_multiplier(1.5).with_name("Elite Slime")
});

// Dungeon scaling: deeper floors = stronger mobs
let depth_multiplier = 1.0 + (floor as f32 * 0.2);
let scaled_goblin = MobId::Goblin.spawn_modified(|spec| {
    spec.with_multiplier(depth_multiplier)
});

// Boss variant of a normal mob
let mini_boss = MobId::Cow.spawn_modified(|spec| {
    spec.with_multiplier(3.0)
        .with_name("Dire Cow")
        .with_quality(MobQuality::Boss)
});
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
| `spawn_modified()` | `&self, FnOnce(&ItemSpec) -> ItemSpec -> Item` | Spawn with modifications |

### Spec Modifiers

| Method | Signature | Description |
|--------|-----------|-------------|
| `with_multiplier()` | `&self, f32 -> ItemSpec` | Scale stats and gold value |
| `with_name()` | `&self, impl Into<String> -> ItemSpec` | Change display name |
| `with_quality()` | `&self, ItemQuality -> ItemSpec` | Set fixed quality |

### Examples

```rust
// Normal spawn
let sword = ItemId::Sword.spawn();

// Enchanted version of a base item
let enchanted_sword = ItemId::Sword.spawn_modified(|spec| {
    spec.with_multiplier(1.5)
        .with_name("Enchanted Sword")
        .with_quality(ItemQuality::Masterworked)
});

// Dungeon loot scaling
let loot_multiplier = 1.0 + (floor as f32 * 0.1);
let dungeon_drop = ItemId::BronzeSword.spawn_modified(|spec| {
    spec.with_multiplier(loot_multiplier)
});
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
    base.spec()
        .with_multiplier(multiplier)
        .spawn_with_id(Some(base))
}
```

### Elite/Rare Variants

```rust
fn maybe_make_elite(mob: &MobSpec) -> MobSpec {
    if rand::random::<f32>() < 0.1 {
        mob.with_multiplier(2.0)
           .with_name(format!("Elite {}", mob.name))
    } else {
        mob.clone()
    }
}
```

### Procedural Loot

```rust
fn generate_treasure(tier: u32) -> Item {
    let base = match tier {
        0..=2 => ItemId::Dagger.spec(),
        3..=5 => ItemId::Sword.spec(),
        _ => ItemId::BronzeSword.spec(),
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
