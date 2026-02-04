# Schedules

## Quick Reference

```rust
// Startup - runs once at app launch
app.add_systems(Startup, setup_game);

// Update - runs every frame (main game logic)
app.add_systems(Update, game_logic);

// FixedUpdate - runs at fixed intervals (physics, deterministic logic)
app.add_systems(FixedUpdate, physics_system);

// State transitions
app.add_systems(OnEnter(GameState::Playing), setup_level);
app.add_systems(OnExit(GameState::Playing), cleanup_level);

// Pre/Post update for engine-level systems
app.add_systems(PreUpdate, preprocess_input);
app.add_systems(PostUpdate, late_visual_updates);
```

## Overview

Schedules are collections of systems that run together at specific points in the game loop. The `Main` schedule orchestrates all other schedules in a fixed order.

## Built-in Schedules

### Startup Schedules (Run Once)

These schedules run once during the first frame, in order:

| Schedule | Purpose | Use Cases |
|----------|---------|-----------|
| `PreStartup` | Early initialization | Core resource setup |
| `Startup` | Main initialization | Spawn entities, load assets, setup UI |
| `PostStartup` | Post-initialization | Systems depending on Startup results |

```rust
// Codebase example from src/plugins/game.rs
app.add_systems(Startup, setup);
```

### Frame Schedules (Run Every Frame)

These schedules run every frame in this order:

| Order | Schedule | Purpose | Use Cases |
|-------|----------|---------|-----------|
| 1 | `First` | Absolute first | Diagnostics, frame setup |
| 2 | `PreUpdate` | Before game logic | Input processing, engine prep |
| 3 | `StateTransition` | State changes | Handled by Bevy |
| 4 | `RunFixedMainLoop` | Fixed timestep | Triggers FixedUpdate iterations |
| 5 | `Update` | **Main game logic** | Most user systems |
| 6 | `PostUpdate` | After game logic | Transform propagation, rendering prep |
| 7 | `Last` | Absolute last | Cleanup, diagnostics |

**Important**: Most game logic belongs in `Update`. Use other schedules sparingly.

### Fixed Timestep Schedules

Run at a fixed rate (default 64 Hz), independent of frame rate:

| Schedule | Purpose |
|----------|---------|
| `FixedFirst` | Before fixed systems |
| `FixedPreUpdate` | Engine prep for fixed |
| `FixedUpdate` | **Physics, AI, deterministic logic** |
| `FixedPostUpdate` | Engine cleanup for fixed |
| `FixedLast` | After fixed systems |

See [fixed-timestep.md](fixed-timestep.md) for details.

### State Schedules

Run during state transitions:

| Schedule | When It Runs |
|----------|--------------|
| `OnEnter(State::Variant)` | Transitioning INTO state |
| `OnExit(State::Variant)` | Transitioning OUT OF state |
| `OnTransition { exited, entered }` | Specific transition |

```rust
// Codebase example from src/ui/screens/dungeon/plugin.rs
app.add_systems(OnEnter(AppState::Dungeon), enter_dungeon)
   .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon);
```

**Transition Order**:
1. `StateTransitionEvent<S>` is sent
2. `OnExit(old_state)` runs
3. `OnTransition { exited, entered }` runs
4. `OnEnter(new_state)` runs

## Schedule Selection Guide

| System Type | Schedule | Why |
|-------------|----------|-----|
| Asset loading | `Startup` | Once at launch |
| Entity spawning (initial) | `Startup` | Once at launch |
| State setup | `OnEnter(State)` | Per state entry |
| State cleanup | `OnExit(State)` | Per state exit |
| Game logic | `Update` | Every frame |
| Physics | `FixedUpdate` | Deterministic |
| Input reading | `PreUpdate` | Before game logic |
| Visual updates | `PostUpdate` | After transforms |
| State tracking | `StateTransition` | During transitions |

## Codebase Examples

### Multiple Schedule Usage

```rust
// From src/states/app_state.rs
app.add_systems(
    StateTransition,
    track_state_transitions.run_if(state_changed::<AppState>),
)
.add_systems(
    PreUpdate,
    handle_state_transition_requests.run_if(on_message::<StateTransitionRequest>),
);
```

### PostUpdate for Visual Systems

```rust
// From src/ui/screens/forge_modal/plugin.rs
app.add_systems(
    PostUpdate,
    (update_forge_slot_selector, animate_forge_slot_selector)
        .chain()
        .run_if(in_forge_modal),
);
```

## Custom Schedules

Create custom schedules for specialized execution:

```rust
use bevy::prelude::*;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct MyCustomSchedule;

fn setup_custom_schedule(app: &mut App) {
    // Initialize the schedule
    app.init_schedule(MyCustomSchedule);

    // Add systems to it
    app.add_systems(MyCustomSchedule, my_system);
}

// Insert into main schedule order
fn configure_schedule_order(mut schedule_order: ResMut<MainScheduleOrder>) {
    // Run after StateTransition, before Update
    schedule_order.insert_after(StateTransition, MyCustomSchedule);
}
```

## Schedule Configuration

Configure schedule behavior for debugging:

```rust
app.edit_schedule(Update, |schedule| {
    schedule.set_build_settings(ScheduleBuildSettings {
        // Warn about systems with conflicting access but no ordering
        ambiguity_detection: LogLevel::Warn,

        // Warn about redundant ordering constraints
        hierarchy_detection: LogLevel::Warn,

        // Auto-insert apply_deferred between systems
        auto_insert_apply_deferred: true,

        // Use short type names in logs
        use_shortnames: true,

        // Include system sets in conflict reports
        report_sets: true,
    });
});
```

## Executor Types

| Executor | Behavior | Use Case |
|----------|----------|----------|
| `MultiThreadedExecutor` | Parallel execution (default) | Production |
| `SingleThreadedExecutor` | Sequential, deterministic | Debugging, replays |

```rust
// Force single-threaded for debugging
app.edit_schedule(Update, |schedule| {
    schedule.set_executor_kind(ExecutorKind::SingleThreaded);
});
```

## Common Mistakes

### Using wrong schedule for game logic
```rust
// Wrong: PostUpdate is for engine systems
app.add_systems(PostUpdate, player_movement);

// Correct: Update is for game logic
app.add_systems(Update, player_movement);
```

### Physics in Update instead of FixedUpdate
```rust
// Wrong: frame-rate dependent physics
app.add_systems(Update, apply_gravity);

// Correct: frame-rate independent
app.add_systems(FixedUpdate, apply_gravity);
```

### Heavy setup in OnEnter
```rust
// Wrong: blocking asset loading in OnEnter
app.add_systems(OnEnter(GameState::Playing), load_all_assets);

// Correct: load in Startup, spawn in OnEnter
app.add_systems(Startup, load_assets);
app.add_systems(OnEnter(GameState::Playing), spawn_level);
```
