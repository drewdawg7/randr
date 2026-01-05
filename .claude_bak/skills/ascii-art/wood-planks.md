# Wood Grain Pattern

## Overview

A dense braille-based wood grain pattern used as a tiling background for the store menu screen.

## Pattern

```
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤
⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤
⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒
⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒
⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠤
⠒⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤
⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒
⠒⠒⠤⠤⠤⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒
⠤⠤⠤⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠤
⠒⠒⠒⠤⠤⠒⠒⠒⠒⠤⠤⠤⠒⠒⠒⠤⠤⠒⠒⠒⠤⠤⠤⠒⠒⠒⠒⠤⠤⠒
```

- **Dimensions**: 30 chars wide x 10 rows
- **Characters**: Dense braille (⠤ and ⠒) for smooth wood grain
- **Usage**: Tiling background for Store

## Colors Used

From `src/ui/theme.rs`:
- `DARK_WALNUT` - Rgb(50, 35, 20) - darkest shade
- `WOOD_BROWN` - Rgb(101, 67, 33) - medium shade
- `OAK_BROWN` - Rgb(139, 90, 43) - lightest shade

## Implementation

**File**: `src/ui/components/store/wood_planks_art.rs`

Key functions:
- `render_wood_planks(frame, area)` - Renders tiled pattern filling entire area
- `generate_plank_line(row, width)` - Generates single line with color variation

## Design Notes

- Dense, uniform braille creates subtle wood grain texture
- Alternating ⠤ and ⠒ suggest horizontal grain direction
- Subtle variation avoids visual distraction
- Color variation adds depth without overwhelming
