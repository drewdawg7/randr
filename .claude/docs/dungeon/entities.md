# Dungeon Entities

Entities that can be spawned on dungeon tiles.

## Core Concepts

### Entity Spawning Rules
- Only tiles where `TileType::can_spawn_entity()` returns `true` can have entities
- Currently: `Floor` and `DoorOpen` tiles allow spawning
- `Entrance`, `Exit`, `Wall`, and `Door` do not allow entity spawning

### DungeonEntity Enum (`src/dungeon/entity.rs`)
```rust
pub enum DungeonEntity {
    Chest { variant: u8, size: EntitySize },
    Mob { mob_id: MobId, size: EntitySize },
    Npc { mob_id: MobId, size: EntitySize },
    Stairs { size: EntitySize },
    Rock { rock_type: RockType, sprite_variant: u8, size: EntitySize },
    CraftingStation { station_type: CraftingStationType, size: EntitySize },
    Door { size: EntitySize },
}
```

Each variant includes a `size: EntitySize` field indicating how many grid cells the entity occupies. Access via `entity.size()`.

### Grid Size
- All entities are 1x1 in the logical grid (`GridSize::single()`)
- Visual sprite size is controlled independently via `ENTITY_VISUAL_SCALE` (mobs render at 2x tile size, static entities at 1x)
- `GridSize::new(w, h)` exists for future multi-cell entities but is not currently used

## EntityRenderData (`src/dungeon/entity.rs`)

All entity rendering is driven by `DungeonEntity::render_data()`, which returns an `EntityRenderData` enum:

```rust
pub enum EntityRenderData {
    SpriteSheet {
        sheet_key: SpriteSheetKey,
        sprite_name: &'static str,
    },
    AnimatedMob { mob_id: MobId },
    Invisible,
}
```

The render loop in `add_entity_visuals()` uses this to spawn entities:
- `SpriteSheet` → looks up sprite and spawns with collider + RigidBody
- `AnimatedMob` → spawns animated mob sprite with collider + RigidBody
- `Invisible` → spawns only Transform + Collider + Sensor (no visual)

Adding a new entity type only requires:
1. Adding the variant to `DungeonEntity`
2. Adding a case to `render_data()`

## Entity Types

### Static Entities (Chest)
- `render_data()` returns `SpriteSheet { Chests, "Slice_1" }`
- Sprite sheet: `assets/sprites/dungeon_entities/chests.{png,json}` (128x96, 43 slices)

### Animated Entities (Mob)
- `render_data()` returns `AnimatedMob { mob_id }`
- `DungeonMobSprite` marker triggers `populate_sprite_markers()` observer
- Reuses `MobSpriteSheets` and `MobAnimation` from mob compendium

### Rock Entity
- `render_data()` returns `SpriteSheet { CaveTileset, rock_type.sprite_data(sprite_variant) }`
- Four types: `RockType::Coal`, `RockType::Copper`, `RockType::Iron`, `RockType::Gold`
- Sprites from cave tileset with random variants:
  - Coal: `rock_1` or `rock_2`
  - Copper: `copper_rock_1` or `copper_rock_2`
  - Iron: `iron_rock_1` or `iron_rock_2`
  - Gold: `gold_rock_1` or `gold_rock_2`
- `DungeonEntity::Rock { rock_type, sprite_variant, size }` - variant is 0 or 1, randomly selected at spawn
- Always 1x1 (`GridSize::single()`)
- Blocks movement (obstacles, like chests)
- Mined via Space key when adjacent → shows results modal with ore drops
- Rock module: `src/rock/` (definition, enums, traits)
- `Rock::new(rock_type)` creates a rock with appropriate loot table
- Implements `HasLoot` trait for `roll_drops(magic_find)`
- Spawned via `SpawnTable::rock(count_range)` (e.g., `.rock(2..=4)`)
- Loot: Coal → Coal (1-2), Copper → CopperOre (1-3), Iron → IronOre (1-3), Gold → GoldOre (1-3)

### Stairs Entity
- `render_data()` returns `SpriteSheet { DungeonTileset, "stairs" }`
- Always 1x1 (`GridSize::single()`)
- On collision: triggers `FloorTransition::AdvanceFloor`
- Spawned via `SpawnTable::stairs(count_range)` (e.g., `.stairs(1..=1)`)

### Door Entity
- `render_data()` returns `Invisible` (no sprite, collision detection only)
- The door visual is the cave opening tile in the tilemap itself
- Spawned automatically at tiles with `is_door` property
- Uses `Sensor` collider so player can walk through
- On collision: triggers `FloorTransition::EnterDoor`
- Spawned by `spawn_doors()` in `on_map_created` observer

### NPC Entity
- `render_data()` returns `AnimatedMob { mob_id }` (reuses mob sprite system)
- Uses any `MobId` that has a registered sprite sheet in `MobSpriteSheets`
- Always 1x1 (`GridSize::single()`)
- Blocks movement like Chest/Rock (no combat triggered)
- Spawned via `SpawnTable::npc(mob_id, count_range)` (e.g., `.npc(MobId::Merchant, 1..=1)`)
- Collision handling in `handle_dungeon_movement()` treats NPCs same as Chest/Rock (empty match arm)
- Current NPCs: `MobId::Merchant` (sprite: `merchant.png`, 23x1 grid of 32x32, idle frames 0-3)

