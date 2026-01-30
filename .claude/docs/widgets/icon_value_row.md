# IconValueRow

Icon and value in a row.

**File:** `src/ui/widgets/icon_value_row.rs`

## Usage

```rust
use crate::ui::widgets::IconValueRow;
use crate::assets::ItemDetailIconsSlice;
use crate::stats::StatType;

// With specific icon
parent.spawn(IconValueRow::new(ItemDetailIconsSlice::AttackIcon, "15"));

// For stat type (auto-selects icon)
parent.spawn(IconValueRow::for_stat(StatType::Attack, 15));

// Customized
parent.spawn(
    IconValueRow::new(ItemDetailIconsSlice::HealthIcon, "10/20")
        .icon_size(20.0)
        .font_size(18.0)
);
```

## Builder Methods

| Method | Default |
|--------|---------|
| `new(slice, value)` | - |
| `for_stat(stat_type, value)` | - |
| `icon_size(f32)` | 16.0 |
| `font_size(f32)` | 18.0 |
| `column_gap(f32)` | 4.0 |
| `text_color(Color)` | brown (0.4, 0.25, 0.15) |

Defaults available in `icon_value_row::defaults`.
