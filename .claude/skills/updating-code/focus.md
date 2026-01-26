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
    pub fn toggle_between(&mut self, first: FocusPanel, second: FocusPanel);
}
```

The `toggle_between` method is useful for Tab key handlers to cycle between two panels:

```rust
pub fn handle_modal_tab(
    mut action_reader: EventReader<GameAction>,
    focus_state: Option<ResMut<FocusState>>,
) {
    let Some(mut focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            focus_state.toggle_between(FocusPanel::PanelA, FocusPanel::PanelB);
        }
    }
}
```

### Usage Pattern

**Important:** Use `Option<Res<FocusState>>` or `Option<ResMut<FocusState>>` since the resource only exists when a modal is open.

**Run Conditions:** Modal input systems use `run_if(in_*_modal)` conditions in the plugin,
so individual handlers don't need to check `ActiveModal`. See [modals.md](modals.md) for details.

```rust
// input.rs - no ActiveModal check needed; plugin uses run_if(in_my_modal)
pub fn handle_modal_tab(
    mut action_reader: EventReader<GameAction>,
    focus_state: Option<ResMut<FocusState>>,
) {
    let Some(mut focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            focus_state.toggle_between(FocusPanel::PanelA, FocusPanel::PanelB);
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

Detail pane systems are split into two responsibilities using Bevy's change detection:

**1. Source Update System** - Determines which item to show:
```rust
pub fn update_modal_detail_pane_source(
    focus_state: Option<Res<FocusState>>,
    grid_a: Query<Ref<ItemGrid>, With<GridAMarker>>,
    grid_b: Query<Ref<ItemGrid>, With<GridBMarker>>,
    mut panes: Query<&mut ItemDetailPane>,
) {
    let Some(focus_state) = focus_state else { return };

    // Only run when focus or grid selection changed
    let focus_changed = focus_state.is_changed();
    let grid_a_changed = grid_a.get_single().map(|g| g.is_changed()).unwrap_or(false);
    let grid_b_changed = grid_b.get_single().map(|g| g.is_changed()).unwrap_or(false);

    if !focus_changed && !grid_a_changed && !grid_b_changed {
        return;
    }

    // Determine source from focused grid
    let source = if focus_state.is_focused(FocusPanel::GridA) {
        grid_a.get_single().ok().map(|g| InfoPanelSource::Equipment { selected_index: g.selected_index })
    } else {
        grid_b.get_single().ok().map(|g| InfoPanelSource::Inventory { selected_index: g.selected_index })
    };

    // Update pane source (only if different to avoid unnecessary Changed trigger)
    for mut pane in &mut panes {
        if let Some(source) = source {
            if pane.source != source {
                pane.source = source;
            }
        }
    }
}
```

**2. Content Population System** - Renders content when source or data changes:
```rust
pub fn populate_modal_detail_pane_content(
    mut commands: Commands,
    inventory: Res<Inventory>,
    panes: Query<Ref<ItemDetailPane>>,  // Use Ref<T> for change detection
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let inventory_changed = inventory.is_changed();

    for pane in &panes {
        // Check if we need to update: pane.source changed OR data changed
        if !pane.is_changed() && !inventory_changed {
            continue;
        }

        // Despawn existing content and spawn new based on pane.source
    }
}
```

**Key Benefits:**
- Systems only run when relevant state changes
- Uses `Ref<T>` for manual change detection without query filters
- Separates source selection from content rendering
- Handles both source changes and underlying data changes

### Modals Using FocusState

| Modal | Panels | Default Focus |
|-------|--------|---------------|
| Inventory | EquipmentGrid, BackpackGrid | EquipmentGrid |
| Merchant | MerchantStock, PlayerInventory | MerchantStock |
| Forge | ForgeCraftingSlots, ForgeInventory | ForgeCraftingSlots |
| Anvil | RecipeGrid, AnvilInventory | RecipeGrid |
| Monster Compendium | CompendiumMonsterList, CompendiumDropsList | CompendiumMonsterList |

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
| `CompendiumListState` | `monster_compendium/state.rs` | Clamped | Monster list selection |
| `DropsListState` | `monster_compendium/state.rs` | Clamped | Drops list selection |
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

## Files Changed in Issue #374 (Modal Run Conditions)

| File | Change |
|------|--------|
| `src/ui/screens/modal.rs` | Added `in_inventory_modal`, `in_merchant_modal`, `in_forge_modal`, `in_anvil_modal` run conditions |
| `src/ui/focus.rs` | Added `toggle_between` method to `FocusState` |
| `src/ui/screens/inventory_modal/plugin.rs` | Added `.run_if(in_inventory_modal)` to input systems |
| `src/ui/screens/inventory_modal/input.rs` | Removed `ActiveModal` checks, simplified tab handler |
| `src/ui/screens/merchant_modal/plugin.rs` | Added `.run_if(in_merchant_modal)` to input systems |
| `src/ui/screens/merchant_modal/input.rs` | Removed `ActiveModal` checks, simplified tab handler |
| `src/ui/screens/forge_modal/plugin.rs` | Added `.run_if(in_forge_modal)` to input systems |
| `src/ui/screens/forge_modal/input.rs` | Removed `ActiveModal` checks, simplified tab handler |
| `src/ui/screens/anvil_modal/plugin.rs` | Added `.run_if(in_anvil_modal)` to input systems |
| `src/ui/screens/anvil_modal/input.rs` | Removed `ActiveModal` checks, simplified tab handler |

## Related Documentation

- [modals.md](modals.md) - Modal screen patterns
- [navigation.md](navigation.md) - Navigation system
- [inventory-modal.md](inventory-modal.md) - Inventory modal implementation
- [merchant-modal.md](merchant-modal.md) - Merchant modal implementation
- [forge-modal.md](forge-modal.md) - Forge modal implementation
- [anvil-modal.md](anvil-modal.md) - Anvil modal implementation
