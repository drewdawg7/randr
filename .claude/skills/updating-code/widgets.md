# UI Widgets

Reusable widget components in `src/ui/widgets/`.

## ItemStatsDisplay

Widget for rendering item stats with configurable display modes.

**File:** `src/ui/widgets/item_stats_display.rs`

### Display Modes

- `TextOnly`: Renders stats as "HP: +5", "ATK: +3"
- `IconValue`: Renders stats with icon + value format (uses `ItemDetailIconsSlice`)

### Usage

```rust
use crate::ui::widgets::ItemStatsDisplay;
use crate::stats::StatType;

// Create from stats iterator (automatically filters zero values)
parent.spawn(
    ItemStatsDisplay::from_stats_iter(
        StatType::all().iter().map(|st| (*st, item.stats.value(*st)))
    )
    .text_only()  // or .icon_value() for icon mode
    .with_color(text_color.0)
    .with_font_size(18.0),
);
```

### Builder Methods

| Method | Description |
|--------|-------------|
| `from_stats_iter(iter)` | Create from iterator of `(StatType, i32)` pairs |
| `text_only()` | Use text-only format ("HP: +5") |
| `icon_value()` | Use icon + value format |
| `with_color(Color)` | Set text color |
| `with_font_size(f32)` | Set font size (default: 18.0) |

### Implementation Pattern

Uses observer-based component pattern (like `GoldDisplay`):
1. Spawn entity with `ItemStatsDisplay` component
2. Observer triggers on `OnAdd`
3. Observer removes component and builds UI children

## GoldDisplay

Widget for displaying gold amounts with coin icon.

**File:** `src/ui/widgets/gold_display.rs`

```rust
use crate::ui::widgets::GoldDisplay;

parent.spawn(
    GoldDisplay::new(100)
        .with_font_size(18.0)
        .with_color(text_color),
);
```

## ItemGrid

5x5 grid widget for displaying items with selection highlight.

**File:** `src/ui/widgets/item_grid.rs`

```rust
use crate::ui::widgets::{ItemGrid, ItemGridEntry};

parent.spawn(ItemGrid {
    items: vec![ItemGridEntry { sprite_name: "sword".to_string() }],
    selected_index: 0,
    is_focused: true,
});
```

## CentralDetailPanel

Nine-slice panel for item details in buy screen.

**File:** `src/ui/widgets/central_detail_panel.rs`

Uses `InfoPanelSource` to determine which item to display:

```rust
use crate::ui::widgets::CentralDetailPanel;
use crate::screens::town::tabs::store::InfoPanelSource;

row.spawn(CentralDetailPanel {
    source: InfoPanelSource::Store { selected_index: 0 },
});
```

The `populate_central_detail_panel` system in `panels.rs` handles rendering item details.

## Adding New Widgets

1. Create file in `src/ui/widgets/`
2. Define component struct and plugin
3. Use observer pattern for building UI on component add
4. Export from `src/ui/widgets/mod.rs`
5. Register plugin in `src/plugins/game.rs`
