---
name: sprites
description: Bevy sprite sheets and fonts. Use when adding sprites, spawning text, loading custom fonts, finding sprite dimensions, or creating sprite sheets.
---

# Sprites

## Finding Sprites

Look up sprite dimensions:

```bash
python3 .claude/skills/sprites/scripts/find_sprite.py ui_all Slice_3353
```

Output: `Slice_3353: 92x26 at (202, 1280)`

> **Important**: Always use this script to look up sprite dimensions. The user provides the exact sprite name - trust it and use the script output directly. Do not manually search JSON files.

## Quick Start

### 1. Export from Aseprite

**GUI:** `File > Export Sprite Sheet` with "JSON Data" checked, Format: "Hash"

**CLI:** Extract a slice directly:
```bash
ASEPRITE="/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite"
"$ASEPRITE" --batch input.aseprite --slice "SpriteName" --save-as output.png
```

See [aseprite.md](references/aseprite.md) for full CLI options.

**Convert slice-based sheets to tagged frames:**
```bash
"$ASEPRITE" -b input.aseprite --script tools/aseprite_add_tags.lua
```

Reads existing slices from a sprite sheet, groups by row, detects empty cells to find animation boundaries, and outputs a multi-frame file with tagged animations (`a_1`, `a_2`, etc.). Assumes 32x32 frame size. Tags must use short names (max ~4 chars) to display correctly in Aseprite.

### 2. Place Files

```
assets/sprites/
├── my_sheet.png
└── my_sheet.json
```

### 3. Use in Code

```rust
// Spawn functions should be SYSTEMS, not helper functions
fn spawn_icon(mut commands: Commands, game_sprites: Res<GameSprites>) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiIcons) else { return };

    // For UI: ImageNode with atlas
    if let Some(idx) = sheet.get("heart_full") {
        commands.spawn((
            ImageNode::from_atlas_image(
                sheet.texture.clone(),
                TextureAtlas { layout: sheet.layout.clone(), index: idx },
            ),
            Node { width: Val::Px(32.0), height: Val::Px(32.0), ..default() },
        ));
    }

    // For world sprites: Sprite component
    if let Some(sprite) = sheet.sprite("heart_full") {
        commands.spawn((sprite, Transform::from_xyz(100.0, 100.0, 0.0)));
    }
}
```

## API Reference

### SpriteSheet

```rust
impl SpriteSheet {
    fn get(&self, name: &str) -> Option<usize>;           // Get sprite index
    fn contains(&self, name: &str) -> bool;               // Check existence
    fn names(&self) -> impl Iterator<Item = &str>;        // All sprite names
    fn sprite(&self, name: &str) -> Option<Sprite>;       // Create Sprite component
    fn sprite_sized(&self, name: &str, size: Vec2) -> Option<Sprite>;  // With custom size
}
```

### GameSprites Resource

```rust
// Access via SpriteSheetKey enum - NOT direct fields
let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else { return };
let Some(idx) = sheet.get(UiAllSlice::Book.as_str()) else { return };
```

See [game-sprites.md](references/game-sprites.md) for full details on adding new sprite sheets.

## Typed Sprite Slices

Use typed enums instead of magic strings for sprite slice names. This provides compile-time safety and semantic naming.

### Available Enums

| Enum | SpriteSheetKey | Common Slices |
|------|---------------|---------------|
| `UiAllSlice` | `UiAll` | CellBackground, HeartIcon, GoldIcon, TitleBanner, InfoPanelBg, Book, Button* |
| `UiSelectorsSlice` | `UiSelectors` | SelectorFrame1, SelectorFrame2 |
| `HealthBarSlice` | `UiAll` | Health0 through Health100 |
| `TravelBookSlice` | `TravelBook` | Banner |
| `BookSlotSlice` | `BookSlot` | Slot |

### Usage

```rust
use crate::assets::{UiAllSlice, UiSelectorsSlice, HealthBarSlice};

// Instead of magic strings:
let cell = sheet.image_node("Slice_10");  // BAD

// Use typed enums:
let cell = sheet.image_node(UiAllSlice::CellBackground.as_str());  // GOOD

// Health bar with helper method:
let slice = HealthBarSlice::for_percent(75.0);  // Returns Health80
let img = sheet.image_node(slice.as_str());
```

### Adding New Slices

Edit `src/assets/sprite_slices.rs`:
1. Add variant to the appropriate enum
2. Add mapping in `as_str()` method

See [sprite-slices.md](references/sprite-slices.md) for full documentation.

## 9-Slice Scaling

Use 9-slice to stretch sprites without distorting corners (e.g., UI panels, buttons, frames).

### Manual Nine-Slice (Recommended)

Bevy's built-in `ImageScaleMode::Sliced` has issues with atlas textures. Use a **manual 3x3 grid layout approach** instead:

**1. Export from Aseprite as 9 separate frames:**
```bash
"$ASEPRITE" --batch input.aseprite \
  --sheet assets/sprites/my_bg_slices.png \
  --sheet-type horizontal \
  --data assets/sprites/my_bg_slices.json \
  --format json-hash
```

The frames should be ordered: top-left, top-center, top-right, middle-left, center, middle-right, bottom-left, bottom-center, bottom-right.

