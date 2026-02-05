# Sprites

The `Sprite` component renders 2D images in Bevy. In Bevy 0.15+, `SpriteBundle` was removed - spawn `Sprite` directly and required components are auto-inserted.

## Quick Reference

```rust
// Basic sprite from image
commands.spawn(Sprite::from_image(asset_server.load("sprite.png")));

// Sprite from texture atlas
commands.spawn(Sprite::from_atlas_image(
    texture_handle,
    TextureAtlas { layout: layout_handle, index: 0 },
));

// Solid color sprite (no image needed)
commands.spawn(Sprite::from_color(Color::srgb(0.2, 0.7, 0.9), Vec2::new(100.0, 50.0)));

// With custom size
commands.spawn(Sprite::sized(Vec2::new(64.0, 64.0)));
```

## Sprite Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `image` | `Handle<Image>` | Required | The texture to render |
| `texture_atlas` | `Option<TextureAtlas>` | `None` | Optional atlas for sprite sheets |
| `color` | `Color` | `Color::WHITE` | Tint color (white = no tint) |
| `flip_x` | `bool` | `false` | Mirror horizontally |
| `flip_y` | `bool` | `false` | Mirror vertically |
| `custom_size` | `Option<Vec2>` | `None` | Override image dimensions |
| `rect` | `Option<Rect>` | `None` | Render only a region of image |
| `image_mode` | `SpriteImageMode` | `Auto` | How the image scales |

## Constructor Methods

```rust
// From image handle
Sprite::from_image(handle: Handle<Image>)

// From texture atlas
Sprite::from_atlas_image(image: Handle<Image>, atlas: TextureAtlas)

// Solid color rectangle
Sprite::from_color(color: impl Into<Color>, size: Vec2)

// With custom size
Sprite::sized(custom_size: Vec2)
```

## Required Components (Auto-Inserted)

When you spawn a `Sprite`, Bevy automatically adds:
- `Transform` - Position, rotation, scale
- `GlobalTransform` - World-space transform
- `Visibility` - Visibility state
- `Anchor` - Positioning anchor point

## Spawning Patterns

### Basic Sprite

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("player.png")));
}
```

### Sprite with Properties

```rust
commands.spawn((
    Sprite {
        image: asset_server.load("player.png"),
        color: Color::srgb(1.0, 0.5, 0.5), // Red tint
        flip_x: true,                       // Mirror horizontally
        custom_size: Some(Vec2::new(64.0, 64.0)),
        ..default()
    },
    Transform::from_xyz(100.0, 50.0, 0.0),
));
```

### Sprite from Texture Atlas

```rust
commands.spawn((
    Sprite::from_atlas_image(
        texture_handle,
        TextureAtlas {
            layout: atlas_layout_handle,
            index: 0, // Frame index
        },
    ),
    Transform::from_scale(Vec3::splat(4.0)), // Scale up
));
```

## Color and Tinting

### Color Creation

```rust
// RGB (0.0-1.0)
Color::srgb(1.0, 0.5, 0.0)          // Orange
Color::srgba(1.0, 0.0, 0.0, 0.5)    // Semi-transparent red

// RGB (0-255)
Color::srgb_u8(255, 128, 0)

// HSV
Color::hsv(180.0, 1.0, 1.0)         // Cyan

// Constants
Color::WHITE    // No tint
Color::BLACK
Color::NONE     // Fully transparent
```

### CSS Color Palette

```rust
use bevy::color::palettes::css;

Sprite {
    color: css::GOLD.into(),
    ..default()
}

// Available: RED, BLUE, GREEN, GOLD, SILVER, CORAL, CRIMSON,
// DARK_BLUE, LIGHT_GREEN, SKY_BLUE, TOMATO, etc.
```

### HDR Colors for Bloom

Colors with values > 1.0 will glow when bloom is enabled:

```rust
// This sprite will glow
Sprite {
    color: Color::srgb(5.0, 2.0, 1.0), // Bright orange glow
    ..default()
}
```

## Anchor Points

The `Anchor` component controls where the sprite's origin is:

```rust
// Constants
Anchor::CENTER        // Default
Anchor::TOP_LEFT
Anchor::TOP_CENTER
Anchor::TOP_RIGHT
Anchor::CENTER_LEFT
Anchor::CENTER_RIGHT
Anchor::BOTTOM_LEFT
Anchor::BOTTOM_CENTER
Anchor::BOTTOM_RIGHT

