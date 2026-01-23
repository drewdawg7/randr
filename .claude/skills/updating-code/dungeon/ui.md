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
3. Builds the full UI hierarchy (root, stats, container, grid, tiles, entities, player)
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

Both `spawn_dungeon_screen` and `advance_floor_system` are now ~10-15 lines of pure state management:

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

The dungeon uses **grid-span z-ordering** for entities:

```
DungeonRoot
├── PlayerStats
└── DungeonContainer (fixed pixel size)
    └── DungeonGrid (CSS Grid)
        ├── DungeonCell (x, y) - tile backgrounds
        │   └── Tile background (ImageNode)
        ├── Entity sprites (grid spans + ZIndex by Y position)
        └── Player sprite (grid span + high ZIndex)
```

### Entity Z-Ordering
Entities use `ZIndex(y)` so entities lower on the grid render on top of entities above them. The player uses `ZIndex(player_pos.y + 100)` to always render above entities.

### Key Components
- `DungeonFloor` - Trigger component: spawning it renders the floor via observer
- `DungeonContainer` - Holds the grid, sets pixel dimensions
- `DungeonGrid` - CSS Grid containing tiles, entities, and player
- `DungeonCell` - Grid cell marker (coordinates stored but unused after refactor)

## CSS Grid Rendering
```rust
let tile_size = BASE_TILE * scale as f32; // BASE_TILE = 8.0

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

## Entity Positioning
Entities use grid-span placement within the DungeonGrid:
```rust
let size = entity.size();
let width_px = size.width as f32 * tile_size;
let height_px = size.height as f32 * tile_size;

grid.spawn((
    DungeonEntityMarker { pos: *pos, size, entity_type: *entity },
    z_for_entity(pos.y),  // ZIndex(y) for z-ordering
    image_node,
    Node {
        grid_column: GridPlacement::start_span(pos.x as i16 + 1, size.width as u16),
        grid_row: GridPlacement::start_span(pos.y as i16 + 1, size.height as u16),
        width: Val::Px(width_px),
        height: Val::Px(height_px),
        ..default()
    },
));
```

## Player Movement
Player movement updates grid placement based on `GridPosition` and `GridSize`:
```rust
// Update occupancy grid
occupancy.vacate(state.player_pos, state.player_size);
occupancy.occupy(new_pos, state.player_size, player_entity);

// Update state and visual grid placement
state.player_pos = new_pos;
player_node.grid_column = GridPlacement::start_span(new_pos.x as i16 + 1, state.player_size.width as u16);
player_node.grid_row = GridPlacement::start_span(new_pos.y as i16 + 1, state.player_size.height as u16);
```

Movement validation uses `GridOccupancy` for multi-cell collision detection. See [mod.md](mod.md) for details.

## Window Resize
`handle_window_resize` updates:
1. Grid track sizes (columns and rows)
2. Container dimensions (width and height)
3. Player size (width and height based on grid size)
4. Entity sizes (width and height based on their GridSize)

Grid placement (column/row spans) handles position automatically on resize.

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
