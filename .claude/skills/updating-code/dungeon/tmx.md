# TMX Map Support

Load Tiled Map Editor (.tmx) files directly as dungeon layouts. TMX maps render **1:1 exactly as designed in Tiled**.

## File Structure

```
assets/maps/
    cave_floor.tmx       # TMX map file
    cave.tsx             # Tileset definition
    Cave Tileset.png     # Tileset sprite sheet
```

## How It Works

1. TMX tile IDs map directly to sprite positions in tileset PNG
2. Bypasses procedural TileRenderer system
3. Tile properties in TSX control walkability/spawning

## Usage

```rust
use crate::dungeon::{parse_tmx, TmxMap};

let tmx = parse_tmx(Path::new("assets/maps/cave_floor.tmx"))?;
let layout = tmx.to_layout();
```

## Tile Properties (in TSX)

| Property | Type | Default | Effect |
|----------|------|---------|--------|
| `is_solid` | bool | `true` | Wall if true, Floor if false |
| `can_have_entity` | bool | `false` | Allows entity spawning |
| `can_spawn_player` | bool | `false` | Valid player spawn |

**Important:** Tiles without properties default to **solid walls**.

## Tile ID Mapping

- Tile ID 0 = empty (not rendered, `TileType::Empty`)
- Tile ID 1+ → grid index `(id - firstgid)`
- Position: `col = index % columns`, `row = index / columns`
- Cave tileset: 14 columns, 32×32 pixels

## Creating a TMX Floor

1. **Design in Tiled** - Set tile properties in TSX

2. **Add loader** `src/dungeon/layouts/my_floor.rs`:
```rust
use crate::dungeon::tmx::parse_tmx;
use crate::dungeon::DungeonLayout;

pub fn create() -> DungeonLayout {
    parse_tmx(Path::new("assets/maps/my_floor.tmx"))
        .map(|tmx| tmx.to_layout())
        .unwrap_or_else(|_| DungeonLayout::new(10, 10))
}
```

3. **Add to LayoutId** in `layouts/layout_id.rs`

4. **Add FloorType variant** if reusable template needed

## Key Types

| Type | Purpose |
|------|---------|
| `TmxMap` | Parsed map with tiles and tileset |
| `Tileset` | Parsed TSX with tile properties |
| `TileProperties` | Per-tile walkability/spawning |
| `TmxTilesetGrid` | Resource for direct tile rendering |

## Rendering

`FloorType::TmxCaveFloor` uses `TmxTilesetGrid` resource:
- Tiles render by exact tile ID
- No roof rows added (TMX defines complete layout)
- Entity spawning uses standard SpawnTable
