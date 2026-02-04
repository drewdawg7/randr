# Systems

## Quick Reference

```rust
// Basic system function signature
fn system_name(
    query: Query<&Component>,           // Query entities
    res: Res<MyResource>,               // Immutable resource
    mut res: ResMut<MyResource>,        // Mutable resource
    commands: Commands,                 // Spawn/despawn/modify
    events: MessageReader<MyEvent>,     // Read events
    writer: MessageWriter<MyEvent>,     // Write events
) { }

// System with instrumentation (project standard)
#[instrument(level = "debug", skip_all)]
fn instrumented_system(/* params */) { }

// System with recorded fields
#[instrument(level = "debug", skip_all, fields(event_count = events.len()))]
fn system_with_fields(events: MessageReader<MyEvent>) { }

// Adding systems to app
app.add_systems(Update, my_system);
app.add_systems(Startup, setup_system);
app.add_systems(OnEnter(AppState::Menu), enter_menu);

// Chained systems with run conditions
app.add_systems(
    Update,
    (system_a, system_b, system_c)
        .chain()
        .run_if(in_state(AppState::Game))
        .run_if(resource_exists::<ActiveResource>),
);
```

## Overview

Systems are functions that operate on entities, components, and resources. They run during schedules and can be configured with run conditions and ordering constraints.

**System function rules:**
- All parameters must implement `SystemParam`
- Systems run in parallel by default unless ordered

## Schedule Sets

| Schedule | When It Runs | Use Cases |
|----------|--------------|-----------|
| `Startup` | Once at app launch | Asset loading, initial spawning |
| `PreUpdate` | Before `Update` | Input processing, physics prep |
| `Update` | Every frame | Main game logic |
| `PostUpdate` | After `Update` | Cleanup, late updates |
| `OnEnter(state)` | When entering state | State-specific setup |
| `OnExit(state)` | When exiting state | State-specific cleanup |

## Run Conditions

| Condition | Description |
|-----------|-------------|
| `in_state(S)` | Run only in specific state |
| `on_message::<T>` | Run when event of type T exists |
| `resource_exists::<R>` | Run when resource exists |
| `any_with_component::<C>` | Run when any entity has component |
| `not(condition)` | Invert a condition |
| Closure | Custom inline condition |

**Codebase example** from `src/combat/plugin.rs`:
```rust
app.add_systems(
    Update,
    (
        process_player_attack.run_if(on_message::<PlayerAttackMob>),
        handle_mob_death.run_if(on_message::<EntityDied>),
        handle_player_death.run_if(on_message::<EntityDied>),
    )
        .chain()
        .run_if(in_state(AppState::Dungeon))
        .run_if(resource_exists::<ActiveCombat>),
);
```

**Inline closure** from `src/ui/screens/dungeon/plugin.rs`:
```rust
handle_dungeon_movement
    .run_if(on_message::<GameAction>)
    .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
```

## System Chaining

Use `.chain()` to enforce execution order. **Chain when:**
- System B reads data that system A writes
- Processing events in a specific order

**Codebase example** from `src/ui/screens/dungeon/plugin.rs`:
```rust
app.add_systems(
    Update,
    (
        handle_floor_ready.run_if(on_message::<FloorReady>),
        spawn_player_when_ready.run_if(resource_exists::<PendingPlayerSpawn>),
        handle_dungeon_movement.run_if(on_message::<GameAction>),
        handle_move_result.run_if(on_message::<MoveResult>),
        update_player_sprite_direction,
    )
        .chain()
        .run_if(in_state(AppState::Dungeon)),
);
```

## System Ordering

Beyond chaining, use explicit ordering with `.after()`, `.before()`, or system sets:

```rust
app.add_systems(Update, (system_a, system_b.after(system_a)));

// Named system sets for complex ordering
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum MySet { Input, Movement }

app.configure_sets(Update, MySet::Movement.after(MySet::Input));
app.add_systems(Update, read_input.in_set(MySet::Input));
```

## Instrumentation with #[instrument]

Project standard: use `#[instrument]` from `tracing` for system observability.

