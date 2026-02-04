# Component Lifecycle Events

## Quick Reference

```rust
// Observe component addition
app.add_observer(|trigger: On<Add, Health>| {
    println!("Health added to {:?}", trigger.target());
});

// Observe component removal
app.add_observer(|trigger: On<Remove, Health>, mut commands: Commands| {
    if let Some(entity) = trigger.target() {
        commands.trigger_targets(EntityDied, entity);
    }
});

// Observe entity despawn
app.add_observer(|trigger: On<Despawn, Enemy>| {
    println!("Enemy despawned: {:?}", trigger.target());
});

// Entity-attached lifecycle observer
commands.spawn((Player, Health(100)))
    .observe(|trigger: On<Remove, Health>| {
        println!("This player lost health component");
    });
```

## Overview

Component lifecycle events trigger observers when components are added, removed, or modified. They're useful for:

- Cleanup when components are removed
- Initialization when components are added
- Tracking entity state changes
- Maintaining invariants

## Lifecycle Event Types

### Add

Triggers when a component is added for the **first time**:

```rust
app.add_observer(|trigger: On<Add, Health>| {
    let entity = trigger.target().unwrap();
    println!("Entity {:?} now has Health", entity);
});
```

**Does NOT trigger for:**
- Replacing an existing component
- Re-adding after removal (that's a new Add)

### Insert

Triggers whenever a component is inserted (add **or** replace):

```rust
app.add_observer(|trigger: On<Insert, Transform>| {
    // Fires for both new additions AND overwrites
    println!("Transform set on {:?}", trigger.target());
});
```

### Replace

Triggers when an existing component is **overwritten**:

```rust
app.add_observer(|trigger: On<Replace, Health>| {
    // Only fires when Health already existed and was replaced
    println!("Health updated on {:?}", trigger.target());
});
```

### Remove

Triggers when a component is removed from an entity:

```rust
app.add_observer(|trigger: On<Remove, Health>| {
    if let Some(entity) = trigger.target() {
        // Component is being removed
        // Entity still exists (for now)
    }
});
```

### Despawn

Triggers when an entity is despawned:

```rust
// Watch all despawns of entities with Enemy component
app.add_observer(|trigger: On<Despawn, Enemy>| {
    println!("Enemy despawned");
});

// Generic despawn watcher
app.add_observer(|trigger: On<Despawn>| {
    println!("Some entity despawned: {:?}", trigger.target());
});
```

## Event Comparison

| Event | Triggers When |
|-------|---------------|
| `Add<C>` | Component added (first time only) |
| `Insert<C>` | Component added OR replaced |
| `Replace<C>` | Existing component overwritten |
| `Remove<C>` | Component removed from entity |
| `Despawn` | Entity despawned |

### Trigger Sequences

**Spawning with component:**
```rust
commands.spawn(Health(100));
// Triggers: Add<Health>
```

**Inserting on existing entity:**
```rust
commands.entity(e).insert(Health(100));
// If no Health: Add<Health>
// If has Health: Replace<Health>
// Both cases: Insert<Health>
```

**Removing component:**
```rust
commands.entity(e).remove::<Health>();
// Triggers: Remove<Health>
```

**Despawning entity:**
```rust
commands.entity(e).despawn();
// Triggers: Remove<C> for each component C
// Then: Despawn
```

## Observer Parameters

### Accessing Event Data

```rust
app.add_observer(|trigger: On<Add, Health>| {
    // Get target entity
    let entity = trigger.target();  // Option<Entity>

    // Get component ID
    let component = trigger.components();  // impl Iterator<Item = ComponentId>
});
```

### With System Parameters

```rust
app.add_observer(
    |trigger: On<Remove, Equipped>,
     mut inventory: ResMut<Inventory>,
     query: Query<&ItemData>| {
        if let Some(entity) = trigger.target() {
            if let Ok(item) = query.get(entity) {
                inventory.unequip(item.id);
            }
        }
    }
);
```

### With Commands

```rust
app.add_observer(
    |trigger: On<Add, Enemy>,
     mut commands: Commands| {
        if let Some(entity) = trigger.target() {
            // Add related components
            commands.entity(entity)
                .insert(AiController::default())
                .insert(CombatStats::default());
        }
    }
);
```

## Registration Methods

### Global Observers

Watch all entities:

```rust
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_health_added)
           .add_observer(on_health_removed);
    }
}

fn on_health_added(trigger: On<Add, Health>) {
    // Fires for any entity gaining Health
}
```

### Entity-Specific Observers

Watch only one entity:

```rust
commands.spawn((Player, Health(100)))
    .observe(|trigger: On<Remove, Health>| {
        // Only fires for this specific player
    });
```

### Watching Multiple Components

```rust
// Manual observer with multiple components
world.spawn(
    Observer::new(my_handler)
        .with_components([
            ComponentId::of::<Health>(),
            ComponentId::of::<Mana>(),
        ])
);
```

## Hooks vs Observers

Bevy has two systems for component lifecycle:

| Aspect | Hooks | Observers |
|--------|-------|-----------|
| Definition | On Component trait | Registered at runtime |
| Multiplicity | One per component | Multiple allowed |
| Add order | Hooks first | Observers after hooks |
| Remove order | Observers first | Hooks after observers |
| Use case | "Innate" behavior | External reactions |

### Component Hooks

Define behavior on the component itself:

```rust
#[derive(Component)]
#[component(on_add = on_player_add, on_remove = on_player_remove)]
struct Player;

fn on_player_add(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    // Runs BEFORE observers
}

fn on_player_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    // Runs AFTER observers
}
```

### When to Use Each

| Scenario | Use |
|----------|-----|
| Component must always do X when added | Hook |
| External system needs to react | Observer |
| Multiple reactions needed | Observers |
| Self-contained component logic | Hook |

## Common Patterns

### Cleanup on Removal

```rust
app.add_observer(
    |trigger: On<Remove, RenderMesh>,
     mut assets: ResMut<Assets<Mesh>>| {
        // Clean up associated asset
        if let Some(handle) = get_mesh_handle(trigger.target()) {
            assets.remove(handle);
        }
    }
);
```

### Initialize Related Components

```rust
app.add_observer(
    |trigger: On<Add, Enemy>,
     mut commands: Commands| {
        if let Some(entity) = trigger.target() {
            commands.entity(entity)
                .insert(Health::default())
                .insert(CombatStats::default())
                .insert(LootTable::default());
        }
    }
);
```

### Track Active Entities

```rust
#[derive(Resource, Default)]
struct ActiveEnemies(HashSet<Entity>);

app.add_observer(
    |trigger: On<Add, Enemy>,
     mut active: ResMut<ActiveEnemies>| {
        if let Some(entity) = trigger.target() {
            active.0.insert(entity);
        }
    }
);

app.add_observer(
    |trigger: On<Remove, Enemy>,
     mut active: ResMut<ActiveEnemies>| {
        if let Some(entity) = trigger.target() {
            active.0.remove(&entity);
        }
    }
);
```

### Propagate State Changes

```rust
app.add_observer(
    |trigger: On<Remove, Health>,
     mut commands: Commands| {
        if let Some(entity) = trigger.target() {
            // Entity lost health component = death
            commands.trigger_targets(EntityDied, entity);
        }
    }
);
```

### Parent-Child Relationships

```rust
app.add_observer(
    |trigger: On<Despawn, Parent>,
     children: Query<&Children>,
     mut commands: Commands| {
        if let Some(parent) = trigger.target() {
            if let Ok(kids) = children.get(parent) {
                for &child in kids.iter() {
                    commands.entity(child).despawn();
                }
            }
        }
    }
);
```

## Common Mistakes

### Accessing Removed Component

```rust
// Wrong: component is being removed, can't query it
app.add_observer(
    |trigger: On<Remove, Health>,
     query: Query<&Health>| {
        let entity = trigger.target().unwrap();
        let health = query.get(entity);  // May fail!
    }
);

// Correct: component data may not be available
// Use a different approach or store data elsewhere
```

### Modifying During Despawn

```rust
// Wrong: entity is being despawned
app.add_observer(
    |trigger: On<Despawn>,
     mut commands: Commands| {
        let entity = trigger.target().unwrap();
        commands.entity(entity).insert(Something);  // Won't work!
    }
);

// Correct: spawn new entity or modify others
app.add_observer(
    |trigger: On<Despawn, Enemy>,
     mut commands: Commands| {
        // Spawn replacement or effect
        commands.spawn(DeathEffect::at(/* position */));
    }
);
```

### Circular Triggers

```rust
// Wrong: infinite loop
app.add_observer(
    |trigger: On<Add, A>,
     mut commands: Commands| {
        commands.entity(trigger.target().unwrap()).insert(B);
    }
);
app.add_observer(
    |trigger: On<Add, B>,
     mut commands: Commands| {
        commands.entity(trigger.target().unwrap()).insert(A);  // Loop!
    }
);

// Correct: check for existing component
app.add_observer(
    |trigger: On<Add, A>,
     query: Query<(), With<B>>,
     mut commands: Commands| {
        let entity = trigger.target().unwrap();
        if query.get(entity).is_err() {
            commands.entity(entity).insert(B);
        }
    }
);
```

### Forgetting Option<Entity>

```rust
// Wrong: unwrap may panic
app.add_observer(|trigger: On<Add, Health>| {
    let entity = trigger.target().unwrap();  // Could panic!
});

// Correct: handle Option
app.add_observer(|trigger: On<Add, Health>| {
    let Some(entity) = trigger.target() else { return };
    // Safe to use entity
});
```

## Migration Note (0.16 to 0.17+)

| Old (0.16) | New (0.17+) |
|------------|-------------|
| `Trigger<OnAdd, C>` | `On<Add, C>` |
| `Trigger<OnInsert, C>` | `On<Insert, C>` |
| `Trigger<OnRemove, C>` | `On<Remove, C>` |
| `trigger.entity()` | `trigger.target()` (returns `Option<Entity>`) |
