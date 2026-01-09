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

    /// Create Sprite component by name
    fn sprite(&self, name: &str) -> Option<Sprite>;

    /// Create Sprite with custom size
    fn sprite_sized(&self, name: &str, size: Vec2) -> Option<Sprite>;
}
```

### `GameSprites`

Resource containing all loaded sprite sheets:

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

### JSON Format Example (Frames)

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

### JSON Format Example (Slices)

The system also supports Aseprite's **slices** format, which is useful for sprite sheets with irregular regions:

```json
{
  "frames": {
    "UI_ALL.aseprite": {
      "frame": {"x": 0, "y": 0, "w": 2048, "h": 2576}
    }
  },
  "meta": {
    "size": {"w": 2048, "h": 2576},
    "slices": [
      {
        "name": "Slice_3013",
        "keys": [{"frame": 0, "bounds": {"x": 1, "y": 993, "w": 14, "h": 14}}]
      },
      {
        "name": "heart_empty",
        "keys": [{"frame": 0, "bounds": {"x": 17, "y": 993, "w": 14, "h": 14}}]
      }
    ]
  }
}
```

Both frames and slices are loaded into the same `SpriteSheet.sprites` map, accessible by name.

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

### Step 2: Load in load_assets

```rust
fn load_assets(...) {
    // ... existing loads ...
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

---

## Common Patterns

### Sprites in UI Widgets (Marker + System Pattern)

When adding sprites to UI widgets built with `ChildBuilder`, you can't directly access `Res<GameSprites>`. Use the **marker + system pattern**:

1. **Define a marker component** for the sprite placeholder
2. **Spawn a placeholder node** with the marker in your widget function
3. **Create a system** that queries for the marker and populates the sprite

```rust
use bevy::prelude::*;
use crate::assets::GameSprites;

/// Marker component for sprite placeholder
#[derive(Component)]
struct HeartIconPlaceholder;

/// Widget function - spawns placeholder (no GameSprites needed)
pub fn spawn_health_display(parent: &mut ChildBuilder, hp: i32, max_hp: i32) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(4.0),
        ..default()
    }).with_children(|row| {
        // Placeholder - will be populated by system
        row.spawn((
            HeartIconPlaceholder,
            Node {
                width: Val::Px(16.0),
                height: Val::Px(16.0),
                ..default()
            },
        ));

        // Text
        row.spawn(Text::new(format!("{}/{}", hp, max_hp)));
    });
}

/// System that populates placeholders with actual sprites
fn populate_heart_icons(
    mut commands: Commands,
    query: Query<Entity, With<HeartIconPlaceholder>>,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = &game_sprites.ui_all else { return };
    let Some(index) = sheet.get("Slice_3013") else { return };

    for entity in &query {
        commands.entity(entity)
            .remove::<HeartIconPlaceholder>()
            .insert(ImageNode::from_atlas_image(
                sheet.texture.clone(),
                TextureAtlas {
                    layout: sheet.layout.clone(),
                    index,
                },
            ));
    }
}

/// Plugin to register the system
pub struct HealthDisplayPlugin;

impl Plugin for HealthDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, populate_heart_icons);
    }
}
```

**Why this pattern?**
- Widget functions using `ChildBuilder` can't access Bevy resources
- Threading `GameSprites` through all intermediate functions creates tight coupling
- The marker + system pattern keeps widget signatures clean and decouples sprite loading

**Key points:**
- The system runs every frame but only processes entities with the marker
- Once populated, the marker is removed so the entity isn't processed again
- Register the plugin in `src/plugins/game.rs`

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
    let Some(sheet) = &game_sprites.ui_icons else { return };

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

### Stateful Sprite Buttons (Selected/Unselected)

For menu buttons or toggles that change appearance based on state, use a component that tracks both sprite variants and a system that swaps them:

```rust
/// Component for UI elements with selected/unselected sprite states
#[derive(Component)]
struct SpriteMenuItem {
    index: usize,                       // Which menu item (for selection tracking)
    unselected_slice: &'static str,     // Slice name when not selected
    selected_slice: &'static str,       // Slice name when selected
}

/// Spawn a sprite button (placeholder - system populates the image)
parent.spawn((
    SpriteMenuItem {
        index: 0,
        unselected_slice: "Slice_295",  // Gray/inactive state
        selected_slice: "Slice_329",    // Highlighted/active state
    },
    Node {
        width: Val::Px(141.0),   // 47 * 3 = 141 (3x scale)
        height: Val::Px(42.0),   // 14 * 3 = 42
        ..default()
    },
));

/// System that updates sprites based on selection state
fn update_sprite_menu_items(
    mut commands: Commands,
    menu_selection: Res<MenuSelection>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(Entity, &SpriteMenuItem, Option<&mut ImageNode>)>,
) {
    let Some(ui_all) = &game_sprites.ui_all else { return };

    for (entity, sprite_item, image_node) in &mut query {
        // Choose slice based on selection state
        let slice_name = if sprite_item.index == menu_selection.index {
            sprite_item.selected_slice
        } else {
            sprite_item.unselected_slice
        };

        let Some(index) = ui_all.get(slice_name) else { continue };

        match image_node {
            Some(mut node) => {
                // Update existing sprite's atlas index
                if let Some(atlas) = &mut node.texture_atlas {
                    atlas.index = index;
                }
            }
            None => {
                // First time - insert the ImageNode
                commands.entity(entity).insert(ImageNode::from_atlas_image(
                    ui_all.texture.clone(),
                    TextureAtlas {
                        layout: ui_all.layout.clone(),
                        index,
                    },
                ));
            }
        }
    }
}
```

**Key points:**
- The system handles both initial population AND runtime updates
- No marker removal needed - the component persists for ongoing state changes
- Sprite swapping is efficient (just changes the atlas index)

---

## Finding Slice Dimensions

To find the dimensions of a slice in `ui_all.json`:

```bash
# Search for specific slices
grep -A 1 '"Slice_193"\|"Slice_227"' assets/sprites/ui_all.json
```

Example output:
```json
{ "name": "Slice_193", "keys": [{ "frame": 0, "bounds": {"x": 1632, "y": 337, "w": 47, "h": 14 } }] },
{ "name": "Slice_227", "keys": [{ "frame": 0, "bounds": {"x": 1680, "y": 338, "w": 47, "h": 13 } }] },
```

The `bounds` field gives you `w` (width) and `h` (height). For UI scaling:
- Menu buttons are typically 47x14 pixels
- Scale 3x for readable size: 141x42 pixels in the Node

---

## Troubleshooting

### Sprites Look Blurry

The game should already have `ImagePlugin::default_nearest()` configured in `src/main.rs`. If sprites are blurry, verify this is set.

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