```rust
use tracing::instrument;

#[instrument(level = "debug", skip_all)]                              // Basic
#[instrument(level = "debug", skip_all, fields(count = q.len()))]     // With field
#[instrument(level = "debug", skip_all, fields(computed))]            // Deferred field
fn system(query: Query<&Health>) {
    tracing::Span::current().record("computed", query.iter().count());
}
```

**Codebase example** from `src/dungeon/systems/movement.rs`:
```rust
#[instrument(level = "debug", skip_all, fields(event_count = events.len()))]
pub fn handle_player_move(
    mut events: MessageReader<PlayerMoveIntent>,
    mut player_query: Query<(&mut LinearVelocity, &Transform, &Collider), With<DungeonPlayer>>,
    movement: Res<MovementConfig>,
    tile_size: Res<TileWorldSize>,
) { /* ... */ }
```

## Common System Parameters

| Parameter | Description |
|-----------|-------------|
| `Query<Q, F>` | Query entities with components |
| `Res<R>` / `ResMut<R>` | Resource access (immutable/mutable) |
| `Commands` | Deferred world modifications |
| `MessageReader<E>` / `MessageWriter<E>` | Event reading/writing |
| `Option<Res<R>>` | Optional resource (no panic if missing) |
| `Local<T>` | System-local persistent state |
| `Single<Q>` | Query expecting exactly one result |

### Custom SystemParam

Group related parameters. From `src/combat/plugin.rs`:
```rust
#[derive(SystemParam)]
struct PlayerResources<'w> {
    gold: ResMut<'w, PlayerGold>,
    progression: ResMut<'w, Progression>,
    inventory: ResMut<'w, Inventory>,
    stats: ResMut<'w, StatSheet>,
}
```

## Examples

### Event-Driven System (src/combat/plugin.rs)
```rust
#[instrument(level = "debug", skip_all)]
fn process_player_attack(
    mut events: MessageReader<PlayerAttackMob>,
    mut deal_damage_events: MessageWriter<DealDamage>,
    mut mob_query: Query<(&mut Health, &CombatStats)>,
) {
    for event in events.read() {
        let Ok((mut health, stats)) = mob_query.get_mut(event.target) else { continue };
        // Process attack...
    }
}
```

### Collision Handler (src/dungeon/systems/movement.rs)
```rust
#[instrument(level = "debug", skip_all, fields(collision_count = collision_events.len()))]
pub fn handle_player_collisions(
    mut collision_events: MessageReader<CollisionStart>,
    mut result_events: MessageWriter<MoveResult>,
    player_query: Query<Entity, With<DungeonPlayer>>,
) {
    let Ok(player_entity) = player_query.single() else { return; };
    for event in collision_events.read() { /* ... */ }
}
```

## Common Mistakes

### Forgetting run conditions on event systems
```rust
// Wrong: runs every frame
app.add_systems(Update, handle_attack);
// Correct: only runs when events exist
app.add_systems(Update, handle_attack.run_if(on_message::<Attack>));
```

### Using debug! instead of #[instrument]
```rust
// Wrong: manual logging
fn my_system() { debug!("called"); }
// Correct: automatic span
#[instrument(level = "debug", skip_all)]
fn my_system() { }
```

### Not chaining dependent systems
```rust
// Wrong: race condition
app.add_systems(Update, (write_data, read_data));
// Correct: guaranteed order
app.add_systems(Update, (write_data, read_data).chain());
```

### Applying run conditions incorrectly to tuples
```rust
// Wrong: only applies to c
app.add_systems(Update, (a, b, c.run_if(condition)));
// Correct: applies to entire tuple
app.add_systems(Update, (a, b, c).chain().run_if(condition));
```

### Missing state condition for state-specific systems
```rust
// Wrong: runs in all states
app.add_systems(Update, dungeon_logic);
// Correct: state-gated
app.add_systems(Update, dungeon_logic.run_if(in_state(AppState::Dungeon)));
```

### Not using Option for potentially missing resources
```rust
// Wrong: panics if missing
fn my_system(resource: Res<MaybeNotThere>) { }
// Correct: safe access
fn my_system(resource: Option<Res<MaybeNotThere>>) {
    let Some(resource) = resource else { return; };
}
```
