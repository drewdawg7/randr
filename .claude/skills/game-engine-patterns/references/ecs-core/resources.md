# Resources

## Quick Reference

```rust
// Derive the Resource trait
#[derive(Resource)]
pub struct MyResource { pub value: i32 }

// With Default for init_resource
#[derive(Resource, Default)]
pub struct MyDefaultResource(pub i32);

// Plugin initialization
app.init_resource::<MyDefaultResource>()     // Uses Default::default()
   .insert_resource(MyResource { value: 42 }); // Explicit value

// System access
fn my_system(
    res: Res<MyResource>,                   // Immutable access
    mut res_mut: ResMut<MyResource>,        // Mutable access
    opt: Option<Res<MyResource>>,           // Optional (may not exist)
    opt_mut: Option<ResMut<MyResource>>,    // Optional mutable
) { }

// Runtime insertion/removal
commands.insert_resource(MyResource { value: 1 });
commands.remove_resource::<MyResource>();

// Run condition
.run_if(resource_exists::<MyResource>)
```

## Overview

Resources are globally-unique singleton data in Bevy's ECS. Unlike components (attached to entities), resources exist as a single instance accessible from any system. Use resources for:

- **Global state**: Player stats, game settings, current dungeon floor
- **Configuration**: Movement speed, tile sizes, spawn tables
- **Registries**: Lookups for dungeons, items, mobs
- **Runtime flags**: Active combat, active modal, pending spawns
- **Caches**: Sprite sheets, fonts, precomputed data

Resources require the `Resource` trait, typically derived. For `init_resource`, the type must also implement `Default`.

## Patterns

| Pattern | Use Case | Example |
|---------|----------|---------|
| **Configuration** | Static game settings | `MovementConfig`, `TileWorldSize` |
| **Registry** | Data lookups by ID | `DungeonRegistry`, `NavigationTable` |
| **State tracking** | Current game state | `DungeonState`, `FocusState` |
| **Runtime flag** | Temporary existence signals | `ActiveCombat`, `PendingPlayerSpawn` |
| **Asset cache** | Loaded assets | `GameSprites`, `GameFonts` |
| **Selection state** | UI selection tracking | `MenuSelection`, `CompendiumListState` |

### Initialization Methods

| Method | When to Use |
|--------|-------------|
| `init_resource::<T>()` | Type implements Default; use for most resources |
| `insert_resource(value)` | Need specific initial value or no Default impl |
| `commands.insert_resource()` | Runtime insertion from systems |

## Examples

### Configuration Resource with Default
File: `src/dungeon/state.rs`

```rust
#[derive(Resource, Clone, Copy, Debug)]
pub struct MovementConfig {
    pub tiles_per_second: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self { tiles_per_second: 6.25 }
    }
}

// In plugin:
app.init_resource::<MovementConfig>();
```

### Registry Pattern
File: `src/dungeon/plugin.rs`

```rust
#[derive(Resource, Clone, Debug)]
pub struct DungeonRegistry {
    configs: HashMap<LocationId, DungeonConfig>,
}

impl DungeonRegistry {
    pub fn config(&self, location: LocationId) -> Option<&DungeonConfig> {
        self.configs.get(&location)
    }
}

// In plugin (explicit value required - no Default):
app.insert_resource(self.registry.clone());
```

### Runtime Resource (Existence as Signal)
File: `src/combat/plugin.rs`

```rust
#[derive(Resource)]
pub struct ActiveCombat {
    pub mob_entity: Entity,
}

// Insert when combat starts:
commands.insert_resource(ActiveCombat { mob_entity: target });

// Remove when combat ends:
commands.remove_resource::<ActiveCombat>();

// Systems run only during combat:
.run_if(resource_exists::<ActiveCombat>)
```

### State Tracking Resource
File: `src/dungeon/state.rs`

```rust
#[derive(Resource, Default)]
pub struct DungeonState {
    pub current_location: Option<LocationId>,
    pub floor_index: usize,
    pub floor_sequence: Vec<FloorId>,
    pub dungeon_cleared: bool,
}

impl DungeonState {
    pub fn enter_dungeon(&mut self, location: LocationId, registry: &DungeonRegistry) {
        self.current_location = Some(location);
        self.floor_index = 0;
        // ...
    }
}
```

### SystemParam for Grouping Related Resources
File: `src/combat/plugin.rs`

```rust
#[derive(SystemParam)]
struct PlayerResources<'w> {
    gold: ResMut<'w, PlayerGold>,
    progression: ResMut<'w, Progression>,
    inventory: ResMut<'w, Inventory>,
    stats: ResMut<'w, StatSheet>,
}

fn handle_mob_death(mut player: PlayerResources, /* ... */) {
    player.gold.0 += reward;
    // Access all player resources through single parameter
}
```

### Optional Resource Access
File: `src/combat/plugin.rs`

```rust
fn process_combat(
    active_combat: Option<Res<ActiveCombat>>,  // May not exist
) {
    let Some(ref combat) = active_combat else {
        return;  // No active combat
    };
    // Use combat...
}
```

## Common Mistakes

### Forgetting Default for init_resource
```rust
// WRONG: No Default impl
#[derive(Resource)]
pub struct MyConfig { pub value: i32 }
app.init_resource::<MyConfig>(); // Compile error!

// CORRECT: Add Default
#[derive(Resource, Default)]
pub struct MyConfig { pub value: i32 }
```

### Panicking on Missing Resource
```rust
// WRONG: Panics if resource doesn't exist
fn bad_system(res: Res<MaybeResource>) { }

// CORRECT: Use Option for resources that may not exist
fn good_system(res: Option<Res<MaybeResource>>) {
    let Some(res) = res else { return; };
}
```

### Forgetting to Remove Runtime Resources
```rust
// WRONG: Resource lingers after combat ends
fn end_combat(mut commands: Commands) {
    // Forgot to remove ActiveCombat!
}

// CORRECT: Clean up runtime resources
fn end_combat(mut commands: Commands) {
    commands.remove_resource::<ActiveCombat>();
}
```

### Using Resource for Per-Entity Data
```rust
// WRONG: Should be a component
#[derive(Resource)]
pub struct MobHealth(pub i32);  // Only one mob can exist!

// CORRECT: Use component for per-entity data
#[derive(Component)]
pub struct Health(pub i32);
```

### Mutating Without ResMut
```rust
// WRONG: Res is immutable
fn bad_system(res: Res<Counter>) {
    res.0 += 1;  // Compile error!
}

// CORRECT: Use ResMut for mutation
fn good_system(mut res: ResMut<Counter>) {
    res.0 += 1;
}
```
