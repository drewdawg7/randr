# ItemStatsDisplay

Renders item stats with configurable display modes.

**File:** `src/ui/widgets/item_stats_display.rs`

## Display Modes

- `TextOnly`: "HP: +5", "ATK: +3"
- `IconValue`: icon + value (uses `ItemDetailIconsSlice`)

## Usage

```rust
use crate::ui::widgets::ItemStatsDisplay;
use crate::stats::StatType;

parent.spawn(
    ItemStatsDisplay::from_stats_iter(
        StatType::all().iter().map(|st| (*st, item.stats.value(*st)))
    )
    .text_only()  // or .icon_value()
    .with_color(text_color.0)
    .with_font_size(18.0),
);
```

## Builder Methods

| Method | Description |
|--------|-------------|
| `from_stats_iter(iter)` | Create from `(StatType, i32)` pairs |
| `text_only()` | Text format |
| `icon_value()` | Icon + value format |
| `with_color(Color)` | Text color |
| `with_font_size(f32)` | Font size (default: 18.0) |

## Implementation

Uses observer-based pattern: spawn entity with component, observer triggers on `OnAdd`, removes component and builds UI children.
