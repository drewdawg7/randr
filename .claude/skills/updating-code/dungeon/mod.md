# Dungeon Module

Data-driven dungeon system at `src/dungeon/`.

## File Structure
```
src/dungeon/
    mod.rs              # Re-exports
    tile.rs             # TileType, Tile
    entity.rs           # DungeonEntity enum
    grid.rs             # GridSize, GridPosition, GridOccupancy
    layout.rs           # DungeonLayout (tiles + entities)
    layout_builder.rs   # LayoutBuilder (declarative layout creation)
    generator.rs        # LayoutGenerator trait
    rendering.rs        # TileRenderer (logical -> visual)
    layouts/
        mod.rs          # LayoutId enum
        starting_room.rs
```

## Core Types

### TileType (`tile.rs`)
Logical tile types for gameplay:
```rust
pub enum TileType {
    Wall,      // Impassable
    Floor,     // Walkable
    Entrance,  // Player spawn
    Exit,      // Stairs/door
    Door,      // Closed door
    DoorOpen,  // Open door
}
```

### Tile (`tile.rs`)
```rust
pub struct Tile {
    pub tile_type: TileType,
    pub variant: u8,    // Visual variety
    pub flip_x: bool,
}
```

### Grid Types (`grid.rs`)

Types for grid-based positioning and multi-cell entity occupancy.

#### GridSize
Represents entity size in grid cells (supports multi-cell entities like bosses):
```rust
use crate::dungeon::GridSize;

let single = GridSize::single();           // 1x1 entity
let boss = GridSize::new(3, 2);            // 3x2 entity
boss.cells();                              // 6 total cells
boss.cell_offsets();                       // Iterator: (0,0), (1,0), (2,0), (0,1), (1,1), (2,1)
```

#### GridPosition
Grid position using **top-left anchor convention**:
```rust
use crate::dungeon::{GridPosition, GridSize};

let pos = GridPosition::new(5, 3);
let size = GridSize::new(2, 2);
pos.occupied_cells(size);  // Iterator: (5,3), (6,3), (5,4), (6,4)
```

**Anchor convention:** Position specifies top-left cell. Entities expand rightward (+x) and downward (+y).

#### GridOccupancy
Resource tracking which grid cells are occupied by entities:
```rust
use crate::dungeon::{GridOccupancy, GridPosition, GridSize};

let mut occupancy = GridOccupancy::new(10, 10);

let pos = GridPosition::new(2, 2);
let size = GridSize::new(3, 2);

// Check placement
occupancy.can_place(pos, size);            // true if all 6 cells free

// Occupy cells
occupancy.occupy(pos, size, entity);
occupancy.is_occupied(3, 2);               // true
occupancy.entity_at(3, 2);                 // Some(entity)

// Vacate cells
occupancy.vacate(pos, size);
```

Out-of-bounds coordinates return `false`/`None` rather than panicking.

### DungeonLayout (`layout.rs`)
2D grid of tiles with entrance/exit positions and entities:
```rust
let layout = LayoutId::StartingRoom.layout();
layout.width();           // Grid width
layout.height();          // Grid height
layout.tile_at(x, y);     // Get tile
layout.is_walkable(x, y); // Check passability
layout.is_floor(x, y);    // Check floor-like tile

// Entity methods
layout.spawn_points();    // Get all tiles where entities can spawn
layout.add_entity(x, y, entity);  // Add entity at position
layout.entity_at(x, y);   // Get entity at position (if any)
layout.entities();        // Get all entities
```

### DungeonEntity (`entity.rs`)
Entities that can be placed on tiles:
```rust
pub enum DungeonEntity {
    Chest { variant: u8 },  // variant 0-3 for visual variety
    // Future: Mob, Trap, etc.
}

// Get sprite info for rendering
entity.sprite_sheet_key() // Returns SpriteSheetKey
entity.sprite_name()      // Returns sprite name in sheet
```

See [entities.md](entities.md) for detailed entity documentation.

### LayoutBuilder (`layout_builder.rs`)
Declarative builder for creating dungeon layouts:
```rust
use crate::dungeon::LayoutBuilder;

let layout = LayoutBuilder::new(40, 21)
    .entrance(20, 19)  // Interior position (player spawn)
    .exit(20, 20)      // Bottom wall (stairs/door)
    .build();
```

