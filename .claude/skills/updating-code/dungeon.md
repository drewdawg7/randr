# Dungeon System

## Overview
The dungeon tab displays a tile-based dungeon room in the town screen. It uses the dungeon tileset to render walls, floors, and decorative elements.

## Key Files
- `src/screens/town/tabs/dungeon.rs` - DungeonTabPlugin and rendering
- `src/assets/sprite_slices.rs` - DungeonTileSlice enum
- `src/assets/sprites.rs` - SpriteSheetKey::DungeonTileset
- `assets/sprites/dungeon_tileset.json` - Tileset metadata with named slices
- `assets/sprites/dungeon_tileset.png` - Tileset image (160x160, 10x10 grid of 16x16 tiles)

## DungeonTileSlice Enum
Located in `src/assets/sprite_slices.rs`. Maps semantic names to tileset slice names:

```rust
pub enum DungeonTileSlice {
    // Floor tiles (use FloorTile2-4, avoid FloorTile1 and FloorTile5-8)
    FloorTile2, FloorTile3, FloorTile4,

    // Walls
    TopWall1, TopWall2, TopWall3, TopWall4,
    BottomWall1, BottomWall2, BottomWall3, BottomWall4,
    SideWall2, SideWall3, SideWall4, SideWall5, SideWall6, SideWall7, SideWall8,

    // Corners
    BottomLeftWall, BottomRightWall, WallCornerTopLeft,

    // Decorative
    WallColumn, WallColumnRed1, WallColumnRed2, WallColumnBlue1, WallColumnBlue2,
    TorchWall1, TorchWall2, TorchWall3, TorchWall4,
    Gate, GateFloor, Stairs,
}
```

## Tile Layout Pattern
The dungeon uses a 2D array of `(DungeonTileSlice, bool)` tuples where the bool indicates `flip_x`:

```rust
let layout: [[(DungeonTileSlice, bool); WIDTH]; HEIGHT] = [
    // Left wall tiles need flip_x = true to mirror horizontally
    [(DungeonTileSlice::SideWall5, true), /* ... */, (DungeonTileSlice::SideWall5, false)],
    // ...
];
```

### Flipping Convention
- **Left side walls**: Use the same tile as right side but with `flip_x = true`
- **Bottom left corner**: Use `BottomRightWall` with `flip_x = true`
- **Right side walls and corners**: `flip_x = false`

## Rendering
Tiles are rendered in a CSS Grid layout:

```rust
const TILE_SIZE: f32 = 48.0; // 16px * 3x scale

content.spawn(Node {
    display: Display::Grid,
    grid_template_columns: vec![GridTrack::px(TILE_SIZE); 8],
    grid_template_rows: vec![GridTrack::px(TILE_SIZE); 6],
    ..default()
}).with_children(|grid| {
    for row in &layout {
        for (tile, flip_x) in row {
            let mut cell = grid.spawn(Node { width: Val::Px(TILE_SIZE), height: Val::Px(TILE_SIZE), ..default() });
            if let Some(mut img) = sheet.image_node(tile.as_str()) {
                if *flip_x {
                    img.flip_x = true;
                }
                cell.insert(img);
            }
        }
    }
});
```

## Adding the Tab
The Dungeon tab was added by:
1. Adding `Dungeon` variant to `TownTab` enum in `src/screens/town/state.rs`
2. Updating `name()`, `all()`, `next()`, `prev()` methods
3. Adding cleanup system in `src/screens/town/plugin.rs`
4. Creating `DungeonTabPlugin` in `src/screens/town/tabs/dungeon.rs`
5. Registering in `src/screens/town/tabs/mod.rs`

## Tileset Source
Original tileset: `/Users/drewstewart/Downloads/2D Dungeon Asset Pack_v5.2/character and tileset/`
- Exported JSON with named slices
- Image path in JSON must be `"sprites/dungeon_tileset.png"` (not the original filename)
