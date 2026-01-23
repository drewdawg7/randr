# Dungeon UI

Dungeon screen rendering at `src/ui/screens/dungeon/plugin.rs`.

## DungeonScreenPlugin
Renders dungeon layout as a top-level screen (AppState::Dungeon).

```rust
use crate::dungeon::{LayoutId, TileRenderer};

let layout = LayoutId::StartingRoom.layout();

for y in 0..layout.height() {
    for x in 0..layout.width() {
        if let Some((slice, flip_x)) = TileRenderer::resolve(&layout, x, y) {
            // Render tile with slice and flip_x
        }
    }
}
```

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
use crate::dungeon::{DungeonLayout, GridPosition, GridSize};

// Spawn a dungeon floor (observer handles all rendering)
commands.spawn(DungeonFloor {
    layout: layout.clone(),
    player_pos: state.player_pos,
    player_size: state.player_size,
});
```

### System Functions

Both `spawn_dungeon_screen` and `advance_floor_system` are ~10-15 lines of pure state management:

```rust
// spawn_dungeon_screen: enters dungeon, loads layout, spawns DungeonFloor
fn spawn_dungeon_screen(mut commands: Commands, registry: Res<DungeonRegistry>, mut state: ResMut<DungeonState>) {
    if !state.is_in_dungeon() { state.enter_dungeon(LocationId::GoblinCave, &registry); }
    state.load_floor_layout();
    let Some(layout) = state.layout.clone() else { return; };
    commands.spawn(DungeonFloor { layout, player_pos: state.player_pos, player_size: state.player_size });
}

