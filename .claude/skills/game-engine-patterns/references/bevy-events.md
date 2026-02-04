# Bevy Events & Communication

Overview of Bevy 0.18 event systems used in this codebase.

## Quick Navigation

| Topic | Use When | Reference |
|-------|----------|-----------|
| Messages | Buffered system-to-system communication | [events/messages.md](events/messages.md) |
| Observers | Immediate reactions to events/triggers | [events/observers.md](events/observers.md) |
| Triggers | Firing events for observer execution | [events/triggers.md](events/triggers.md) |
| Component Lifecycle | Reacting to Add/Remove/Insert | [events/lifecycle.md](events/lifecycle.md) |

## Event Architecture (Bevy 0.17+)

Bevy 0.17 introduced a major rearchitecture. The key distinction:

| Concept | Purpose | Execution | Use Case |
|---------|---------|-----------|----------|
| **Message** | Buffered queue | Deferred (next frame) | High-frequency, system ordering |
| **Event** | Trigger observers | Immediate or deferred | Entity-specific reactions |

### Messages (Buffered Events)

Messages are queued and processed by systems on schedules. They persist for 2 frames.

```rust
// Define
#[derive(Message, Debug, Clone)]
struct PlayerDamaged { entity: Entity, amount: i32 }

// Register
app.add_message::<PlayerDamaged>();

// Write
fn deal_damage(mut writer: MessageWriter<PlayerDamaged>) {
    writer.write(PlayerDamaged { entity, amount: 10 });
}

// Read
fn handle_damage(mut reader: MessageReader<PlayerDamaged>) {
    for event in reader.read() { /* process */ }
}

// Run condition
app.add_systems(Update, handle_damage.run_if(on_message::<PlayerDamaged>));
```

### Events (Triggers + Observers)

Events trigger observers immediately. Use for entity-specific reactions.

```rust
// Define
#[derive(Event)]
struct OpenModal(pub ModalType);

// Add observer
app.add_observer(|trigger: On<OpenModal>| { /* react */ });

// Trigger
commands.trigger(OpenModal(ModalType::Inventory));
```

## This Codebase

### Pattern: Hybrid Event Types

Some events derive both for maximum flexibility:

```rust
// From src/ui/screens/modal.rs
#[derive(Event, Message, Debug, Clone, Copy)]
pub struct OpenModal(pub ModalType);
```

This allows both:
- `commands.trigger(OpenModal(...))` - for immediate observer reactions
- `writer.write(OpenModal(...))` - for buffered processing if needed

### Pattern: Event Chains

Combat uses chained message-driven systems:

```rust
// From src/combat/plugin.rs
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

Flow: `PlayerAttackMob` -> `EntityDied` -> Victory/Defeat handling

### Pattern: Observer-Triggered Commands

NPC interactions use observers to trigger modal events:

```rust
// From src/game/npc_interactions.rs
fn on_merchant_interaction(_trigger: On<MerchantInteraction>, mut commands: Commands) {
    commands.insert_resource(MerchantStock::generate());
    commands.trigger(OpenModal(ModalType::MerchantModal));
}
```

## Decision Guide

| Scenario | Use | Why |
|----------|-----|-----|
| Combat damage/death | Message | Order matters, multiple handlers |
| Input actions (GameAction) | Message | Buffered, run conditions |
| Modal open/close | Event + Observer | Immediate UI response |
| NPC interaction | Event + Observer | Direct reaction to collision |
| Component added/removed | Observer | Lifecycle hook |
| High-frequency updates | Message | Buffered, no immediate overhead |
| Entity-specific behavior | Event | Target specific entity |

## Common Mistakes

### Not registering messages
```rust
// Wrong: message type not registered
fn send_damage(mut writer: MessageWriter<Damage>) { }

// Correct: register in plugin
app.add_message::<Damage>();
```

### Missing run conditions
```rust
// Wrong: runs every frame
app.add_systems(Update, handle_damage);

// Correct: only when messages exist
app.add_systems(Update, handle_damage.run_if(on_message::<Damage>));
```

### Relying on observer order
```rust
// Wrong: assumes order
app.add_observer(first_handler);
app.add_observer(second_handler); // may run first!

// Correct: use messages for ordered processing
```

### Using World::trigger in systems
```rust
// Usually wrong: blocks until observers complete
world.trigger(MyEvent);

// Usually correct: deferred execution
commands.trigger(MyEvent);
```
