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

// ImageNode + Node + SpriteAnimation bundle
sheet.image_bundle_animated("chest", 64.0, 64.0, AnimationConfig::default()) -> Option<impl Bundle>
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

### Spawning Animated UI Sprites

```rust
// With default animation (frames 0-3, 0.15s per frame)
cell.spawn(sheet.image_bundle_animated("chest", 64.0, 64.0, AnimationConfig::default())?);

// With custom animation
let config = AnimationConfig {
    first_frame: 0,
    last_frame: 7,
    frame_duration: 0.1,
};
cell.spawn(sheet.image_bundle_animated("explosion", 128.0, 128.0, config)?);
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
| `image_bundle_animated()` | Animated UI sprites with fixed size |
| `sprite()` | 2D world sprites |
| `sprite_sized()` | 2D world sprites with custom size |
