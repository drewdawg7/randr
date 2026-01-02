# UI Backgrounds

## Overview

Backgrounds can be added to screens by rendering them first, then layering UI components on top. Ratatui renders widgets in order, so later widgets appear on top of earlier ones.

## Implementation Pattern

### Tab-Level Background Rendering

Backgrounds should be rendered at the **tab level** (e.g., `BlacksmithTab`), not in individual screen modules like `menu.rs`. This keeps screen modules reusable and clean.

**Example from `src/ui/components/blacksmith/tab.rs`:**
```rust
fn view(&mut self, frame: &mut Frame, area: Rect) {
    match self.state {
        BlacksmithState::Menu => {
            // Render background first
            super::stone_wall_art::render_stone_wall(frame, area);
            // Then render UI components on top
            menu::render(frame, area, &mut self.menu_list_state);
        }
        // ... other states
    }
}
```

### Tiling Backgrounds

For patterns that should fill any screen size, use a tiling approach:

1. Define the base pattern as a constant
2. Generate lines dynamically based on area dimensions
3. Use modulo to tile the pattern

**Example from `stone_wall_art.rs`:**
```rust
const WALL_PATTERN: &[&str] = &[
    "⠒⠂⠤⠀⠀⠂⠀⠀⠤⠀⠐⠚⠂⠒⢲⠒⠒⠒⠓⠒⠒⡖⠐⠒⠒⠀⠀⠠⠀⠀",
    // ... more rows
];

fn generate_wall_line(row_in_pattern: usize, width: usize) -> Line<'static> {
    let pattern_row = WALL_PATTERN[row_in_pattern % PATTERN_HEIGHT];
    let pattern_chars: Vec<char> = pattern_row.chars().collect();

    for col in 0..width {
        let pattern_col = col % PATTERN_WIDTH;
        let ch = pattern_chars.get(pattern_col).copied().unwrap_or(' ');
        // ... build spans with color variation
    }
}
```

## Critical: Explicit Foreground Colors

When layering UI over a background, **all text must have explicit foreground colors**. Text using `Span::raw()` or default styles will appear incorrectly.

**Wrong:**
```rust
Span::raw(format!("{}", gold))  // Will inherit background colors
```

**Correct:**
```rust
let text_style = Style::default().fg(colors::WHITE);
Span::styled(format!("{}", gold), text_style)
```

### Files with explicit colors for backgrounds:
- `src/ui/components/blacksmith/menu.rs` - Menu item text
- `src/ui/components/utilities.rs` - `blacksmith_header()`, `store_header()` functions
- `src/ui/components/store/menu.rs` - Menu item text
- `src/ui/components/alchemist/menu.rs` - Menu item text, header
- `src/ui/components/field/menu.rs` - Menu item text, header

## Color Variation for Depth

Use multiple shades to give backgrounds depth:

```rust
let dark = Style::default().fg(colors::DARK_STONE);
let mid = Style::default().fg(colors::GRANITE);
let light = Style::default().fg(colors::LIGHT_STONE);

// Vary based on position
let style = match ((col / 5) + (row / 2)) % 3 {
    0 => dark,
    1 => mid,
    _ => light,
};
```

## File Locations

- Background art modules: `src/ui/components/<location>/<name>_art.rs`
- Theme colors: `src/ui/theme.rs`
- Tab rendering: `src/ui/components/<location>/tab.rs`

## Implemented Backgrounds

| Location | File | Pattern | Colors |
|----------|------|---------|--------|
| Blacksmith | `stone_wall_art.rs` | Stone wall (braille) | DARK_STONE, GRANITE, LIGHT_STONE |
| Store | `wood_planks_art.rs` | Wood grain (dense braille) | DARK_WALNUT, WOOD_BROWN, OAK_BROWN |
| Field | `grass_art.rs` | Grass/meadow (braille) | DARK_FOREST, FOREST_GREEN, LIME_GREEN |
| Alchemist | `lab_stone_art.rs` | Dark lab stone (braille) | DEEP_VIOLET, DARK_PURPLE, MYSTIC_PURPLE |

All patterns use 30x10 character tiles with 3-color depth variation.
