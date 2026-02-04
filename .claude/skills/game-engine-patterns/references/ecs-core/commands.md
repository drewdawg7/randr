# Commands

## Quick Reference

```rust
// System parameter
fn my_system(mut commands: Commands) { ... }

// Spawning
commands.spawn(component);                    // Single component
commands.spawn((CompA, CompB));               // Component tuple
let entity = commands.spawn(component).id();  // Get Entity ID

// Entity modification
commands.entity(entity).insert(component);    // Add component
commands.entity(entity).remove::<CompType>(); // Remove component
commands.entity(entity).despawn();            // Delete entity

// Resources
commands.insert_resource(MyResource { ... }); // Add/replace resource
commands.remove_resource::<MyResource>();     // Remove resource

// Observers
commands.trigger(MyEvent { ... });            // Trigger observer event
```

## Overview

Commands provide **deferred world mutations** - operations queued during system execution and applied at the next sync point. This is essential because systems borrow the World immutably for queries; direct mutation would cause borrow conflicts.

**When to use Commands:**
- Spawning/despawning entities
- Adding/removing components on entities you don't have mutable access to
- Managing resources from within systems
- Triggering observer events

**When NOT to use Commands (use direct mutation):**
- Modifying component data via `Query<&mut Component>` - already has mutable access
- Modifying resources via `ResMut<T>` - already has mutable access

## Patterns

| Pattern | Method | Use Case |
|---------|--------|----------|
| Spawn entity | `commands.spawn((components))` | Create new entities with initial components |
| Get spawned ID | `.spawn(...).id()` | Store entity reference for later use |
| Add component | `commands.entity(e).insert(comp)` | Add component to existing entity |
| Add multiple | `commands.entity(e).insert((A, B))` | Add component tuple to existing entity |
| Remove component | `commands.entity(e).remove::<T>()` | Strip component from entity |
| Despawn entity | `commands.entity(e).despawn()` | Delete entity and all its components |
| Insert resource | `commands.insert_resource(r)` | Add or replace a resource |
| Remove resource | `commands.remove_resource::<T>()` | Delete a resource from the world |
| Trigger observer | `commands.trigger(event)` | Fire an observer event immediately |

## Examples

### Spawning Entities with Components

From `src/dungeon/systems/spawning.rs` - spawning entities with component tuples:

```rust
fn spawn_entity<C: Component>(&self, commands: &mut Commands, world_pos: Vec2, component: C) {
    let marker = DungeonEntityMarker {
        pos: world_pos,
        size: self.entity_size(),
    };
    let entity = commands.spawn((marker, component)).id();
    if let Some(root) = self.floor_root {
        commands.entity(entity).insert(ChildOf(root));
    }
}
```

Key points:
- `spawn()` accepts component tuples `(A, B, C)`
- `.id()` returns the `Entity` for later reference
- Chain `entity().insert()` to add more components after spawn

### Despawning Entities

From `src/combat/plugin.rs` - despawning a defeated mob:

```rust
fn handle_mob_death(
    mut commands: Commands,
    mut events: MessageReader<EntityDied>,
    // ...
) {
    for event in events.read() {
        if event.is_player {
            continue;
        }
        // ... process death rewards ...

        commands.entity(event.entity).despawn();
        commands.remove_resource::<ActiveCombat>();
    }
}
```

### Resource Management

From `src/dungeon/systems/spawning.rs` - inserting and removing resources:

```rust
fn on_map_created(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    // ...
) {
    // Insert resource derived from map data
    commands.insert_resource(TileWorldSize(tile_size));

    let info = compute_tilemap_info(map_size, tilemap_tile_size, transform);
    commands.insert_resource(info);

    // ... spawn entities ...

    // Remove temporary resource after use
    commands.remove_resource::<SpawnTable>();
}
```

### Triggering Observers

From `src/crafting_station/plugin.rs` - triggering observer events:

```rust
fn poll_forge_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ForgeActiveTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.trigger(ForgeTimerFinished { entity });
        }
    }
}
```

The observer that handles this event:

```rust
fn on_forge_timer_finished(
    trigger: On<ForgeTimerFinished>,
    mut commands: Commands,
    mut crafting_events: MessageWriter<ForgeCraftingCompleteEvent>,
) {
    let entity = trigger.event().entity;
    crafting_events.write(ForgeCraftingCompleteEvent { entity });
    commands.entity(entity).remove::<ForgeActiveTimer>();
}
```

### Inserting Components in Observers

From `src/dungeon/plugin.rs` - using commands within an observer to modify entities:

```rust
fn on_collider_created(
    trigger: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
    door_query: Query<&is_door>,
    // ...
) {
    let collider_entity = trigger.event().origin;

    if let Ok(child_of) = parent_query.get(collider_entity) {
        if door_query.get(child_of.parent()).is_ok() {
            commands.entity(collider_entity).insert((Sensor, is_door(true)));
            return;
        }
    }

    commands.entity(collider_entity).insert((RigidBody::Static, TiledWallCollider));
}
```

## Common Mistakes

### Using Commands When Direct Mutation Works

```rust
// WRONG: Unnecessary command for data you already have mutable access to
fn update_health(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in &query {
        commands.entity(entity).insert(Health(100)); // Deferred, replaces component
    }
}

// RIGHT: Direct mutation via query
fn update_health(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.0 = 100; // Immediate, modifies in place
    }
}
```

### Forgetting Commands Are Deferred

```rust
// WRONG: Expecting immediate effect
fn spawn_and_query(mut commands: Commands, query: Query<&MyComponent>) {
    let entity = commands.spawn(MyComponent).id();
    // This will NOT find the entity - spawn hasn't executed yet!
    if query.get(entity).is_ok() { /* never reached */ }
}

// RIGHT: Use observers or next-frame systems for follow-up
fn spawn_entity(mut commands: Commands) {
    commands.spawn(MyComponent);
}

fn process_new_entities(query: Query<&MyComponent, Added<MyComponent>>) {
    for comp in &query { /* Runs after spawn is applied */ }
}
```

### Despawning Without Checking References

```rust
// RISKY: Other systems may hold this entity reference
commands.entity(entity).despawn();

// SAFER: Mark for cleanup, let dedicated system handle despawn
commands.entity(entity).insert(MarkedForDespawn);
```

### Using Stale Entity References

```rust
// WRONG: Entity may have been despawned
let entity = some_stored_entity;
commands.entity(entity).insert(Component); // Panic if entity doesn't exist

// RIGHT: Verify entity exists first
if world.entities().contains(entity) {
    commands.entity(entity).insert(Component);
}
// Or use get_entity() which returns Option
if let Some(mut entity_commands) = commands.get_entity(entity) {
    entity_commands.insert(Component);
}
```
