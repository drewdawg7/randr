# Wood Planks Pattern

## Overview

A braille-based wood plank pattern used as a tiling background for the store menu screen.

## Pattern

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⠀⠀⠤⠀⠀⠀⠀⠀⠐⠀⠀⠀⠤⠀⠀⠀⠀⠀⠀⠤⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠤⠀⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠤⠀⠀⠀⠐⠀⠀
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⠀⠤⠀⠀⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠤⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠤⠀⠀⠀
⠀⠀⠀⠐⠀⠀⠤⠀⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠤⠀⠀⠀⠐⠀⠀⠀⠀⠀
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
⠀⠀⠀⠀⠀⠤⠀⠀⠀⠀⠐⠀⠀⠀⠀⠤⠀⠀⠀⠀⠀⠀⠐⠀⠀⠀⠀⠀⠤⠀
⠀⠐⠀⠀⠀⠀⠀⠀⠤⠀⠀⠀⠀⠀⠐⠀⠀⠀⠀⠤⠀⠀⠀⠀⠀⠀⠀⠐⠀⠀
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

- **Dimensions**: 30 chars wide x 10 rows
- **Characters**: Box drawing (━) + braille (sparse dots for grain)
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

- Horizontal lines (━) suggest wooden plank separations
- Sparse braille dots create wood grain texture
- Pattern alternates between grain rows and separator rows
- Evokes a merchant shop floor/counter feel
