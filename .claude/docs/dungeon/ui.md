# Dungeon UI

Dungeon screen rendering at `src/ui/screens/dungeon/plugin.rs`.

## DungeonScreenPlugin
Renders dungeon layout as a top-level screen (AppState::Dungeon).

## DungeonFloor Observer

All dungeon rendering is driven by the `DungeonFloor` component. Spawning it triggers an `OnAdd` observer that:
1. Calculates `UiScale` from window dimensions
2. Inserts `UiScale` resource
3. Builds the full UI hierarchy (root, stats, container, tile grid, entity layer)
4. Populates and inserts `GridOccupancy` resource
5. Despawns the trigger entity (consumed)

### Usage

```rust
use crate::ui::screens::dungeon::plugin::DungeonFloor;
use crate::dungeon::{DungeonLayout, GridPosition, GridSize, FloorType};

commands.spawn(DungeonFloor {
    layout: layout.clone(),
    player_pos: state.player_pos,
    player_size: state.player_size,
    floor_type: FloorType::CaveFloor,
});
```

### System Functions

Both `spawn_dungeon_screen` and `advance_floor_system` are pure state management:

```rust
fn spawn_dungeon_screen(mut commands: Commands, registry: Res<DungeonRegistry>, mut state: ResMut<DungeonState>) {
    if !state.is_in_dungeon() { state.enter_dungeon(LocationId::Home, &registry); }
    state.load_floor_layout();
    let Some(layout) = state.layout.clone() else { return; };
    let floor_type = state.current_floor().map(|f| f.floor_type()).unwrap_or(FloorType::CaveFloor);
    commands.spawn(DungeonFloor { layout, player_pos: state.player_pos, player_size: state.player_size, floor_type });
}
```

## UI Architecture

The dungeon uses a **two-layer system**: a CSS Grid for static tiles and an absolute-positioned overlay for entities/player:

```
DungeonRoot
├── PlayerStats
└── DungeonContainer (fixed pixel size)
    ├── DungeonGrid (CSS Grid — tiles only)
    │   └── DungeonCell → Tile background (ImageNode)
    └── EntityLayer (position: absolute, same size as grid)
        ├── Entity nodes (position: absolute, left/top pixels)
        └── Player node (position: absolute, left/top pixels, interpolated)
```

### Why Two Layers
- CSS Grid snaps items to cells — no in-between state for smooth movement
- Tile backgrounds are static and benefit from grid layout
- Entities and player need sub-cell pixel positioning for smooth interpolation

### Entity Z-Ordering
Entities use `ZIndex(y)` so entities lower on the grid render on top of entities above them. The player uses `ZIndex(player_pos.y + 100)` to always render above entities.

### Key Components
- `DungeonFloor` - Trigger component: spawning it renders the floor via observer
- `DungeonContainer` - Holds the grid and entity layer, sets pixel dimensions
- `DungeonGrid` - CSS Grid containing only tile backgrounds
- `EntityLayer` - Absolute-positioned overlay for entities and player
- `DungeonCell` - Unit struct marker for grid cells
- `SmoothPosition` - Pixel interpolation component (current/target/moving)

## Constants

| Constant | Value | Purpose |
|----------|-------|---------|
| `DUNGEON_SCALE` | 1.5 | Scale factor for dungeon tiles |
| `BASE_TILE_UNSCALED` | 8.0 | Original sprite size / 2 |
| `BASE_TILE` | 12.0 | BASE_TILE_UNSCALED * DUNGEON_SCALE |
| `MOVE_SPEED` | 6.0 | Tiles per second (movement speed) |
| `ENTITY_VISUAL_SCALE` | 2.0 | Visual size multiplier for player/mobs |

## Tile Rendering

All tiles are rendered from TMX tileset IDs. The rendering loop checks for `tileset_id` first:

```rust
if let Some(tile) = layout.tile_at(x, y) {
    if let Some(tileset_id) = tile.tileset_id {
        if let Some(img) = tmx_tileset.image_node_for_tile(tileset_id) {
            cell.spawn((img, node));
        }
    } else if let Some(resolved) = resolve_tile(floor_type, &layout, x, y) {
        // Fallback for non-TMX tiles
    }
}
```

The `resolve_tile()` function provides fallback rendering via `CaveTileRenderer` for tiles without TMX IDs.

## Entity Positioning (Absolute)

