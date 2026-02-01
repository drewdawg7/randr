# Player Movement

Movement system uses Avian2d physics in `src/dungeon/systems/movement.rs`.

## Physics-Based Movement

Player movement uses Avian2d `LinearVelocity` for smooth physics:

```rust
velocity.0 = direction * PLAYER_SPEED;
```

Movement stops when no keys are pressed via `stop_player_when_idle`.

## Collision Handling

Collisions are detected via Avian2d `CollisionStart` events in `handle_player_collisions`:

| Entity | Behavior |
|--------|----------|
| Mob | Triggers fight modal |
| Door | Triggers `FloorTransition::EnterDoor` (pass-through sensor) |
| Stairs | Triggers `FloorTransition::AdvanceFloor` |
| Chest | Blocked (obstacle) |
| Rock | Blocked until mined |
| NPC | Opens merchant modal |

## Door Collision

Doors use `Sensor` colliders (pass-through):
- Player walks through the door visual (cave opening tile)
- Invisible sensor entity detects collision
- Triggers `FloorTransition::EnterDoor` event
- `handle_floor_transition` processes the transition

## Player Collider

Player has a 16x16 collider offset to align with sprite feet:

```rust
Collider::compound(vec![(
    Vec2::new(0.0, -(32.0 / 2.0) + (16.0 / 2.0)),
    0.0,
    Collider::rectangle(16.0, 16.0),
)])
```

## Collision Layers

Defined in `src/dungeon/mod.rs`:

| Layer | Collides With |
|-------|---------------|
| Player | Default, Mob, StaticEntity, Trigger |
| Mob | Player |
| StaticEntity | Player |
| Trigger | Player |

Doors and Stairs use `GameLayer::Trigger`.

## TileIndex

The `TileIndex` resource provides O(1) lookup for tile properties:

```rust
#[derive(Resource, Default)]
pub struct TileIndex {
    solid: HashSet<(u32, u32)>,
    doors: HashSet<(u32, u32)>,
}
```

Built by `build_tile_index` observer when map is created, querying `is_solid` and `is_door` tile components.

## Key Functions

| Function | Purpose |
|----------|---------|
| `handle_player_move` | Apply velocity from input |
| `stop_player_when_idle` | Zero velocity when no keys pressed |
| `handle_player_collisions` | Process collision events |
| `handle_floor_transition` | Handle door/stairs transitions |

## GridOccupancy

Tracks cell occupation:

```rust
let mut occupancy = GridOccupancy::new(width, height);
occupancy.occupy(pos, size, entity);
occupancy.is_occupied(x, y);
occupancy.entity_at(x, y);
occupancy.vacate(pos, size);
```
