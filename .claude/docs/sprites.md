# Sprite System Documentation

This guide covers how to use 2D pixel art sprites in the game, including loading sprite sheets from Aseprite exports.

## Quick Start

### 1. Prepare Your Sprites in Aseprite

1. Open your sprite sheet PNG in Aseprite
2. Use `File > Import Sprite Sheet` for grid-aligned sprites
3. Name each frame/sprite meaningfully
4. Export: `File > Export Sprite Sheet`
   - Check "JSON Data"
   - Format: "Hash"
   - Save as `spritename.json` next to `spritename.png`

### 2. Place Files in Assets

```
assets/sprites/
├── ui_icons.png      # Your sprite sheet image
├── ui_icons.json     # Aseprite JSON export
├── ui_buttons.png
├── ui_buttons.json
└── ...
```

### 3. Use in Code

```rust
fn spawn_heart(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
) {
    if let Some(icons) = &game_sprites.ui_icons {
        // Option 1: Get the sprite component directly
        if let Some(sprite) = icons.sprite("heart_full") {
            commands.spawn((
                sprite,
                Transform::from_xyz(100.0, 100.0, 0.0),
            ));
        }

        // Option 2: With custom size (scale 16px sprite to 32px)
        if let Some(sprite) = icons.sprite_sized("heart_full", Vec2::splat(32.0)) {
            commands.spawn((
                sprite,
                Transform::from_xyz(150.0, 100.0, 0.0),
            ));
        }

        // Option 3: Manual control over TextureAtlas
        if let Some(index) = icons.get("heart_full") {
            commands.spawn((
                Sprite::from_atlas_image(
                    icons.texture.clone(),
                    TextureAtlas {
                        layout: icons.layout.clone(),
                        index,
                    },
                ),
                Transform::from_xyz(200.0, 100.0, 0.0)
                    .with_scale(Vec3::splat(2.0)),
            ));
        }
    }
}
```

---

## Concepts

### Texture Atlas

A texture atlas is Bevy's way of managing sprite sheets:
- `TextureAtlasLayout` - defines where sprites are in the sheet
- `TextureAtlas` - component that references the layout + current sprite index
- The game's `SpriteSheet` wraps these with named access

### Pixel-Perfect Rendering

The game uses `ImagePlugin::default_nearest()` to ensure pixel art isn't blurry when scaled. This uses nearest-neighbor filtering instead of bilinear interpolation.

---

## API Reference

### `SpriteSheet`

```rust
pub struct SpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub sprites: HashMap<String, usize>,
}

impl SpriteSheet {
    /// Get sprite index by name
    fn get(&self, name: &str) -> Option<usize>;

    /// Check if sprite exists
    fn contains(&self, name: &str) -> bool;

    /// Get all sprite names
    fn names(&self) -> impl Iterator<Item = &str>;

    /// Number of sprites
    fn len(&self) -> usize;

    /// Create Sprite component by name
    fn sprite(&self, name: &str) -> Option<Sprite>;

    /// Create Sprite with custom size
    fn sprite_sized(&self, name: &str, size: Vec2) -> Option<Sprite>;

    /// Load from Aseprite export
    fn load(name: &str, asset_server: &AssetServer, layouts: &mut Assets<TextureAtlasLayout>) -> Option<Self>;
}
```

### `GameSprites`

Resource containing all loaded sprite sheets:

```rust
#[derive(Resource, Default)]
pub struct GameSprites {
    // UI elements
    pub ui_icons: Option<SpriteSheet>,
    pub ui_buttons: Option<SpriteSheet>,
    pub book_ui: Option<SpriteSheet>,
    pub ui_frames: Option<SpriteSheet>,
    pub ui_bars: Option<SpriteSheet>,
    pub ui_sliders: Option<SpriteSheet>,
    pub ui_ribbons: Option<SpriteSheet>,

    // Game elements
    pub mine: Option<SpriteSheet>,
    pub fight: Option<SpriteSheet>,
    pub dungeon: Option<SpriteSheet>,
    pub characters: Option<SpriteSheet>,
    pub items: Option<SpriteSheet>,
}
```

---

## Aseprite Workflow

### For Grid-Aligned Sprites (e.g., 16x16 icons)

