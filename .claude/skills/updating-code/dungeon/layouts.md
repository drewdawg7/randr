# Layouts

Layout system in `src/dungeon/layout.rs` and `layouts/`.

## DungeonLayout (`layout.rs`)

2D tile grid with entities:

```rust
let layout = LayoutId::StartingRoom.layout();

layout.width();
layout.height();
layout.tile_at(x, y);
layout.is_walkable(x, y);
layout.is_floor(x, y);

// Entity methods
layout.spawn_points();          // 1x1 spawn positions
layout.spawn_areas(size);       // Positions for multi-cell entities
layout.add_entity(pos, entity);
layout.entity_at(x, y);
layout.entities();              // &[(GridPosition, DungeonEntity)]
```

## LayoutBuilder (`layout_builder.rs`)

Declarative layout creation:

```rust
let layout = LayoutBuilder::new(40, 21)
    .entrance(20, 1)       // Player spawn
    .door(20, 0)           // Decorative door
    .torches(2..=4)        // Random torch count
    .variant_strategy(VariantStrategyKind::Percentage(75))
    .spawn(SpawnTable::new()
        .mob(MobId::Goblin, 3)
        .chest(1..=2))
    .build();
```

**Automatic features:**
- 1-tile wall border
- Interior filled with Floor tiles
- Floor variants via VariantStrategy

## LayoutId (`layouts/layout_id.rs`)

Registry of predefined layouts:

```rust
pub enum LayoutId {
    StartingRoom,
    ClusteredFloor,
    DungeonFloorWithStairs,
    DungeonFloorFinal,
    CaveFloorWithStairs,
    CaveFloorFinal,
    TmxCaveFloor,
    TmxHomeFloor,  // TMX-based home with door tiles
}

let layout = LayoutId::StartingRoom.layout();
```

## Tile Struct

```rust
pub struct Tile {
    pub tile_type: TileType,
    pub variant: u8,
    pub flip_x: bool,
    pub tileset_id: Option<u32>,  // For TMX
}
```

## Adding a New Layout

1. Create `src/dungeon/layouts/my_layout.rs`:
```rust
pub fn create() -> DungeonLayout {
    LayoutBuilder::new(30, 20)
        .entrance(15, 1)
        .door(15, 0)
        .torches(2..=3)
        .build()
}
```

2. Add to `LayoutId` enum and match arm

3. Reference from `FloorSpec` or `FloorType`

For TMX-based layouts, see [tmx.md](tmx.md).
