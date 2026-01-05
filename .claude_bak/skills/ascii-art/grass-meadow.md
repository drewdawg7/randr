# Grass/Meadow Pattern

## Overview

A braille-based grass pattern used as a tiling background for the field menu screen.

## Pattern

```
⡀⠀⢀⠀⠀⡀⠀⠀⢀⠀⡀⠀⠀⢀⠀⠀⡀⠀⢀⠀⠀⡀⠀⠀⢀⠀⡀⠀⠀⢀
⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠀⠁⠈⠀⠁⠈⠀⠁⠀⠁⠈⠀⠁⠈⠀⠁⠀⠁⠈⠀
⢀⠀⡀⠀⢀⠀⠀⡀⠀⢀⠀⠀⡀⢀⠀⠀⡀⠀⢀⠀⡀⠀⠀⢀⠀⡀⠀⢀⠀⠀
⠀⠁⠀⠈⠀⠁⠈⠀⠁⠀⠈⠁⠀⠀⠁⠈⠀⠁⠀⠀⠁⠈⠁⠀⠀⠁⠈⠀⠁⠈
⡀⠀⠀⢀⠀⡀⠀⢀⠀⠀⡀⠀⢀⠀⡀⠀⠀⢀⠀⡀⠀⢀⠀⠀⡀⠀⢀⠀⡀⠀
⠈⠁⠈⠀⠁⠀⠁⠀⠈⠁⠀⠁⠀⠀⠁⠈⠁⠀⠁⠀⠁⠀⠈⠁⠀⠁⠀⠀⠁⠈
⠀⢀⠀⡀⠀⠀⢀⠀⡀⠀⢀⠀⠀⡀⢀⠀⡀⠀⠀⢀⠀⡀⠀⢀⠀⠀⡀⢀⠀⡀
⠁⠀⠁⠀⠈⠁⠀⠁⠀⠁⠀⠈⠁⠀⠀⠁⠀⠈⠁⠀⠁⠀⠁⠀⠈⠁⠀⠀⠁⠀
⢀⠀⠀⡀⠀⢀⠀⠀⡀⢀⠀⡀⠀⠀⢀⠀⡀⠀⢀⠀⠀⡀⢀⠀⡀⠀⠀⢀⠀⡀
⠀⠈⠁⠀⠁⠀⠈⠁⠀⠀⠁⠀⠈⠁⠀⠁⠀⠁⠀⠈⠁⠀⠀⠁⠀⠈⠁⠀⠁⠀
```

- **Dimensions**: 30 chars wide x 10 rows
- **Characters**: Braille dots suggesting grass blades
- **Usage**: Tiling background for Field

## Colors Used

From `src/ui/theme.rs`:
- `DARK_FOREST` - Rgb(34, 85, 51) - darkest shade
- `FOREST_GREEN` - Rgb(34, 139, 34) - medium shade
- `LIME_GREEN` - Rgb(124, 179, 66) - lightest shade

## Implementation

**File**: `src/ui/components/field/grass_art.rs`

Key functions:
- `render_grass_field(frame, area)` - Renders tiled pattern filling entire area
- `generate_grass_line(row, width)` - Generates single line with color variation

## Design Notes

- Alternating rows of upward (⡀⢀) and downward (⠁⠈) braille dots
- Creates organic, natural grass blade appearance
- Low density provides subtle texture without overwhelming
- Color variation adds depth and natural feel
