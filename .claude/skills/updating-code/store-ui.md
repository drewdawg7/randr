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

## Key Files

- `src/ui/widgets/item_grid.rs` - ItemGrid widget with focus support
- `src/screens/town/tabs/store/render.rs` - StoreInfoPanel, InfoPanelSource, spawn_info_panel
- `src/item/definitions.rs` - ItemId::sprite_name() method
