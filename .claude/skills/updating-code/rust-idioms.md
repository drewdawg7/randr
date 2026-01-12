# Rust Idioms

Preferred Rust patterns used throughout the codebase.

## Option Handling

### Use `map_or` for Option Defaults

When extracting a value from an `Option` with a default fallback, prefer `map_or` over `match`:

```rust
// Preferred
self.stat(t).map_or(0, |si| si.current_value)

// Avoid
match self.stat(t) {
    Some(si) => si.current_value,
    None     => 0
}
```

**Examples in codebase:**
- `StatSheet::value()` in `src/stats/definition.rs:41`
- `StatSheet::max_value()` in `src/stats/definition.rs:45`

### Use `let-else` for Early Returns

When checking an `Option` and returning early on `None`, prefer `let-else` over `is_none()` + `unwrap()`:

```rust
// Preferred
let Some(room) = dungeon.current_room() else {
    next_mode.set(DungeonMode::Navigation);
    return;
};

// Avoid - potential panic if logic changes
let current_room = dungeon.current_room();
if current_room.is_none() {
    next_mode.set(DungeonMode::Navigation);
    return;
}
let room = current_room.unwrap();
```

**Examples in codebase:**
- `spawn_room_entry_ui()` in `src/screens/dungeon/room_entry.rs:38`

### `unwrap()` in Tests is Acceptable

Using `unwrap()` in test code (`#[cfg(test)]`) is fine since tests should panic on unexpected failures.

## Bevy Query Type Aliases

### Use Type Aliases for Complex Query Types

When Query types become complex (multiple `With`/`Without` filters), define type aliases to improve readability:

```rust
// Define at module level after imports
type PlayerHealthBarQuery<'w, 's> =
    Query<'w, 's, Entity, (With<PlayerHealthBar>, Without<EnemyHealthBar>)>;

type EnemyHealthBarQuery<'w, 's> =
    Query<'w, 's, Entity, (With<EnemyHealthBar>, Without<PlayerHealthBar>)>;

// Use in system signatures
pub fn update_combat_visuals(
    player_health_bar: PlayerHealthBarQuery,
    enemy_health_bar: EnemyHealthBarQuery,
    // ...
)
```

**Examples in codebase:**
- `PlayerHealthBarQuery`, `EnemyHealthBarQuery` in `src/screens/fight/ui.rs:23-28`
