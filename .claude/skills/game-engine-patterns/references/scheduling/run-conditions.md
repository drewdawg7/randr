# Run Conditions

## Quick Reference

```rust
// State condition
app.add_systems(Update, game_logic.run_if(in_state(GameState::Playing)));

// Message condition
app.add_systems(Update, handle_damage.run_if(on_message::<Damage>));

// Resource conditions
app.add_systems(Update, use_item.run_if(resource_exists::<SelectedItem>));

// Component condition
app.add_systems(Update, update_timers.run_if(any_with_component::<Timer>));

// Multiple conditions (all must be true)
app.add_systems(Update, combat_system
    .run_if(in_state(AppState::Dungeon))
    .run_if(resource_exists::<ActiveCombat>));

// Combining conditions
app.add_systems(Update, system
    .run_if(condition_a.and(condition_b))  // AND
    .run_if(condition_a.or(condition_b))); // OR

// Custom condition function
fn player_alive(query: Query<&Health, With<Player>>) -> bool {
    query.iter().any(|h| h.current > 0)
}
app.add_systems(Update, player_controls.run_if(player_alive));

// Inline closure condition
app.add_systems(Update, handle_input
    .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()));
```

## Overview

Run conditions are predicates that determine whether a system executes. They must:
- Return `bool`
- Use only **read-only** system parameters (no `ResMut`, no mutable queries)
- Be cheap to evaluate (run every frame)

## Applying Run Conditions

### To Individual Systems

```rust
app.add_systems(Update, my_system.run_if(my_condition));
```

### To System Tuples

```rust
// Condition applies to entire group
app.add_systems(Update, (sys_a, sys_b, sys_c).run_if(condition));
```

### Multiple Conditions (AND)

Multiple `run_if` calls create AND logic - all must be true:

```rust
app.add_systems(Update, combat_system
    .run_if(in_state(AppState::Dungeon))
    .run_if(resource_exists::<ActiveCombat>)
    .run_if(on_message::<PlayerAttack>));
```

## Built-in Conditions

### State Conditions

From `bevy::state::condition`:

| Condition | True When |
|-----------|-----------|
| `in_state(S::Variant)` | State machine is in specified state |
| `state_changed::<S>` | State just changed |
| `state_exists::<S>` | State machine exists |

```rust
use bevy::prelude::*;

// Only run in Playing state
app.add_systems(Update,
    gameplay_system.run_if(in_state(GameState::Playing))
);

// React to any state change
app.add_systems(Update,
    log_state_change.run_if(state_changed::<GameState>)
);
```

**Codebase example** from `src/states/app_state.rs`:
```rust
app.add_systems(
    StateTransition,
    track_state_transitions.run_if(state_changed::<AppState>),
);
```

### Message/Event Conditions

From `bevy::ecs::schedule::common_conditions`:

| Condition | True When |
|-----------|-----------|
| `on_message::<M>` | New messages of type `M` exist in queue |

```rust
// Only run when damage events exist
app.add_systems(Update,
    handle_damage.run_if(on_message::<Damage>)
);
```

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

### Resource Conditions

From `bevy::ecs::schedule::common_conditions`:

| Condition | True When |
|-----------|-----------|
| `resource_exists::<R>` | Resource `R` exists |
| `resource_added::<R>` | Resource was just added this frame |
| `resource_changed::<R>` | Resource was added or mutated |
| `resource_removed::<R>` | Resource was removed this frame |
| `resource_changed_or_removed::<R>` | Added, mutated, or removed |
| `resource_exists_and_changed::<R>` | Exists AND was changed |
| `resource_equals(value)` | Resource equals specific value |
| `resource_exists_and_equals(value)` | Exists AND equals value |

```rust
// Only run when combat is active
app.add_systems(Update,
    combat_ui.run_if(resource_exists::<ActiveCombat>)
);

// React to resource changes
app.add_systems(Update,
    update_score_display.run_if(resource_changed::<Score>)
);
```

