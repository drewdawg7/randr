# Messages (Buffered Events)

## Quick Reference

```rust
// Define a message
#[derive(Message, Debug, Clone)]
struct MyMessage {
    entity: Entity,
    data: i32,
}

// Register in plugin
app.add_message::<MyMessage>();

// Write messages
fn sender(mut writer: MessageWriter<MyMessage>) {
    writer.write(MyMessage { entity, data: 42 });
}

// Read messages
fn receiver(mut reader: MessageReader<MyMessage>) {
    for msg in reader.read() {
        println!("{:?}", msg);
    }
}

// Run condition
app.add_systems(Update, receiver.run_if(on_message::<MyMessage>));
```

## Overview

Messages are Bevy's **buffered event system** for system-to-system communication. They're double-buffered and persist for at least 2 frames before being dropped.

**Key characteristics:**
- Written by `MessageWriter<M>`, read by `MessageReader<M>`
- Automatically cleared after 2 frame update cycles
- Multiple readers can process the same message
- Order of writes within a frame is preserved

## Message Trait

### Basic Definition

```rust
#[derive(Message, Debug, Clone)]
struct PlayerDamaged {
    pub entity: Entity,
    pub amount: i32,
}
```

**Required derives:** `Message` (mandatory), `Debug` and `Clone` (recommended)

### Unit Messages

For simple notifications without data:

```rust
#[derive(Message)]
struct GamePaused;

#[derive(Message, Default)]
struct Tick; // Default enables write_default()
```

### Complex Messages

Messages can contain any data:

```rust
// From src/combat/events.rs
#[derive(Message, Debug, Clone)]
pub struct VictoryAchieved {
    pub mob_id: MobId,
    pub mob_name: String,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}
```

### Enum Messages

Use enums for multiple related outcomes:

```rust
// From src/dungeon/events.rs
#[derive(Message, Debug, Clone)]
pub enum MoveResult {
    Moved { new_pos: Vec2 },
    Blocked,
    TriggeredCombat { mob_entity: Entity },
    TriggeredStairs,
    TriggeredDoor { destination: DoorDestination },
}
```

## Registration

Messages must be registered before use:

```rust
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerAttackMob>()
           .add_message::<DealDamage>()
           .add_message::<EntityDied>()
           .add_message::<VictoryAchieved>();
    }
}
```

**Note:** Forgetting to register causes a runtime panic when the MessageWriter/Reader is accessed.

## MessageWriter

Writes messages to the queue. Only one `MessageWriter<M>` can exist per system (exclusive access).

### Basic Writing

