# Stone Wall Pattern

## Overview

A braille-based stone wall pattern used as a tiling background for the blacksmith menu screen.

## Pattern

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

- **Dimensions**: 30 chars wide × 10 rows
- **Characters**: Braille unicode (U+2800 block)
- **Usage**: Tiling background

## Colors Used

From `src/ui/theme.rs`:
- `DARK_STONE` - Rgb(50, 50, 55) - darkest shade
- `GRANITE` - Rgb(80, 80, 85) - medium shade
- `LIGHT_STONE` - Rgb(120, 120, 125) - lightest shade

Colors vary based on position to create depth effect:
```rust
let style = match ((col / 5) + (row / 2)) % 3 {
    0 => dark,
    1 => mid,
    _ => light,
};
```

## Implementation

**File**: `src/ui/components/blacksmith/stone_wall_art.rs`

Key functions:
- `render_stone_wall(frame, area)` - Renders tiled wall filling entire area
- `generate_wall_line(row, width)` - Generates single line with color variation

## Usage

Rendered at tab level before other components:
```rust
// In BlacksmithTab::view()
BlacksmithState::Menu => {
    super::stone_wall_art::render_stone_wall(frame, area);
    menu::render(frame, area, &mut self.menu_list_state);
}
```

## Notes

- Pattern tiles seamlessly horizontally and vertically
- Density remains consistent regardless of terminal size
- UI components layered on top need **explicit foreground colors** (see `ui/backgrounds.md`)
