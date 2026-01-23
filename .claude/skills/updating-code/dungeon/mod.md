# Dungeon Module

Data-driven dungeon system at `src/dungeon/`.

## Conceptual Hierarchy

```
Dungeon Location (e.g., "Goblin Cave")
  └── Floor (e.g., "Floor 1", "Boss Floor")
       └── Layout (the tile grid + entities)
```

- **Location**: A dungeon destination (registered in `DungeonPlugin`)
- **Floor**: One level within a dungeon, with its own layout and spawns (defined by `FloorSpec`/`FloorId`)
- **Layout**: The actual tile grid and entity placement (defined by `LayoutId`)

## File Structure
```
src/dungeon/
    mod.rs              # Re-exports
    plugin.rs           # DungeonPlugin, DungeonBuilder, DungeonRegistry
    state.rs            # DungeonState (runtime state + progression)
    tile.rs             # TileType, Tile
    entity.rs           # DungeonEntity enum
    grid.rs             # GridSize, GridPosition, GridOccupancy
    layout.rs           # DungeonLayout (tiles + entities)
    layout_builder.rs   # LayoutBuilder (declarative layout creation)
    spawn.rs            # SpawnTable (declarative entity spawning)
    generator.rs        # LayoutGenerator trait
    rendering.rs        # TileRenderer (logical -> visual)
    floor/
        mod.rs          # Re-exports FloorSpec, FloorId
        definitions.rs  # define_data! macro invocation
    layouts/
        mod.rs          # LayoutId enum
        starting_room.rs
```

## Core Types

### DungeonPlugin (`plugin.rs`)

Plugin for registering dungeon locations and their floor sequences. Uses a fluent builder API similar to `NavigationPlugin`.

```rust
use crate::dungeon::{DungeonPlugin, FloorId};
use crate::location::LocationId;

// In game.rs plugin registration:
app.add_plugins(
    DungeonPlugin::new()
        .location(LocationId::GoblinCave)
            .floor(FloorId::GoblinCave1)
            .floor(FloorId::GoblinCave2)
        .location(LocationId::CrystalMine)
            .floor(FloorId::CrystalMine1)
        .build()
);
```

**Builder methods:**
- `DungeonPlugin::new()` - Returns a `DungeonBuilder`
- `.location(LocationId)` - Sets context for subsequent `.floor()` calls
- `.floor(FloorId)` - Adds floor to current location (first added = first floor)
- `.build()` - Returns final `DungeonPlugin` (panics if no locations registered)

### DungeonRegistry (Resource)

Inserted by `DungeonPlugin`, provides runtime floor queries:

```rust
fn my_system(registry: Res<DungeonRegistry>) {
    // Get all floors for a location
    let floors: &[FloorId] = registry.floors(LocationId::GoblinCave);

    // Get next floor (for progression)
    if let Some(next) = registry.next_floor(LocationId::GoblinCave, FloorId::GoblinCave1) {
        // next == FloorId::GoblinCave2
    }

    // Check if floor is the final one
    let is_boss = registry.is_final_floor(LocationId::GoblinCave, FloorId::GoblinCave2);
}
```

**Methods:**
- `floors(location) -> &[FloorId]` - All floors for a location in order
- `next_floor(location, current) -> Option<FloorId>` - Next floor after current
- `is_final_floor(location, floor) -> bool` - Check if this is the last floor

### DungeonState (Resource, `state.rs`)

Tracks runtime dungeon state and player progression. Combines:
- **Progression tracking**: Which dungeon/floor the player is on, which floors have been cleared
- **Runtime state**: Current layout and player position

```rust
use crate::dungeon::{DungeonState, DungeonRegistry};
use crate::location::LocationId;

fn my_system(
    mut state: ResMut<DungeonState>,
    registry: Res<DungeonRegistry>,
) {
    // Enter a dungeon (sets current_location, current_floor, floor_index)
    state.enter_dungeon(LocationId::GoblinCave, &registry);

    // Load the layout for the current floor (sets layout, player_pos, player_size)
    state.load_floor_layout();

    // Check if on final floor
    if state.is_current_floor_final(&registry) {
        // Boss floor!
    }

    // Advance to next floor after clearing (marks current as cleared)
    if let Some(next_floor) = state.advance_floor(&registry) {
        state.load_floor_layout();
    } else {
        // Dungeon complete!
        state.exit_dungeon();
    }

    // Check if a floor has been cleared
    let cleared = state.is_floor_cleared(FloorId::GoblinCave1);
}
```

