# Dungeon Module

Data-driven dungeon system at `src/dungeon/`.

## Quick Links

| Topic | File |
|-------|------|
| Floors, FloorType, FloorId | [floors.md](floors.md) |
| Layouts, DungeonLayout, LayoutId | [layouts.md](layouts.md) |
| Entity spawning, SpawnTable | [spawning.md](spawning.md) |
| TMX/Tiled map support | [tmx.md](tmx.md) |
| Player movement | [movement.md](movement.md) |
| Dungeon entities | [entities.md](entities.md) |

## Conceptual Hierarchy

```
Location (e.g., "MainDungeon")
  └── Floor (FloorId)
       └── Layout (tile grid + entities)
```

## Core Types (Quick Reference)

| Type | Purpose | File |
|------|---------|------|
| `DungeonPlugin` | Registers dungeon locations | `plugin.rs` |
| `DungeonRegistry` | Runtime config queries | `plugin.rs` |
| `DungeonState` | Player position, current floor | `state.rs` |
| `DungeonLayout` | Tile grid + entities | `layout.rs` |
| `TileType` | Wall, Floor, Empty, etc. | `tile.rs` |
| `GridPosition` | Grid coordinates | `grid.rs` |
| `GridOccupancy` | Cell occupation tracking | `grid.rs` |

## TileType Variants

```rust
pub enum TileType {
    Wall,
    Floor,
    Entrance,
    Exit,
    Door,
    DoorOpen,
    PlayerSpawn,
    SpawnPoint,
    TorchWall,
    Empty,
}
```

## DungeonPlugin Registration

```rust
app.add_plugins(
    DungeonPlugin::new()
        .location(LocationId::Home)
            .floor(FloorId::HomeFloor)
        .location(LocationId::MainDungeon)
            .floor(FloorId::MainDungeon1)
            .floor(FloorId::MainDungeon2)
            .floor(FloorId::MainDungeon3)
        .build()
);
```

## File Structure

```
src/dungeon/
    mod.rs, plugin.rs, state.rs
    tile.rs, entity.rs, grid.rs
    layout.rs, spawn.rs, rendering.rs
    tmx.rs, tmx_tileset.rs
    config.rs
    floor/
        definitions.rs, floor_type.rs
    layouts/
        mod.rs, layout_id.rs
        tmx_cave_floor.rs, tmx_home_floor.rs
```