```rust
fn send_damage(mut writer: MessageWriter<DealDamage>) {
    writer.write(DealDamage {
        target: entity,
        amount: 10,
        source_name: "Player".to_string(),
    });
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `write(msg)` | Write a single message, returns `MessageId` |
| `write_batch(iter)` | Write multiple messages, returns `WriteBatchIds` |
| `write_default()` | Write default message (requires `Default` trait) |

### Conditional Writing

```rust
// From src/combat/plugin.rs
fn process_player_attack(
    mut events: MessageReader<PlayerAttackMob>,
    mut deal_damage_events: MessageWriter<DealDamage>,
    mut entity_died_events: MessageWriter<EntityDied>,
) {
    for event in events.read() {
        // Always write damage event
        deal_damage_events.write(DealDamage { /* ... */ });

        // Conditionally write death event
        if result.target_died {
            entity_died_events.write(EntityDied {
                entity: event.target,
                is_player: false,
            });
        }
    }
}
```

## MessageReader

Reads messages from the queue. Multiple `MessageReader<M>` can exist (shared access).

### Basic Reading

```rust
fn handle_damage(mut reader: MessageReader<DealDamage>) {
    for event in reader.read() {
        println!("Damage dealt: {}", event.amount);
    }
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `read()` | Iterator over unread messages |
| `read_with_id()` | Iterator yielding `(&M, MessageId)` tuples |
| `len()` | Count of unread messages (without consuming) |
| `is_empty()` | Check if no unread messages |
| `clear()` | Mark all messages as read |
| `par_read()` | Parallel iterator (requires `multi_threaded` feature) |

### Reading with IDs

```rust
fn track_messages(mut reader: MessageReader<MyMessage>) {
    for (msg, id) in reader.read_with_id() {
        debug!("Processing message {:?}: {:?}", id, msg);
    }
}
```

### Checking Without Consuming

```rust
#[instrument(level = "debug", skip_all, fields(event_count = reader.len()))]
fn handle_events(mut reader: MessageReader<MyMessage>) {
    // len() doesn't consume messages
    if reader.is_empty() {
        return;
    }
    for msg in reader.read() { /* ... */ }
}
```

## Run Conditions

### on_message::<M>

Only run the system when messages of type M exist:

```rust
app.add_systems(
    Update,
    handle_damage.run_if(on_message::<DealDamage>)
);
```

### Multiple Message Types (OR)

```rust
// From src/plugins/toast_listeners.rs
app.add_systems(
    Update,
    listen_player_events.run_if(
        on_message::<PlayerLeveledUp>
            .or(on_message::<PlayerHealed>)
            .or(on_message::<GoldChanged>)
    )
);
```

### Combined Conditions

```rust
// From src/combat/plugin.rs
app.add_systems(
    Update,
    (
        process_player_attack.run_if(on_message::<PlayerAttackMob>),
        handle_mob_death.run_if(on_message::<EntityDied>),
    )
        .chain()
        .run_if(in_state(AppState::Dungeon))
        .run_if(resource_exists::<ActiveCombat>)
);
```

## SystemParam Grouping

Group multiple readers/writers for cleaner signatures:

```rust
// From src/plugins/toast_listeners.rs
#[derive(SystemParam)]
struct ItemEventReaders<'w, 's> {
    picked_up: MessageReader<'w, 's, ItemPickedUp>,
    equipped: MessageReader<'w, 's, ItemEquipped>,
    unequipped: MessageReader<'w, 's, ItemUnequipped>,
}

fn listen_item_events(events: ItemEventReaders) {
    for event in events.picked_up.read() { /* ... */ }
    for event in events.equipped.read() { /* ... */ }
}
```

## Message Lifecycle

### Timing

```
Frame N:   System A writes message
Frame N:   System B can read (if ordered after A)
Frame N+1: System B can still read (if missed frame N)
Frame N+2: Message dropped
```

### Implications

- **Run conditions matter:** A system with `run_if(some_condition)` that doesn't run for 2 frames will miss messages
- **Chaining matters:** Use `.chain()` to ensure write happens before read in same frame

### Concurrency Rules

| System Parameters | Can Run In Parallel? |
|-------------------|---------------------|
| `MessageReader<M>` + `MessageReader<M>` | Yes |
| `MessageWriter<M>` + `MessageReader<M>` | No |
| `MessageWriter<M>` + `MessageWriter<M>` | No |

## Common Patterns

### Event Chain Processing

```rust
// Read one event type, write another
fn process_attack(
    mut attacks: MessageReader<PlayerAttackMob>,
    mut deaths: MessageWriter<EntityDied>,
) {
    for attack in attacks.read() {
        if calculate_damage(attack).is_lethal {
            deaths.write(EntityDied { entity: attack.target, is_player: false });
        }
    }
}
```

### Filtering Events

```rust
fn handle_mob_death(mut events: MessageReader<EntityDied>) {
    for event in events.read() {
        // Filter to only mob deaths
        if event.is_player {
            continue;
        }
        // Process mob death...
    }
}
```

### Broadcasting Results

```rust
fn calculate_results(
    mut input: MessageReader<CalculateRequest>,
    mut victory: MessageWriter<VictoryAchieved>,
    mut defeat: MessageWriter<DefeatOccurred>,
) {
    for req in input.read() {
        match calculate(req) {
            Outcome::Victory(data) => victory.write(data),
            Outcome::Defeat(data) => defeat.write(data),
        }
    }
}
```

## Common Mistakes

### Forgetting Registration

```rust
// Wrong: panic at runtime
fn my_plugin(app: &mut App) {
    app.add_systems(Update, handle_event);
    // Missing: app.add_message::<MyEvent>();
}

// Correct
fn my_plugin(app: &mut App) {
    app.add_message::<MyEvent>()
       .add_systems(Update, handle_event);
}
```

### Missing Run Condition

```rust
// Wrong: runs every frame, wastes cycles
app.add_systems(Update, handle_damage);

// Correct
app.add_systems(Update, handle_damage.run_if(on_message::<DealDamage>));
```

### Race Condition with Parallel Systems

```rust
// Wrong: write and read may happen simultaneously
app.add_systems(Update, (write_events, read_events));

// Correct: chain ensures order
app.add_systems(Update, (write_events, read_events).chain());
```

### Reading Multiple Times

```rust
// Wrong: second loop gets nothing (already consumed)
fn bad_handler(mut reader: MessageReader<MyEvent>) {
    for e in reader.read() { /* first pass */ }
    for e in reader.read() { /* empty! */ }
}

// Correct: collect if needed multiple times
fn good_handler(mut reader: MessageReader<MyEvent>) {
    let events: Vec<_> = reader.read().collect();
    // Now can iterate events multiple times
}
```
