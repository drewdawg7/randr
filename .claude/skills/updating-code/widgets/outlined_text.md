# OutlinedText

Renders text with a black outline effect using layered shadow text nodes.

**File:** `src/ui/widgets/outlined_text.rs`

## Usage

```rust
use crate::ui::widgets::OutlinedText;

parent.spawn(
    OutlinedText::new("Item Name")
        .with_font_size(16.0)
        .with_color(item.quality.color()),
);
```

## Builder Methods

| Method | Description |
|--------|-------------|
| `new(text)` | Create with text content |
| `with_font_size(f32)` | Font size (default: 16.0) |
| `with_color(Color)` | Main text color (default: WHITE) |
| `with_outline(Color)` | Outline color (default: BLACK) |
| `with_outline_offset(f32)` | Pixel offset for shadows (default: 1.0) |

## Implementation

Uses observer pattern like other widgets. On add:
1. Spawns a relative container node
2. Creates 4 shadow text nodes offset in cardinal directions (up/down/left/right)
3. Creates main text node on top
4. Removes the `OutlinedText` component

## Where Used

Item names in detail panels:
- `src/ui/screens/inventory_modal/render.rs`
- `src/ui/screens/merchant_modal/render.rs`
- `src/ui/screens/forge_modal/render.rs`
- `src/ui/screens/anvil_modal/render.rs` (recipe name + item name)
