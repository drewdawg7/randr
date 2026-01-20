# UI Widgets

Reusable widget components in `src/ui/widgets/`.

## StatRow

Widget for displaying a label-value row with optional bonus text.

**File:** `src/ui/widgets/stat_row.rs`

### Usage

```rust
use crate::ui::widgets::StatRow;

// Basic usage
parent.spawn(StatRow::new("Attack:", "12"));

// With bonus text
parent.spawn(
    StatRow::new("Attack:", "12")
        .with_bonus("(+3)", Color::srgb(0.4, 1.0, 0.4))
);

// Fully customized
parent.spawn(
    StatRow::new("HP:", "10/20")
        .label_width(140.0)
        .font_size(22.0)
        .column_gap(10.0)
        .label_color(Color::srgb(0.8, 0.8, 0.8))
        .value_color(Color::srgb(0.95, 0.3, 0.3))
        .bottom_margin(8.0)
);
```

### Builder Methods

| Method | Description | Default |
|--------|-------------|---------|
| `new(label, value)` | Create with label and value | - |
| `with_bonus(text, color)` | Add bonus text after value | None |
| `label_width(f32)` | Width of label column | 120.0 |
| `font_size(f32)` | Font size for all text | 20.0 |
| `column_gap(f32)` | Gap between columns | 10.0 |
| `label_color(Color)` | Label text color | gray (0.75) |
| `value_color(Color)` | Value text color | white |
| `bottom_margin(f32)` | Bottom margin on row | None |

### Default Constants

Available in `stat_row::defaults`:
- `LABEL_WIDTH`: 120.0
- `FONT_SIZE`: 20.0
- `COLUMN_GAP`: 10.0
- `LABEL_COLOR`: gray (0.75, 0.75, 0.75)
- `VALUE_COLOR`: white

## IconValueRow

Widget for displaying an icon and value in a row.

**File:** `src/ui/widgets/icon_value_row.rs`

### Usage

```rust
use crate::ui::widgets::IconValueRow;
use crate::assets::ItemDetailIconsSlice;
use crate::stats::StatType;

// With a specific icon slice
parent.spawn(IconValueRow::new(ItemDetailIconsSlice::AttackIcon, "15"));

// For a stat type (auto-selects appropriate icon)
parent.spawn(IconValueRow::for_stat(StatType::Attack, 15));

// Customized
parent.spawn(
    IconValueRow::new(ItemDetailIconsSlice::HealthIcon, "10/20")
        .icon_size(20.0)
        .font_size(18.0)
        .text_color(Color::srgb(0.8, 0.3, 0.3))
);
```

### Builder Methods

| Method | Description | Default |
|--------|-------------|---------|
| `new(slice, value)` | Create with icon slice and value | - |
| `for_stat(stat_type, value)` | Create for a stat type (auto-maps icon) | - |
| `icon_size(f32)` | Icon width/height | 16.0 |
| `font_size(f32)` | Font size for value text | 18.0 |
| `column_gap(f32)` | Gap between icon and value | 4.0 |
| `text_color(Color)` | Value text color | brown (0.4, 0.25, 0.15) |

### Default Constants

Available in `icon_value_row::defaults`:
- `ICON_SIZE`: 16.0
- `FONT_SIZE`: 18.0
- `COLUMN_GAP`: 4.0
- `TEXT_COLOR`: brown (0.4, 0.25, 0.15)

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

## spawn_nine_slice_panel

Generic helper function for spawning nine-slice panel backgrounds.

**File:** `src/ui/widgets/nine_slice.rs`

### Usage

```rust
use crate::ui::widgets::spawn_nine_slice_panel;
use crate::assets::ShopBgSlice;  // or DetailPanelSlice

// Spawn a nine-slice panel background
parent.with_children(|parent| {
    spawn_nine_slice_panel::<ShopBgSlice>(parent, &game_sprites, width, height);
});
```

### How It Works

The function creates an absolute-positioned 3x3 CSS grid that:
- Uses corner slices at fixed size (e.g., 48x48)
- Stretches top/bottom edge slices horizontally
- Stretches left/right edge slices vertically
- Stretches center slice in both dimensions

### NineSlice Trait

Types implementing `NineSlice` (in `src/assets/sprite_slices.rs`) can be used with this helper:

```rust
pub trait NineSlice: Copy {
    const ALL: [Self; 9];        // Slices in row-major order (TL, TC, TR, ML, C, MR, BL, BC, BR)
    const SLICE_SIZE: f32;       // Size of corner slices
    const SHEET_KEY: SpriteSheetKey;  // Sprite sheet to use

    fn as_str(self) -> &'static str;  // Slice name for lookup
    fn position(self) -> SlicePosition;  // Auto-derived from index
    fn dimensions(self, stretch_width: f32, stretch_height: f32) -> (f32, f32);
}
```

### Implemented Types

| Type | Sheet Key | Slice Size | Used By |
|------|-----------|------------|---------|
| `ShopBgSlice` | `ShopBgSlices` | 48.0 | `ItemGrid` |
| `DetailPanelSlice` | `DetailPanelBg` | 48.0 | `CentralDetailPanel` |

### Adding New Nine-Slice Panels

1. Add slice enum to `src/assets/sprite_slices.rs` with 9 variants
2. Implement `as_str()` and `const ALL` for the enum
3. Implement `NineSlice` trait with appropriate `SLICE_SIZE` and `SHEET_KEY`
4. Export from `src/assets/mod.rs`
5. Use `spawn_nine_slice_panel::<YourSlice>()` in widgets

## Adding New Widgets

1. Create file in `src/ui/widgets/`
2. Define component struct and plugin
3. Use observer pattern for building UI on component add
4. Export from `src/ui/widgets/mod.rs`
5. Register plugin in `src/plugins/game.rs`
