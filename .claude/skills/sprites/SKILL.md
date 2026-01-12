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
let Some(idx) = sheet.get("Slice_4891") else { return };
```

See [game-sprites.md](references/game-sprites.md) for full details on adding new sprite sheets.

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

1. Place `my_sprite.png` and `my_sprite.json` in `assets/sprites/`
2. Add variant to `SpriteSheetKey` enum in `src/assets/sprites.rs`
3. Add to `SpriteSheetKey::all()` array
4. Add to `SpriteSheetKey::asset_name()` match

**Important**: Each new sprite should be its own sprite sheet. Don't modify existing sheets.

See [game-sprites.md](references/game-sprites.md) for detailed examples and the system pattern for spawn functions.

## Advanced Topics

For detailed patterns and workflows, see:

- [game-sprites.md](references/game-sprites.md) - **GameSprites resource, SpriteSheetKey, adding new sprites, system patterns**
- [patterns.md](references/patterns.md) - Marker+system pattern for UI widgets, animation, stateful buttons
- [aseprite.md](references/aseprite.md) - Grid-aligned vs irregular sprites, JSON formats, finding slice dimensions
- [troubleshooting.md](references/troubleshooting.md) - Blurry sprites, parse errors, loading issues
- [fonts.md](references/fonts.md) - Custom fonts, GameFonts resource, pixel font rendering
