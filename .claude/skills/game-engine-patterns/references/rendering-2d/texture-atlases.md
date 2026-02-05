# Texture Atlases

Texture atlases (sprite sheets) combine multiple sprites into a single image for efficient rendering. Bevy uses `TextureAtlasLayout` to define regions and `TextureAtlas` to select which region to display.

## Quick Reference

```rust
// Create layout from grid
let layout = TextureAtlasLayout::from_grid(
    UVec2::new(32, 32),  // Tile size
    8,                    // Columns
    4,                    // Rows
    None,                 // Padding
    None,                 // Offset
);
let layout_handle = layouts.add(layout);

// Spawn sprite from atlas
commands.spawn(Sprite::from_atlas_image(
    texture_handle,
    TextureAtlas {
        layout: layout_handle,
        index: 0,
    },
));
```

## TextureAtlasLayout

Defines where each sprite region is located within a texture.

### Fields

- `size: UVec2` - Total texture dimensions
- `textures: Vec<URect>` - Individual sprite rectangles

### Creating from Grid

Most common method for uniform sprite sheets:

```rust
TextureAtlasLayout::from_grid(
    tile_size: UVec2,       // Size of each sprite cell
    columns: u32,           // Number of columns
    rows: u32,              // Number of rows
    padding: Option<UVec2>, // Space between cells
    offset: Option<UVec2>,  // Offset from top-left
)
```

**Index calculation:** `index = row * columns + column` (0-indexed, left-to-right, top-to-bottom)

### Examples

```rust
// 8x4 grid of 32x32 sprites (32 total sprites)
let layout = TextureAtlasLayout::from_grid(
    UVec2::splat(32),
    8, 4,
    None, None,
);

// With 2px padding between sprites
let layout = TextureAtlasLayout::from_grid(
    UVec2::new(16, 16),
    10, 5,
    Some(UVec2::splat(2)),
    None,
);

// With 4px offset from top-left
let layout = TextureAtlasLayout::from_grid(
    UVec2::new(24, 24),
    6, 6,
    None,
    Some(UVec2::splat(4)),
);
```

### Creating Manually (Irregular Regions)

For sprite sheets with non-uniform sprite sizes:

```rust
// Create empty layout with texture dimensions
let mut layout = TextureAtlasLayout::new_empty(UVec2::new(256, 256));

// Add regions manually, returns index
let small_icon = layout.add_texture(URect::new(0, 0, 16, 16));
let large_sprite = layout.add_texture(URect::new(16, 0, 80, 64));
let button = layout.add_texture(URect::new(0, 64, 48, 80));

// Use indices when spawning
commands.spawn(Sprite::from_atlas_image(
    texture,
    TextureAtlas { layout: layout_handle, index: large_sprite },
));
```

## TextureAtlas Component

References a layout and specifies which sprite to display.

### Fields

- `layout: Handle<TextureAtlasLayout>` - Reference to layout asset
- `index: usize` - Current sprite index

### Methods

```rust
// Create with different index
atlas.with_index(5)

// Get current sprite rectangle
atlas.texture_rect() -> Option<URect>
```

## Complete Setup Example

```rust
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load sprite sheet image
    let texture: Handle<Image> = asset_server.load("sprites/characters.png");

    // Create layout: 8 columns, 4 rows, 32x32 sprites
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        8, 4,
        None, None,
    );
    let layout_handle = layouts.add(layout);

    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn sprite at index 0
    commands.spawn((
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn another at index 5
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: layout_handle,
                index: 5,
            },
        ),
        Transform::from_xyz(50.0, 0.0, 0.0),
    ));
}
```

## Changing Sprite Index at Runtime

```rust
fn change_frame(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Sprite>,
) {
    for mut sprite in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if keyboard.just_pressed(KeyCode::ArrowRight) {
                atlas.index = (atlas.index + 1) % 8; // Cycle 0-7
            }
            if keyboard.just_pressed(KeyCode::ArrowLeft) {
                atlas.index = atlas.index.saturating_sub(1);
            }
        }
    }
}
```

## TextureAtlasBuilder

For creating atlases from multiple individual images at runtime:

```rust
fn build_atlas(
    mut images: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut builder = TextureAtlasBuilder::default();

    builder
        .initial_size(UVec2::new(512, 512))
        .max_size(UVec2::new(2048, 2048))
        .padding(UVec2::splat(2)); // Prevents texture bleeding

    // Add images
    for (id, image) in images.iter() {
        builder.add_texture(Some(id), image);
    }

    // Build atlas
    let (layout, sources, atlas_image) = builder.build().unwrap();

    let atlas_texture = images.add(atlas_image);
    let layout_handle = layouts.add(layout);

    // sources.texture_index(&original_id) -> Option<usize>
}
```

## This Codebase

### SpriteSheetMeta JSON Loading

From `src/assets/sprites.rs` - loads atlas metadata from JSON:

```rust
pub fn to_layout(&self) -> (TextureAtlasLayout, HashMap<String, usize>) {
    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(
        self.meta.size.w,
        self.meta.size.h
    ));
    let mut name_to_index = HashMap::new();

    for (name, frame) in &self.frames {
        let rect = URect::new(
            frame.frame.x, frame.frame.y,
            frame.frame.x + frame.frame.w,
            frame.frame.y + frame.frame.h,
        );
        let index = layout.add_texture(rect);
        name_to_index.insert(name.clone(), index);
    }

    (layout, name_to_index)
}
```

### Name-Based Sprite Access

```rust
// Get sprite by name instead of index
let sprite = sprite_sheet.sprite("player_idle_1")?;
```

### MobSpriteSheet Pattern

From `src/ui/mob_animation.rs`:

```rust
pub struct MobSpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation: AnimationConfig,
    pub death_animation: Option<AnimationConfig>,
    pub frame_size: UVec2, // For aspect ratio
}
```

Stores frame size for proper aspect ratio when sprites are non-square.

## Best Practices

1. **Use `ImagePlugin::default_nearest()`** for pixel art

2. **Add padding** when building runtime atlases to prevent bleeding

3. **Store layout handles** if multiple entities share the same atlas

4. **Use `from_grid`** for uniform sheets; `add_texture` for irregular

5. **Track frame_size** separately for non-square sprites

6. **Prefer named lookups** over raw indices for maintainability

## Common Mistakes

### Wrong Index Calculation

```rust
// Indices are row-major: index = row * columns + column
// For 8-column grid:
// Row 0: indices 0-7
// Row 1: indices 8-15
// Row 2: indices 16-23
```

### Forgetting to Add Layout to Assets

```rust
// Wrong: layout goes out of scope
let layout = TextureAtlasLayout::from_grid(...);
commands.spawn(Sprite::from_atlas_image(texture, TextureAtlas {
    layout: ???, // No handle!
    index: 0,
}));

// Correct: add to Assets first
let layout_handle = layouts.add(layout);
commands.spawn(Sprite::from_atlas_image(texture, TextureAtlas {
    layout: layout_handle,
    index: 0,
}));
```

### Texture Bleeding

When sprites have visible edges from adjacent frames:

```rust
// Add padding when creating atlas
TextureAtlasLayout::from_grid(
    UVec2::splat(32),
    8, 4,
    Some(UVec2::splat(2)), // 2px padding
    None,
);
```
