---
name: bevy-patterns
description: Bevy ECS anti-patterns and idiomatic patterns. Use when auditing Bevy code or reviewing game systems.
---

# Bevy Patterns

## Anti-Patterns Checklist

### System Design
- [ ] Systems doing too much (single responsibility)
- [ ] Missing `Changed<T>` or `Added<T>` filters
- [ ] Missing system ordering/sets (race conditions)
- [ ] Overuse of `Commands` when direct access works
- [ ] Not using `run_if` for conditional execution

### Components & Entities
- [ ] Entity spawning without bundles
- [ ] Component bloat (too many fields per component)
- [ ] Missing marker components for categorization
- [ ] Not using `Local<T>` for system-local state

### State Management
- [ ] Missing or improper `States` usage
- [ ] Missing cleanup systems for state transitions
- [ ] Resources that should be events (or vice versa)

### Architecture
- [ ] UI logic mixed with game logic
- [ ] Input handling coupled to game mechanics
- [ ] Rendering concerns in gameplay modules

## Idiomatic Patterns

### Use Changed/Added Filters
```rust
// Anti-pattern: processes all entities every frame
fn update_health(query: Query<&Health>) { ... }

// Idiomatic: only processes changed entities
fn update_health(query: Query<&Health, Changed<Health>>) { ... }
```

### Use Bundles for Entity Creation
```rust
// Anti-pattern: multiple inserts
commands.spawn_empty()
    .insert(Position::default())
    .insert(Velocity::default())
    .insert(Health::default());

// Idiomatic: bundle
commands.spawn(PlayerBundle::default());
```

### Use Marker Components
```rust
// Anti-pattern: check component values
fn player_system(query: Query<&Entity, With<Health>>) {
    // How to tell if this is a player?
}

// Idiomatic: marker component
#[derive(Component)]
struct Player;

fn player_system(query: Query<&Health, With<Player>>) { ... }
```

### Separate Concerns
```rust
// Anti-pattern: UI system modifies game state
fn ui_system(mut game_state: ResMut<GameState>) { ... }

// Idiomatic: UI sends events, game systems handle them
fn ui_system(mut events: EventWriter<GameAction>) { ... }
fn game_system(mut events: EventReader<GameAction>) { ... }
```

### Use run_if for Conditional Systems
```rust
// Anti-pattern: check state inside system
fn gameplay_system(state: Res<State<AppState>>) {
    if *state.get() != AppState::Playing { return; }
    ...
}

// Idiomatic: run_if condition
app.add_systems(Update, gameplay_system.run_if(in_state(AppState::Playing)));
```

## Quick Fixes

| Anti-Pattern | Fix |
|--------------|-----|
| No `Changed<T>` filter | Add `Changed<T>` to query |
| Multiple `.insert()` calls | Create a bundle |
| System does too much | Split into focused systems |
| Missing cleanup | Add `OnExit(State)` system |
| Commands overuse | Use `&mut Component` when possible |
