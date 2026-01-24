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
    Chest { variant: u8, size: GridSize },        // Uses chests sprite sheet (Slice_1)
    Mob { mob_id: MobId, size: GridSize },        // Any mob type (Goblin, Slime, etc.)
    Stairs { size: GridSize },                    // Advances player to next floor
    Rock { rock_type: RockType, size: GridSize }, // Minable rocks (copper, coal, tin)
}
```

Each variant includes a `size: GridSize` field indicating how many grid cells the entity occupies. Access via `entity.size()`.

### Grid Size
- All entities are 1x1 in the logical grid (`GridSize::single()`)
- Visual sprite size is controlled independently via `ENTITY_VISUAL_SCALE` (mobs render at 2x tile size, static entities at 1x)
- `GridSize::new(w, h)` exists for future multi-cell entities but is not currently used

## EntityRenderData (`src/dungeon/entity.rs`)

All entity rendering is driven by `DungeonEntity::render_data()`, which returns an `EntityRenderData` enum:

```rust
pub enum EntityRenderData {
    /// Static sprite from a named sprite sheet (Chests, Rocks, DungeonTileset).
    SpriteSheet {
        sheet_key: SpriteSheetKey,
        sprite_name: &'static str,
    },
    /// Animated mob sprite using the SpriteMarker observer system.
    AnimatedMob { mob_id: MobId },
}
```

The render loop in `render_dungeon_floor()` uses this to spawn entities with only 2 match arms:
- `SpriteSheet` → looks up `game_sprites.get(sheet_key)` and spawns an `ImageNode`
- `AnimatedMob` → spawns a `DungeonMobSprite { mob_id }` marker (populated by observer)

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
- `render_data()` returns `SpriteSheet { Rocks, rock_type.sprite_name() }`
- Three types: `RockType::Copper` ("copper_rock"), `RockType::Coal` ("coal_tin_rock"), `RockType::Tin` ("coal_tin_rock")
- Coal and Tin share the same sprite (icon 859), Copper uses a different one (icon 858)
- Always 1x1 (`GridSize::single()`)
- Blocks movement (obstacles, like chests)
- Mined via Space key when adjacent → shows results modal with ore drops
- Rock module: `src/rock/` (definition, enums, traits)
- `Rock::new(rock_type)` creates a rock with appropriate loot table
- Implements `HasLoot` trait for `roll_drops(magic_find)`
- Spawned via `SpawnTable::rock(count_range)` (e.g., `.rock(2..=4)`)
- Loot: Copper → CopperOre (1-3), Coal → Coal (1-2), Tin → TinOre (1-3)

### Stairs Entity
- `render_data()` returns `SpriteSheet { DungeonTileset, "stairs" }`
- Always 1x1 (`GridSize::single()`)
- On collision: inserts `AdvanceFloor` resource, triggering `advance_floor_system`
- `advance_floor_system` despawns current dungeon UI, increments `floor_index`, calls `load_floor_layout()`, and respawns the dungeon screen with a fresh layout
- Spawned via `SpawnTable::stairs(count_range)` (e.g., `.stairs(1..=1)`)

## DungeonCommands Extension Trait (`src/dungeon/commands.rs`)

Commands extension for dungeon entity lifecycle operations. Follows the same pattern as `ModalCommands` in `src/ui/modal_registry.rs`.

### Usage
```rust
use crate::dungeon::DungeonCommands;

// Despawn entity and vacate its occupancy grid cells
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

## Spawning Entities in Layouts

**Important:** Spawn tables are applied via `LayoutBuilder::spawn()` inside each layout's `create()` function (e.g., `src/dungeon/layouts/starting_room.rs`). The `FloorSpec.spawn_table` field in `src/dungeon/floor/definitions.rs` is currently **not applied at runtime** — only the spawn table passed to `LayoutBuilder` takes effect.

**Preferred: Use SpawnTable** for declarative entity spawning:
```rust
use crate::dungeon::{LayoutBuilder, SpawnTable};
use crate::mob::MobId;

LayoutBuilder::new(40, 21)
    .entrance(20, 19)
    .exit(20, 20)
    .spawn(SpawnTable::new()
        .mob(MobId::Goblin, 3)        // Weight 3 (more common)
        .mob(MobId::Slime, 2)         // Weight 2 (less common)
        .mob_count(3..=5)             // 3-5 total weighted mobs
        .guaranteed_mob(MobId::BlackDragon, 1)  // Exactly 1, always spawned
        .chest(1..=2)                 // 1-2 chests randomly
        .rock(2..=4))                 // 2-4 rocks randomly (random type)
    .build()
```

### Guaranteed vs Weighted Mobs

- `.mob(id, weight)` + `.mob_count(range)` — spawns N mobs chosen randomly by weight
- `.guaranteed_mob(id, count)` — spawns exactly `count` of this mob (before weighted selection), guaranteed every time

**Manual spawning** (for special cases):
```rust
use crate::dungeon::{DungeonEntity, GridSize};
use crate::mob::MobId;

// Specific positions (e.g., boss placement)
let mob_id = MobId::Dragon;
let size = mob_id.spec().grid_size;
layout.add_entity(20, 10, DungeonEntity::Mob { mob_id, size });

// Random positions with shuffle
let mut spawn_points = layout.spawn_points();
spawn_points.shuffle(&mut rng);
for (x, y) in spawn_points.into_iter().take(3) {
    layout.add_entity(x, y, DungeonEntity::Chest {
        variant: rng.gen_range(0..4),
        size: GridSize::single(),
    });
}
```

## Sprite Assets

### Static Sprites
Located in `assets/sprites/dungeon_entities/`:
- `chests.png` / `chests.json` - Full chests sprite sheet (128x96, 43 slices)
  - Currently uses `Slice_1` (16x14 brown chest at x:32, y:1)
  - Other slices available for future chest variants
- `rocks.png` / `rocks.json` - Rock sprite sheet (64x32, 2 slices)
  - `copper_rock` - Copper rock icon (icon858.png, x:0, y:0, 32x32)
  - `coal_tin_rock` - Coal/tin rock icon (icon859.png, x:32, y:0, 32x32)

Source (chests): `16x16 Assorted RPG Icons/chests.{png,json}`
Source (rocks): `icons_8.13.20/fullcolor/individual_32x32/icon858.png`, `icon859.png`

### Animated Mob Sprites
Located in `assets/sprites/mobs/` (shared with combat/compendium):
- `goblin.png` - 27 frames, idle 0-3
- `slime.png` - 18 frames, idle 0-3
- `dragon.png` - 66 frames, idle 0-3

See [mob-sprites.md](../mob-sprites.md) for adding new mob sprites.

## Related
- [mod.md](mod.md) - Dungeon module overview
- [ui.md](ui.md) - Dungeon tab rendering
- [../mob-sprites.md](../mob-sprites.md) - Mob sprite system
