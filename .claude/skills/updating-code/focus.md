# Focus and Selection System

Unified trait-based system for managing selection state across UI screens and modals.

**Module:** `src/ui/focus.rs`

## Overview

The focus system provides:
1. **SelectionState trait** - Standardizes navigation logic for single-list screens
2. **FocusPanel/FocusState** - Centralized focus tracking for multi-panel modals

## Multi-Panel Focus Management

For modals with multiple focusable panels (inventory, merchant, forge, anvil), use the centralized `FocusPanel` enum and `FocusState` resource.

### FocusPanel Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusPanel {
    // Inventory modal
    EquipmentGrid,
    BackpackGrid,
    // Merchant modal
    MerchantStock,
    PlayerInventory,
    // Forge modal
    ForgeCraftingSlots,
    ForgeInventory,
    // Anvil modal
    RecipeGrid,
    AnvilInventory,
}
```

### FocusState Resource

```rust
#[derive(Resource, Default)]
pub struct FocusState {
    pub focused: Option<FocusPanel>,
}

impl FocusState {
    pub fn set_focus(&mut self, panel: FocusPanel);
    pub fn clear(&mut self);
    pub fn is_focused(&self, panel: FocusPanel) -> bool;
}
```

### Usage Pattern

**Important:** Use `Option<Res<FocusState>>` or `Option<ResMut<FocusState>>` since the resource only exists when a modal is open.

```rust
pub fn handle_modal_tab(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    focus_state: Option<ResMut<FocusState>>,
) {
    if active_modal.modal != Some(ModalType::MyModal) {
        return;
    }

    let Some(mut focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            if focus_state.is_focused(FocusPanel::PanelA) {
                focus_state.set_focus(FocusPanel::PanelB);
            } else {
                focus_state.set_focus(FocusPanel::PanelA);
            }
        }
    }
}
```

### ItemGridFocusPanel Marker

Associate an `ItemGrid` with a `FocusPanel` to enable selector visibility based on focus:

```rust
#[derive(Component)]
pub struct ItemGridFocusPanel(pub FocusPanel);

// When spawning a grid:
commands.spawn((
    ItemGrid { items, selected_index: 0, grid_size: 4 },
    ItemGridFocusPanel(FocusPanel::BackpackGrid),
    // ...
));
```

The `update_grid_selector` system automatically shows/hides selectors based on `FocusState`.

### Modal Spawn/Cleanup

Insert `FocusState` when spawning a modal, remove it on cleanup:

```rust
// In spawn system:
commands.insert_resource(FocusState::default());
focus_state.set_focus(FocusPanel::DefaultPanel);

// In cleanup system:
commands.remove_resource::<FocusState>();
```

### Change Detection for Detail Panes

Use `is_changed()` to refresh detail panes when focus changes:

```rust
pub fn update_detail_pane(
    focus_state: Option<Res<FocusState>>,
    // ...
) {
    let Some(focus_state) = focus_state else { return };

    if !focus_state.is_changed() {
        return;
    }

    // Refresh detail pane based on which panel is focused
}
```

### Modals Using FocusState

| Modal | Panels | Default Focus |
|-------|--------|---------------|
| Inventory | EquipmentGrid, BackpackGrid | EquipmentGrid |
| Merchant | MerchantStock, PlayerInventory | MerchantStock |
| Forge | ForgeCraftingSlots, ForgeInventory | ForgeCraftingSlots |
| Anvil | RecipeGrid, AnvilInventory | RecipeGrid |

---

## SelectionState Trait

For single-list screens, the `SelectionState` trait standardizes navigation logic. Instead of each state struct duplicating `up()`, `down()`, `reset()` methods, they implement the trait and get consistent behavior.

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
| `CompendiumListState` | `monster_compendium/state.rs` | Clamped | Added `count` field |
| `ListState` | `town/shared/list_widget.rs` | Wrapping | Overrides `up()`/`down()` with wrapping + scroll |
| `FightScreenState` | `fight/state.rs` | Clamped | Uses wrapper types (see below) |

**Note:** The inventory modal uses 2D grid navigation directly on the `ItemGrid` component (not `SelectionState`). See [inventory-modal.md](inventory-modal.md).

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

## Files Changed in Issue #373 (FocusState)

| File | Change |
|------|--------|
| `src/ui/focus.rs` | Added `FocusPanel` enum and `FocusState` resource |
| `src/ui/widgets/item_grid.rs` | Removed `is_focused`, added `ItemGridFocusPanel` marker |
| `src/ui/widgets/mod.rs` | Export `ItemGridFocusPanel` |
| `src/ui/screens/town/shared/mod.rs` | Extended `InfoPanelSource` with ForgeSlot, Recipe, None variants |
| `src/ui/screens/inventory_modal/input.rs` | Use `Option<Res/ResMut<FocusState>>` |
| `src/ui/screens/inventory_modal/render.rs` | Use FocusState for selector visibility |
| `src/ui/screens/merchant_modal/input.rs` | Use `Option<Res/ResMut<FocusState>>` |
| `src/ui/screens/merchant_modal/render.rs` | Use FocusState, removed MerchantDetailRefresh |
| `src/ui/screens/merchant_modal/state.rs` | Removed `MerchantDetailRefresh` resource |
| `src/ui/screens/forge_modal/input.rs` | Use `Option<Res/ResMut<FocusState>>` |
| `src/ui/screens/forge_modal/render.rs` | Use FocusState |
| `src/ui/screens/forge_modal/state.rs` | Removed `crafting_focused` field |
| `src/ui/screens/forge_modal/mod.rs` | Export `ForgeSlotIndex` |
| `src/ui/screens/anvil_modal/input.rs` | Use `Option<Res/ResMut<FocusState>>` |
| `src/ui/screens/anvil_modal/render.rs` | Use FocusState |
| `src/ui/screens/anvil_modal/state.rs` | Removed `AnvilModalState` |
| `src/ui/screens/anvil_modal/plugin.rs` | Removed AnvilModalState cleanup |
| `src/ui/screens/dungeon/plugin.rs` | Removed AnvilModalState usage |

## Related Documentation

- [modals.md](modals.md) - Modal screen patterns
- [navigation.md](navigation.md) - Navigation system
- [inventory-modal.md](inventory-modal.md) - Inventory modal implementation
- [merchant-modal.md](merchant-modal.md) - Merchant modal implementation
- [forge-modal.md](forge-modal.md) - Forge modal implementation
- [anvil-modal.md](anvil-modal.md) - Anvil modal implementation
