# Triggers

## Quick Reference

```rust
// Define an event
#[derive(Event)]
struct MyEvent { data: i32 }

// In a system: deferred trigger
fn my_system(mut commands: Commands) {
    commands.trigger(MyEvent { data: 42 });
}

// Target specific entity
commands.trigger_targets(Damage { amount: 10 }, enemy_entity);

// Target multiple entities
commands.trigger_targets(Heal { amount: 5 }, [player, ally]);

// In a Command: immediate trigger
impl Command for MyCommand {
    fn apply(self, world: &mut World) {
        world.trigger(MyEvent { data: 42 });
    }
}
```

## Overview

Triggers fire events that observers react to. The key distinction is **when** observers execute:

| Method | Context | Execution |
|--------|---------|-----------|
| `commands.trigger()` | Systems | **Deferred** - at next command sync |
| `world.trigger()` | Command::apply | **Immediate** - right now |

## Commands::trigger

Used in systems for deferred event firing:

```rust
fn my_system(mut commands: Commands) {
    // Event is queued, observers run when commands flush
    commands.trigger(MyEvent { data: 42 });

    // More code runs before observers
    do_other_stuff();
}
```

### Fluent API

Chain trigger with other entity commands:

```rust
commands.spawn(Enemy)
    .insert(Health(100))
    .observe(|t: On<Damage>| { /* ... */ })
    .trigger(Spawned);  // Trigger on newly spawned entity
```

## Commands::trigger_targets

Target specific entities:

```rust
// Single entity
commands.trigger_targets(Damage { amount: 10 }, enemy_entity);

// Multiple entities
commands.trigger_targets(AreaDamage { amount: 5 }, [enemy1, enemy2, enemy3]);

// Entity slice
let targets: Vec<Entity> = query.iter().collect();
commands.trigger_targets(MassHeal { amount: 20 }, targets);
```

**Note:** The event must implement `EntityEvent` (have an entity target field):

```rust
#[derive(EntityEvent)]
struct Damage {
    entity: Entity,  // Auto-detected target
    amount: u32,
}

// Or with explicit target field
#[derive(EntityEvent)]
struct Attack {
    #[event_target]
    target: Entity,
    damage: u32,
}
```

## World::trigger

Used in `Command::apply` for immediate execution:

```rust
// From src/ui/modal_registry.rs
impl<M: RegisteredModal> Command for ToggleModalCommand<M> {
    fn apply(self, world: &mut World) {
        let modal_entity = /* find modal */;

        if modal_entity.is_some() {
            // Immediate: observers run NOW
            world.trigger(CloseModal(M::MODAL_TYPE));
        } else if no_modal_open {
            world.trigger(OpenModal(M::MODAL_TYPE));
        }
    }
}
```

### When to Use

| Scenario | Use |
|----------|-----|
| Normal systems | `commands.trigger()` |
| Command::apply | `world.trigger()` |
| Need immediate response | `world.trigger()` in Command |
| Chaining events | Usually `commands.trigger()` |

## Codebase Examples

### Observer Triggers Another Event

```rust
// From src/game/npc_interactions.rs
fn on_merchant_interaction(_trigger: On<MerchantInteraction>, mut commands: Commands) {
    commands.insert_resource(MerchantStock::generate());
    commands.trigger(OpenModal(ModalType::MerchantModal));  // Trigger next event
}
```

### Command Triggers on Condition

```rust
// From src/ui/modal_registry.rs
impl<M: RegisteredModal> Command for ToggleModalCommand<M> {
    fn apply(self, world: &mut World) {
        let mut query = world.query_filtered::<Entity, With<M::Root>>();
        let modal_entity = query.iter(world).next();

        if modal_entity.is_some() {
            world.trigger(CloseModal(M::MODAL_TYPE));
        } else {
            let active_modal = world.resource::<ActiveModal>();
            if active_modal.modal.is_none() {
                world.trigger(OpenModal(M::MODAL_TYPE));
            }
        }
    }
}
```

### Timer Completion Triggers

```rust
// From src/crafting_station/plugin.rs
fn check_forge_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ForgeTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.trigger(ForgeTimerFinished { entity });
        }
    }
}
```

## Trigger Flow

### Deferred (commands.trigger)