## DungeonCommands Extension Trait (`src/dungeon/commands.rs`)

Commands extension for dungeon entity lifecycle operations. Follows the same pattern as `ModalCommands` in `src/ui/modal_registry.rs`.

### Usage
```rust
use crate::dungeon::DungeonCommands;

commands.despawn_dungeon_entity(entity_id, entity_pos, GridSize::single());
```

### How It Works
- Queues a `DespawnDungeonEntity` command that runs during command application
- Safely gets `GridOccupancy` resource and calls `vacate(pos, size)`
- Safely gets entity and calls `despawn_recursive()`
- Guards against missing resource or already-despawned entities

### Benefits
- Systems no longer need `ResMut<GridOccupancy>` just for despawning (can use `Res<GridOccupancy>`)
- Impossible to forget vacating occupancy when despawning
- Single call replaces two-step pattern:
  ```rust
  // Before:
  occupancy.vacate(entity_pos, GridSize::single());
  commands.entity(entity_id).despawn_recursive();

  // After:
  commands.despawn_dungeon_entity(entity_id, entity_pos, GridSize::single());
  ```

### When to Use
Use `despawn_dungeon_entity` whenever removing a dungeon entity from the grid:
- Mining rocks
- Opening chests
- Defeating mobs (future)
- Opening doors (future)
- Triggering traps (future)

## Mine Interaction (`src/ui/screens/dungeon/plugin.rs`)

Unified handler for mining/opening adjacent entities (chests and rocks).

### Behavior
- Chests and rocks block movement (player stops when colliding)
- When player is adjacent to a chest or rock and presses Space (`GameAction::Mine`):
  - Loot is rolled via `HasLoot::roll_drops(magic_find)`
  - Loot is added to player inventory via `collect_loot_drops()`
  - Entity is despawned via `commands.despawn_dungeon_entity()` (vacates occupancy + despawns)
  - Results modal shows loot items

### Key System: `handle_mine_interaction`
- Gated on `in_state(AppState::Dungeon)` and only runs when no modal is open
- Listens for `GameAction::Mine` (Space key)
- Uses `find_adjacent_minable()` helper for adjacency detection
- Matches on entity type to determine loot source and modal title:
  - `DungeonEntity::Chest` → "Chest Opened!"
  - `DungeonEntity::Rock { rock_type, .. }` → "{type} Rock Mined!"

### Adjacency Detection: `find_adjacent_minable()`
For the 1x1 player at `(px, py)`, checks 4 cardinal neighbors:
- Up: `(px, py-1)`
- Down: `(px, py+1)`
- Left: `(px-1, py)`
- Right: `(px+1, py)`

Each cell is checked in `GridOccupancy` for an entity, then the entity is queried for `DungeonEntityMarker` to confirm it's a `DungeonEntity::Chest` or `DungeonEntity::Rock`.

Returns `(Entity, GridPosition, DungeonEntity)` tuple.

### Chest Definition (`src/chest/`)
```rust
pub struct Chest {
    pub loot: LootTable,
    pub is_locked: bool,
}
```
- `Default::default()` creates a chest with a predefined loot table
- Implements `HasLoot` trait for `roll_drops(magic_find)`
- Default loot: GoldRing (1/3), BronzeChestplate (1/4), BronzeIngot (1/2), QualityUpgradeStone (1/3), BasicHPPotion (1/1)

### Rock Definition (`src/rock/`)
```rust
pub struct Rock {
    pub rock_type: RockType,
    pub loot: LootTable,
}
```
- `Rock::new(rock_type)` creates a rock with type-specific loot table
- Implements `HasLoot` trait for `roll_drops(magic_find)`
- Loot tables: Copper → CopperOre (1/1, qty 1-3), Coal → Coal (1/1, qty 1-2), Tin → TinOre (1/1, qty 1-3)

## Adding New Mob Types

Mobs that already exist in `MobSpriteSheets` (see `src/ui/mob_animation.rs`) automatically work in dungeons. Just spawn them:

```rust
use crate::dungeon::GridPosition;

let mob_id = MobId::Dragon;
let size = mob_id.spec().grid_size;
layout.add_entity(GridPosition::new(x, y), DungeonEntity::Mob { mob_id, size });
```

The `DungeonMobSprite` marker and `populate_dungeon_mob_sprites` system handle the rest.

## Adding Static Entity Types

### Step 1: Add Enum Variant
In `src/dungeon/entity.rs`:
```rust
pub enum DungeonEntity {
    Chest { variant: u8, size: GridSize },
    Mob { mob_id: MobId, size: GridSize },
    Trap { variant: u8, size: GridSize },  // New static entity
}
```
Update `size()` and `render_data()` methods.