**Builder behavior:**
- `new(width, height)` - Creates grid with Floor interior, Wall border
- `entrance(x, y)` - Sets player spawn (must be interior, not on walls)
- `exit(x, y)` - Sets exit (must be on top or bottom wall)
- `build()` - Produces `DungeonLayout`, panics if entrance not set

**Automatic features:**
- 1-tile Wall border around edges
- Interior filled with Floor tiles
- Floor variant pattern `((x + y) % 3)` for visual variety

### LayoutId (`layouts/mod.rs`)
Registry of predefined layouts:
```rust
pub enum LayoutId {
    StartingRoom,
}

let layout = LayoutId::StartingRoom.layout();
```

### TileRenderer (`rendering.rs`)
Maps logical `TileType` to visual `DungeonTileSlice` with adjacency-aware wall selection:
```rust
if let Some((slice, flip_x)) = TileRenderer::resolve(&layout, x, y) {
    // slice: DungeonTileSlice, flip_x: bool
}
```

Wall rendering uses diagonal floor detection for corners:
- Top-left corner: walls right/below, floor diagonally at (x+1, y+1)
- Top-right corner: walls left/below, floor diagonally at (x-1, y+1)
- Bottom-left corner: walls right/above, floor diagonally at (x+1, y-1)
- Bottom-right corner: walls left/above, floor diagonally at (x-1, y-1)

## Adding New Layouts

**Preferred: Use LayoutBuilder** for declarative layout creation:

1. Create `src/dungeon/layouts/my_layout.rs`:
```rust
use crate::dungeon::{DungeonEntity, DungeonLayout, LayoutBuilder};
use crate::mob::MobId;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn create() -> DungeonLayout {
    let mut layout = LayoutBuilder::new(30, 20)
        .entrance(15, 18)  // Player spawn (interior)
        .exit(15, 19)      // Exit at bottom wall
        .build();

    // Add entities (manual spawning until SpawnTable is available)
    let mut spawn_points = layout.spawn_points();
    let mut rng = rand::thread_rng();
    spawn_points.shuffle(&mut rng);
    let mut spawn_iter = spawn_points.into_iter();

    if let Some((x, y)) = spawn_iter.next() {
        layout.add_entity(x, y, DungeonEntity::Chest { variant: rng.gen_range(0..4) });
    }
    if let Some((x, y)) = spawn_iter.next() {
        layout.add_entity(x, y, DungeonEntity::Mob { mob_id: MobId::Goblin });
    }

    layout
}
```

2. Add variant to `LayoutId` in `layouts/mod.rs`:
```rust
pub enum LayoutId {
    StartingRoom,
    MyLayout,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::StartingRoom => starting_room::create(),
            LayoutId::MyLayout => my_layout::create(),
        }
    }
}
```

**Note:** Entity spawning is currently manual. SpawnTable (#342) will add declarative entity placement.

## LayoutGenerator Trait
For future procedural generation:
```rust
pub trait LayoutGenerator {
    fn generate(&self) -> DungeonLayout;
}
```

## Player Movement System

Player movement in the dungeon tab is handled in `src/screens/town/tabs/dungeon.rs`.

### Components and Resources

```rust
// Resource tracking player position and layout
#[derive(Resource)]
pub struct DungeonState {
    pub layout: DungeonLayout,
    pub player_pos: (usize, usize),
}

// Marker for grid cells with coordinates
#[derive(Component)]
pub struct DungeonCell { pub x: usize, pub y: usize }

// Marker for the player entity
#[derive(Component)]
pub struct DungeonPlayer;
```

### Movement Rules
- **Only `TileType::Floor` is walkable** (not Entrance, Exit, DoorOpen, PlayerSpawn)
- Player cannot move onto tiles containing entities (chests, mobs)
- Arrow keys trigger `GameAction::Navigate(NavigationDirection)` events

### How Movement Works
1. `handle_dungeon_movement` listens for `GameAction::Navigate` events
2. Calculates target position from direction
3. Validates: target must be `TileType::Floor` AND have no entity
4. Updates `DungeonState.player_pos`
5. Re-parents player entity to target `DungeonCell` using `commands.entity(player).set_parent(cell)`

### Key Functions
- `spawn_dungeon_content()` - spawns grid with `DungeonCell` markers, initializes `DungeonState`
- `handle_dungeon_movement()` - processes arrow key input, validates moves, re-parents player
- `cleanup_dungeon_state()` - removes `DungeonState` resource on tab exit
