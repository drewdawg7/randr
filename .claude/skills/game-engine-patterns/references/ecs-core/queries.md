# Queries

[API Docs](https://docs.rs/bevy/0.18.0/bevy/ecs/system/struct.Query.html)

## Quick Reference

```rust
// Basic signatures
Query<&Component>                           // Read-only single component
Query<&mut Component>                       // Mutable single component
Query<(Entity, &A, &mut B)>                 // Entity ID + multiple components
Query<&A, With<B>>                          // Data with filter
Query<(&A, &B), (With<C>, Without<D>)>      // Multiple data, multiple filters

// Key methods
query.iter()                    // Iterator over matching entities (read-only)
query.iter_mut()                // Iterator over matching entities (mutable)
query.get(entity)               // Option<...> for specific entity
query.get_mut(entity)           // Option<...> mutable for specific entity
query.single()                  // Result<...> when exactly one entity expected
query.single_mut()              // Result<...> mutable when exactly one expected
query.is_empty()                // Check if no entities match
```

## Overview

Queries are the primary way to access entity data in Bevy systems. A `Query` specifies:
1. **Data** - Which components to fetch (first type parameter)
2. **Filters** - Which entities to include/exclude (optional second type parameter)

Queries are declared as system parameters and automatically run against the ECS world when the system executes.

### Key Concepts

- **Immutable vs Mutable**: Use `&Component` for read-only, `&mut Component` for write access
- **Tuples**: Combine multiple components with `(A, B, C)` syntax
- **Filters**: Narrow results without fetching data using `With<T>`, `Without<T>`, etc.
- **Entity Access**: Include `Entity` in the data tuple to get entity IDs

## Patterns

| Pattern | Use Case | Example |
|---------|----------|---------|
| `Query<&T>` | Read component data | Health display, collision checks |
| `Query<&mut T>` | Modify component data | Update positions, apply damage |
| `Query<Entity, With<T>>` | Get entity IDs with marker | Find all players, all enemies |
| `Query<(), With<T>>` | Check existence only | Verify entity has component |
| `Query<&T, Without<U>>` | Exclude entities | Query non-player entities |
| `Query<&T, Added<T>>` | React to new components | Initialize newly spawned entities |
| `Query<&T, Changed<T>>` | React to modifications | Sync changed values |
| Multiple queries | Complex logic | Collision between entity types |

## Filter Types

### `With<T>` - Include entities that have component

Filters to entities that have `T`, but does not fetch `T` data.

```rust
// Get entity IDs of all players (no component data needed)
fn find_players(query: Query<Entity, With<Player>>) {
    for entity in &query {
        // entity is Entity, not &Player
    }
}
```

### `Without<T>` - Exclude entities that have component

```rust
// Query all entities with Health that are NOT players
fn damage_non_players(query: Query<&mut Health, Without<Player>>) {
    for mut health in &mut query {
        health.current -= 1;
    }
}
```

### `Or<(A, B)>` - Include if ANY filter matches

```rust
// Query entities that have Player OR Enemy marker
fn find_combatants(query: Query<Entity, Or<(With<Player>, With<Enemy>)>>) {
    // ...
}
```

### `Added<T>` - Entities where component was just added

Matches entities where `T` was added since last system run. Useful for initialization.

```rust
// Initialize newly added sprites
fn populate_sprites<M: SpriteMarker>(
    mut commands: Commands,
    query: Query<(Entity, &M), Added<M>>,  // Only newly added markers
    resources: StaticSystemParam<M::Resources>,
) {
    for (entity, marker) in &query {
        if let Some(data) = marker.resolve(&resources) {
            commands.entity(entity).remove::<M>().insert((/* sprite components */));
        }
    }
}
```

### `Changed<T>` - Entities where component was modified

Matches entities where `T` was mutated since last system run.

```rust
// React to health changes
fn on_health_changed(query: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in &query {
        if health.current <= 0 {
            // Handle death
        }
    }
}
```

## Query Methods

### Iteration: `iter()` / `iter_mut()`

```rust
// Read-only iteration
fn display_health(query: Query<&Health>) {
    for health in &query {  // Shorthand for query.iter()
        println!("HP: {}", health.current);
    }
}

// Mutable iteration
fn heal_all(mut query: Query<&mut Health>) {
    for mut health in &mut query {  // Shorthand for query.iter_mut()
        health.current = health.max;
    }
}
```

### Random Access: `get()` / `get_mut()`

Access a specific entity by ID. Returns `Result` (Ok if entity matches query, Err otherwise).

```rust
// Check if specific entity is a mob and get its data
fn handle_collision(
    mob_query: Query<&MobEntity>,
    other_entity: Entity,
) {
    if let Ok(mob) = mob_query.get(other_entity) {
        println!("Hit mob: {:?}", mob.mob_id);
    }
}
```

### Single Entity: `single()` / `single_mut()`

When exactly one entity should match (e.g., player, camera). Returns `Result`.

```rust
// Get the single player entity
fn move_player(
    mut player_query: Query<(&mut LinearVelocity, &Transform), With<DungeonPlayer>>,
    movement: Res<MovementConfig>,
) {
    let Ok((mut velocity, transform)) = player_query.single_mut() else {
        return;  // No player or multiple players - skip
    };
    velocity.0 = Vec2::Y * movement.speed;
}
```

### Empty Check: `is_empty()`

```rust
fn check_enemies(enemy_query: Query<(), With<Enemy>>) {
    if enemy_query.is_empty() {
        println!("All enemies defeated!");
    }
}
```

## Examples

### Multiple Queries for Collision Handling

File: `src/dungeon/systems/movement.rs`

This pattern uses multiple specialized queries to identify collision targets:

```rust
pub fn handle_player_collisions(
    mut collision_events: MessageReader<CollisionStart>,
    mut result_events: MessageWriter<MoveResult>,
    mut transition_events: MessageWriter<FloorTransition>,
    mut overlapping_station: ResMut<OverlappingCraftingStation>,
    // Query 1: Find the player entity
    player_query: Query<Entity, With<DungeonPlayer>>,
    // Query 2: Get marker data for any dungeon entity
    marker_query: Query<&DungeonEntityMarker>,
    // Query 3-7: Type-specific existence checks
    mob_query: Query<&MobEntity>,
    crafting_query: Query<(), With<CraftingStationEntity>>,
    stairs_query: Query<(), With<StairsEntity>>,
    door_entity_query: Query<(), With<DoorEntity>>,
    door_tile_query: Query<(), With<is_door>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    for event in collision_events.read() {
        // Determine which entity is NOT the player
        let other = if event.collider1 == player_entity {
            event.collider2
        } else if event.collider2 == player_entity {
            event.collider1
        } else {
            continue;
        };

        // Check entity type using get() on each query
        if let Ok(mob) = mob_query.get(other) {
            if let Ok(marker) = marker_query.get(other) {
                result_events.write(MoveResult::TriggeredCombat {
                    mob_id: mob.mob_id,
                    entity: other,
                    pos: marker.pos,
                });
            }
            continue;
        }

        if crafting_query.get(other).is_ok() {
            overlapping_station.0 = Some(other);
            continue;
        }

        // ... additional type checks
    }
}
```

### Observer with Multiple Type Queries

File: `src/ui/screens/dungeon/spawn.rs`

Pattern for determining entity type in observers:

```rust
pub fn add_entity_visuals(
    trigger: On<Add, DungeonEntityMarker>,
    mut commands: Commands,
    marker_query: Query<&DungeonEntityMarker>,
    chest_query: Query<&ChestEntity>,
    rock_query: Query<&RockEntity>,
    stairs_query: Query<(), With<StairsEntity>>,
    crafting_query: Query<&CraftingStationEntity>,
    door_query: Query<(), With<DoorEntity>>,
    mob_query: Query<&MobEntity>,
    npc_query: Query<&NpcEntity>,
    // ... resources
) {
    let entity = trigger.entity;
    let Ok(marker) = marker_query.get(entity) else {
        return;
    };

    // Check each type and handle accordingly
    if let Ok(_chest) = chest_query.get(entity) {
        add_static_sprite(/* ... */);
        return;
    }

    if stairs_query.get(entity).is_ok() {
        // Handle stairs (no data needed, just existence)
        return;
    }

    if let Ok(mob) = mob_query.get(entity) {
        add_animated_mob(commands, entity, world_pos, mob.mob_id, &mob_sheets);
        return;
    }
}
```

### Animation System with Component Pairs

File: `src/ui/animation.rs`

```rust
pub fn animate_sprites(
    time: Res<Time>,
    clock: Res<AnimationClock>,
    mut query: Query<(&mut SpriteAnimation, &mut ImageNode)>,
) {
    for (mut animation, mut image) in &mut query {
        advance_animation(&time, &clock, &mut animation);
        if let Some(ref mut atlas) = image.texture_atlas {
            atlas.index = animation.current_frame;
        }
    }
}
```

## Common Mistakes

### Conflicting Mutable Access

**Problem**: Two queries that could match the same entity with overlapping mutable access.

```rust
// BAD: Both queries could match the same entity
fn broken_system(
    query_a: Query<&mut Health>,
    query_b: Query<&mut Health, With<Player>>,
) {
    // Bevy will panic at runtime!
}
```

**Solution**: Use disjoint filters or combine into one query.

```rust
// GOOD: Disjoint filters - entities match only one query
fn fixed_system(
    query_players: Query<&mut Health, With<Player>>,
    query_enemies: Query<&mut Health, Without<Player>>,
) { }

// GOOD: Single query with conditional logic
fn also_fixed(mut query: Query<(&mut Health, Option<&Player>)>) {
    for (mut health, maybe_player) in &mut query {
        if maybe_player.is_some() {
            // Player logic
        } else {
            // Non-player logic
        }
    }
}
```

### Forgetting `mut` on Query Parameter

```rust
// BAD: Can't call iter_mut() without mut
fn broken(query: Query<&mut Health>) {
    for mut health in &mut query {  // Error!
        health.current += 1;
    }
}

// GOOD: Add mut to the parameter
fn fixed(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.current += 1;
    }
}
```

### Using `single()` When Multiple Entities Possible

```rust
// BAD: Panics if zero or multiple enemies exist
fn broken(query: Query<&Enemy>) {
    let enemy = query.single();  // Panics!
}

// GOOD: Handle the Result
fn fixed(query: Query<&Enemy>) {
    let Ok(enemy) = query.single() else {
        return;  // Gracefully handle missing/multiple
    };
}

// GOOD: Use iter() if multiple expected
fn also_fixed(query: Query<&Enemy>) {
    for enemy in &query {
        // Process each
    }
}
```

### Expensive Nested Iteration

```rust
// BAD: O(n*m) - queries every entity for each entity
fn broken(query_a: Query<Entity, With<A>>, query_b: Query<&B>) {
    for entity_a in &query_a {
        for b in &query_b {  // Iterates ALL B entities
            // ...
        }
    }
}

// GOOD: Use get() for targeted lookup
fn fixed(query_a: Query<Entity, With<A>>, query_b: Query<&B>) {
    for entity_a in &query_a {
        if let Ok(b) = query_b.get(entity_a) {
            // Only processes if entity_a has B
        }
    }
}
```

### Query Filter vs Query Data Confusion

```rust
// BAD: Fetches data but doesn't use it
fn wasteful(query: Query<(&Player, &Health)>) {
    for (_, health) in &query {
        // Player data fetched but unused
    }
}

// GOOD: Use With<> filter instead
fn efficient(query: Query<&Health, With<Player>>) {
    for health in &query {
        // Only fetches Health, filters by Player
    }
}
```