// advance_floor_system: cleans up, advances state, spawns DungeonFloor
fn advance_floor_system(mut commands: Commands, mut state: ResMut<DungeonState>, root_query: Query<Entity, With<DungeonRoot>>) {
    commands.remove_resource::<AdvanceFloor>();
    for entity in &root_query { commands.entity(entity).despawn_recursive(); }
    commands.remove_resource::<UiScale>();
    commands.remove_resource::<GridOccupancy>();
    state.floor_index += 1;
    state.load_floor_layout();
    let Some(layout) = state.layout.clone() else { return; };
    commands.spawn(DungeonFloor { layout, player_pos: state.player_pos, player_size: state.player_size });
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

## CSS Grid Rendering (Tiles Only)
```rust
let tile_size = BASE_TILE * scale as f32; // BASE_TILE = 12.0

container.spawn((
    DungeonGrid,
    Node {
        display: Display::Grid,
        grid_template_columns: vec![GridTrack::px(tile_size); layout.width()],
        grid_template_rows: vec![GridTrack::px(tile_size); layout.height()],
        ..default()
    },
))
```

## Entity Positioning (Absolute)

Entities are positioned absolutely within the EntityLayer using pixel coordinates:
```rust
let visual_size = match entity.render_data() {
    EntityRenderData::AnimatedMob { .. } => ENTITY_VISUAL_SCALE * tile_size,  // 2x for mobs
    _ => tile_size,  // 1x for chests, rocks, stairs
};
let offset = -(visual_size - tile_size) / 2.0;  // Center on grid cell
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
All entities are 1x1 in the logical grid (`GridSize::single()`). Visual sprite size is controlled independently via `ENTITY_VISUAL_SCALE`. The old `ENTITY_GRID_SIZE` constant has been removed.

## Smooth Movement System

### SmoothPosition Component
```rust
#[derive(Component)]
pub struct SmoothPosition {
    pub current: Vec2,   // Current pixel position (interpolated each frame)
    pub target: Vec2,    // Target pixel position (set on movement)
    pub moving: bool,    // Whether currently animating toward target
}
```

### Interpolation System (`interpolate_positions`)
Runs each frame, moves `current` toward `target` at constant speed:
```rust
let speed = MOVE_SPEED * tile_size;  // pixels per second
let step = speed * time.delta_secs();
pos.current += delta.normalize() * step.min(distance);
// Snap when distance < 0.5px
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
// Prefer events (for initial press), fall back to held keys
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
// Update logical state
occupancy.vacate(state.player_pos, state.player_size);
occupancy.occupy(new_pos, state.player_size, player_entity);
state.player_pos = new_pos;

// Set interpolation target (replaces grid placement)
let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
smooth_pos.target = Vec2::new(
    new_pos.x as f32 * tile_size + entity_offset,
    new_pos.y as f32 * tile_size + entity_offset,
);
smooth_pos.moving = true;
```

## Window Resize
`handle_window_resize` updates:
1. Grid track sizes (columns and rows)
2. Container dimensions (width and height)
3. EntityLayer dimensions (width and height)
4. Player `SmoothPosition` (recalculate current/target from grid pos)
5. Player node (left, top, width, height)
6. Entity nodes (left, top, width, height based on entity type)

## DungeonTileSlice
Visual tile enum at `src/assets/sprite_slices.rs`:
- Floor: `FloorTile2`, `FloorTile3`, `FloorTile4`, `FloorTileAlt1` (Slice_73), `FloorTileAlt3` (Slice_83)
- Floor alternates (bottom edge only): `FloorTileAlt2` (Slice_74), `FloorTileAlt4` (Slice_84)
- Floor edges: `FloorEdgeTopLeft`, `FloorEdgeTop1/2`, `FloorEdgeTopRight`, `FloorEdgeLeft/Left2`, `FloorEdgeRight1/Right2`
- Top walls: `TopWall1-4`
- Bottom walls: `BottomWall1-4`
- Side walls: `SideWall5-8`
- Corners: `BottomRightWall` (use flip_x for left corners), `SideWall5` (top corners)
- Torches: `TorchWall1-4` (static variants; animated rendering uses separate sprite sheets)
- Special: `Gate`, `GateFloor`, `Stairs`

## Floor Edge Rendering (`rendering.rs`)

`TileRenderer::resolve_floor_edge()` renders floor tiles adjacent to walls with edge sprites:

- **Top edge** (wall above): alternates `FloorEdgeTop1`/`FloorEdgeTop2` by `x % 2`
- **Left edge** (wall left): `FloorEdgeLeft` normally, `FloorEdgeLeft2` only in front of bottom wall
- **Right edge** (wall right): `FloorEdgeRight1` normally, `FloorEdgeRight2` only in front of bottom wall
- **Top-left corner** (wall above + left): `FloorEdgeTopLeft`
- **Top-right corner** (wall above + right): `FloorEdgeTopRight`
- **Bottom edge** (wall below, inner tiles only): alternates `FloorTileAlt2`/`FloorTileAlt4` by `x % 2`
- **No bottom edge on side corners**: left/right edge takes priority

`TileType::PlayerSpawn` always renders as `GateFloor` (overrides edge detection).

The door on the back wall (`TileType::Door`) renders as `Gate` (decorative entrance).

## Animated Tiles (Torches)

`TileType::TorchWall` tiles are rendered with animation instead of static `DungeonTileSlice` variants. They use a separate sprite sheet:

- `SpriteSheetKey::TorchWall` - 3-frame torch wall animation (`torch_wall.json`, references `dungeon_tileset.png`)

The shared `render_dungeon_floor()` function handles both torch and regular tile rendering. It checks `tile.tile_type` before calling `TileRenderer::resolve`. For torch tiles, it uses `image_bundle_animated()` with `AnimationConfig { first_frame: 0, last_frame: 2, frame_duration: 0.4 }`.

`TileRenderer::resolve()` returns `None` for `TorchWall` since it bypasses the static slice system.

**Important:** Any tile rendering loop must include the `TorchWall` match arm. Using only `TileRenderer::resolve()` will skip torches since it returns `None` for them.

### Flip Convention
- Left walls: `flip_x = true`
- Right walls: `flip_x = false`
- Bottom-left corner: `BottomRightWall` with `flip_x = true`
- Top-left corner: `SideWall5` with `flip_x = true`

## Tileset Assets
- `assets/sprites/dungeon_tileset.png` - 160x160, 10x10 grid of 16x16 tiles
- `assets/sprites/dungeon_tileset.json` - Slice metadata
- Original: `/Users/drewstewart/Downloads/2D Dungeon Asset Pack_v5.2/character and tileset/`

## Screen Registration
1. `DungeonPlugin` in `src/ui/screens/dungeon/plugin.rs`
2. Registered in `src/plugins/game.rs` (Screens and modals section)
3. Uses `AppState::Dungeon` state (defined in `src/states/app_state.rs`)
