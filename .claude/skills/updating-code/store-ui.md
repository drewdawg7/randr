# Store UI Patterns

UI for the store screen in `src/screens/town/tabs/store/`.

## ItemGrid Widget

The `ItemGrid` widget (`src/ui/widgets/item_grid.rs`) displays a grid of items with configurable size.

```rust
col.spawn(ItemGrid {
    items: store.inventory.iter().map(|store_item| ItemGridEntry {
        sprite_name: store_item.item_id.sprite_name().to_string(),
    }).collect(),
    selected_index: store_selections.buy.selected,
    is_focused: true,
    grid_size: 5,  // 5x5 grid
});
```

### Grid Navigation

Use `ItemGrid::navigate()` for 2D grid navigation. Allows navigation to all grid slots including empty ones:

```rust
if let GameAction::Navigate(direction) = action {
    if let Ok(mut grid) = grids.get_single_mut() {
        if grid.is_focused {
            grid.navigate(*direction);
        }
    }
}
```

The `navigate()` method handles boundary checking based on `grid_size` (not item count), so the selector can move to empty cells.

## ItemDetailPane

Buy screen central panel (`src/ui/widgets/item_detail_pane.rs`) with nine-slice background using `DetailPanelSlice`.

Uses `InfoPanelSource` to determine which item to display:

```rust
pub enum InfoPanelSource {
    Store { selected_index: usize },      // Show from store.inventory
    Inventory { selected_index: usize },  // Show from player Inventory
}
```

The store's populate system in `panels.rs` renders item details using `ItemStatsDisplay` widget.

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
- `src/screens/town/tabs/store/state.rs` - BuyFocus, StoreSelections, StoreModeKind

### Render Module Structure

The render logic is split into mode-specific files under `src/screens/town/tabs/store/render/`:

| File | Purpose |
|------|---------|
| `mod.rs` | Orchestration (`spawn_store_ui_inner`), types (`InfoPanelSource`) |
| `menu.rs` | Main menu UI, `STORE_MENU_OPTIONS` constant |
| `buy.rs` | Buy screen with dual `ItemGrid` widgets |
| `sell.rs` | Sell screen with inventory list |
| `storage.rs` | Storage menu/view/deposit screens, `STORAGE_MENU_OPTIONS` |
| `helpers.rs` | `spawn_inventory_list` - reusable inventory list widget |
| `panels.rs` | `populate_central_detail_panel` system, item extraction helpers |

### Adding New Store Modes

1. Add variant to `StoreModeKind` in `state.rs`
2. Create new file in `render/` (e.g., `render/newmode.rs`)
3. Add match arm in `spawn_store_ui_inner` in `render/mod.rs`
4. Export from `render/mod.rs` if needed externally