Entities are positioned absolutely within the EntityLayer using pixel coordinates:
```rust
let visual_size = match entity.render_data() {
    EntityRenderData::AnimatedMob { .. } => ENTITY_VISUAL_SCALE * tile_size,
    _ => tile_size,
};
let offset = -(visual_size - tile_size) / 2.0;
let left = pos.x as f32 * tile_size + offset;
let top = pos.y as f32 * tile_size + offset;

layer.spawn((
    DungeonEntityMarker { pos: *pos, entity_type: *entity },
    z_for_entity(pos.y),
    image_node,
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(left),
        top: Val::Px(top),
        width: Val::Px(visual_size),
        height: Val::Px(visual_size),
        ..default()
    },
));
```

### Visual Sizing
- **Player/Mobs**: `ENTITY_VISUAL_SCALE * tile_size` (2x tile size) — centered on grid cell with negative offset
- **Chests/Rocks/Stairs**: `tile_size` (1x) — exact grid cell size, no offset

### Grid Size
All entities are 1x1 in the logical grid (`GridSize::single()`). Visual sprite size is controlled independently via `ENTITY_VISUAL_SCALE`.

## Smooth Movement System

### SmoothPosition Component
```rust
#[derive(Component)]
pub struct SmoothPosition {
    pub current: Vec2,
    pub target: Vec2,
    pub moving: bool,
}
```

### Interpolation System (`interpolate_positions`)
Runs each frame, moves `current` toward `target` at constant speed:
```rust
let speed = MOVE_SPEED * tile_size;
let step = speed * time.delta_secs();
pos.current += delta.normalize() * step.min(distance);
```

### Movement Flow
1. `handle_dungeon_movement` determines direction from events or held keys
2. If `smooth_pos.moving`, input is blocked (one tile at a time)
3. On valid move: update `DungeonState.player_pos`, `GridOccupancy`, set `smooth_pos.target`
4. `interpolate_positions` smoothly moves sprite each frame
5. When interpolation completes, next held-key input is accepted immediately

### Held-Key Detection
The movement handler bypasses the key repeat system for continuous movement:
```rust
let direction = action_reader
    .read()
    .find_map(|a| match a {
        GameAction::Navigate(dir) => Some(*dir),
        _ => None,
    })
    .or_else(|| held_direction(&keyboard));
```
This eliminates the 0.3s initial repeat delay between tile movements.

### Walk Animation
- Animation only switches to walk frames on first movement (not reset on subsequent moves)
- `PlayerWalkTimer` (0.3s) keeps walk animation playing between consecutive moves
- Walk animation: frames 13-18, 0.08s/frame, looping
- Reverts to idle 0.3s after last movement completes

## Player Movement (Code)
```rust
occupancy.vacate(state.player_pos, state.player_size);
occupancy.occupy(new_pos, state.player_size, player_entity);
state.player_pos = new_pos;

let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
smooth_pos.target = Vec2::new(
    new_pos.x as f32 * tile_size + entity_offset,
    new_pos.y as f32 * tile_size + entity_offset,
);
smooth_pos.moving = true;
```

## CaveTileSlice

Visual tile enum for cave tileset at `src/assets/sprite_slices.rs`:
- Floor: `Floor1-6`
- Roof: `LeftRoof`, `RightRoof`, `FrontRoof`, `BackwallRoof`
- Backwall: `Backwall1-3`

Special tiles (from DungeonTileset):
- `Gate`, `GateFloor` - Used for doors and player spawn

## Tile Rendering (rendering.rs)

`CaveTileRenderer::resolve()` handles tile-to-sprite mapping:
- `PlayerSpawn` -> `DungeonTileSlice::GateFloor`
- `Floor/Entrance/SpawnPoint` -> `CaveTileSlice::Floor1-6` (variant-based)
- `Wall` at x=0 -> `CaveTileSlice::RightRoof`
- `Wall` at x=width-1 -> `CaveTileSlice::LeftRoof`
- `Wall` with floor above -> `CaveTileSlice::FrontRoof`
- `Exit/Door` -> `DungeonTileSlice::Gate`
- `DoorOpen` -> `DungeonTileSlice::GateFloor`
- `TorchWall` -> Resolves as wall (no torches in caves)
- `Empty` -> None (not rendered)

## Tileset Assets
- `assets/sprites/cave_tileset.png` - 64x64, 2x2 grid of 32x32 tiles
- `assets/sprites/cave_tileset.json` - Slice metadata
- `assets/sprites/dungeon_tileset.png` - For special tiles (Gate, GateFloor)

## Screen Registration
1. `DungeonPlugin` in `src/ui/screens/dungeon/plugin.rs`
2. Registered in `src/plugins/game.rs` (Screens and modals section)
3. Uses `AppState::Dungeon` state (defined in `src/states/app_state.rs`)
