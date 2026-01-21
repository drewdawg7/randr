# StatRow

Label-value row with optional bonus text.

**File:** `src/ui/widgets/stat_row.rs`

## Usage

```rust
use crate::ui::widgets::StatRow;

parent.spawn(StatRow::new("Attack:", "12"));

// With bonus
parent.spawn(
    StatRow::new("Attack:", "12")
        .with_bonus("(+3)", Color::srgb(0.4, 1.0, 0.4))
);

// Customized
parent.spawn(
    StatRow::new("HP:", "10/20")
        .label_width(140.0)
        .font_size(22.0)
        .value_color(Color::srgb(0.95, 0.3, 0.3))
);
```

## Builder Methods

| Method | Default |
|--------|---------|
| `new(label, value)` | - |
| `with_bonus(text, color)` | None |
| `label_width(f32)` | 120.0 |
| `font_size(f32)` | 20.0 |
| `column_gap(f32)` | 10.0 |
| `label_color(Color)` | gray (0.75) |
| `value_color(Color)` | white |
| `bottom_margin(f32)` | None |

Defaults available in `stat_row::defaults`.
