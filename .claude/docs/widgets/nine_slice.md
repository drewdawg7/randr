# spawn_nine_slice_panel

Generic helper for nine-slice panel backgrounds.

**File:** `src/ui/widgets/nine_slice.rs`

## Usage

```rust
use crate::ui::widgets::spawn_nine_slice_panel;
use crate::assets::ShopBgSlice;  // or DetailPanelSlice

parent.with_children(|parent| {
    spawn_nine_slice_panel::<ShopBgSlice>(parent, &game_sprites, width, height);
});
```

## NineSlice Trait

Types implementing `NineSlice` (in `src/assets/sprite_slices.rs`):

```rust
pub trait NineSlice: Copy {
    const ALL: [Self; 9];             // TL, TC, TR, ML, C, MR, BL, BC, BR
    const SLICE_SIZE: f32;            // Corner size
    const SHEET_KEY: SpriteSheetKey;
    fn as_str(self) -> &'static str;
}
```

## Implemented Types

| Type | Sheet Key | Slice Size |
|------|-----------|------------|
| `ShopBgSlice` | `ShopBgSlices` | 48.0 |
| `DetailPanelSlice` | `DetailPanelBg` | 48.0 |

## Adding New Nine-Slice Panels

1. Add slice enum to `src/assets/sprite_slices.rs` with 9 variants
2. Implement `as_str()` and `const ALL`
3. Implement `NineSlice` trait
4. Export from `src/assets/mod.rs`
