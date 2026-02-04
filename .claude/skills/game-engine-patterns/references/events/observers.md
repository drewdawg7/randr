# Observers

## Quick Reference

```rust
// Define event for observers
#[derive(Event)]
struct MyEvent { data: i32 }

// Global observer (watches all events of this type)
app.add_observer(|trigger: On<MyEvent>| {
    println!("Received: {}", trigger.event().data);
});

// Observer with system parameters
app.add_observer(|trigger: On<MyEvent>, mut commands: Commands, query: Query<&Health>| {
    // Full access to ECS
});

// Entity-targeted observer
commands.spawn(Enemy).observe(|trigger: On<Damage>| {
    // Only this entity receives
});

// Trigger the event
commands.trigger(MyEvent { data: 42 });
```

## Overview

Observers are **push-based, reactive systems** that run when events are triggered. Unlike regular systems that run on schedules, observers execute immediately (or when commands flush).

**Key characteristics:**
- Execute in response to triggers, not on a schedule
- Can access full system parameters (queries, resources, commands)
- Can be global (all events) or entity-targeted
- Observers are entities themselves with the `Observer` component

## Event Trait

Events for observers use `#[derive(Event)]`:

```rust
// Basic event
#[derive(Event)]
struct GameOver {
    score: u32,
}

// Simple event without data
#[derive(Event)]
struct PlayerDied;
```

### EntityEvent

For events targeting specific entities:

```rust
// Auto-detects 'entity' field
#[derive(EntityEvent)]
struct Damage {
    entity: Entity,
    amount: u32,
}

// Custom target field
#[derive(EntityEvent)]
struct Attack {
    #[event_target]
    target: Entity,
    damage: u32,
}

// Tuple struct
#[derive(EntityEvent)]
struct Click(Entity);
```

### Hybrid Events

This codebase uses hybrid events that work with both systems:

```rust
// From src/ui/screens/modal.rs
#[derive(Event, Message, Debug, Clone, Copy)]
pub struct OpenModal(pub ModalType);
```

Benefits:
- Can be triggered for observers: `commands.trigger(OpenModal(...))`
- Can be written for systems: `writer.write(OpenModal(...))`

## Creating Observers

### Global Observers

Watch all events of a type:

```rust
// On App (plugin setup)
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_game_over);
    }
}

fn on_game_over(trigger: On<GameOver>) {
    println!("Score: {}", trigger.event().score);
}
```

### Observer with System Parameters

Observers can use full system parameters:

```rust
// From src/game/npc_interactions.rs
fn on_merchant_interaction(_trigger: On<MerchantInteraction>, mut commands: Commands) {
    commands.insert_resource(MerchantStock::generate());
    commands.trigger(OpenModal(ModalType::MerchantModal));
}
```

More complex example:

```rust
app.add_observer(
    |trigger: On<ExplodeMines>,
     mines: Query<(Entity, &Transform), With<Mine>>,
     mut commands: Commands| {
        let explosion_pos = trigger.event().position;
        for (entity, transform) in &mines {
            if transform.translation.distance(explosion_pos) < 50.0 {
                commands.trigger_targets(Explode, entity);
            }
        }
    }
);
```

### Entity-Targeted Observers

Attach observers to specific entities:

```rust
// Via observe() on EntityCommands
commands.spawn(Enemy)
    .observe(|trigger: On<Damage>| {
        println!("This enemy took damage");
    });

// Observer only fires for this entity
```

### Generic Observers

Use generics for reusable patterns:

```rust
// From src/ui/modal_registry.rs
pub fn on_open_modal<M: RegisteredModal>(
    trigger: On<OpenModal>,
    mut commands: Commands,
) {
    if trigger.event().0 != M::MODAL_TYPE {
        return;
    }
    commands.queue(SpawnModalCommand::<M>(PhantomData));
}

// Register for each modal type
app.add_observer(on_open_modal::<InventoryModal>)
   .add_observer(on_open_modal::<SettingsModal>);
```

## The On<E> Type

`On<E>` (formerly `Trigger<E>`) is the parameter observers receive:

### Accessing Event Data

```rust
fn my_observer(trigger: On<MyEvent>) {
    // Get reference to event data
    let data = trigger.event();  // &MyEvent

    // For EntityEvents, get target
    let target = trigger.target();  // Option<Entity>
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `event()` | Reference to the event data (`&E`) |
| `target()` | Target entity for EntityEvents (`Option<Entity>`) |
| `original_target()` | Original target before propagation |
| `propagate(bool)` | Control event propagation (if enabled) |

### Underscore Pattern

If you don't need the trigger data:

```rust
// From src/game/npc_interactions.rs
fn on_merchant_interaction(_trigger: On<MerchantInteraction>, mut commands: Commands) {
    // Only care about side effects
}
```

## Observer Registration Patterns

### In Plugin Build

```rust
impl Plugin for NpcInteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_merchant_interaction);
    }
}
```

### Extension Trait Pattern

```rust
// From src/ui/modal_registry.rs
pub trait RegisterModalExt {
    fn register_modal<M: RegisteredModal>(&mut self) -> &mut Self;
}

