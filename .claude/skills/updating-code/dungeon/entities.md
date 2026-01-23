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
    Chest { variant: u8, size: GridSize },  // Uses chests sprite sheet (Slice_1)
    Mob { mob_id: MobId, size: GridSize },  // Any mob type (Goblin, Slime, etc.)
    Stairs { size: GridSize },              // Advances player to next floor
}
```

Each variant includes a `size: GridSize` field indicating how many grid cells the entity occupies. Access via `entity.size()`.

### Grid Size
- `GridSize::single()` - 1x1 cell (default for most entities)
- `GridSize::new(w, h)` - Custom size (e.g., 2x2 for bosses)
- Mobs get their size from `MobSpec::grid_size` (see `src/mob/definitions.rs`)

## Entity Types

### Static Entities (Chest)
- Use `SpriteSheetKey::Chests` and `GameSprites` for rendering
- `sprite_sheet_key()` returns `SpriteSheetKey::Chests`
- `sprite_name()` returns `"Slice_1"` (all variants use the same slice)
- Rendered directly as `ImageNode` in dungeon.rs
- Sprite sheet: `assets/sprites/dungeon_entities/chests.{png,json}` (128x96, 43 slices)

### Animated Entities (Mob)
- Use marker component pattern for decoupled rendering
- Spawn with `DungeonMobSprite { mob_id }` marker
- `populate_dungeon_mob_sprites()` system populates sprite + animation
- Reuses `MobSpriteSheets` and `MobAnimation` from mob compendium

### Stairs Entity
- Uses `SpriteSheetKey::DungeonTileset` with slice `"stairs"` (`DungeonTileSlice::Stairs`)
- Always 1x1 (`GridSize::single()`)
- On collision: inserts `AdvanceFloor` resource, triggering `advance_floor_system`
- `advance_floor_system` despawns current dungeon UI, increments `floor_index`, calls `load_floor_layout()`, and respawns the dungeon screen with a fresh layout
- Spawned via `SpawnTable::stairs(count_range)` (e.g., `.stairs(1..=1)`)

## Chest Interaction (`src/ui/screens/dungeon/plugin.rs`)

### Behavior
- Chests block movement (player stops when colliding)
- When player is adjacent to a chest and presses Space (`GameAction::Mine`), the chest opens
- Loot is rolled from the chest's `LootTable` via `HasLoot::roll_drops(magic_find)`
- Loot is added to player inventory via `collect_loot_drops()`
- Chest is despawned from the grid (occupancy vacated, entity despawned)
- Results modal shows "Chest Opened!" with loot items

### Key System: `handle_chest_interaction`
- Gated on `in_state(AppState::Dungeon)` and only runs when no modal is open
- Listens for `GameAction::Mine` (Space key)
- Uses `find_adjacent_chest()` helper for adjacency detection

### Adjacency Detection: `find_adjacent_chest()`
For player at `(px, py)` with size `(w, h)`, checks border cells:
- Top row: `(px..px+w, py-1)`
- Bottom row: `(px..px+w, py+h)`
- Left column: `(px-1, py..py+h)`
- Right column: `(px+w, py..py+h)`

Each cell is checked in `GridOccupancy` for an entity, then the entity is queried for `DungeonEntityMarker` to confirm it's a `DungeonEntity::Chest`.

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
Update the `size()` method to handle the new variant.

### Step 2: Add Sprite Assets
Create sprite sheet in `assets/sprites/dungeon_entities/`:
- `trap.png` - The sprite image (16x16)
- `trap.json` - Metadata with frame definitions

### Step 3: Add SpriteSheetKey
In `src/assets/sprites.rs`, add to `SpriteSheetKey` enum and `asset_name()`.

### Step 4: Update Entity Methods
In `src/dungeon/entity.rs`:
```rust
pub fn sprite_sheet_key(&self) -> SpriteSheetKey {
    match self {
        Self::Chest { .. } => SpriteSheetKey::Chests,
        Self::Mob { .. } => panic!("Mob entities use DungeonMobSprite marker"),
        Self::Trap { .. } => SpriteSheetKey::Trap,
    }
}

pub fn size(&self) -> GridSize {
    match self {
        Self::Chest { size, .. } => *size,
        Self::Mob { size, .. } => *size,
        Self::Trap { size, .. } => *size,
    }
}
```

### Step 5: Update Rendering
In `src/screens/town/tabs/dungeon.rs`, add match arm in entity rendering.

## Entity Rendering Architecture

Entities render in an **overlay layer** on top of tiles to support multi-cell sprites:

```
DungeonContainer
├── DungeonGrid (tiles only)
│   └── DungeonCell → Tile background
└── EntityOverlay (renders on top)
    ├── Player sprite
    └── Entity sprites (Chest, Mob)
```

### Why Overlay?
Without the overlay, multi-cell entities would be hidden behind neighboring tiles due to z-order (later grid cells render on top of earlier cells' overflow).

### Entity Positioning
Entities use absolute pixel positioning based on GridSize:
```rust
let size = entity.size();
overlay.spawn((
    DungeonMobSprite { mob_id },
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(pos.x as f32 * tile_size),
        top: Val::Px(pos.y as f32 * tile_size),
        width: Val::Px(tile_size * size.width as f32),
        height: Val::Px(tile_size * size.height as f32),
        ..default()
    },
));
```

### Sprite Population Flow
```
EntityOverlay                      Mob Animation (mob_animation.rs)
┌─────────────────────────┐        ┌──────────────────────────────┐
│ spawn entities:         │        │ populate_sprite_markers      │
│   Chest → ImageNode     │        │   DungeonMobSprite marker    │
│   Mob → DungeonMobSprite│───────>│   + MobSpriteSheets lookup   │
│         marker only     │        │   = ImageNode + SpriteAnim   │
└─────────────────────────┘        └──────────────────────────────┘
       No sprite knowledge              Handles sprite loading
```

### Key Components
- `DungeonMobSprite { mob_id: MobId }` - Marker component in `src/ui/mob_animation.rs`
- `populate_sprite_markers()` - Generic system that detects `Added<DungeonMobSprite>` and inserts sprite

## Spawning Entities in Layouts

**Preferred: Use SpawnTable** for declarative entity spawning:
```rust
use crate::dungeon::{LayoutBuilder, SpawnTable};
use crate::mob::MobId;

LayoutBuilder::new(40, 21)
    .entrance(20, 19)
    .exit(20, 20)
    .spawn(SpawnTable::new()
        .mob(MobId::Goblin, 3)   // Weight 3 (more common)
        .mob(MobId::Slime, 2)    // Weight 2 (less common)
        .chest(1..=2))           // 1-2 chests randomly
    .build()
```

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

Source: `16x16 Assorted RPG Icons/chests.{png,json}`

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
