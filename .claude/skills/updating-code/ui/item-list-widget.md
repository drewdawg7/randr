# ItemList Widget

## Overview

The `ItemList<T, F>` widget provides a reusable UI component for displaying scrollable, filterable lists of items. It abstracts common list patterns like selection, scrolling, filtering, and Back button support.

**Location**: `src/ui/components/widgets/item_list/`

## Module Structure

```
item_list/
  mod.rs          # Re-exports public types
  traits.rs       # ListItem, ItemFilter traits, InventoryFilter, NoFilter
  definition.rs   # ItemList<T, F> struct and methods
  impls.rs        # Wrapper types: InventoryListItem, StoreBuyItem, etc.
```

## Core Traits

### ListItem (`traits.rs:9`)
Items in the list must implement this trait:
```rust
pub trait ListItem {
    fn item(&self) -> Option<&Item>;           // For quality color, lock icon
    fn display_name(&self) -> Cow<'static, str>;
    fn quantity(&self) -> Option<u32>;          // None for equipment
    fn suffix_spans(&self) -> Vec<Span<'static>> { vec![] }  // Price, cost, etc.
    fn show_lock(&self) -> bool { ... }
    fn forge_material(&self) -> Option<ForgeMaterial> { None }  // For forge filtering
}
```

### ItemFilter<T> (`traits.rs:35`)
Filters must implement this trait:
```rust
pub trait ItemFilter<T>: Clone + Default {
    fn label(&self) -> &'static str;
    fn matches(&self, item: &T) -> bool;
    fn next(&self) -> Self;  // Cycles to next filter state
}
```

### Built-in Filters
- `NoFilter` - Matches all items (default)
- `InventoryFilter` - Cycles: All -> Equipment -> Materials -> Consumables
- `ForgeFilter` - Cycles: All -> Copper -> Tin -> Bronze -> Other (uses `forge_material()`)

## ItemList Struct (`definition.rs:62`)

### Configuration
```rust
pub struct ItemListConfig {
    pub show_filter_button: bool,      // Show [F: Filter] button
    pub show_scroll_indicators: bool,  // "... more above/below ..."
    pub visible_count: usize,          // Items shown (default: 10)
    pub show_back_button: bool,        // Add "Back" option at end
    pub back_label: &'static str,      // Label for Back button
    pub background: Option<Color>,     // Optional background color
}
```

### Key Methods
- `set_items(items: Vec<T>)` - Update list items
- `move_up()` / `move_down()` - Navigate with wrapping
- `cycle_filter()` - Cycle through filter states (call from 'f' key handler)
- `selected_item() -> Option<&T>` - Get currently selected item
- `is_back_selected() -> bool` - Check if Back button is selected
- `reset_selection()` - Reset to first item
- `render(&mut self, Frame, Rect)` - Render the list (clears background)
- `render_to_buffer(&mut self, Frame, Rect, &str)` - Render preserving background (for ASCII art)

## Wrapper Types (`impls.rs`)

| Type | Used In | Suffix Display |
|------|---------|----------------|
| `InventoryListItem` | InventoryModal | Equipment slot, quantity |
| `StoreBuyItem` | StoreTab Buy | Price in gold |
| `SellableItem` | StoreTab Sell | Sell value |
| `UpgradeableItem` | Blacksmith Upgrade | Cost or "MAX" |
| `QualityItem` | Blacksmith Quality | Next quality tier |
| `RecipeItem` | Blacksmith Forge | Ingredient requirements (have/need) |
| `DepositableItem` | StorageScreen (player panel) | None (not selectable if locked/equipped) |
| `StoredItem` | StorageScreen (storage panel) | None |

## Usage Pattern

1. Create config and ItemList in component's `new()`:
```rust
let config = ItemListConfig {
    show_filter_button: true,
    show_scroll_indicators: true,
    visible_count: 10,
    show_back_button: true,
    back_label: "Back",
    background: None,
};
let item_list: ItemList<MyItem, InventoryFilter> = ItemList::new(config);
```

2. In render, rebuild items and call render:
```rust
fn render(&mut self, frame: &mut Frame, area: Rect) {
    self.rebuild_items();  // Refresh item_list.set_items(...)
    self.item_list.render(frame, area);
}
```

3. Handle key events:
```rust
Key::Up => item_list.move_up(),
Key::Down => item_list.move_down(),
Key::Char('f') => item_list.cycle_filter(),
Key::Enter => {
    if item_list.is_back_selected() {
        // Go back
    } else if let Some(item) = item_list.selected_item() {
        // Use item
    }
}
```

## Components Using ItemList

| Component | File | Item Type | Filter |
|-----------|------|-----------|--------|
| InventoryModal | `player/inventory_modal.rs` | InventoryListItem | InventoryFilter |
| StoreTab (buy) | `store/tab.rs` | StoreBuyItem | InventoryFilter |
| StoreTab (sell) | `store/tab.rs` | SellableItem | InventoryFilter |
| StorageScreen (player) | `store/storage.rs` | DepositableItem | InventoryFilter |
| StorageScreen (storage) | `store/storage.rs` | StoredItem | InventoryFilter |
| Blacksmith upgrade | `blacksmith/upgrade.rs` | UpgradeableItem | InventoryFilter |
| Blacksmith quality | `blacksmith/quality.rs` | QualityItem | InventoryFilter |
| Blacksmith forge | `blacksmith/forge.rs` | RecipeItem | ForgeFilter |

## Visual Features

- **Selection prefix**: `> ` (yellow) for selected, `  ` for unselected
- **Lock icon**: LOCK (\u{F023}) shown in bronze before name if item is locked
- **Equip icon**: SHIRT (\u{EE1C}) shown in cyan before name if item is equipped
- **Quality coloring**: Item names colored by ItemQuality
- **Scroll indicators**: "... more above/below ..." when scrolled
- **Filter button**: `[F: All]` or similar at top when enabled
- **Back button**: `< Back` at end of list when enabled
- **Wrapping navigation**: Up at top goes to bottom, Down at bottom goes to top

## Adding New Item Types

1. Create wrapper struct in `impls.rs`
2. Implement `ListItem` trait
3. Add to `mod.rs` exports if needed externally
4. Use with `ItemList<NewWrapper, InventoryFilter>` or `ItemList<NewWrapper, NoFilter>`

## Buffer Rendering (Background Preservation)

When rendering over ASCII art or custom backgrounds (like the forge anvil), use `render_to_buffer()` instead of `render()`. The standard `render()` uses ratatui's `List` widget which clears the background.

```rust
// In render function:
let menu_padding = " ".repeat(h_padding as usize);
item_list.render_to_buffer(frame, menu_area, &menu_padding);
```

The `left_padding` parameter adds consistent left margin to all lines. The method skips unstyled space characters to preserve the underlying background art.

## Hotkeys

- `Up/Down` - Navigate list
- `f/F` - Cycle filter (when enabled)
- `Enter` - Select item or Back button
- `Esc` - Usually bound externally to go back
