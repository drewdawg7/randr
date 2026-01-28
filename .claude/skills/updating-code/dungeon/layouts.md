# Layouts

Layout system in `src/dungeon/layout.rs` and `layouts/`.

## DungeonLayout (`layout.rs`)

2D tile grid with entities:

```rust
let layout = LayoutId::TmxCaveFloor.layout();

layout.width();
layout.height();
layout.tile_at(x, y);
layout.is_walkable(x, y);
layout.is_floor(x, y);

layout.spawn_points();
layout.spawn_areas(size);
layout.add_entity(pos, entity);
layout.entity_at(x, y);
layout.entities();
```

## LayoutId (`layouts/layout_id.rs`)

Registry of TMX-based layouts:

```rust
pub enum LayoutId {
    TmxCaveFloor,
    TmxHomeFloor,
}

let layout = LayoutId::TmxCaveFloor.layout();
```

## Tile Struct

```rust
pub struct Tile {
    pub tile_type: TileType,
    pub variant: u8,
    pub flip_x: bool,
    pub tileset_id: Option<u32>,
}
```

## Adding a New TMX Layout

1. Create the TMX file in `assets/maps/`

2. Create `src/dungeon/layouts/my_layout.rs`:
```rust
use crate::dungeon::tmx::parse_tmx;
use crate::dungeon::DungeonLayout;
use std::path::Path;

const MY_LAYOUT_TMX: &str = "assets/maps/my_layout.tmx";

pub fn create() -> DungeonLayout {
    match parse_tmx(Path::new(MY_LAYOUT_TMX)) {
        Ok(tmx_map) => tmx_map.to_layout(),
        Err(e) => {
            eprintln!("Failed to load TMX map: {}", e);
            DungeonLayout::new(10, 10)
        }
    }
}
```

3. Add to `LayoutId` enum and match arm in `layout_id.rs`

4. Reference from `FloorSpec` in `definitions.rs`

For TMX details, see [tmx.md](tmx.md).
