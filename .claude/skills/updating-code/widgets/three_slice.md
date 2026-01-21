# spawn_three_slice_banner

Generic helper for horizontal 3-slice banners (left edge, stretchable center, right edge).

**File:** `src/ui/widgets/three_slice.rs`

## Usage

```rust
use crate::ui::widgets::spawn_three_slice_banner;
use crate::assets::FightBannerSlice;

parent.with_children(|parent| {
    spawn_three_slice_banner::<FightBannerSlice>(parent, &game_sprites, width);
});
```

## ThreeSlice Trait

Types implementing `ThreeSlice` (in `src/assets/sprite_slices.rs`):

```rust
pub trait ThreeSlice: Copy {
    const ALL: [Self; 3];             // Left, Center, Right
    const EDGE_WIDTH: f32;            // Width of left/right edges
    const HEIGHT: f32;                // Fixed height of banner
    const SHEET_KEY: SpriteSheetKey;
    fn as_str(self) -> &'static str;
}
```

## Implemented Types

| Type | Sheet Key | Edge Width | Height |
|------|-----------|------------|--------|
| `FightBannerSlice` | `FightBannerSlices` | 32.0 | 39.0 |

## Usage Example: Fight Modal

```rust
const BANNER_WIDTH: f32 = 160.0;

// In fight modal, banner above each sprite:
column.with_children(|column| {
    spawn_three_slice_banner::<FightBannerSlice>(column, &game_sprites, BANNER_WIDTH);
    // Sprite below
});
```

## Adding New Three-Slice Banners

1. Export Aseprite file with 3 slices (Left, Center, Right) - use `--list-slices` flag
2. Add slice enum to `src/assets/sprite_slices.rs` with 3 variants
3. Implement `as_str()` and `const ALL`
4. Implement `ThreeSlice` trait with `EDGE_WIDTH`, `HEIGHT`, and `SHEET_KEY`
5. Export from `src/assets/mod.rs`
6. Add `SpriteSheetKey` variant in `src/assets/sprites.rs`
