# Components

Components are data containers attached to entities. In Bevy's ECS, components are simple Rust structs or enums that derive the `Component` trait.

## Quick Reference

```rust
// Marker component (zero-sized type)
#[derive(Component)]
pub struct MyMarker;

// Data component with fields
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

// Newtype component for type safety
#[derive(Component, Debug, Clone)]
pub struct GoldReward(pub i32);

// Component with methods
impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}
```

## Overview

Components serve different purposes based on their structure:

| Type | Purpose | Example |
|------|---------|---------|
| Marker | Filter entities in queries | `DungeonPlayer`, `FightModalRoot` |
| Data | Store entity state | `Health`, `CombatStats` |
| Newtype | Type-safe wrappers | `GoldReward(i32)`, `XpReward(i32)` |
| Wrapper | Contain complex types | `MobLootTable(LootTable)` |

## Patterns

### Marker Components

Zero-sized types used to tag entities for query filtering. They carry no data.

```rust
// src/ui/screens/dungeon/components.rs
#[derive(Component)]
pub struct DungeonPlayer;

#[derive(Component)]
pub struct DungeonRoot;

#[derive(Component)]
pub struct FloorRoot;
```

Use markers when you need to:
- Identify specific entity types in queries
- Filter systems to operate on specific entities
- Create hierarchical relationships (root markers)

**Query usage:**
```rust
fn system(query: Query<&Transform, With<DungeonPlayer>>) {
    for transform in &query {
        // Only entities with DungeonPlayer marker
    }
}
```

### Data Components

Structs with fields that store entity state. Include methods to encapsulate behavior.

```rust
// src/mob/components.rs
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }
}

#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub attack: i32,
    pub defense: i32,
}
```

**Best practices for data components:**
- Add methods for common operations (`take_damage`, `heal`)
- Use clamping to enforce invariants (health cannot go below 0)
- Derive `Debug` and `Clone` for debugging and copying

### Newtype Components

Tuple structs wrapping a single value. Provides type safety by distinguishing semantically different values.

```rust
// src/mob/components.rs
#[derive(Component, Debug, Clone)]
pub struct GoldReward(pub i32);

#[derive(Component, Debug, Clone)]
pub struct XpReward(pub i32);

// Both are i32, but type system prevents mixing them up
```

Use newtypes when you have multiple components of the same underlying type that serve different purposes.

**With methods:**
```rust
// src/player/definition.rs (as Resource, same pattern applies)
#[derive(Debug, Clone, Default)]
pub struct PlayerGold(pub i32);

impl PlayerGold {
    pub fn add(&mut self, amount: i32) {
        self.0 += amount;
    }

    pub fn subtract(&mut self, amount: i32) {
        self.0 = (self.0 - amount).max(0);
    }
}
```

### Wrapper Components

Contain complex types, often from other modules, to make them into components.

```rust
// src/mob/components.rs
use crate::loot::LootTable;

#[derive(Component, Debug, Clone)]
pub struct MobLootTable(pub LootTable);
```

### Marker Components with Data

Sometimes markers carry context needed for their purpose.

```rust
// src/mob/components.rs
#[derive(Component, Debug, Clone, Copy)]
pub struct MobMarker(pub MobId);

// src/ui/screens/fight_modal/state.rs
#[derive(Component)]
pub struct FightModalMobSprite {
    pub mob_id: MobId,
}
```

### Timer Components

Components that track time-based state.

```rust
// src/crafting_station/mod.rs
#[derive(Component)]
pub struct ForgeActiveTimer(pub Timer);

#[derive(Component)]
pub struct AnvilActiveTimer(pub Timer);
```

### Flag Components with Default

For boolean state that needs explicit initialization.

```rust
// src/mob/components.rs
#[derive(Component, Debug, Clone, Default)]
pub struct DeathProcessed(pub bool);
```

### Tile Property Components (Macro Pattern)

For repetitive component definitions, use macros.

```rust
// src/dungeon/tile_components.rs
macro_rules! tile_property {
    ($name:ident) => {
        #[derive(Component, Reflect, Default)]
        #[reflect(Component, Default, type_path = false)]
        pub struct $name(pub bool);

        impl TypePath for $name {
            fn type_path() -> &'static str { stringify!($name) }
            fn short_type_path() -> &'static str { stringify!($name) }
        }
    };
}

tile_property!(is_solid);
tile_property!(can_have_entity);
tile_property!(can_spawn_player);
tile_property!(is_door);
```

## Examples

### Complete Component Module

```rust
// src/mob/components.rs
use bevy::prelude::*;
use crate::loot::LootTable;
use super::MobId;

/// Marker component identifying a mob entity and its type.
#[derive(Component, Debug, Clone, Copy)]
pub struct MobMarker(pub MobId);

/// Health component for entities that can take damage.
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}

/// Combat stats for attack and defense calculations.
#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub attack: i32,
    pub defense: i32,
}

/// Gold reward dropped when this entity dies.
#[derive(Component, Debug, Clone)]
pub struct GoldReward(pub i32);

/// XP reward given when this entity dies.
#[derive(Component, Debug, Clone)]
pub struct XpReward(pub i32);

/// Loot table for item drops on death.
#[derive(Component, Debug, Clone)]
pub struct MobLootTable(pub LootTable);
```

## Common Mistakes

### Using Strings Instead of Newtypes

```rust
// Bad: Easy to confuse different string purposes
#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Description(pub String);

// Better: Named newtypes make intent clear
#[derive(Component)]
pub struct PlayerName(pub String);

#[derive(Component)]
pub struct ItemDescription(pub String);
```

### Forgetting Debug Derive

```rust
// Bad: Can't print for debugging
#[derive(Component)]
pub struct Health { current: i32, max: i32 }

// Good: Can inspect in logs
#[derive(Component, Debug)]
pub struct Health { current: i32, max: i32 }
```

### Logic Outside Component Methods

```rust
// Bad: Logic scattered in systems
fn damage_system(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.current = (health.current - 10).max(0);
    }
}

// Good: Logic encapsulated in component
fn damage_system(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.take_damage(10);
    }
}
```

### Overly Large Components

```rust
// Bad: Monolithic component
#[derive(Component)]
pub struct Mob {
    health: i32, max_health: i32,
    attack: i32, defense: i32,
    gold: i32, xp: i32,
    loot_table: LootTable,
}

// Good: Separate concerns into focused components
#[derive(Component)]
pub struct Health { current: i32, max: i32 }

#[derive(Component)]
pub struct CombatStats { attack: i32, defense: i32 }

#[derive(Component)]
pub struct GoldReward(pub i32);
```

### Storing Derived Data

```rust
// Bad: Storing computed values that can become stale
#[derive(Component)]
pub struct Health {
    current: i32,
    max: i32,
    percent: f32,  // Gets out of sync when current/max change
}

// Good: Compute derived values on demand
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn percent(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}
```
