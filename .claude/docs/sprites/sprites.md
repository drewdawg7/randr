# Sprite Sheets and GameSprites

## Overview

The `SpriteSheet` struct provides methods for working with sprite atlases, including spawning UI sprites with sizing and animation.

## Key Files

| File | Purpose |
|------|---------|
| `src/assets/sprites.rs` | `SpriteSheet`, `GameSprites`, `SpriteSheetKey` |
| `src/ui/animation.rs` | `AnimationConfig`, `SpriteAnimation` |

## SpriteSheet Methods

### Basic Lookups

```rust
// Get sprite index by name
sheet.get("heart") -> Option<usize>

// Check if sprite exists
sheet.contains("heart") -> bool

// List all sprite names
sheet.names() -> Iterator<&str>
```

### World Sprites (2D)

```rust
// Basic sprite
sheet.sprite("heart") -> Option<Sprite>

// Sprite with custom size
sheet.sprite_sized("heart", Vec2::new(32.0, 32.0)) -> Option<Sprite>
```

### UI Sprites (ImageNode)

```rust
// Basic ImageNode (no sizing)
sheet.image_node("heart") -> Option<ImageNode>

// Nine-slice ImageNode
sheet.image_node_sliced("panel_bg", 8.0) -> Option<ImageNode>

// ImageNode + Node bundle with sizing
sheet.image_bundle("heart", 32.0, 32.0) -> Option<impl Bundle>
```

## Common Patterns

### Spawning Sized UI Sprites

**Before (manual assembly):**
```rust
let index = sheet.get("heart")?;
cell.spawn((
    ImageNode::from_atlas_image(texture.clone(), TextureAtlas { layout: layout.clone(), index }),
    Node { width: Val::Px(32.0), height: Val::Px(32.0), ..default() },
));
```

**After (using image_bundle):**
```rust
cell.spawn(sheet.image_bundle("heart", 32.0, 32.0)?);
```

## GameSprites Resource

Access sprite sheets by key:

```rust
fn my_system(sprites: Res<GameSprites>) {
    let ui_sheet = sprites.get(SpriteSheetKey::UiIcons)?;
    let item_sheet = sprites.get(SpriteSheetKey::IconItems)?;
}
```

## AnimationConfig

```rust
pub struct AnimationConfig {
    pub first_frame: usize,  // First frame index (default: 0)
    pub last_frame: usize,   // Last frame index inclusive (default: 3)
    pub frame_duration: f32, // Seconds per frame (default: 0.15)
}
```

## When to Use Each Method

| Method | Use When |
|--------|----------|
| `image_node()` | Need ImageNode without sizing (parent controls size) |
| `image_node_sliced()` | Nine-slice backgrounds/panels |
| `image_bundle()` | UI sprites with fixed size, no animation |
| `sprite()` | 2D world sprites |
| `sprite_sized()` | 2D world sprites with custom size |

## Shared Image Sprite Sheets

Multiple sprite sheet JSONs can reference the same image file via `meta.image`. This is useful for creating animation subsets from a larger tileset without duplicating the PNG:

```json
{
  "frames": {},
  "meta": {
    "image": "sprites/dungeon_tileset.png",
    "size": { "w": 160, "h": 160 },
    "slices": [
      { "name": "frame_1", "keys": [{ "frame": 0, "bounds": {"x": 0, "y": 128, "w": 16, "h": 16 } }] },
      { "name": "frame_2", "keys": [{ "frame": 0, "bounds": {"x": 16, "y": 128, "w": 16, "h": 16 } }] }
    ]
  }
}
```

This gives contiguous atlas indices (0, 1, ...) for the subset, making `SpriteAnimation` work correctly even when the source frames aren't contiguous in the original tileset.

**Used by:** `SpriteSheetKey::TorchWall` (3-frame torch animation from `dungeon_tileset.png`).
