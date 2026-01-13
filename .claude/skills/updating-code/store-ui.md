# Store UI Patterns

## ItemGrid Widget

The `ItemGrid` widget (`src/ui/widgets/item_grid.rs`) displays a 5x5 grid of items with optional selection.

### Fields

```rust
pub struct ItemGrid {
    pub items: Vec<ItemGridEntry>,  // Items to display (up to 25)
    pub selected_index: usize,       // Currently selected cell
    pub is_focused: bool,            // Show selector when true (default: true)
}

pub struct ItemGridEntry {
    pub sprite_name: String,  // Slice name in IconItems sheet (e.g., "Slice_337")
}
```

### Usage

```rust
// Focused grid (shows selector)
col.spawn(ItemGrid {
    items: items.iter().map(|item| ItemGridEntry {
        sprite_name: item.item_id.sprite_name().to_string(),
    }).collect(),
    selected_index: selection_state.selected,
    is_focused: true,
});

// Unfocused grid (read-only, no selector)
col.spawn(ItemGrid {
    items: inventory_entries,
    selected_index: 0,
    is_focused: false,
});
```

## ItemId Sprite Mapping

Use `ItemId::sprite_name()` to get the icon sprite slice name for any item:

```rust
// In src/item/definitions.rs
impl ItemId {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            ItemId::BasicHPPotion => "Slice_337",
            ItemId::Sword => "Slice_155",
            ItemId::BasicShield => "Slice_100",
            // ... add more as needed
            _ => "Slice_337", // fallback
        }
    }
}
```

## StoreInfoPanel

The `StoreInfoPanel` component displays item details above grids. It uses `InfoPanelSource` to determine what item to show.

### Pattern

```rust
// In src/screens/town/tabs/store/render.rs
pub enum InfoPanelSource {
    Store { selected_index: usize },      // Show from BUYABLE_ITEMS
    Inventory { selected_index: usize },  // Show from player Inventory
}

#[derive(Component)]
pub struct StoreInfoPanel {
    pub source: InfoPanelSource,
}
```

### Usage

```rust
// Spawn with helper function
spawn_info_panel(
    parent,
    InfoPanelSource::Store { selected_index: store_selections.buy.selected },
    game_sprites,
);

// The populate_store_info_panel system handles both sources
```

## Dual-Grid Layout

For side-by-side grids (store on left, inventory on right), use `JustifyContent::SpaceBetween`:

```rust
parent
    .spawn(Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Percent(100.0),
        ..default()
    })
    .with_children(|row| {
        // Left column
        row.spawn(Node { flex_direction: FlexDirection::Column, ..default() })
            .with_children(|col| { /* info panel + grid */ });

        // Right column
        row.spawn(Node { flex_direction: FlexDirection::Column, ..default() })
            .with_children(|col| { /* info panel + grid */ });
    });
```

## Focus Toggle for Dual-Grid Layouts

When using side-by-side grids, use a focus enum to track which grid is active. The `BuyFocus` pattern in the store demonstrates this:

### State (`src/screens/town/tabs/store/state.rs`)

```rust
/// Which panel is focused in the buy screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuyFocus {
    #[default]
    Store,
    Inventory,
}

pub struct StoreSelections {
    pub buy: SelectionState,           // Store grid selection
    pub buy_inventory: SelectionState, // Inventory grid selection
    pub buy_focus: BuyFocus,           // Which grid is focused
    // ...
}
```

### Input Handling (`src/screens/town/tabs/store/input.rs`)

```rust
// Toggle focus with Space (GameAction::Mine)
GameAction::Mine => {
    store_selections.buy_focus = match store_selections.buy_focus {
        BuyFocus::Store => BuyFocus::Inventory,
        BuyFocus::Inventory => BuyFocus::Store,
    };
}

// Navigate within focused grid
GameAction::Navigate(dir) => {
    match store_selections.buy_focus {
        BuyFocus::Store => navigate_grid(&mut store_selections.buy, ...),
        BuyFocus::Inventory => navigate_grid(&mut store_selections.buy_inventory, ...),
    }
}
```

### Rendering (`src/screens/town/tabs/store/render.rs`)

```rust
let store_focused = store_selections.buy_focus == BuyFocus::Store;

// Store grid - focused when store_focused is true
col.spawn(ItemGrid {
    selected_index: store_selections.buy.selected,
    is_focused: store_focused,
    ...
});

// Inventory grid - focused when store_focused is false
col.spawn(ItemGrid {
    selected_index: store_selections.buy_inventory.selected,
    is_focused: !store_focused,
    ...
});
```

### Navigation Hint

Include the toggle key in the hint:
```rust
spawn_navigation_hint(parent, "[↑↓←→] Navigate  [Space] Switch Panel  [Enter] Buy  [Backspace] Back");
```

## Key Files

- `src/ui/widgets/item_grid.rs` - ItemGrid widget with focus support
- `src/screens/town/tabs/store/render.rs` - StoreInfoPanel, InfoPanelSource, spawn_info_panel
- `src/screens/town/tabs/store/state.rs` - BuyFocus enum, StoreSelections with dual selection states
- `src/screens/town/tabs/store/input.rs` - Focus toggle handling, navigate_grid helper
- `src/item/definitions.rs` - ItemId::sprite_name() method