**Fields:**
- `current_location: Option<LocationId>` - Active dungeon (None if not in dungeon)
- `current_floor: Option<FloorId>` - Active floor (None if not in dungeon)
- `floor_index: usize` - 0-indexed position in location's floor sequence
- `cleared_floors: HashSet<FloorId>` - Set of floors player has cleared
- `layout: Option<DungeonLayout>` - Current floor layout (None until loaded)
- `player_pos: GridPosition` - Player's grid position
- `player_size: GridSize` - Player's grid size (default 1x1)

**Methods:**
- `enter_dungeon(location, registry)` - Begin dungeon run at first floor
- `load_floor_layout() -> Option<&DungeonLayout>` - Load layout for current floor
- `advance_floor(registry) -> Option<FloorId>` - Move to next floor, mark current as cleared
- `is_current_floor_final(registry) -> bool` - Check if on last floor
- `exit_dungeon()` - Leave dungeon (clears runtime state, preserves cleared_floors)
- `is_floor_cleared(floor) -> bool` - Check if specific floor has been cleared
- `is_in_dungeon() -> bool` - Check if currently in a dungeon

### TileType (`tile.rs`)
Logical tile types for gameplay:
```rust
pub enum TileType {
    Wall,       // Impassable
    Floor,      // Walkable
    Entrance,   // Player spawn
    Exit,       // Stairs/door
    Door,       // Closed door
    DoorOpen,   // Open door
    PlayerSpawn,// Player spawn point
    TorchWall,  // Animated torch on back wall (impassable)
}
```

**Torch tiles**: `TorchWall` is impassable (wall behavior). Rendered with a 3-frame animated sprite from `SpriteSheetKey::TorchWall`. Placed randomly on back wall (y=0) by `LayoutBuilder::torches(range)`.

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
layout.spawn_points();              // Get all 1x1 tiles where entities can spawn
layout.spawn_areas(size);           // Get all valid positions for entity of given size
layout.add_entity(pos, entity);     // Add entity at GridPosition
layout.entity_at(x, y);             // Get entity at cell (checks multi-cell entities)
layout.entities();                  // Get all entities as &[(GridPosition, DungeonEntity)]
```

**Multi-cell entity support:**
- `entity_at(x, y)` checks if any entity occupies the cell, including multi-cell entities
- `spawn_areas(size)` finds all positions where an entity of given size fits without overlapping
- Entities are stored with `GridPosition` (top-left anchor)

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
use crate::dungeon::{LayoutBuilder, SpawnTable};
use crate::mob::MobId;

let layout = LayoutBuilder::new(40, 21)
    .entrance(20, 19)  // Interior position (player spawn)
    .exit(20, 20)      // Bottom wall (stairs/door)
    .spawn(SpawnTable::new()
        .mob(MobId::Goblin, 3)
        .mob(MobId::Slime, 2)
        .chest(1..=2))
    .build();
```

**Builder behavior:**
- `new(width, height)` - Creates grid with Floor interior, Wall border
- `entrance(x, y)` - Sets player spawn (must be interior, not on walls)
- `exit(x, y)` - Sets exit (must be on top or bottom wall)
- `torches(range)` - Sets torch count range (e.g., `2..=4`), placed randomly on back wall
- `spawn(SpawnTable)` - Sets entity spawn rules (applied during build)
- `build()` - Produces `DungeonLayout`, panics if entrance not set

**Automatic features:**
- 1-tile Wall border around edges
- Interior filled with Floor tiles
- Floor variant pattern `((x + y) % 3)` for visual variety
- Torches placed randomly on back wall (y=0) if set, avoiding corners and exit
- Spawn table applied automatically if set

### SpawnTable (`spawn.rs`)
Declarative entity spawning with weighted mob selection and multi-cell entity support:
```rust
use crate::dungeon::SpawnTable;
use crate::mob::MobId;

// Floor with weighted mob spawns, chests, and stairs
let spawn_table = SpawnTable::new()
    .mob(MobId::Goblin, 5)   // 5/8 chance per mob spawn (uses Goblin's grid_size)
    .mob(MobId::Slime, 3)    // 3/8 chance per mob spawn (uses Slime's grid_size)
    .mob_count(3..=5)        // Spawn 3-5 mobs total
    .chest(1..=2)            // Spawn 1 or 2 chests (always 1x1)
    .stairs(1..=1);          // Spawn 1 stairs (always 1x1, advances floor)

// Apply manually if not using LayoutBuilder
spawn_table.apply(&mut layout, &mut rand::thread_rng());

// Empty spawn table for boss rooms (no random spawns)
let boss_spawns = SpawnTable::empty();

// Treasure room (only chests, no mobs)
let treasure = SpawnTable::new().chest(5..=8);
```

**Types:**
- `SpawnEntityType` - Enum for spawn entry types (`Mob(MobId)`)
- `SpawnEntry` - Entry with `entity_type`, `weight`, and `size` fields