impl RegisterModalExt for App {
    fn register_modal<M: RegisteredModal>(&mut self) -> &mut Self {
        self.add_observer(on_open_modal::<M>)
            .add_observer(on_close_modal::<M>)
    }
}

// Usage
app.register_modal::<InventoryModal>();
```

### Manual Spawning

```rust
// Spawn observer as entity
world.spawn(Observer::new(my_handler));

// With configuration
world.spawn(
    Observer::new(my_handler)
        .with_entity(some_entity)
);
```

## Observer Configuration

### Watching Specific Entities

```rust
// Single entity
Observer::new(my_handler).with_entity(entity)

// Multiple entities
Observer::new(my_handler).with_entities([entity_a, entity_b])
```

### Watching Component Lifecycle

```rust
// Watch when Health is added to any entity
app.add_observer(|trigger: On<Add, Health>| {
    println!("Health added to {:?}", trigger.target());
});
```

## Execution Timing

### Commands::trigger (Deferred)

```rust
fn my_system(mut commands: Commands) {
    commands.trigger(MyEvent { data: 42 });
    // Observers haven't run yet!
    // They run at next command sync point
}
```

### World::trigger (Immediate)

```rust
// In a Command's apply method
impl Command for MyCommand {
    fn apply(self, world: &mut World) {
        world.trigger(MyEvent);
        // Observers have already run
    }
}
```

### Command Queuing in Observers

Commands queued by observers don't apply immediately:

```rust
app.add_observer(|trigger: On<Event1>, mut commands: Commands| {
    commands.trigger(Event2);  // Queued, not immediate
    commands.spawn(Entity);     // Also queued
});
// All observers for Event1 run first
// Then all queued commands apply
// Then observers for Event2 run
```

## Observer Ordering

**Important:** Bevy does NOT guarantee observer execution order.

```rust
// Order is arbitrary!
app.add_observer(observer_a);
app.add_observer(observer_b);
```

### Workarounds

1. **Chain events:** First observer triggers second event
2. **Use messages:** For ordered processing with `.chain()`
3. **Single observer:** Combine logic into one observer

## Common Patterns

### Chain Reaction

```rust
// Mine explosion triggers nearby mines
app.add_observer(
    |trigger: On<Explode>,
     transforms: Query<&Transform>,
     mines: Query<Entity, With<Mine>>,
     mut commands: Commands| {
        let pos = transforms.get(trigger.target().unwrap()).unwrap();
        for mine in &mines {
            let mine_pos = transforms.get(mine).unwrap();
            if pos.translation.distance(mine_pos.translation) < 50.0 {
                commands.trigger_targets(Explode, mine);
            }
        }
    }
);
```

### Observer Writes Message

Hybrid pattern: observer detects, message processes:

```rust
app.add_observer(
    |trigger: On<Collision>,
     mut writer: MessageWriter<DamageEvent>| {
        // Observer for immediate detection
        // Message for ordered system processing
        writer.write(DamageEvent {
            target: trigger.target().unwrap(),
            amount: 10,
        });
    }
);
```

### Conditional Response

```rust
fn on_open_modal<M: RegisteredModal>(
    trigger: On<OpenModal>,
    mut commands: Commands,
) {
    // Filter to only this modal type
    if trigger.event().0 != M::MODAL_TYPE {
        return;
    }
    // Process...
}
```

## Common Mistakes

### Assuming Observer Order

```rust
// Wrong: order not guaranteed
app.add_observer(save_score);   // May run second!
app.add_observer(update_score); // May run first!

// Correct: use event chaining
app.add_observer(|trigger: On<ScoreChanged>, mut commands: Commands| {
    update_score();
    commands.trigger(ScoreSaved);  // Chain to next observer
});
```

### Blocking in Observers

```rust
// Wrong: blocks all observers
app.add_observer(|trigger: On<Calculate>| {
    expensive_blocking_operation();
});

// Correct: defer work
app.add_observer(|trigger: On<Calculate>, mut commands: Commands| {
    commands.trigger(ProcessCalculation { data: trigger.event().data.clone() });
});
```

### Forgetting Entity Observer Cleanup

Entity observers despawn when their target entity despawns. Don't store references expecting persistence.

```rust
// Observer is automatically cleaned up when enemy despawns
commands.spawn(Enemy).observe(|t: On<Damage>| { /* ... */ });
```

### Mixing World::trigger and Commands::trigger

```rust
// In Command::apply - this is immediate
world.trigger(MyEvent);

// In systems - this is deferred
commands.trigger(MyEvent);

// Don't mix expectations!
```
