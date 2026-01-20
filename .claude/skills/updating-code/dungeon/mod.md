# Dungeon Module

Data-driven dungeon system at `src/dungeon/`.

## File Structure
```
src/dungeon/
    mod.rs              # Re-exports
    tile.rs             # TileType, Tile
    entity.rs           # DungeonEntity enum
    layout.rs           # DungeonLayout (tiles + entities)
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

1. Create `src/dungeon/layouts/my_layout.rs`:
```rust
use crate::dungeon::{DungeonLayout, Tile, TileType};

pub fn create() -> DungeonLayout {
    let mut layout = DungeonLayout::new(WIDTH, HEIGHT);
    // Set tiles...
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

## LayoutGenerator Trait
For future procedural generation:
```rust
pub trait LayoutGenerator {
    fn generate(&self) -> DungeonLayout;
}
```