**Methods:**
- `new()` / `empty()` - Creates empty spawn table
- `mob(MobId, weight)` - Adds mob type with weight; size auto-loaded from `MobSpec`
- `mob_count(range)` - Sets mob count range (e.g., `2..=4`)
- `chest(range)` - Sets chest count range (e.g., `1..=2`), always 1x1
- `stairs(range)` - Sets stairs count range (e.g., `1..=1`), always 1x1
- `rock(range)` - Sets rock count range (e.g., `2..=4`), always 1x1, random RockType
- `apply(&mut layout, &mut rng)` - Applies spawns to layout

**Algorithm:**
1. Spawns random chest count (range) first, each with variant 0-3
   - Uses `layout.spawn_areas(GridSize::single())` to find valid 1x1 positions
2. Spawns random stairs count (range), always 1x1
   - Uses `layout.spawn_areas(GridSize::single())` to find valid positions
3. Spawns random rock count (range), random RockType (equal weight Copper/Coal/Tin)
   - Uses `layout.spawn_areas(GridSize::single())` to find valid positions
4. Spawns random mob count using weighted selection from entries
   - Uses `layout.spawn_areas(entry.size)` to find valid positions for each mob's size
   - Entities never overlap due to `spawn_areas()` checking existing entities

### LayoutId (`layouts/mod.rs`)
Registry of predefined layouts:
```rust
pub enum LayoutId {
    StartingRoom,
}

let layout = LayoutId::StartingRoom.layout();
```

### FloorSpec and FloorId (`floor/definitions.rs`)

Static floor specifications using the `define_data!` macro pattern. A floor ties together a layout, spawn rules, and metadata.

```rust
use crate::dungeon::{FloorId, FloorSpec};

// Get floor info
let floor = FloorId::GoblinCave1;
let spec = floor.spec();

// Access floor data
println!("Floor: {}", spec.name);          // "Goblin Cave - Floor 1"
let layout_id = spec.layout_id;            // LayoutId to generate layout
let spawns = &spec.spawn_table;            // SpawnTable for this floor

// Generate layout for this floor
let layout = spec.layout_id.layout();

// Iterate all floors
for floor_id in FloorId::ALL {
    println!("{}", floor_id.spec().name);
}
```

**FloorSpec fields:**
- `name: &'static str` - Display name (e.g., "Goblin Cave - Floor 1")
- `layout_id: LayoutId` - Reference to the layout definition
- `spawn_table: SpawnTable` - Entity spawn rules for this floor

**Generated API:**
- `FloorId::spec(&self) -> &'static FloorSpec` - Get the spec for a floor
- `FloorId::ALL` - Slice of all floor variants

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

**Preferred: Use LayoutBuilder with SpawnTable** for declarative layout creation:

1. Create `src/dungeon/layouts/my_layout.rs`:
```rust
use crate::dungeon::{DungeonLayout, LayoutBuilder, SpawnTable};
use crate::mob::MobId;

pub fn create() -> DungeonLayout {
    LayoutBuilder::new(30, 20)
        .entrance(15, 18)  // Player spawn (interior)
        .exit(15, 19)      // Exit at bottom wall
        .spawn(SpawnTable::new()
            .mob(MobId::Goblin, 3)
            .mob(MobId::Slime, 2)
            .chest(1..=2))
        .build()
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

**Special layout patterns:**
```rust
// Boss room - no random spawns, handle specially
LayoutBuilder::new(40, 30)
    .entrance(20, 28)
    .exit(20, 0)
    .spawn(SpawnTable::empty())  // Explicit: no random spawns
    .build()

// Treasure room - only chests
LayoutBuilder::new(20, 15)
    .entrance(10, 13)
    .exit(10, 14)
    .spawn(SpawnTable::new().chest(5..=8))
    .build()
```

## Adding New Floors

Add floor variants to `src/dungeon/floor/definitions.rs`:

```rust
entity_macros::define_data! {
    spec FloorSpec {
        pub name: &'static str,
        pub layout_id: LayoutId,
        pub spawn_table: SpawnTable,
    }

    id FloorId;

    variants {
        GoblinCave1 {
            name: "Goblin Cave - Floor 1",
            layout_id: LayoutId::StartingRoom,
            spawn_table: SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=5)
                .chest(1..=2),
        }
        // Add new floors here
        GoblinCave2 {
            name: "Goblin Cave - Floor 2",
            layout_id: LayoutId::StartingRoom,  // Use appropriate LayoutId
            spawn_table: SpawnTable::new()
                .mob(MobId::Goblin, 8)
                .mob(MobId::Slime, 5)
                .mob_count(4..=6)
                .chest(2..=3),
        }
    }
}
```

**Guidelines:**
- Each floor references a `LayoutId` for its tile grid
- The `spawn_table` defines what entities spawn (separate from layout's default spawns)
- Floor names should follow pattern: "Location Name - Floor N"
- Boss floors can use `SpawnTable::empty()` for manual boss placement

## LayoutGenerator Trait
For future procedural generation:
```rust
pub trait LayoutGenerator {
    fn generate(&self) -> DungeonLayout;
}
```

## Player Movement System

Player movement in the dungeon is handled in `src/ui/screens/dungeon/plugin.rs`.

### Components and Resources

The UI uses `DungeonState` from `crate::dungeon` (see above) for player position and layout. Additional UI-specific components:

```rust
// Marker for grid cells with coordinates
#[derive(Component)]
pub struct DungeonCell { pub x: usize, pub y: usize }

