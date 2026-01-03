# Parchment Texture Pattern

## Overview

A medium-density ASCII pattern used as a background for the player profile modal, creating a paper/parchment texture effect.

## Pattern

```
~:~*~:~~:*~:~*~~:~*~:~~:*~:~*~
:*:~:*::~*:~:*::*:~:*::~*:~:*:
*~~:*~~*:~~:*~~*~~:*~~*:~~:*~~
~:*~:~*~:*~~:*~:~*~:*~~:*~:~*~
:~~*:~~:*~::~~*:~~:*~::~~*:~~:
*:~*:*~*:~**:~*:*~*:~**:~*:*~*
~*:~~*:~:*~~*:~~*:~:*~~*:~~*:~
:~*::~*:*~::~*::~*:*~::~*::~*:
*~:~*~:~~:*~:~*~:~~:*~:~*~:~~:
~:~~:~*~:*~~:~~:~*~:*~~:~~:~*~
```

- **Dimensions**: 30 chars wide × 10 rows
- **Characters**: Medium density (`~`, `*`, `:`)
- **Usage**: Tiling background for profile modal

## Why Medium Density?

Per the ascii-art skill documentation:
- **Low density** (`, /, :, -, .`) - too sparse, barely visible
- **Medium density** (`@, %, $, #, ~, *`) - visible texture without overwhelming
- **High density** (`█, ▓, ▒`) - too solid, blocks content

Medium density creates visible texture while allowing text to remain readable on top.

## Colors Used

From `src/ui/theme.rs` (warm parchment tones):
```rust
let fiber_colors = [
    Color::Rgb(70, 60, 50),   // Dark fibers
    Color::Rgb(85, 72, 58),   // Medium fibers
    Color::Rgb(95, 80, 65),   // Light fibers
];
```

Background: `PARCHMENT_BG` - `Color::Rgb(58, 52, 46)`

## Implementation

**File**: `src/ui/components/player/profile_modal.rs`

Key approach:
- Render pattern directly to buffer (not via Paragraph widget)
- Vary fiber colors based on position for depth effect
- Content rendered on top skips spaces to preserve background texture

```rust
// Render directly to buffer
for row in 0..area.height {
    for col in 0..area.width {
        let ch = pattern_chars[col % PATTERN_WIDTH];
        cell.set_char(ch);
        cell.set_fg(fiber_colors[color_idx]);
        cell.set_bg(PARCHMENT_BG);
    }
}
```

## Usage

Used in ProfileModal for fantasy book aesthetic:
```rust
// In render():
self.render_parchment_background(frame, border_area);
self.render_ascii_border(frame, border_area);
self.render_content_to_buffer(frame, inner_area);
```

## Notes

- Pattern tiles seamlessly
- Text overlay uses direct buffer rendering, skipping unstyled spaces
- All text needs explicit foreground colors to be visible
