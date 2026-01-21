# Dungeon UI

Dungeon screen rendering at `src/screens/dungeon/plugin.rs`.

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

## CSS Grid Rendering
```rust
const TILE_SIZE: f32 = 48.0; // 16px * 3x scale

content.spawn(Node {
    display: Display::Grid,
    grid_template_columns: vec![GridTrack::px(TILE_SIZE); layout.width()],
    grid_template_rows: vec![GridTrack::px(TILE_SIZE); layout.height()],
    ..default()
})
```

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
1. `DungeonPlugin` in `src/screens/dungeon/plugin.rs`
2. Registered in `src/plugins/game.rs` (Screens and modals section)
3. Uses `AppState::Dungeon` state (defined in `src/states/app_state.rs`)