### Step 2: Add Sprite Assets
Create sprite sheet in `assets/sprites/dungeon_entities/`:
- `trap.png` - The sprite image (16x16)
- `trap.json` - Metadata with frame definitions

### Step 3: Add SpriteSheetKey
In `src/assets/sprites.rs`, add to `SpriteSheetKey` enum and `asset_name()`.

### Step 4: Add render_data() Arm
In `src/dungeon/entity.rs`, add the new variant to `render_data()`:
```rust
pub fn render_data(&self) -> EntityRenderData {
    match self {
        // ...existing arms...
        Self::Trap { .. } => EntityRenderData::SpriteSheet {
            sheet_key: SpriteSheetKey::Trap,
            sprite_name: "trap_sprite",
        },
    }
}
```
No changes needed in the renderer — it already handles all `SpriteSheet` variants generically.

## Entity Rendering Architecture

Entities render within the **EntityLayer** (absolute-positioned overlay):

```
DungeonContainer
├── DungeonGrid (CSS Grid — tiles only)
└── EntityLayer (position: absolute)
    ├── Entity sprites (absolute left/top + ZIndex by Y)
    └── Player sprite (absolute left/top + ZIndex(y + 100))
```

### Z-Order Strategy
- Entities use `ZIndex(y)` so lower entities render on top of higher ones
- Player uses `ZIndex(player_pos.y + 100)` to always render above entities

### Entity Positioning
Entities use absolute pixel positioning within EntityLayer:
```rust
let visual_size = match entity.render_data() {
    EntityRenderData::AnimatedMob { .. } => ENTITY_VISUAL_SCALE * tile_size,
    _ => tile_size,
};
let offset = -(visual_size - tile_size) / 2.0;

layer.spawn((
    DungeonEntityMarker { pos: *pos, entity_type: *entity },
    z_for_entity(pos.y),
    image_node,
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(pos.x as f32 * tile_size + offset),
        top: Val::Px(pos.y as f32 * tile_size + offset),
        width: Val::Px(visual_size),
        height: Val::Px(visual_size),
        ..default()
    },
));
```

### Sprite Population Flow
```
DungeonGrid (on_add_dungeon_floor)    Mob Animation (mob_animation.rs)
┌─────────────────────────────┐       ┌──────────────────────────────┐
│ spawn entities:             │       │ populate_sprite_markers      │
│   SpriteSheet → ImageNode   │       │   DungeonMobSprite marker    │
│   AnimatedMob →             │──────>│   + MobSpriteSheets lookup   │
│     DungeonMobSprite marker │       │   = ImageNode + SpriteAnim   │
└─────────────────────────────┘       └──────────────────────────────┘
       Uses EntityRenderData               Handles sprite loading
```

### Key Components
- `DungeonMobSprite { mob_id: MobId }` - Marker component in `src/ui/mob_animation.rs`
- `populate_sprite_markers()` - Generic system that detects `Added<DungeonMobSprite>` and inserts sprite

## Spawning Entities

Spawn tables are defined in `FloorSpec` (fixed floors) or `FloorType` (generated floors), not in layouts. `DungeonState::load_floor_layout()` applies the spawn table after loading the layout.

```
Layout      = physical structure (walls, tiles, entrances, torches)
SpawnTable  = what entities spawn (mobs, chests, NPCs, stairs)
FloorType   = Layout + SpawnTable combined
```

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

### Guaranteed vs Weighted Mobs

- `.mob(id, weight)` + `.mob_count(range)` — spawns N mobs chosen randomly by weight
- `.guaranteed_mob(id, count)` — spawns exactly `count` of this mob, guaranteed every time

## Sprite Assets

### Static Sprites
Located in `assets/sprites/dungeon_entities/`:
- `chests.png` / `chests.json` - Full chests sprite sheet (128x96, 43 slices)
  - Currently uses `Slice_1` (16x14 brown chest at x:32, y:1)
  - Other slices available for future chest variants

Rock sprites from `assets/sprites/cave_tileset.png` (32x32 each):
- `rock_1`, `rock_2` - Plain rocks (for Coal)
- `copper_rock_1`, `copper_rock_2` - Copper rocks
- `iron_rock_1`, `iron_rock_2` - Iron rocks
- `gold_rock_1`, `gold_rock_2` - Gold rocks

Source (chests): `16x16 Assorted RPG Icons/chests.{png,json}`

### Animated Mob Sprites
Located in `assets/sprites/mobs/` (shared with combat/compendium):
- `goblin.png` - 6x6 grid of 32x32, idle 0-3
- `slime.png` - 8x6 grid of 32x32, idle 0-3
- `dragon.png` - 66x1 grid of 64x32, idle 0-3
- `black_dragon.png` - 16x7 grid of 64x32, idle 2-5
- `merchant.png` - 23x1 grid of 32x32, idle 0-3 (NPC only, no combat)

See [mob-sprites.md](../mob-sprites.md) for adding new mob sprites.

## Related
- [mod.md](mod.md) - Dungeon module overview
- [ui.md](ui.md) - Dungeon tab rendering
- [../mob-sprites.md](../mob-sprites.md) - Mob sprite system
