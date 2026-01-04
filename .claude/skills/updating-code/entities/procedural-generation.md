# Procedural Generation System

## Overview

The procedural generation system allows creating modified or entirely custom entities at runtime. Both `MobSpec` and `ItemSpec` support:
- **Scaling**: Multiply stats by a factor for elite/dungeon variants
- **Naming**: Custom names for unique entities
- **Quality**: Fixed quality levels instead of random rolls
- **Spawning**: Create instances without a base `MobId`/`ItemId`

## Key Concepts

| Term | Description |
|------|-------------|
| Spec | Template defining entity properties (ranges, base stats) |
| Instance | Spawned entity with rolled/computed values |
| `Option<MobId>` | `None` for procedural mobs, `Some(id)` for registry mobs |
| `Option<ItemId>` | `None` for procedural items, `Some(id)` for registry items |

## MobSpec Procedural Generation

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `spawn()` | `&self -> Mob` | Spawn with `spec: None` |
| `spawn_with_id()` | `&self, Option<MobId> -> Mob` | Spawn with explicit id |
| `with_multiplier()` | `&self, f32 -> MobSpec` | Scale all stat ranges |
| `with_name()` | `&self, impl Into<String> -> MobSpec` | Change display name |
| `with_quality()` | `&self, MobQuality -> MobSpec` | Set quality (Normal/Boss) |

### Examples

```rust
// Elite variant: 1.5x stats with custom name
let elite_slime = MobId::Slime.spec()
    .with_multiplier(1.5)
    .with_name("Elite Slime")
    .spawn();

// Dungeon scaling: deeper floors = stronger mobs
let depth_multiplier = 1.0 + (floor as f32 * 0.2);
let scaled_goblin = MobId::Goblin.spec()
    .with_multiplier(depth_multiplier)
    .spawn_with_id(Some(MobId::Goblin));

// Boss variant of a normal mob
let mini_boss = MobId::Cow.spec()
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

## ItemSpec Procedural Generation

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `spawn()` | `&self -> Item` | Spawn with `item_id: None` |
| `spawn_with_id()` | `&self, Option<ItemId> -> Item` | Spawn with explicit id |
| `with_multiplier()` | `&self, f32 -> ItemSpec` | Scale stats and gold value |
| `with_name()` | `&self, impl Into<String> -> ItemSpec` | Change display name |
| `with_quality()` | `&self, ItemQuality -> ItemSpec` | Set fixed quality |

### Examples

```rust
// Enchanted version of a base item
let enchanted_sword = ItemId::Sword.spec()
    .with_multiplier(1.5)
    .with_name("Enchanted Sword")
    .with_quality(ItemQuality::Masterworked)
    .spawn();

// Dungeon loot scaling
let loot_multiplier = 1.0 + (floor as f32 * 0.1);
let dungeon_drop = ItemId::BronzeSword.spec()
    .with_multiplier(loot_multiplier)
    .spawn_with_id(Some(ItemId::BronzeSword));

// Fully custom item (no base ItemId)
let custom_spec = ItemSpec {
    name: String::from("Ancient Blade"),
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    quality: Some(ItemQuality::Mythic),
    stats: StatSheet::new().with(StatType::Attack, 50),
    max_upgrades: 10,
    max_stack_quantity: 1,
    gold_value: 1000,
};
let ancient_blade = custom_spec.spawn();
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

## Checking Procedural vs Registry Entities

```rust
// Check if mob is from registry or procedural
match mob.spec {
    Some(mob_id) => println!("Registry mob: {:?}", mob_id),
    None => println!("Procedural mob: {}", mob.name),
}

// Check if item is from registry or procedural
match item.item_id {
    Some(item_id) => println!("Registry item: {:?}", item_id),
    None => println!("Procedural item: {}", item.name),
}
```

## Key Files

| File | Purpose |
|------|---------|
| `src/mob/definitions.rs` | `MobSpec` proc-gen methods |
| `src/item/definitions.rs` | `ItemSpec` proc-gen methods |
| `src/mob/definition.rs` | `Mob` struct with `spec: Option<MobId>` |
| `src/item/definition.rs` | `Item` struct with `item_id: Option<ItemId>` |

## Related Documentation

- [entities/mob.md](mob.md) - Base mob system
- [entities/items.md](items.md) - Base item system
- [dungeon/overview.md](../dungeon/overview.md) - Dungeon system (uses proc-gen)
