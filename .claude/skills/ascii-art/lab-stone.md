# Laboratory Stone Pattern

## Overview

A braille-based dark stone pattern with purple tint, used as a tiling background for the alchemist menu screen. Based on the blacksmith's stone wall pattern but with mystic purple colors.

## Pattern

Same as `stone-wall.md` - uses the same braille pattern:

```
⠒⠂⠤⠀⠀⠂⠀⠀⠤⠀⠐⠚⠂⠒⢲⠒⠒⠒⠓⠒⠒⡖⠐⠒⠒⠀⠀⠠⠀⠀
⠒⠒⠒⠒⠒⡖⠒⠒⠓⠀⠀⣤⠄⠀⠘⠒⠒⢶⡖⠖⠖⠓⠒⠒⢶⠒⠒⠒⠒⠒
⠤⠤⡤⠤⠤⠷⠤⢤⡤⠤⠤⠼⠤⠤⢬⡤⠮⠴⠃⠁⠰⡤⠄⠐⠚⠂⠐⠦⠄⠀
⠤⠤⡧⡤⠤⡤⠠⠤⠧⠤⠤⢤⠤⠤⠼⠧⠤⠤⡦⠤⠤⠳⠰⠰⠤⠤⠤⡐⠂⠀
⣀⣠⣄⣱⣀⣇⣀⣀⣀⣀⣀⣼⣀⣀⣀⣠⠀⢠⡇⠀⠠⣤⠢⠤⠬⠤⠤⢧⡄⠀
⣀⣀⣅⣀⣀⣀⣐⣀⣋⣀⣀⣀⣀⣀⣀⣇⣀⣀⣰⣀⣀⣷⣄⣀⣀⣀⠤⢼⠧⠤
⠀⠀⢀⢀⢀⣏⠀⠀⠀⠀⢀⢸⢀⡀⢀⣄⠀⣀⣏⣰⣈⣀⡀⠀⣸⣀⠀⠀⡀⠀
⠀⠘⡏⠁⠀⠀⠀⠀⢹⠉⠀⠀⠙⠈⢙⡧⠀⢀⡀⠀⠀⢸⢀⣀⣀⣀⣀⣀⣁⠀
⠀⠈⠉⠉⠉⡟⠁⠋⠙⠁⠉⢻⠙⠉⠛⠛⠉⠩⡏⠉⠉⠉⠭⠉⠉⡇⠀⠀⠀⠀
⠀⠐⡖⠒⠒⠛⠒⠒⢲⠒⠒⠚⠒⠒⠒⠶⠒⠒⠛⠂⠂⢰⠂⠚⠙⠓⠂⠐⡖⠐
```

- **Dimensions**: 30 chars wide x 10 rows
- **Characters**: Dense braille (same as stone wall)
- **Usage**: Tiling background for Alchemist

## Colors Used

From `src/ui/theme.rs` (mystic/alchemy colors):
- `DEEP_VIOLET` - Rgb(48, 25, 70) - darkest shade
- `DARK_PURPLE` - Rgb(75, 35, 100) - medium shade
- `MYSTIC_PURPLE` - Rgb(120, 60, 150) - lightest shade

## Implementation

**File**: `src/ui/components/alchemist/lab_stone_art.rs`

Key functions:
- `render_lab_stone(frame, area)` - Renders tiled pattern filling entire area
- `generate_lab_line(row, width)` - Generates single line with color variation

## Design Notes

- Reuses the stone wall structure but with purple tint
- Creates dark, mystical laboratory atmosphere
- Same tiling and depth variation as blacksmith
- Darker overall to evoke mysterious alchemist's lair