```
1. System A: commands.trigger(Event1)
2. System A continues executing
3. Other systems run
4. Command flush point
5. Observers for Event1 execute
6. If observers queued commands, those flush next
```

### Immediate (world.trigger)

```
1. Command::apply calls world.trigger(Event1)
2. All observers for Event1 run immediately
3. Command::apply continues
4. Any commands queued by observers are queued (not immediate)
```

## Custom TriggerTargets

For advanced targeting scenarios:

```rust
struct ComponentTarget(Entity, ComponentId);

impl TriggerTargets for ComponentTarget {
    fn entities(&self) -> impl ExactSizeIterator<Item = Entity> {
        std::iter::once(self.0)
    }

    fn components(&self) -> impl ExactSizeIterator<Item = ComponentId> {
        std::iter::once(self.1)
    }
}

// Usage
commands.trigger_targets(MyEvent, ComponentTarget(entity, component_id));
```

## Common Patterns

### State Machine Transitions

```rust
fn handle_game_state(
    mut commands: Commands,
    state: Res<GameState>,
) {
    match state.current {
        State::Playing if player_won() => {
            commands.trigger(GameWon);
        }
        State::Playing if player_lost() => {
            commands.trigger(GameLost);
        }
        _ => {}
    }
}
```

### Cascading Events

```rust
// First observer triggers second event
app.add_observer(|t: On<EnemyDefeated>, mut commands: Commands| {
    let xp = calculate_xp(t.event());
    commands.trigger(XpGained { amount: xp });
});

// Second observer triggers third
app.add_observer(|t: On<XpGained>, mut commands: Commands, mut player: ResMut<Player>| {
    player.xp += t.event().amount;
    if player.xp >= player.next_level_xp {
        commands.trigger(LevelUp);
    }
});
```

### Conditional Targeting

```rust
fn area_attack(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    let player_pos = player.single().translation;

    let targets: Vec<Entity> = enemies
        .iter()
        .filter(|(_, t)| t.translation.distance(player_pos) < 100.0)
        .map(|(e, _)| e)
        .collect();

    if !targets.is_empty() {
        commands.trigger_targets(AreaDamage { amount: 25 }, targets);
    }
}
```

## Common Mistakes

### Expecting Immediate Execution in Systems

```rust
// Wrong: observers haven't run yet
fn my_system(mut commands: Commands, resource: Res<MyResource>) {
    commands.trigger(UpdateResource);
    // resource is still the old value!
    println!("{:?}", resource);  // Old data
}

// Correct: chain systems or use observer result
fn update_and_use(
    mut commands: Commands,
    mut events: MessageWriter<ResourceUpdated>,
) {
    commands.trigger(UpdateResource);
    // Or write a message for a chained system to read
}
```

### Using world.trigger in Systems

```rust
// Wrong: can't access World directly in systems
fn my_system(world: &mut World) {  // Invalid signature!
    world.trigger(MyEvent);
}

// Correct: use commands in systems
fn my_system(mut commands: Commands) {
    commands.trigger(MyEvent);
}
```

### Forgetting to Register Observer

```rust
// Wrong: trigger fires but nothing happens
fn my_system(mut commands: Commands) {
    commands.trigger(UnobservedEvent);  // No observer registered!
}

// Correct: register observer in plugin
app.add_observer(|t: On<MyEvent>| { /* ... */ });
```

### Infinite Trigger Loops

```rust
// Wrong: infinite loop
app.add_observer(|t: On<Ping>, mut commands: Commands| {
    commands.trigger(Pong);
});
app.add_observer(|t: On<Pong>, mut commands: Commands| {
    commands.trigger(Ping);  // Back to Ping forever!
});

// Correct: add termination condition
app.add_observer(|t: On<Ping>, mut commands: Commands, mut count: ResMut<PingCount>| {
    if count.0 < 10 {
        count.0 += 1;
        commands.trigger(Pong);
    }
});
```

### Mixing Event Types

```rust
// Wrong: trigger requires Event derive
#[derive(Message)]  // Not Event!
struct MyMessage;

commands.trigger(MyMessage);  // Compile error!

// Correct: use Event derive for triggers
#[derive(Event)]
struct MyEvent;

commands.trigger(MyEvent);  // Works
```
