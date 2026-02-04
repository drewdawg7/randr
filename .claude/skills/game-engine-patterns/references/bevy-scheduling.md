# Bevy Scheduling

Overview of Bevy 0.18 scheduling system for controlling when and how systems execute.

## Quick Navigation

| Topic | Use When | Reference |
|-------|----------|-----------|
| Schedules | Choosing where to add systems | [scheduling/schedules.md](scheduling/schedules.md) |
| Run Conditions | Controlling when systems execute | [scheduling/run-conditions.md](scheduling/run-conditions.md) |
| System Ordering | Enforcing execution order | [scheduling/system-ordering.md](scheduling/system-ordering.md) |
| System Sets | Grouping systems with shared config | [scheduling/system-sets.md](scheduling/system-sets.md) |
| Fixed Timestep | Frame-rate independent logic | [scheduling/fixed-timestep.md](scheduling/fixed-timestep.md) |

## Scheduling Architecture

Bevy's scheduling system controls:
- **When** systems run (schedules, run conditions)
- **In what order** systems run (ordering, chaining, sets)
- **How often** systems run (frame-based vs fixed timestep)

### Key Concepts

| Concept | Purpose |
|---------|---------|
| **Schedule** | A collection of systems that run together (e.g., `Update`, `Startup`) |
| **Run Condition** | A predicate that controls whether a system executes |
| **System Set** | A named group of systems with shared configuration |
| **Ordering** | Constraints on execution order (`before`, `after`, `chain`) |

## This Codebase

### Pattern: State-Gated Systems

Most gameplay systems are gated by `AppState`:

```rust
// From src/ui/screens/dungeon/plugin.rs
app.add_systems(
    Update,
    (
        handle_floor_ready.run_if(on_message::<FloorReady>),
        spawn_player_when_ready.run_if(resource_exists::<PendingPlayerSpawn>),
        handle_dungeon_movement
            .run_if(on_message::<GameAction>)
            .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
    )
        .chain()
        .run_if(in_state(AppState::Dungeon)),
);
```

### Pattern: Message-Driven Chains

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

### Pattern: Custom Reusable Conditions

Modal-specific run conditions for reuse:

```rust
// From src/ui/screens/modal.rs
pub fn in_inventory_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::Inventory)
}

// Usage
app.add_systems(Update,
    handle_inventory_navigation.run_if(in_inventory_modal)
);
```

## Decision Guide

| Scenario | Schedule | Run Condition |
|----------|----------|---------------|
| One-time setup | `Startup` | None |
| State entry setup | `OnEnter(State)` | None |
| State exit cleanup | `OnExit(State)` | None |
| Main game logic | `Update` | `in_state()` |
| Event handling | `Update` | `on_message::<T>()` |
| Physics/deterministic | `FixedUpdate` | As needed |
| Late visual updates | `PostUpdate` | As needed |
| Input preprocessing | `PreUpdate` | As needed |

## Common Mistakes

### Not chaining dependent systems
```rust
// Wrong: race condition between systems
app.add_systems(Update, (write_data, read_data));

// Correct: guaranteed order
app.add_systems(Update, (write_data, read_data).chain());
```

### Applying run_if to wrong scope
```rust
// Wrong: only applies to system_c
app.add_systems(Update, (a, b, c.run_if(condition)));

// Correct: applies to all systems
app.add_systems(Update, (a, b, c).run_if(condition));
```

### Missing state condition
```rust
// Wrong: runs in all states
app.add_systems(Update, dungeon_logic);

// Correct: state-gated
app.add_systems(Update, dungeon_logic.run_if(in_state(AppState::Dungeon)));
```

### Event systems without run conditions
```rust
// Wrong: runs every frame, wastes cycles
app.add_systems(Update, handle_damage);

// Correct: only when events exist
app.add_systems(Update, handle_damage.run_if(on_message::<Damage>));
```