**2. Fix the JSON `image` path** (critical!):

Aseprite exports `"image": "my_bg_slices.png"` but the asset loader expects `"image": "sprites/my_bg_slices.png"`. Edit the JSON:
```json
"meta": {
  "image": "sprites/my_bg_slices.png",  // Add sprites/ prefix!
  ...
}
```

**3. Create a slice enum in `src/assets/sprite_slices.rs`:**
```rust
pub enum MyBgSlice {
    TopLeft, TopCenter, TopRight,
    MiddleLeft, Center, MiddleRight,
    BottomLeft, BottomCenter, BottomRight,
}

impl MyBgSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopLeft => "frame_0.aseprite",
            // ... map to Aseprite frame names
        }
    }

    pub const ALL: [Self; 9] = [
        Self::TopLeft, Self::TopCenter, Self::TopRight,
        Self::MiddleLeft, Self::Center, Self::MiddleRight,
        Self::BottomLeft, Self::BottomCenter, Self::BottomRight,
    ];
}
```

**4. Spawn as a 3x3 grid layout:**
```rust
const SLICE_SIZE: f32 = 48.0;  // Corner/edge size

fn spawn_nine_slice_background(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    width: f32,
    height: f32,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::MyBgSlices) else { return };

    let stretch_width = width - (SLICE_SIZE * 2.0);
    let stretch_height = height - (SLICE_SIZE * 2.0);

    parent.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        top: Val::Px(0.0),
        width: Val::Px(width),
        height: Val::Px(height),
        display: Display::Grid,
        grid_template_columns: vec![
            GridTrack::px(SLICE_SIZE),
            GridTrack::px(stretch_width),
            GridTrack::px(SLICE_SIZE),
        ],
        grid_template_rows: vec![
            GridTrack::px(SLICE_SIZE),
            GridTrack::px(stretch_height),
            GridTrack::px(SLICE_SIZE),
        ],
        ..default()
    }).with_children(|grid| {
        for slice in MyBgSlice::ALL {
            let (w, h) = match slice {
                // Corners: fixed size
                MyBgSlice::TopLeft | MyBgSlice::TopRight
                | MyBgSlice::BottomLeft | MyBgSlice::BottomRight => (SLICE_SIZE, SLICE_SIZE),
                // Top/bottom edges: stretch horizontal
                MyBgSlice::TopCenter | MyBgSlice::BottomCenter => (stretch_width, SLICE_SIZE),
                // Left/right edges: stretch vertical
                MyBgSlice::MiddleLeft | MyBgSlice::MiddleRight => (SLICE_SIZE, stretch_height),
                // Center: stretch both
                MyBgSlice::Center => (stretch_width, stretch_height),
            };

            let mut cell = grid.spawn(Node {
                width: Val::Px(w),
                height: Val::Px(h),
                ..default()
            });
            if let Some(img) = sheet.image_node(slice.as_str()) {
                cell.insert(img);
            }
        }
    });
}
```

**5. Size the container appropriately:**
- Calculate grid content size (cells × cell_size + gaps × gap_size)
- Add padding for the border decorations
- Example: 4×4 grid of 48px cells with 4px gaps = 204px content, use ~320px container for nice border

### Built-in Sliced Mode (Simple Cases Only)

For non-atlas images or simple cases, Bevy's built-in slicer may work:

```rust
use bevy::ui::widget::NodeImageMode;

ImageNode::new(texture_handle)
    .with_mode(NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect::square(8.0),
        ..default()
    }))
```

> **Warning**: This does NOT work reliably with `TextureAtlas` images. Use manual nine-slice above.

### Border Configuration

```rust
// Same border on all sides
border: BorderRect::square(8.0)

// Different horizontal/vertical
border: BorderRect::axes(10.0, 8.0)  // (horizontal, vertical)

// Each side different
border: BorderRect {
    left: 10.0,
    right: 10.0,
    top: 8.0,
    bottom: 8.0,
}
```

## Adding New Sprite Sheets

1. Place `my_sprite.png` and `my_sprite.json` in `assets/sprites/`
2. Add variant to `SpriteSheetKey` enum in `src/assets/sprites.rs`
3. Add to `SpriteSheetKey::all()` array
4. Add to `SpriteSheetKey::asset_name()` match

**Important**: Each new sprite should be its own sprite sheet. Don't modify existing sheets.

See [game-sprites.md](references/game-sprites.md) for detailed examples and the system pattern for spawn functions.

## Advanced Topics

For detailed patterns and workflows, see:

- [game-sprites.md](references/game-sprites.md) - **GameSprites resource, SpriteSheetKey, adding new sprites, system patterns**
- [sprite-slices.md](references/sprite-slices.md) - **Typed slice enums (UiAllSlice, HealthBarSlice, etc.), semantic naming, adding new slices**
- [patterns.md](references/patterns.md) - Marker+system pattern for UI widgets, animation, stateful buttons
- [aseprite.md](references/aseprite.md) - Grid-aligned vs irregular sprites, JSON formats, finding slice dimensions
- [troubleshooting.md](references/troubleshooting.md) - Blurry sprites, parse errors, loading issues
- [fonts.md](references/fonts.md) - Custom fonts, GameFonts resource, pixel font rendering
