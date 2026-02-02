# Dungeon Entities

Entities that can be spawned on dungeon tiles.

## Core Concepts

### Entity Spawning Rules
- Only tiles where `TileType::can_spawn_entity()` returns `true` can have entities
- Currently: `Floor` and `DoorOpen` tiles allow spawning
- `Entrance`, `Exit`, `Wall`, and `Door` do not allow entity spawning

### Component-Based Entity Types (`src/dungeon/entity.rs`)

Entity types are represented as distinct marker components rather than an enum:

```rust
#[derive(Component)]
pub struct DungeonEntityMarker {
    pub pos: Vec2,
    pub size: EntitySize,
}

#[derive(Component)]
pub struct ChestEntity { pub variant: u8 }

#[derive(Component)]
pub struct RockEntity { pub rock_type: RockType, pub sprite_variant: u8 }

#[derive(Component)]
pub struct StairsEntity;

#[derive(Component)]
pub struct MobEntity { pub mob_id: MobId }

#[derive(Component)]
pub struct NpcEntity { pub mob_id: MobId }

#[derive(Component)]
pub struct CraftingStationEntity { pub station_type: CraftingStationType }

#[derive(Component)]
pub struct DoorEntity;
```

All entities have a `DungeonEntityMarker` component with position and size. The specific entity type is determined by which additional marker component is present.

### Benefits of Component-Based Design
- No exhaustive match statements required
- Adding a new entity type doesn't require updating every file
- ECS queries automatically filter by component presence
- Better separation of concerns

### Grid Size
- All entities are 1x1 in the logical grid
- Visual sprite size is controlled independently (mobs render at 2x tile size, static entities at 1x)

## Entity Rendering (`src/ui/screens/dungeon/spawn.rs`)

The `add_entity_visuals` observer triggers on `Add<DungeonEntityMarker>` and uses component queries to determine how to render each entity:

```rust
pub fn add_entity_visuals(
    trigger: On<Add, DungeonEntityMarker>,
    chest_query: Query<&ChestEntity>,
    rock_query: Query<&RockEntity>,
    stairs_query: Query<(), With<StairsEntity>>,
    // ... other queries
) {
    if let Ok(_chest) = chest_query.get(entity) {
        // Render chest sprite
    }
    if let Ok(rock) = rock_query.get(entity) {
        // Render rock sprite based on rock_type
    }
    // ... etc
}
```

Adding a new entity type requires:
1. Adding a new component struct in `entity.rs`
2. Adding a query in `add_entity_visuals`
3. Adding rendering logic for that component

## Entity Types

### Chest Entity
- Component: `ChestEntity { variant: u8 }`
- Sprite: `SpriteSheetKey::Chests`, "Slice_1"
- Collision: Static entity layer, blocks movement

### Rock Entity
- Component: `RockEntity { rock_type: RockType, sprite_variant: u8 }`
- Four types: `RockType::Coal`, `RockType::Copper`, `RockType::Iron`, `RockType::Gold`
- Sprites from cave tileset with variants (0 or 1):
  - Coal: `rock_1` or `rock_2`
  - Copper: `copper_rock_1` or `copper_rock_2`
  - Iron: `iron_rock_1` or `iron_rock_2`
  - Gold: `gold_rock_1` or `gold_rock_2`
- Blocks movement, mined via Space key when adjacent
- Rock module: `src/rock/` (definition, enums, traits)

### Stairs Entity
- Component: `StairsEntity` (unit struct)
- Sprite: `SpriteSheetKey::DungeonTileset`, "stairs"
- Uses Sensor collider (trigger layer)
- On collision: triggers `FloorTransition::AdvanceFloor`

### Door Entity
- Component: `DoorEntity` (unit struct)
- No sprite (invisible) - door visual is the cave opening tile in the tilemap
- Uses Sensor collider (trigger layer)
- On collision: triggers `FloorTransition::EnterDoor`
- Spawned automatically at tiles with `is_door` property

### Mob Entity
- Component: `MobEntity { mob_id: MobId }`
- Rendered as animated mob sprite via `MobSpriteSheets`
- Uses mob collision layer
- On collision: triggers combat

### NPC Entity
- Component: `NpcEntity { mob_id: MobId }`
- Rendered same as mobs (animated sprite)
- Uses mob collision layer but no combat triggered
- Interaction via Space key when adjacent