1. **Open** the PNG in Aseprite
2. **Import**: `File > Import Sprite Sheet`
   - Type: "By Cell Size" or "By Rows and Columns"
   - Cell size: 16x16 (or your grid size)
3. **Rename** frames in the timeline (double-click frame label)
4. **Export**: `File > Export Sprite Sheet`
   - Sheet Type: Keep original layout
   - Check: "JSON Data"
   - JSON Format: "Hash"

### For Irregular Sprites (e.g., mixed sizes)

1. **Open** the PNG in Aseprite
2. Use **Slices** (`Frame > Slices > New Slice`) to define regions
3. **Name** each slice descriptively
4. **Export** with "Slices" checked in the export dialog

### JSON Format Example

```json
{
  "frames": {
    "heart_full": {
      "frame": {"x": 0, "y": 0, "w": 16, "h": 16}
    },
    "heart_half": {
      "frame": {"x": 16, "y": 0, "w": 16, "h": 16}
    },
    "heart_empty": {
      "frame": {"x": 32, "y": 0, "w": 16, "h": 16}
    }
  },
  "meta": {
    "size": {"w": 48, "h": 16}
  }
}
```

---

## Adding New Sprite Sheets

### Step 1: Add Field to GameSprites

```rust
// In src/assets/sprites.rs
#[derive(Resource, Default)]
pub struct GameSprites {
    // ... existing fields ...
    pub my_new_sheet: Option<SpriteSheet>,
}
```

### Step 2: Load in load_sprites

```rust
fn load_sprites(...) {
    // ... existing loads ...
    game_sprites.my_new_sheet =
        SpriteSheet::load("my_new_sheet", &asset_server, &mut texture_atlas_layouts);
}
```

### Step 3: Place Files

```
assets/sprites/
├── my_new_sheet.png
└── my_new_sheet.json
```

---

## Common Patterns

### Animating Sprites

```rust
#[derive(Component)]
struct Animation {
    frames: Vec<String>,  // ["walk_1", "walk_2", "walk_3"]
    current: usize,
    timer: Timer,
}

fn animate_sprites(
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(&mut Animation, &mut Sprite)>,
) {
    let Some(sheet) = &game_sprites.characters else { return };

    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current = (anim.current + 1) % anim.frames.len();
            if let Some(index) = sheet.get(&anim.frames[anim.current]) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = index;
                }
            }
        }
    }
}
```

### Changing Sprite at Runtime

```rust
fn update_health_icon(
    game_sprites: Res<GameSprites>,
    health: Res<PlayerHealth>,
    mut query: Query<&mut Sprite, With<HealthIcon>>,
) {
    let Some(icons) = &game_sprites.ui_icons else { return };

    for mut sprite in &mut query {
        let name = match health.percent() {
            p if p > 0.5 => "heart_full",
            p if p > 0.0 => "heart_half",
            _ => "heart_empty",
        };

        if let Some(index) = icons.get(name) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = index;
            }
        }
    }
}
```

### Scaling Sprites to Grid

```rust
const TILE_SIZE: f32 = 40.0;

// 16x16 sprite scaled to 40x40 tile grid
if let Some(sprite) = sheet.sprite_sized("floor", Vec2::splat(TILE_SIZE)) {
    commands.spawn((sprite, Transform::from_xyz(x, y, 0.0)));
}
```

---

## Troubleshooting

### Sprites Look Blurry

The game should already have `ImagePlugin::default_nearest()` configured. If sprites are blurry, check `src/main.rs`.

### JSON Parse Error

Check Aseprite export settings:
- Format must be "Hash" (not "Array")
- File must be valid JSON (no trailing commas, etc.)

### Sprite Not Found

1. Check the frame name in Aseprite matches exactly (case-sensitive)
2. Verify both `.png` and `.json` are in `assets/sprites/`
3. Check logs for loading errors

### Sheet Not Loading

The system logs info when sheets load:
```
INFO Loaded sprite sheet 'ui_icons' with 48 sprites
```

If missing, check:
1. Files exist at `assets/sprites/{name}.json` and `assets/sprites/{name}.png`
2. JSON is valid (try opening in a JSON validator)
3. Run with `RUST_LOG=debug` to see detailed errors