**Codebase example** from `src/ui/screens/dungeon/plugin.rs`:
```rust
spawn_player_when_ready.run_if(resource_exists::<PendingPlayerSpawn>)
```

### Component Conditions

From `bevy::ecs::schedule::common_conditions`:

| Condition | True When |
|-----------|-----------|
| `any_with_component::<C>` | At least one entity has component `C` |
| `any_component_removed::<C>` | Any entity lost component `C` this frame |
| `any_match_filter::<F>` | Any entity matches query filter `F` |

```rust
// Only update timers if any exist
app.add_systems(Update,
    poll_timers.run_if(any_with_component::<CraftingTimer>)
);
```

**Codebase example** from `src/crafting_station/plugin.rs`:
```rust
app.add_systems(
    Update,
    (
        handle_try_start_forge_crafting.run_if(on_message::<TryStartForgeCrafting>),
        handle_try_start_anvil_crafting.run_if(on_message::<TryStartAnvilCrafting>),
        poll_forge_timers.run_if(any_with_component::<ForgeActiveTimer>),
        poll_anvil_timers.run_if(any_with_component::<AnvilActiveTimer>),
    ),
);
```

### Utility Conditions

From `bevy::ecs::schedule::common_conditions`:

| Condition | True When |
|-----------|-----------|
| `run_once` | Only on first execution (then always false) |
| `not(condition)` | Inverts the condition |
| `condition_changed(cond)` | Condition result changed from last frame |
| `condition_changed_to(cond, value)` | Condition transitioned to specific value |

```rust
// Run setup exactly once
app.add_systems(Update, one_time_setup.run_if(run_once));

// Run when NOT in menu
app.add_systems(Update,
    game_logic.run_if(not(in_state(GameState::Menu)))
);
```

### Time Conditions

From `bevy::time::common_conditions`:

| Condition | True When |
|-----------|-----------|
| `on_timer(duration)` | At regular intervals (virtual time) |
| `on_real_timer(duration)` | At regular intervals (real time) |
| `once_after_delay(duration)` | Once after delay (virtual) |
| `once_after_real_delay(duration)` | Once after delay (real) |
| `repeating_after_delay(delay, interval)` | Repeating after initial delay |
| `paused` | Virtual clock is paused |

```rust
use bevy::time::common_conditions::*;
use std::time::Duration;

// Spawn enemy every 2 seconds
app.add_systems(Update,
    spawn_enemy.run_if(on_timer(Duration::from_secs(2)))
);

// Autosave every 5 minutes (real time, ignores pause)
app.add_systems(Update,
    autosave.run_if(on_real_timer(Duration::from_secs(300)))
);
```

### Input Conditions

From `bevy::input::common_conditions`:

| Condition | True When |
|-----------|-----------|
| `input_pressed(input)` | Input is currently held |
| `input_just_pressed(input)` | Input was pressed this frame |
| `input_just_released(input)` | Input was released this frame |
| `input_toggle_active(input, default)` | Toggleable state |

```rust
use bevy::input::common_conditions::*;

app.add_systems(Update, (
    jump.run_if(input_just_pressed(KeyCode::Space)),
    shoot.run_if(input_pressed(MouseButton::Left)),
    debug_mode.run_if(input_toggle_active(KeyCode::F3, false)),
));
```

## Combining Conditions

The `SystemCondition` trait provides combinators:

| Combinator | Logic | True When |
|------------|-------|-----------|
| `.and(other)` | AND (short-circuit) | Both true |
| `.or(other)` | OR (short-circuit) | Either true |
| `.nand(other)` | NAND | Not both true |
| `.nor(other)` | NOR | Neither true |
| `.xor(other)` | XOR | Exactly one true |
| `.xnor(other)` | XNOR | Both same value |

