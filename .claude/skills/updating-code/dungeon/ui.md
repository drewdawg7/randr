# Dungeon UI

Dungeon screen rendering at `src/ui/screens/dungeon/plugin.rs`.

## DungeonPlugin
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

## UI Architecture

The dungeon uses an **entity overlay** pattern to support multi-cell sprites:

```
DungeonRoot
├── PlayerStats
└── DungeonContainer (fixed pixel size)
    ├── DungeonGrid (CSS Grid - tiles only)
    │   └── DungeonCell (x, y)
    │       └── Tile background (ImageNode)
    └── EntityOverlay (absolute positioned, same size as grid)
        ├── Player sprite (absolute: left/top in pixels)
        └── Entity sprites (absolute: left/top, width/height based on GridSize)
```

### Why Entity Overlay?
Multi-cell entities (2x2, 4x4) need to render ON TOP of all tiles. Without the overlay:
- Grid cells spawn in order (0,0), (1,0)...
- An entity at (5,3) overflows into neighboring cells
- Later cells' tile backgrounds render ON TOP of the overflow
- Result: entity hidden behind tiles

The overlay solves this by rendering ALL entities after ALL tiles.

### Key Components
- `DungeonContainer` - Holds grid and overlay, sets pixel dimensions
- `DungeonGrid` - CSS Grid containing only tile backgrounds
- `EntityOverlay` - Absolute positioned layer for player + entities
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
Entities use absolute pixel positioning within the overlay:
```rust
let size = entity.size();
let x_px = pos.x as f32 * tile_size;
let y_px = pos.y as f32 * tile_size;
let w_px = tile_size * size.width as f32;
let h_px = tile_size * size.height as f32;

overlay.spawn((
    DungeonMobSprite { mob_id },
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x_px),
        top: Val::Px(y_px),
        width: Val::Px(w_px),
        height: Val::Px(h_px),
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
1. Grid track sizes
2. Container dimensions
3. Overlay dimensions
4. Player position and size

Note: Entity positions are not updated on resize (set at spawn time).

## DungeonTileSlice
Visual tile enum at `src/assets/sprite_slices.rs`:
- Floor: `FloorTile2`, `FloorTile3`, `FloorTile4`
- Top walls: `TopWall1-4`
- Bottom walls: `BottomWall1-4`
- Side walls: `SideWall5-8`
- Corners: `BottomRightWall` (use flip_x for left corners), `SideWall5` (top corners)
- Special: `Gate`, `GateFloor`, `Stairs`

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
