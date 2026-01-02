# Campfire ASCII Art

## Location
`src/ui/components/dungeon/campfire_art.rs`

## Purpose
Animated campfire for dungeon rest rooms. Demonstrates animation timing and transparent background rendering.

## Dimensions
- Width: 27 characters
- Height: 10 rows

## Animation
- 4 frames at 150ms intervals
- Uses `OnceLock<Instant>` for global timing across re-renders
- Animates: sparks, flame tips, flame body, coal glow

```rust
const FRAME_DURATION_MS: u64 = 150;
const NUM_FRAMES: u64 = 4;

fn get_animation_frame() -> usize {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start = START.get_or_init(Instant::now);
    let elapsed = start.elapsed().as_millis() as u64;
    ((elapsed / FRAME_DURATION_MS) % NUM_FRAMES) as usize
}
```

## Structure (10 rows)
1. Sparks - scattered `*`, `.`, `'` characters
2. Flame tips - `~^~^~^~` pattern
3. Upper flames - orange with deep orange accents
4. Mid-upper flames - `@#@` pattern in red
5. Mid flames - larger flame body
6. Lower flames - widest part with `O` highlights
7. Coals - `░▓█▄` block characters with animated glow
8. Top logs - `▄` bars with `\` `/` ends
9. Bottom logs - `▀` bars crossed pattern
10. Stone circle - `oO.` characters

## Colors Used
| Variable | Color Constant | Purpose |
|----------|---------------|---------|
| `spark_style` | `HOT_WHITE` | Bright sparks |
| `tip_yellow` | `BRIGHT_YELLOW` | Flame tips |
| `tip_orange` | `FLAME_ORANGE` | Flame tips alternate |
| `flame_orange` | `FLAME_ORANGE` | Main flame body |
| `flame_deep` | `DEEP_ORANGE` | Flame depth |
| `flame_red` | `EMBER_RED` | Core flame |
| `coal_glow` | `EMBER_RED` | Glowing coals |
| `coal_dark` | `COAL_BLACK` | Dark coals |
| `wood_brown` | `WOOD_BROWN` | Log surface |
| `wood_dark` | `DARK_WALNUT` | Log ends |
| `stone` | `GRANITE` | Stone circle |
| `stone_dark` | `DARK_STONE` | Stone circle alt |

## Transparent Background Rendering
To show the stone wall background through the campfire, spaces are skipped when rendering:

```rust
// In render_rest_room()
let buf = frame.buffer_mut();
for (i, line) in campfire_lines.into_iter().enumerate() {
    let y = y_offset + i as u16;
    let mut x = campfire_x;
    for span in line.spans {
        for ch in span.content.chars() {
            if ch != ' ' {
                buf.set_string(x, y, ch.to_string(), span.style);
            }
            x += 1;
        }
    }
}
```

**Key insight:** Using `Paragraph::render()` would fill the entire area with the default background. Writing directly to the buffer and skipping spaces preserves whatever was rendered behind.

## Usage
```rust
use crate::ui::components::dungeon::campfire_art::{render_campfire_art, campfire_width};

let lines = render_campfire_art();
let width = campfire_width(); // 27
```
