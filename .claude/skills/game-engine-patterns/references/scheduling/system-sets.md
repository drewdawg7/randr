# System Sets

## Quick Reference

```rust
// Define a system set
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MySet;

// Enum-based sets for related groups
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameplaySet {
    Input,
    Movement,
    Combat,
    Rendering,
}

// Configure set with run condition and ordering
app.configure_sets(Update, (
    GameplaySet::Input,
    GameplaySet::Movement
        .after(GameplaySet::Input)
        .run_if(in_state(GameState::Playing)),
    GameplaySet::Combat.after(GameplaySet::Movement),
));

// Add systems to sets
app.add_systems(Update, (
    read_keyboard.in_set(GameplaySet::Input),
    read_gamepad.in_set(GameplaySet::Input),
    player_movement.in_set(GameplaySet::Movement),
    enemy_movement.in_set(GameplaySet::Movement),
));
```

## Overview

System sets group systems with shared configuration:
- **Run conditions** applied to all systems in the set
- **Ordering** relative to other sets or systems
- **Organization** of related systems

## Defining System Sets

### Struct-Based Sets

For single-purpose sets:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct AudioSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PhysicsSet;
```

### Enum-Based Sets

For related groups with shared traits:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameplaySet {
    Input,
    Movement,
    Combat,
    UI,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum LoadingSet {
    Assets,
    Spawning,
    Initialization,
}
```

## Configuring Sets

### Run Conditions on Sets

Apply conditions to all systems in a set:

```rust
app.configure_sets(Update,
    GameplaySet::Movement.run_if(in_state(GameState::Playing))
);

// All systems in Movement set now require Playing state
app.add_systems(Update, (
    player_movement.in_set(GameplaySet::Movement),
    enemy_movement.in_set(GameplaySet::Movement),
    npc_movement.in_set(GameplaySet::Movement),
));
```

### Ordering Between Sets

```rust
app.configure_sets(Update, (
    GameplaySet::Input,
    GameplaySet::Movement.after(GameplaySet::Input),
    GameplaySet::Combat.after(GameplaySet::Movement),
    GameplaySet::UI.after(GameplaySet::Combat),
));
```

### Combined Configuration

```rust
app.configure_sets(Update, (
    GameplaySet::Input
        .run_if(in_state(GameState::Playing)),
    GameplaySet::Movement
        .after(GameplaySet::Input)
        .run_if(in_state(GameState::Playing))
        .run_if(not(resource_exists::<Paused>)),
    GameplaySet::Combat
        .after(GameplaySet::Movement)
        .run_if(resource_exists::<ActiveCombat>),
));
```

## Adding Systems to Sets

### Single Set

```rust
app.add_systems(Update, my_system.in_set(MySet));
```

### Multiple Sets

Systems can belong to multiple sets:

```rust
app.add_systems(Update,
    player_input
        .in_set(GameplaySet::Input)
        .in_set(PlayerSet)
);
```

### Tuple of Systems

```rust
app.add_systems(Update, (
    read_keyboard,
    read_mouse,
    read_gamepad,
).in_set(GameplaySet::Input));
```

## Anonymous Sets (Tuples)

Tuples of systems act as anonymous sets:

```rust
// This tuple acts as an implicit set
app.add_systems(Update,
    (system_a, system_b, system_c)
        .run_if(common_condition)
        .after(some_system)
);
```

## Per-Schedule Configuration

**Important**: Set configuration is stored per-schedule. Configure sets in the same schedule where you use them:

```rust
// CORRECT: Configure and use in same schedule
app.configure_sets(Update, MySet.run_if(my_condition));
app.add_systems(Update, my_system.in_set(MySet));

// WRONG: Configuration in different schedule has no effect
app.configure_sets(Update, MySet.run_if(my_condition));
app.add_systems(FixedUpdate, my_system.in_set(MySet)); // Condition not applied!

// CORRECT: Configure for each schedule separately
app.configure_sets(Update, MySet.run_if(my_condition));
app.configure_sets(FixedUpdate, MySet.run_if(my_condition));
```

## Pattern: Feature Sets

Organize systems by game feature:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum CombatSet {
    Detection,
    Damage,
    Effects,
    Cleanup,
}

app.configure_sets(Update, (
    CombatSet::Detection,
    CombatSet::Damage.after(CombatSet::Detection),
    CombatSet::Effects.after(CombatSet::Damage),
    CombatSet::Cleanup.after(CombatSet::Effects),
).run_if(in_state(GameState::Combat)));

app.add_systems(Update, (
    detect_hits.in_set(CombatSet::Detection),
    calculate_damage.in_set(CombatSet::Damage),
    apply_damage.in_set(CombatSet::Damage),
    spawn_particles.in_set(CombatSet::Effects),
    play_sounds.in_set(CombatSet::Effects),
    despawn_dead.in_set(CombatSet::Cleanup),
));
```

## Pattern: Phase Sets

Organize by execution phase:

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum Phase {
    Gather,    // Collect inputs and events
    Process,   // Main game logic
    Apply,     // Apply changes
    Sync,      // Synchronize state
}

app.configure_sets(Update, (
    Phase::Gather,
    Phase::Process.after(Phase::Gather),
    Phase::Apply.after(Phase::Process),
    Phase::Sync.after(Phase::Apply),
));
```

## Pattern: Plugin Sets

Each plugin defines its own set:

```rust
// In combat plugin
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CombatSystemSet;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update,
            CombatSystemSet.run_if(in_state(GameState::Playing))
        );

        app.add_systems(Update, (
            process_attacks,
            handle_damage,
        ).in_set(CombatSystemSet));
    }
}

// In main app, order plugin sets
app.configure_sets(Update,
    CombatSystemSet.after(InputSystemSet)
);
```

## Sets vs Chaining

| Use Sets When | Use Chaining When |
|---------------|-------------------|
| Many systems with shared config | Few systems in sequence |
| Cross-plugin ordering | Local ordering within a module |
| Reusable groupings | One-time ordering |
| Complex dependency graphs | Simple linear flow |

```rust
// Sets: shared config, cross-module
app.configure_sets(Update, CombatSet.run_if(in_combat));
app.add_systems(Update, (attack, defend, heal).in_set(CombatSet));

// Chaining: simple sequence, local
app.add_systems(Update, (read_input, process, apply).chain());
```

## Common Mistakes

### Configuring set in wrong schedule
```rust
// Wrong: set configured in Update, used in FixedUpdate
app.configure_sets(Update, PhysicsSet.run_if(in_state(GameState::Playing)));
app.add_systems(FixedUpdate, physics.in_set(PhysicsSet)); // Condition not applied!

// Correct: configure in same schedule as use
app.configure_sets(FixedUpdate, PhysicsSet.run_if(in_state(GameState::Playing)));
app.add_systems(FixedUpdate, physics.in_set(PhysicsSet));
```

### Circular dependencies
```rust
// Wrong: circular ordering
app.configure_sets(Update, (
    SetA.after(SetB),
    SetB.after(SetA), // Error!
));
```

### Over-using sets for simple cases
```rust
// Overkill for two systems
#[derive(SystemSet, ...)]
struct TinySet;
app.configure_sets(Update, TinySet.run_if(condition));
app.add_systems(Update, (sys_a, sys_b).in_set(TinySet));

// Simpler: just use tuple
app.add_systems(Update, (sys_a, sys_b).run_if(condition));
```

### Forgetting set derives
```rust
// Wrong: missing required derives
#[derive(SystemSet)]
struct MySet;

// Correct: all required derives
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MySet;
```
