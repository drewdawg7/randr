# Fonts

## GameFonts Resource

```rust
#[derive(Resource, Default)]
pub struct GameFonts {
    pub pixel: Handle<Font>,  // CuteFantasy-5x9 pixel font
}
```

Access via `Res<GameFonts>` in systems.

## Using Custom Fonts

### Basic Usage

```rust
fn spawn_text(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
) {
    commands.spawn((
        Text::new("Hello"),
        TextFont {
            font: game_fonts.pixel.clone(),
            font_size: 16.0,
            font_smoothing: FontSmoothing::None,  // Crisp pixels
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}
```

### TextFont Fields

```rust
pub struct TextFont {
    pub font: Handle<Font>,           // Font asset handle
    pub font_size: f32,               // Vertical height in pixels
    pub line_height: LineHeight,      // Line spacing (default: 1.2x)
    pub font_smoothing: FontSmoothing, // Antialiasing mode
}
```

### FontSmoothing Options

- `FontSmoothing::AntiAliased` - Default, smooth edges (good for vector fonts)
- `FontSmoothing::None` - **Required for pixel fonts** to keep crisp edges

## Text in UI Widgets (ChildBuilder)

When spawning text inside `with_children()`, you can't access `Res<GameFonts>`. Use the marker pattern:

```rust
#[derive(Component)]
struct TextPlaceholder {
    content: String,
    font_size: f32,
    color: Color,
}

/// Widget function - spawns placeholder
pub fn spawn_label(parent: &mut ChildBuilder, text: &str) {
    parent.spawn((
        TextPlaceholder {
            content: text.to_string(),
            font_size: 16.0,
            color: Color::WHITE,
        },
        Node { ..default() },
    ));
}

/// System that populates placeholders with actual text
fn populate_text_placeholders(
    mut commands: Commands,
    query: Query<(Entity, &TextPlaceholder)>,
    game_fonts: Res<GameFonts>,
) {
    for (entity, placeholder) in &query {
        commands.entity(entity)
            .remove::<TextPlaceholder>()
            .insert((
                Text::new(&placeholder.content),
                TextFont {
                    font: game_fonts.pixel.clone(),
                    font_size: placeholder.font_size,
                    font_smoothing: FontSmoothing::None,
                    ..default()
                },
                TextColor(placeholder.color),
            ));
    }
}
```

## Adding New Fonts

### Step 1: Place Font File

```
assets/fonts/
└── MyNewFont.ttf
```

### Step 2: Add to GameFonts

```rust
// In src/assets/fonts.rs (or sprites.rs)
#[derive(Resource, Default)]
pub struct GameFonts {
    pub pixel: Handle<Font>,
    pub my_new_font: Handle<Font>,  // Add field
}
```

### Step 3: Load in load_assets

```rust
fn load_assets(
    asset_server: Res<AssetServer>,
    mut game_fonts: ResMut<GameFonts>,
    // ...
) {
    game_fonts.pixel = asset_server.load("fonts/CuteFantasy-5x9.ttf");
    game_fonts.my_new_font = asset_server.load("fonts/MyNewFont.ttf");
}
```

## Self-Verification Checklist

- [ ] Font file (.ttf or .otf) in `assets/fonts/`?
- [ ] Field added to `GameFonts` resource?
- [ ] Font loaded in `load_assets()`?
- [ ] Using `FontSmoothing::None` for pixel fonts?
- [ ] Systems have `Res<GameFonts>` parameter?

## Common Issues

### Blurry Pixel Font
**Cause:** Default antialiasing smooths edges
**Fix:** Use `font_smoothing: FontSmoothing::None`

### Can't Access GameFonts in ChildBuilder
**Cause:** `with_children()` doesn't allow resource access
**Fix:** Use marker + system pattern (see above)

### Font Not Rendering
**Cause:** Font file not found or not loaded
**Fix:** Check path, ensure `load_assets()` runs in PreStartup