```rust
// AND - both must be true
my_system.run_if(resource_exists::<A>.and(resource_exists::<B>))

// OR - either must be true
my_system.run_if(resource_exists::<A>.or(resource_exists::<B>))

// Complex combination
my_system.run_if(
    in_state(GameState::Playing)
        .and(resource_exists::<Player>)
        .and(not(resource_exists::<Paused>))
)
```

## Custom Run Conditions

### Function Conditions

```rust
// Simple function returning bool
fn player_alive(query: Query<&Health, With<Player>>) -> bool {
    query.iter().any(|h| h.current > 0)
}

app.add_systems(Update, player_controls.run_if(player_alive));
```

### Inline Closure Conditions

```rust
// Closure for simple checks
app.add_systems(Update, handle_input
    .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()));
```

**Codebase example** from `src/ui/screens/dungeon/plugin.rs`:
```rust
handle_dungeon_movement
    .run_if(on_message::<GameAction>)
    .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
```

### Reusable Parameterized Conditions

```rust
// Return impl SystemCondition for reusable conditions
fn score_above(threshold: u32) -> impl SystemCondition<()> {
    resource_exists::<Score>
        .and(move |score: Res<Score>| score.value > threshold)
}

app.add_systems(Update, (
    show_bronze.run_if(score_above(100)),
    show_silver.run_if(score_above(500)),
    show_gold.run_if(score_above(1000)),
));
```

### Codebase Pattern: Modal Conditions

From `src/ui/screens/modal.rs`:

```rust
/// Run condition: returns true when the inventory modal is active.
pub fn in_inventory_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::Inventory)
}

/// Run condition: returns true when the merchant modal is active.
pub fn in_merchant_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::MerchantModal)
}

/// Run condition: returns true when the forge modal is active.
pub fn in_forge_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::ForgeModal)
}

// ... similar for other modals
```

Usage:
```rust
// From src/ui/screens/inventory_modal/plugin.rs
app.add_systems(
    Update,
    (
        modal_close_system::<InventoryModal>,
        (
            tab_toggle_system(FocusPanel::EquipmentGrid, FocusPanel::BackpackGrid),
            handle_inventory_modal_navigation,
            handle_inventory_modal_select,
            // ...
        )
            .run_if(in_inventory_modal),
    ),
);
```

## Distributive Run Conditions

### run_if vs distributive_run_if

```rust
// run_if: Condition evaluated ONCE for the group
app.add_systems(Update, (a, b, c).run_if(condition));
// All systems run or none run

// distributive_run_if: Condition evaluated PER SYSTEM
app.add_systems(Update, (a, b, c).distributive_run_if(condition));
// Equivalent to:
app.add_systems(Update, (
    a.run_if(condition),
    b.run_if(condition),
    c.run_if(condition),
));
```

**Use `run_if`** when all systems should run together or not at all.

**Use `distributive_run_if`** when each system should independently check the condition.

## Common Mistakes

### Mutable parameters in conditions
```rust
// Wrong: conditions must be read-only
fn bad_condition(mut res: ResMut<Counter>) -> bool {
    res.0 += 1;
    res.0 > 10
}

// Correct: read-only access
fn good_condition(res: Res<Counter>) -> bool {
    res.0 > 10
}
```

### Expensive condition logic
```rust
// Wrong: expensive computation every frame
fn expensive_condition(query: Query<&Transform>) -> bool {
    query.iter().map(|t| complex_calculation(t)).sum::<f32>() > 100.0
}

// Correct: cache result in resource, check resource
fn cached_condition(cache: Res<CalculationCache>) -> bool {
    cache.result > 100.0
}
```

### Forgetting run conditions on event handlers
```rust
// Wrong: runs every frame, wastes cycles
app.add_systems(Update, handle_damage);

// Correct: only when events exist
app.add_systems(Update, handle_damage.run_if(on_message::<Damage>));
```

### Condition scope confusion
```rust
// Wrong: condition only applies to system_c
app.add_systems(Update, (a, b, c.run_if(condition)));

// Correct: condition applies to all
app.add_systems(Update, (a, b, c).run_if(condition));
```