### Crafting Station Entity
- Component: `CraftingStationEntity { station_type: CraftingStationType }`
- Two types: `Forge` and `Anvil`
- Static sprites from `SpriteSheetKey::CraftingStations`
- Forge has custom collider offset

## Collision Handling (`src/dungeon/systems/movement.rs`)

Collisions are handled via component queries:

```rust
pub fn handle_player_collisions(
    mob_query: Query<&MobEntity>,
    stairs_query: Query<(), With<StairsEntity>>,
    door_entity_query: Query<(), With<DoorEntity>>,
    // ...
) {
    if let Ok(mob) = mob_query.get(other) {
        // Trigger combat with mob.mob_id
    }
    if stairs_query.get(other).is_ok() {
        // Advance floor
    }
    // ...
}
```

## Interaction Handling (`src/ui/screens/dungeon/plugin.rs`)

The `handle_interact_action` system checks adjacent entities:

```rust
fn handle_interact_action(
    npc_query: Query<&NpcEntity>,
    crafting_query: Query<&CraftingStationEntity>,
    chest_query: Query<(), With<ChestEntity>>,
    rock_query: Query<&RockEntity>,
    // ...
) {
    if let Ok(npc) = npc_query.get(entity) {
        // Handle NPC interaction
    }
    if chest_query.get(entity).is_ok() {
        // Mine chest
    }
    // ...
}
```

## Mining Events

Mining uses a `MineableEntityType` enum for event payloads:

```rust
pub enum MineableEntityType {
    Chest,
    Rock { rock_type: RockType },
}

pub struct MineEntity {
    pub entity: Entity,
    pub pos: Vec2,
    pub mineable_type: MineableEntityType,
}
```

## DungeonCommands Extension Trait (`src/dungeon/commands.rs`)

Commands extension for dungeon entity lifecycle operations.

### Usage
```rust
use crate::dungeon::DungeonCommands;

commands.despawn_dungeon_entity(entity_id);
```

## Adding New Entity Types

### Step 1: Add Component
In `src/dungeon/entity.rs`:
```rust
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TrapEntity {
    pub trap_type: TrapType,
}
```

### Step 2: Export Component
In `src/dungeon/mod.rs`, add to exports:
```rust
pub use entity::{..., TrapEntity};
```

### Step 3: Update Spawning
In `src/dungeon/systems/spawning.rs`, add spawn function using `spawn_n_entities`:
```rust
fn spawn_traps(...) {
    spawn_n_entities(commands, count, available, used, ctx, rng, |rng| {
        TrapEntity { trap_type: TrapType::Spike }
    });
}
```

### Step 4: Add Rendering
In `src/ui/screens/dungeon/spawn.rs`:
1. Add query parameter: `trap_query: Query<&TrapEntity>`
2. Add rendering logic in `add_entity_visuals`:
```rust
if let Ok(trap) = trap_query.get(entity) {
    add_static_sprite(..., SpriteSheetKey::Traps, trap.trap_type.sprite_name(), ...);
}
```

### Step 5: Add Collision/Interaction (if needed)
In the appropriate system, add a query and handling logic.

## Spawning Entities

Spawn tables are defined in `FloorSpec` (fixed floors) or `FloorType` (generated floors).

### SpawnTable API
```rust
SpawnTable::new()
    .mob(MobId::Goblin, 3)
    .mob(MobId::Slime, 2)
    .mob_count(3..=5)
    .guaranteed_mob(MobId::BlackDragon, 1)
    .npc(MobId::Merchant, 1..=1)
    .chest(1..=2)
    .stairs(1..=1)
    .rock(2..=4)
    .forge_chance(0.33)
    .anvil_chance(0.33)
```

## Sprite Assets

### Static Sprites
Located in `assets/sprites/dungeon_entities/`:
- `chests.png` / `chests.json` - Chest sprites (128x96, 43 slices)

Rock sprites from `assets/sprites/cave_tileset.png` (32x32 each).

### Animated Mob Sprites
Located in `assets/sprites/mobs/` (shared with combat/compendium).

See [mob-sprites.md](../sprites/mob-sprites.md) for adding new mob sprites.

## Related
- [mod.md](mod.md) - Dungeon module overview
- [spawning.md](spawning.md) - Spawn system details
- [movement.md](movement.md) - Movement and collision
