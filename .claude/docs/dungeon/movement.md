# Player Movement

Movement system in `src/ui/screens/dungeon/plugin.rs`.

## DungeonFloor Observer

Spawn `DungeonFloor` component to render a floor:

```rust
commands.spawn(DungeonFloor {
    layout: layout.clone(),
    player_pos: state.player_pos,
    player_size: state.player_size,
    floor_type: FloorType::BasicDungeonFloor,
});
```

The `on_add_dungeon_floor` observer handles all rendering.

## Movement Rules

- Only walkable tiles (`layout.is_walkable()`)
- Cannot move onto occupied cells (`GridOccupancy`)
- Arrow keys → `GameAction::Navigate` events

## Collision Handling

| Entity | Behavior |
|--------|----------|
| Mob | Triggers fight modal |
| Chest | Blocked (obstacle) |
| Stairs | Advances floor |
| Rock | Blocked until mined |
| NPC | Opens merchant modal |

## Multi-Cell Collision

Movement validates **all cells** player would occupy:

```rust
fn all_cells_walkable(layout: &DungeonLayout, pos: GridPosition, size: GridSize) -> bool {
    pos.occupied_cells(size).all(|(x, y)| layout.is_walkable(x, y))
}
```

## Key Functions

| Function | Purpose |
|----------|---------|
| `spawn_dungeon_screen()` | Enter dungeon, load layout |
| `on_add_dungeon_floor()` | Render UI hierarchy |
| `handle_dungeon_movement()` | Process input, check collisions |
| `advance_floor_system()` | Handle stairs interaction |
| `cleanup_dungeon()` | Exit dungeon cleanup |

## GridOccupancy

Tracks cell occupation:

```rust
let mut occupancy = GridOccupancy::new(width, height);
occupancy.occupy(pos, size, entity);
occupancy.is_occupied(x, y);
occupancy.entity_at(x, y);
occupancy.vacate(pos, size);
```

## SmoothPosition

Visual interpolation for movement:

```rust
#[derive(Component)]
pub struct SmoothPosition {
    pub current: Vec2,
    pub target: Vec2,
}
```

Updated each frame to smoothly animate between grid positions.
