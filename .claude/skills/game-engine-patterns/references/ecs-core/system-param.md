# SystemParam

## Quick Reference

```rust
use bevy::{ecs::system::SystemParam, prelude::*};

// Basic derive - resources only (no system state needed)
#[derive(SystemParam)]
struct MyResources<'w> {
    gold: ResMut<'w, PlayerGold>,
    stats: ResMut<'w, StatSheet>,
}

// With system state - for EventReaders, Local, etc.
#[derive(SystemParam)]
struct MyEventReaders<'w, 's> {
    damage: MessageReader<'w, 's, DealDamage>,
    death: MessageReader<'w, 's, EntityDied>,
}

// Usage in system
fn my_system(mut params: MyResources, query: Query<&Health>) {
    params.gold.0 += 10;
}
```

## Overview

`SystemParam` is a trait that allows grouping multiple system parameters into a single reusable struct. This reduces function signature bloat and enables sharing common parameter combinations across systems.

**When to use SystemParam:**
- System has 5+ parameters that logically group together
- Same parameter set appears in multiple systems
- Want to encapsulate related data access patterns
- Need cleaner, more readable system signatures

**When NOT to use SystemParam:**
- Only 2-3 parameters (overhead not worth it)
- Parameters don't logically group
- One-off system with unique parameter needs

## Patterns

| Pattern | Lifetime | Use Case | Example |
|---------|----------|----------|---------|
| Resource bundle | `'w` only | Group related `Res`/`ResMut` | Player resources |
| Event reader bundle | `'w, 's` | Group related event readers | Item events |
| Query + Resource | `'w, 's` | Combine queries with resources | Combat system |
| Commands + Events | `'w, 's` | Spawning with event emission | Entity factory |

### Lifetime Requirements

- **`'w` (world lifetime)**: Required for accessing world data (`Res`, `ResMut`, `Query`)
- **`'s` (system state lifetime)**: Required for stateful parameters (`MessageReader`, `MessageWriter`, `Local`)

**Rule of thumb**: If all fields are `Res`/`ResMut`, use `'w` only. If any field is an event reader/writer or `Local`, use both `'w, 's`.

## Examples

### Resource Bundle Pattern
Groups related resources for cleaner signatures.

**File:** `src/combat/plugin.rs`
```rust
#[derive(SystemParam)]
struct PlayerResources<'w> {
    gold: ResMut<'w, PlayerGold>,
    progression: ResMut<'w, Progression>,
    inventory: ResMut<'w, Inventory>,
    stats: ResMut<'w, StatSheet>,
}

fn handle_mob_death(
    mut commands: Commands,
    mut events: MessageReader<EntityDied>,
    mut player: PlayerResources,  // Clean: 4 resources in 1 param
    // ... other params
) {
    // Access fields directly
    player.gold.0 += gold_reward;
    player.stats.add_xp(xp_amount);
}
```

### Event Reader Bundle Pattern
Groups related event readers when a system handles multiple event types.

**File:** `src/plugins/toast_listeners.rs`
```rust
#[derive(SystemParam)]
struct ItemEventReaders<'w, 's> {
    picked_up: MessageReader<'w, 's, ItemPickedUp>,
    equipped: MessageReader<'w, 's, ItemEquipped>,
    unequipped: MessageReader<'w, 's, ItemUnequipped>,
    used: MessageReader<'w, 's, ItemUsed>,
    dropped: MessageReader<'w, 's, ItemDropped>,
    deposited: MessageReader<'w, 's, ItemDeposited>,
    withdrawn: MessageReader<'w, 's, ItemWithdrawn>,
}

fn listen_item_events(
    mut events: ItemEventReaders,  // 7 readers in 1 param
    mut toast_writer: MessageWriter<ShowToast>,
) {
    for event in events.picked_up.read() {
        // Handle pickup
    }
    for event in events.equipped.read() {
        // Handle equip
    }
    // ... handle other events
}
```

### Mixed Pattern (Queries + Resources)
Combine queries and resources for domain-specific access.

```rust
#[derive(SystemParam)]
struct CombatContext<'w, 's> {
    stats: ResMut<'w, StatSheet>,
    inventory: Res<'w, Inventory>,
    mobs: Query<'w, 's, (&'static Health, &'static CombatStats), With<MobMarker>>,
}

fn process_combat(mut ctx: CombatContext) {
    for (health, combat_stats) in ctx.mobs.iter() {
        // Use ctx.stats, ctx.inventory alongside query results
    }
}
```

## Common Mistakes

### Missing System State Lifetime

```rust
// WRONG: MessageReader requires 's lifetime
#[derive(SystemParam)]
struct BadParams<'w> {
    events: MessageReader<'w, DealDamage>,  // Compile error!
}

// CORRECT: Include 's lifetime
#[derive(SystemParam)]
struct GoodParams<'w, 's> {
    events: MessageReader<'w, 's, DealDamage>,
}
```

### Forgetting Static Lifetime on Query Components

```rust
// WRONG: Component types in queries need 'static
#[derive(SystemParam)]
struct BadQuery<'w, 's> {
    query: Query<'w, 's, &Health>,  // May cause lifetime issues
}

// CORRECT: Use 'static for component references
#[derive(SystemParam)]
struct GoodQuery<'w, 's> {
    query: Query<'w, 's, &'static Health>,
}
```

### Overusing SystemParam for Simple Cases

```rust
// OVERKILL: Don't wrap 1-2 params
#[derive(SystemParam)]
struct JustGold<'w> {
    gold: ResMut<'w, PlayerGold>,
}

// BETTER: Use directly in system
fn simple_system(mut gold: ResMut<PlayerGold>) {
    gold.0 += 10;
}
```

### Not Making Fields `mut` When Needed

```rust
// WRONG: Can't mutate through immutable binding
fn bad_system(params: PlayerResources) {
    params.gold.0 += 10;  // Error: cannot borrow as mutable
}

// CORRECT: Use mut binding
fn good_system(mut params: PlayerResources) {
    params.gold.0 += 10;
}
```

## See Also

- [Bevy ECS Core](../bevy-ecs-core.md) - Components, Resources, Queries
- [Bevy Events](../bevy-events.md) - MessageReader/MessageWriter
