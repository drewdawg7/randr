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

1. Open sprite sheet in Aseprite
2. Export: `File > Export Sprite Sheet`
   - Check "JSON Data"
   - Format: "Hash"
3. Save as `name.json` next to `name.png`

### 2. Place Files

```
assets/sprites/
├── my_sheet.png
└── my_sheet.json
```

### 3. Use in Code

```rust
fn spawn_icon(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = &game_sprites.ui_icons else { return };

    // Option 1: Get sprite component
    if let Some(sprite) = sheet.sprite("heart_full") {
        commands.spawn((sprite, Transform::from_xyz(100.0, 100.0, 0.0)));
    }

    // Option 2: Custom size (scale 16px to 32px)
    if let Some(sprite) = sheet.sprite_sized("heart_full", Vec2::splat(32.0)) {
        commands.spawn((sprite, Transform::from_xyz(150.0, 100.0, 0.0)));
    }

    // Option 3: Manual atlas control
    if let Some(index) = sheet.get("heart_full") {
        commands.spawn((
            Sprite::from_atlas_image(
                sheet.texture.clone(),
                TextureAtlas { layout: sheet.layout.clone(), index },
            ),
            Transform::from_xyz(200.0, 100.0, 0.0).with_scale(Vec3::splat(2.0)),
        ));
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
#[derive(Resource, Default)]
pub struct GameSprites {
    pub ui_icons: Option<SpriteSheet>,
    pub ui_buttons: Option<SpriteSheet>,
    pub book_ui: Option<SpriteSheet>,
    pub ui_frames: Option<SpriteSheet>,
    pub ui_bars: Option<SpriteSheet>,
    pub ui_all: Option<SpriteSheet>,  // Combined UI sprite sheet
}
```

## 9-Slice Scaling

Use 9-slice to stretch sprites without distorting corners (e.g., UI panels, buttons, frames).

### For UI Nodes (ImageNode)

```rust
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

// When creating the ImageNode, chain .with_mode()
let background = ui_all.get("Slice_8").map(|idx| {
    ImageNode::from_atlas_image(
        ui_all.texture.clone(),
        TextureAtlas { layout: ui_all.layout.clone(), index: idx },
    )
    .with_mode(NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect::square(8.0),  // 8px border on all sides
        ..default()
    }))
});

// Insert normally
commands.entity(entity).insert(background);
```

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

### Scale Modes

```rust
TextureSlicer {
    border: BorderRect::square(8.0),
    center_scale_mode: SliceScaleMode::Stretch,  // default - stretch center
    sides_scale_mode: SliceScaleMode::Stretch,   // default - stretch sides
    max_corner_scale: 1.0,                        // don't scale corners beyond 1x
}

// For tiling instead of stretching:
center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.0 }
```

### Choosing Border Size

The border value should match the corner size in your sprite:
- Look at the sprite in an image editor
- Measure the corner radius or decorative corner area
- Use that as your border value

For a 28x28 sprite with ~8px rounded corners, use `BorderRect::square(8.0)`.

## Adding New Sprite Sheets

### Step 1: Add Field

```rust
// In src/assets/sprites.rs
pub struct GameSprites {
    // ... existing ...
    pub my_new_sheet: Option<SpriteSheet>,
}
```

### Step 2: Load in load_assets

```rust
fn load_assets(...) {
    game_sprites.my_new_sheet =
        GameSprites::load_sheet("my_new_sheet", &asset_server, &mut texture_atlas_layouts);
}
```

### Step 3: Place Files

```
assets/sprites/
├── my_new_sheet.png
└── my_new_sheet.json
```

## Self-Verification Checklist

- [ ] JSON exported with Hash format (not Array)?
- [ ] Both .png and .json in assets/sprites/?
- [ ] Field added to GameSprites?
- [ ] load_assets updated?
- [ ] Frame names match exactly (case-sensitive)?

## Advanced Topics

For detailed patterns and workflows, see:

- [patterns.md](references/patterns.md) - Marker+system pattern for UI widgets, animation, stateful buttons
- [aseprite.md](references/aseprite.md) - Grid-aligned vs irregular sprites, JSON formats, finding slice dimensions
- [troubleshooting.md](references/troubleshooting.md) - Blurry sprites, parse errors, loading issues
- [fonts.md](references/fonts.md) - Custom fonts, GameFonts resource, pixel font rendering