// Marker for the player entity
#[derive(Component)]
pub struct DungeonPlayer;

// Resource tracking cell occupancy (separate from DungeonState)
// See GridOccupancy in grid.rs
```

### Movement Rules
- **Only walkable tiles** via `layout.is_walkable(x, y)` (Floor tiles)
- Player cannot move onto cells occupied by entities (checked via `GridOccupancy`)
- Arrow keys trigger `GameAction::Navigate(NavigationDirection)` events
- Colliding with a mob triggers the fight modal
- Colliding with a chest blocks movement (chests are obstacles)
- Colliding with stairs advances to a new floor (`AdvanceFloor` resource → `advance_floor_system`)

### Multi-Cell Collision Detection

Movement validation checks **all cells** the player would occupy:

```rust
// Check if all destination cells are walkable (layout is Option<&DungeonLayout>)
fn all_cells_walkable(layout: Option<&DungeonLayout>, pos: GridPosition, size: GridSize) -> bool {
    let Some(layout) = layout else { return false };
    pos.occupied_cells(size)
        .all(|(x, y)| layout.is_walkable(x, y))
}

// Check for entity collision at any cell player would occupy
fn check_entity_collision(
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
    pos: GridPosition,
    size: GridSize,
) -> Option<DungeonEntity> {
    for (x, y) in pos.occupied_cells(size) {
        if let Some(entity) = occupancy.entity_at(x, y) {
            // Return the entity type for collision handling
        }
    }
    None
}
```

### How Movement Works
1. `handle_dungeon_movement` listens for `GameAction::Navigate` events
2. Calculates target position from direction as `GridPosition`
3. Validates all destination cells are walkable via `all_cells_walkable(state.layout.as_ref(), ...)`
4. Checks for entity collisions via `GridOccupancy` using `check_entity_collision()`
5. If collision with mob: triggers fight modal
6. If collision with chest: blocks movement
7. If no collision: updates `GridOccupancy` (vacate old, occupy new), updates `state.player_pos`
8. Updates player grid placement via `grid_column`/`grid_row` Node properties

### Key Functions
- `spawn_dungeon_screen()` - enters dungeon, loads floor layout, spawns grid/entities/player, initializes `GridOccupancy`
- `handle_dungeon_movement()` - processes arrow key input, validates via occupancy, handles collisions
- `all_cells_walkable()` - checks if all player destination cells are walkable tiles
- `check_entity_collision()` - uses `GridOccupancy` to detect entity at any destination cell
- `advance_floor_system()` - triggered by `AdvanceFloor` resource; despawns UI, increments `floor_index`, regenerates layout, respawns dungeon screen
- `cleanup_dungeon()` - calls `state.exit_dungeon()`, removes `UiScale` and `GridOccupancy` resources

## Adding a New Dungeon

Complete workflow for adding a new dungeon location:

1. **Add LocationId variant** in `src/location/spec/definitions.rs`:
```rust
// In the variants section:
CrystalMine {
    name: "Crystal Mine",
    description: "A mine filled with dangerous crystals",
    refresh_interval: None,
    min_level: None,
    data: LocationData::Dungeon(DungeonData {}),
}
```

2. **Update location_type()** in the same file:
```rust
LocationId::CrystalMine => LocationType::Combat(CombatSubtype::Dungeon),
```

3. **Create layout(s)** using LayoutBuilder (see "Adding New Layouts")

4. **Add FloorId variants** in `src/dungeon/floor/definitions.rs`:
```rust
CrystalMine1 {
    name: "Crystal Mine - Floor 1",
    layout_id: LayoutId::CrystalMineEntrance,
    spawn_table: SpawnTable::new()
        .mob(MobId::CrystalGolem, 3)
        .chest(1..=2),
}
```

5. **Register in DungeonPlugin** in `src/plugins/game.rs`:
```rust
DungeonPlugin::new()
    .location(LocationId::GoblinCave)
        .floor(FloorId::GoblinCave1)
    .location(LocationId::CrystalMine)  // Add new location
        .floor(FloorId::CrystalMine1)
    .build()
```
