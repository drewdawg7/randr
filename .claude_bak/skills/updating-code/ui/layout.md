# UI Layout Patterns

## Centering Content

Use `Layout` with `Constraint::Fill` to center content both vertically and horizontally. The Fill constraint distributes remaining space proportionally.

### Centered Menu Pattern

**Example from `src/ui/components/blacksmith/menu.rs`:**
```rust
use ratatui::layout::{Constraint, Direction, Layout};

const MENU_HEIGHT: u16 = 5;
const MENU_WIDTH: u16 = 28;

// Vertical centering with upward offset (2:3 ratio)
let vertical_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Fill(2),           // Top space (smaller)
        Constraint::Length(MENU_HEIGHT),
        Constraint::Fill(3),           // Bottom space (larger)
    ])
    .split(content_area);

// Horizontal centering (1:1 ratio = exact center)
let horizontal_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Fill(1),
        Constraint::Length(MENU_WIDTH),
        Constraint::Fill(1),
    ])
    .split(vertical_chunks[1]);

let centered_area = horizontal_chunks[1];
```

### Fill Ratio Guide

| Ratio (top:bottom) | Effect |
|--------------------|--------|
| 1:1 | Exact center |
| 2:3 | Shifted up ~20% |
| 1:2 | Upper third |
| 1:3 | Near top |

### When to Use

- Simple menus that should float in available space
- Content that needs to remain centered on window resize
- Modal-like content within a larger area

### Files Using This Pattern

| Location | File | MENU_HEIGHT | MENU_WIDTH | Notes |
|----------|------|-------------|------------|-------|
| Blacksmith | `menu.rs` | 5 | 28 | 5 menu items |
| Store | `menu.rs` | 4 | 16 | 3 menu items |
| Field | `menu.rs` | 4 | 16 | 3 menu items |
| Alchemist | `menu.rs` | 3 | 20 | 2 menu items |

All use 2:3 vertical ratio for upward offset and 1:1 horizontal ratio for exact center.
