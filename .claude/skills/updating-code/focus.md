# Focus and Selection System

Unified trait-based system for managing selection state across UI screens and modals.

**Module:** `src/ui/focus.rs`

## Overview

The focus system provides a single `SelectionState` trait that standardizes navigation logic across all screens. Instead of each state struct duplicating `up()`, `down()`, `reset()` methods, they implement the trait and get consistent behavior.

## Core Trait

### SelectionState

```rust
pub trait SelectionState {
    /// Returns the currently selected index.
    fn selected(&self) -> usize;

    /// Returns the total number of items.
    fn count(&self) -> usize;

    /// Sets the selected index.
    fn set_selected(&mut self, index: usize);

    // Default implementations (can be overridden):
    fn up(&mut self);         // Clamped navigation
    fn down(&mut self);       // Clamped navigation
    fn up_wrap(&mut self);    // Wrapping navigation
    fn down_wrap(&mut self);  // Wrapping navigation
    fn reset(&mut self);      // Reset to 0
    fn clamp_to_bounds(&mut self);  // Clamp after count changes
}
```

## Implementations

| Resource | File | Navigation | Notes |
|----------|------|------------|-------|
| `InventorySelection` | `inventory_modal/state.rs` | Clamped | Uses default `up()`/`down()` |
| `CompendiumListState` | `monster_compendium/state.rs` | Clamped | Added `count` field |
| `ListState` | `town/shared/list_widget.rs` | Wrapping | Overrides `up()`/`down()` with wrapping + scroll |
| `FightScreenState` | `fight/state.rs` | Clamped | Uses wrapper types (see below) |

### FightScreenState Wrappers

`FightScreenState` manages two separate selections (action menu and post-combat menu). Instead of implementing `SelectionState` directly, it uses wrapper types:

```rust
// In fight/state.rs
pub struct ActionSelection<'a>(pub &'a mut FightScreenState);
pub struct PostCombatSelection<'a>(pub &'a mut FightScreenState);

// Usage in input.rs:
ActionSelection(&mut fight_state).up();
PostCombatSelection(&mut fight_state).down();
```

## Navigation Patterns

### Clamped Navigation (Default)

Stops at boundaries. Used for menus where wrapping doesn't make sense.

```rust
// Default trait implementations
fn up(&mut self) {
    if self.selected() > 0 {
        self.set_selected(self.selected() - 1);
    }
}

fn down(&mut self) {
    if self.selected() + 1 < self.count() {
        self.set_selected(self.selected() + 1);
    }
}
```

### Wrapping Navigation

Wraps around at boundaries. Override `up()` and `down()` to use `up_wrap()` and `down_wrap()`:

```rust
impl SelectionState for ListState {
    fn up(&mut self) {
        self.up_wrap();
        self.update_scroll();  // Additional scroll management
    }

    fn down(&mut self) {
        self.down_wrap();
        self.update_scroll();
    }
}
```

## Adding SelectionState to a New Screen

1. Add the trait import:
   ```rust
   use crate::ui::SelectionState;
   ```

2. Implement the trait:
   ```rust
   impl SelectionState for MyScreenState {
       fn selected(&self) -> usize { self.selected_index }
       fn count(&self) -> usize { self.item_count }
       fn set_selected(&mut self, index: usize) { self.selected_index = index; }
   }
   ```

3. Use in input handlers:
   ```rust
   pub fn handle_input(mut state: ResMut<MyScreenState>) {
       // Trait methods available via import
       state.up();    // or state.down(), state.reset()
   }
   ```

4. For dynamic counts, use `set_count()` pattern:
   ```rust
   impl MyScreenState {
       pub fn set_count(&mut self, count: usize) {
           self.item_count = count;
           self.clamp_to_bounds();  // Trait method
       }
   }
   ```

## Generic Visual Update Systems

The module provides system factories for reactive visual updates:

```rust
// Text color updates
app.add_systems(Update,
    selection_text_color_system::<MyMarker, MyState>(my_color_fn)
);

// Background color updates
app.add_systems(Update,
    selection_background_system::<MyMarker, MyState>(SELECTED_COLOR, NORMAL_COLOR)
);
```

These only run when the state resource changes (`is_changed()`).

## Files Changed in Issue #337

| File | Change |
|------|--------|
| `src/ui/focus.rs` | **NEW** - Core trait and systems |
| `src/ui/mod.rs` | Added `focus` module |
| `src/ui/screens/fight/state.rs` | Wrapper types for dual selection |
| `src/ui/screens/fight/input.rs` | Use wrapper types |
| `src/ui/screens/inventory_modal/state.rs` | Implement trait |
| `src/ui/screens/inventory_modal/input.rs` | Import trait |
| `src/ui/screens/monster_compendium/state.rs` | Implement trait, add count |
| `src/ui/screens/monster_compendium/input.rs` | Use trait methods |
| `src/ui/screens/town/shared/list_widget.rs` | Implement trait with wrapping |
| `src/ui/screens/town/shared/mod.rs` | Removed `SelectionState` alias |
| `src/navigation/systems.rs` | Import trait for `reset()` |

## Related Documentation

- [modals.md](modals.md) - Modal screen patterns
- [navigation.md](navigation.md) - Navigation system