// Custom anchor (Vec2 from -0.5 to 0.5)
Anchor(Vec2::new(-0.5, -0.5))  // Bottom-left
```

### Usage

```rust
commands.spawn((
    Sprite::from_image(texture),
    Anchor::BOTTOM_CENTER, // Position from bottom-center
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```

## SpriteImageMode

Controls how sprites scale to fit `custom_size`:

### Auto (Default)

```rust
// Uses natural size, stretches if custom_size set
SpriteImageMode::Auto
```

### Scale Mode

```rust
// Proportional scaling with alignment
SpriteImageMode::Scale(SpriteScalingMode::FillCenter)  // Fill, centered
SpriteImageMode::Scale(SpriteScalingMode::FitCenter)   // Fit inside, centered
SpriteImageMode::Scale(SpriteScalingMode::FillStart)   // Fill, top-left
SpriteImageMode::Scale(SpriteScalingMode::FitEnd)      // Fit, bottom-right
```

### 9-Slice Mode

For scalable UI panels with fixed borders:

```rust
use bevy::sprite::{TextureSlicer, BorderRect, SliceScaleMode};

Sprite {
    image: asset_server.load("panel.png"),
    custom_size: Some(Vec2::new(200.0, 150.0)),
    image_mode: SpriteImageMode::Sliced(TextureSlicer {
        border: BorderRect::all(16.0), // 16px border on all sides
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    }),
    ..default()
}
```

### Tiled Mode

```rust
SpriteImageMode::Tiled {
    tile_x: true,
    tile_y: true,
    stretch_value: 1.0,
}
```

## TextureSlicer (9-Slice)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `border` | `BorderRect` | - | Pixel insets defining 9 regions |
| `center_scale_mode` | `SliceScaleMode` | `Stretch` | How center scales |
| `sides_scale_mode` | `SliceScaleMode` | `Stretch` | How edges scale |
| `max_corner_scale` | `f32` | `1.0` | Max corner scaling |

### BorderRect Constructors

```rust
BorderRect::all(16.0)           // Same on all sides
BorderRect::axes(20.0, 15.0)    // Horizontal, vertical
BorderRect::ZERO                // No border
```

### SliceScaleMode

```rust
SliceScaleMode::Stretch         // Scale to fit
SliceScaleMode::Tile {          // Repeat pattern
    stretch_value: 0.5
}
```

## Partial Image Rendering

Render only a portion of an image:

```rust
Sprite {
    image: asset_server.load("tileset.png"),
    rect: Some(Rect::new(0.0, 0.0, 32.0, 32.0)), // Top-left 32x32
    ..default()
}
```

## This Codebase

### SpriteSheet Wrapper

From `src/assets/sprites.rs` - provides ergonomic sprite creation:

```rust
pub struct SpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub sprites: HashMap<String, usize>,
}

impl SpriteSheet {
    // World sprite
    pub fn sprite(&self, name: &str) -> Option<Sprite>

    // World sprite with custom size
    pub fn sprite_sized(&self, name: &str, size: Vec2) -> Option<Sprite>

    // UI sprite (ImageNode)
    pub fn image_node(&self, name: &str) -> Option<ImageNode>
}
```

### Usage Pattern

```rust
fn spawn_entity(game_sprites: Res<GameSprites>) {
    let sheet = game_sprites.get(SpriteSheetKey::Dungeon).unwrap();
    let sprite = sheet.sprite("torch").unwrap();
    commands.spawn((sprite, Transform::from_xyz(x, y, z)));
}
```

## Common Mistakes

### Forgetting Custom Size for Atlas Sprites

```rust
// Problem: atlas sprite renders at original atlas size
Sprite::from_atlas_image(texture, atlas)

// Solution: set custom_size for consistent sizing
let mut sprite = Sprite::from_atlas_image(texture, atlas);
sprite.custom_size = Some(Vec2::new(32.0, 32.0));
```

### Using SpriteBundle (Removed in 0.15)

```rust
// Old (doesn't compile)
commands.spawn(SpriteBundle { ... });

// New: spawn Sprite directly
commands.spawn(Sprite::from_image(texture));
```

### Pixel Art Blurring

```rust
// Add to app plugins for crisp pixel art
App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```
