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

---

# spawn_outlined_quantity_text

Spawns quantity text with an 8-direction outline effect at the bottom-right corner of a parent node.

## Usage

```rust
use crate::ui::widgets::{spawn_outlined_quantity_text, OutlinedQuantityConfig};

// Define a marker component for querying later
#[derive(Component)]
struct MyQuantityMarker;

// Inside a ChildBuilder context:
spawn_outlined_quantity_text(
    parent,
    &game_fonts,
    item.quantity,
    OutlinedQuantityConfig::default(),
    MyQuantityMarker,
);
```

## OutlinedQuantityConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `font_size` | `f32` | 14.0 | Font size for the quantity text |
| `text_color` | `Color` | WHITE | Main text color |
| `outline_color` | `Color` | BLACK | Outline/shadow color |
| `right` | `f32` | 2.0 | Position from right edge in pixels |
| `bottom` | `f32` | 0.0 | Position from bottom edge in pixels |

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `parent` | `&mut ChildBuilder` | Parent to spawn the text under |
| `game_fonts` | `&GameFonts` | Font resource for pixel font |
| `quantity` | `u32` | The quantity number to display |
| `config` | `OutlinedQuantityConfig` | Styling and position configuration |
| `marker` | `M: Bundle` | Marker component(s) for caller-specific queries |

## Implementation

1. Creates an absolute-positioned container node at bottom-right
2. Spawns 8 shadow text nodes offset in all directions (cardinal + diagonal) for a thick outline
3. Spawns the main text node on top
4. Attaches the marker component to the container for later querying/despawning

## Where Used

Item quantity display in grids:
- `src/ui/widgets/item_grid.rs` (ItemGrid widget)
- `src/ui/screens/forge_modal/render.rs` (ForgeModal ingredient quantities)
