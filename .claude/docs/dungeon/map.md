# TMX Map Support

Load Tiled Map Editor (.tmx) files directly as dungeon layouts. TMX maps render **1:1 exactly as designed in Tiled**.

## File Structure

```
assets/maps/
    cave_floor.tmx       # TMX cave dungeon floor
    home_floor.tmx       # TMX home floor with door tiles
    cave.tsx             # Tileset definition (shared)
    Cave Tileset.png     # Tileset sprite sheet
```

## How It Works

1. TMX tile IDs map directly to sprite positions in tileset PNG
2. Bypasses procedural TileRenderer system
3. Tile properties in TSX control walkability/spawning

## Usage

```rust
use crate::dungeon::{parse_map, Map};

let tmx = parse_map(Path::new("assets/maps/cave_floor.tmx"))?;
let layout = tmx.to_layout();
```

## Tile Properties (in TSX)

| Property | Type | Default | Effect |
|----------|------|---------|--------|
| `is_solid` | bool | `true` | Wall if true, Floor if false |
| `can_have_entity` | bool | `false` | Allows entity spawning |
| `can_spawn_player` | bool | `false` | Valid player spawn |
| `is_door` | bool | `false` | Door tile (triggers location transition) |

**Important:** Tiles without properties default to **solid walls**.

**Door tiles:** When `is_door=true`, the tile maps to `TileType::Door`. Walking into a door tile triggers dungeon entry (see `plugin.rs:682`).

## Tile ID Mapping

- Tile ID 0 = empty (not rendered, `TileType::Empty`)
- Tile ID 1+ → grid index `(id - firstgid)`
- Position: `col = index % columns`, `row = index / columns`
- Cave tileset: 14 columns, 32×32 pixels

## Creating a TMX Floor

1. **Design in Tiled** - Set tile properties in TSX

2. **Add loader** `src/dungeon/layouts/my_floor.rs`:
```rust
use crate::dungeon::tmx::parse_map;
use crate::dungeon::DungeonLayout;

pub fn create() -> DungeonLayout {
    parse_map(Path::new("assets/maps/my_floor.tmx"))
        .map(|tmx| tmx.to_layout())
        .unwrap_or_else(|_| DungeonLayout::new(10, 10))
}
```

3. **Add to LayoutId** in `layouts/layout_id.rs`

4. **Add FloorType variant** if reusable template needed

## Key Types

| Type | Purpose |
|------|---------|
| `Map` | Parsed map with tiles and tileset |
| `Tileset` | Parsed TSX with tile properties |
| `TileProperties` | Per-tile walkability/spawning |
| `TilesetGrid` | Resource for direct tile rendering |

## Rendering

`FloorType::CaveFloor` uses `TilesetGrid` resource:
- Tiles render by exact tile ID
- No roof rows added (TMX defines complete layout)
- Entity spawning uses standard SpawnTable

## TMX Floors

| Layout | File | Description |
|--------|------|-------------|
| `CaveFloor` | `cave_floor.tmx` | Dungeon cave floor |
| `HomeFloor` | `home_floor.tmx` | Home with door to dungeon |

**Home Floor:** Uses `FloorType::CaveFloor` for rendering. The `FloorId::floor_type()` method returns the appropriate floor type for Fixed floors (see `definitions.rs`).
