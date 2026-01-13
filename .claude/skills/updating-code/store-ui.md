# Store UI Patterns

UI for the store screen in `src/screens/town/tabs/store/`.

## ItemGrid Widget

The `ItemGrid` widget (`src/ui/widgets/item_grid.rs`) displays a 5x5 grid of items.

```rust
col.spawn(ItemGrid {
    items: store.inventory.iter().map(|store_item| ItemGridEntry {
        sprite_name: store_item.item_id.sprite_name().to_string(),
    }).collect(),
    selected_index: store_selections.buy.selected,
    is_focused: true,
});
```

## StoreInfoPanel

Displays item details above grids using `InfoPanelSource`:

```rust
pub enum InfoPanelSource {
    Store { selected_index: usize },      // Show from store.inventory
    Inventory { selected_index: usize },  // Show from player Inventory
}
```

The `populate_store_info_panel` system reads from `Res<Store>` and `Res<Inventory>`.

## Focus Toggle for Dual-Grid Layouts

The buy screen has two grids (store left, inventory right) with focus toggling:

### State (`src/screens/town/tabs/store/state.rs`)

```rust
#[derive(Default)]
pub enum BuyFocus {
    #[default]
    Store,
    Inventory,
}
```

### Input (`src/screens/town/tabs/store/input.rs`)

```rust
GameAction::Mine => {
    // Toggle focus with Space
    store_selections.buy_focus = match store_selections.buy_focus {
        BuyFocus::Store => BuyFocus::Inventory,
        BuyFocus::Inventory => BuyFocus::Store,
    };
}

GameAction::Select => {
    match store_selections.buy_focus {
        BuyFocus::Store => {
            // Purchase from store
            purchase_events.send(PurchaseEvent { index: store_selections.buy.selected });
        }
        BuyFocus::Inventory => {
            // Sell from inventory
            sell_events.send(SellEvent { inventory_index: store_selections.buy_inventory.selected });
        }
    }
}
```

## Key Files

- `src/screens/town/tabs/store/input.rs` - Input handling, sends events
- `src/screens/town/tabs/store/render.rs` - UI rendering, uses Store resource
- `src/screens/town/tabs/store/state.rs` - BuyFocus, StoreSelections
