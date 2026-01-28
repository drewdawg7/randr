# Dungeon Module

Data-driven dungeon system at `src/dungeon/`.

## Quick Links

| Topic | File |
|-------|------|
| Floors, FloorType, FloorId | [floors.md](floors.md) |
| Layouts, LayoutBuilder | [layouts.md](layouts.md) |
| Entity spawning, SpawnTable | [spawning.md](spawning.md) |
| TMX/Tiled map support | [tmx.md](tmx.md) |
| Player movement | [movement.md](movement.md) |
| Dungeon entities | [entities.md](entities.md) |

## Conceptual Hierarchy

```
Location (e.g., "Goblin Cave")
  └── Floor (Fixed FloorId OR Generated from FloorType)
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
    Wall,        // Impassable
    Floor,       // Walkable
    Entrance,    // Player spawn
    Exit,        // Stairs/door
    Door,        // Decorative (impassable)
    DoorOpen,    // Open door
    PlayerSpawn, // Renders as GateFloor
    SpawnPoint,  // Renders as normal floor
    TorchWall,   // Animated torch
    Empty,       // TMX tile ID 0
}
```

## DungeonPlugin Registration

```rust
// In game.rs
app.add_plugins(
    DungeonPlugin::new()
        .location(LocationId::Home)
            .floor(FloorId::HomeFloor)
        .location(LocationId::MainDungeon)
            .generated_floors(3, WeightedFloorPool::new()
                .add(FloorType::BasicDungeonFloor, 100))
        .build()
);
```

## File Structure

```
src/dungeon/
    mod.rs, plugin.rs, state.rs
    tile.rs, entity.rs, grid.rs
    layout.rs, layout_builder.rs
    spawn.rs, generator.rs, rendering.rs
    tmx.rs, tmx_tileset.rs          # TMX support
    floor/
        definitions.rs, floor_type.rs
        weighted_pool.rs, generated.rs
    layouts/
        mod.rs, starting_room.rs
        tmx_cave_floor.rs           # TMX loader
```
