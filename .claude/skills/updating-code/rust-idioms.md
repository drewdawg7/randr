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

## Collection Type Aliases

### Use Type Aliases for Semantic Collections

When a collection type (HashMap, Vec, etc.) represents a specific domain concept, define a type alias for clarity:

```rust
// Define after imports
pub type EquipmentMap = HashMap<EquipmentSlot, InventoryItem>;

// Use in struct fields and return types
pub struct Inventory {
    equipment: EquipmentMap,
}

pub fn equipment(&self) -> &EquipmentMap {
    &self.equipment
}
```

**Examples in codebase:**
- `EquipmentMap` in `src/inventory/definition.rs:14`

## Integer Safety

### Use Saturating Arithmetic for Quantities

When incrementing or decrementing quantities that could overflow/underflow, use saturating arithmetic:

```rust
// Preferred - consistent and safe
pub fn increase_quantity(&mut self, amount: u32) {
    self.quantity = self.quantity.saturating_add(amount);
}

pub fn decrease_quantity(&mut self, amount: u32) {
    self.quantity = self.quantity.saturating_sub(amount);
}

// Avoid - inconsistent and can panic in debug or wrap in release
pub fn increase_quantity(&mut self, amount: u32) {
    self.quantity += amount;  // Can overflow!
}
```

**Examples in codebase:**
- `InventoryItem::increase_quantity()` and `decrease_quantity()` in `src/inventory/definition.rs:39-45`

### Bounds Check Before Signed-to-Unsigned Casts

When adding signed offsets to unsigned coordinates, check bounds before casting to prevent underflow:

```rust
// Preferred - explicit bounds checking
let new_x = self.player_x as i32 + dx;
let new_y = self.player_y as i32 + dy;

if new_x < 0 || new_y < 0 || new_x as usize >= WIDTH || new_y as usize >= HEIGHT {
    return false;
}

let (new_x, new_y) = (new_x as usize, new_y as usize);

// Avoid - underflow when result is negative
let new_x = (self.player_x as i32 + dx) as usize;  // -1 becomes usize::MAX!
let new_y = (self.player_y as i32 + dy) as usize;

if new_x >= WIDTH || new_y >= HEIGHT {  // This check catches underflow but is confusing
    return false;
}
```

**Examples in codebase:**
- `CaveLayout::move_player()` in `src/location/mine/cave.rs:255-263`
- `CaveLayout::find_center_floor()` in `src/location/mine/cave.rs:194-217`
- `CaveLayout::find_spawn_point()` in `src/location/mine/cave.rs:219-252`
- `CaveLayout::count_wall_neighbors()` in `src/location/mine/cave.rs:459-478`
