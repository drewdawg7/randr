# Mob Sprites

## Overview

Mob sprites are displayed during combat on the fight screen and in the MonsterCompendium (opened with 'b' key). Each `MobId` can have an associated animated sprite sheet with idle animation support.

## Adding a New Mob Sprite (with Animation)

When adding a new mob sprite, **always include the idle animation if it exists** in the Aseprite file.

### 1. Export the Sprite Sheet

Export all frames as a horizontal sprite sheet with JSON metadata:

```bash
ASEPRITE="/Users/drewstewart/Library/Application Support/Steam/steamapps/common/Aseprite/Aseprite.app/Contents/MacOS/aseprite"
"$ASEPRITE" --batch "input.aseprite" \
  --sheet assets/sprites/mobs/<mob_name>.png \
  --data assets/sprites/mobs/<mob_name>.json \
  --format json-hash \
  --sheet-type horizontal
```

### 2. Register the Sprite Sheet in `MobSpriteSheets`

Add the mob to `load_mob_sprite_sheets()` in `src/ui/mob_animation.rs`:

```rust
// <MobName>: <total_frames> frames total, 32x32 each, idle is frames <first>-<last>
let <mob_name>_texture: Handle<Image> = asset_server.load("sprites/mobs/<mob_name>.png");
let <mob_name>_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), <total_frames>, 1, None, None);
let <mob_name>_layout_handle = layouts.add(<mob_name>_layout);
mob_sheets.insert(
    MobId::<MobName>,
    MobSpriteSheet {
        texture: <mob_name>_texture,
        layout: <mob_name>_layout_handle,
        animation: MobAnimationConfig {
            first_frame: 0,  // First frame of idle animation
            last_frame: 3,   // Last frame of idle animation (inclusive)
            frame_duration: 0.2,  // Seconds per frame (0.2-0.25 is typical)
        },
    },
);
```

### 3. Determine Animation Frame Range

Check the Aseprite file for animation tags:
- The **idle** animation is typically the first few frames (e.g., frames 0-3)
- Look at the timeline tags in Aseprite to identify the idle range
- Frame indices are zero-based

### 4. Frame Duration Guidelines

- `0.2` seconds - Normal/fast animations (goblin)
- `0.25` seconds - Slower, bouncier animations (slime)
- Adjust based on how the animation looks in-game

## Key Files

| File | Purpose |
|------|---------|
| `src/ui/animation.rs` | `SpriteAnimation` component, `AnimationConfig`, `animate_sprites()` system |
| `src/ui/sprite_marker.rs` | `SpriteMarker` trait, `SpriteData`, generic `populate_sprite_markers<M>()` system |
| `src/ui/mob_animation.rs` | `MobAnimationPlugin`, `MobSpriteSheets` resource, `DungeonMobSprite` marker |
| `src/ui/screens/fight_modal/state.rs` | `FightModalMobSprite` marker with `SpriteMarker` impl |
| `src/ui/screens/fight/ui.rs` | `populate_mob_sprite()` - displays animated sprite in combat |
| `src/ui/screens/monster_compendium/render.rs` | `update_compendium_mob_sprite()` - displays animated sprite in MonsterCompendium |
| `assets/sprites/mobs/` | Sprite sheet PNGs and JSON metadata |

## Sprite Marker System

The codebase uses a **generic sprite marker pattern** to reduce code duplication. See `src/ui/sprite_marker.rs`.

### SpriteMarker Trait

```rust
pub trait SpriteMarker: Component + Sized {
    type Resources: SystemParam;
    fn resolve(&self, resources: &<Self::Resources as SystemParam>::Item<'_, '_>) -> Option<SpriteData>;
}
```

### Existing Mob Sprite Markers

| Marker | File | Resource | flip_x |
|--------|------|----------|--------|
| `DungeonMobSprite` | `mob_animation.rs` | `MobSpriteSheets` | false |
| `FightModalMobSprite` | `fight_modal/state.rs` | `MobSpriteSheets` | true |

### Registering a New Mob Sprite Marker

1. Define the marker component with `mob_id` field
2. Implement `SpriteMarker` trait
3. Register with `app.register_sprite_marker::<YourMarker>()`

Example:
```rust
#[derive(Component)]
pub struct MyMobSprite {
    pub mob_id: MobId,
}

impl SpriteMarker for MyMobSprite {
    type Resources = Res<'static, MobSpriteSheets>;

    fn resolve(&self, sheets: &Res<MobSpriteSheets>) -> Option<SpriteData> {
        let sheet = sheets.get(self.mob_id)?;
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation: sheet.animation.clone().into(),
            flip_x: false,
        })
    }
}

// In plugin:
app.register_sprite_marker::<MyMobSprite>();
```

## How It Works

### Animation System

1. **Loading**: `load_mob_sprite_sheets()` runs at `PreStartup`, loading textures and creating `TextureAtlasLayout` for each mob
2. **Populating**: When a marker entity is detected via `Added<M>`, the generic `populate_sprite_markers<M>()` system:
   - Calls `marker.resolve()` to get sprite data
   - Removes the marker
   - Inserts `ImageNode` + `SpriteAnimation`
3. **Animating**: `animate_sprites()` runs every frame, ticking timers and updating atlas indices for all `SpriteAnimation` components

### Components

```rust
/// Unified animation config (converts from MobAnimationConfig)
pub struct AnimationConfig {
    pub first_frame: usize,
    pub last_frame: usize,
    pub frame_duration: f32,
}

/// Unified animation component for all animated sprites
pub struct SpriteAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub first_frame: usize,
    pub last_frame: usize,
}
```

## Currently Supported Mobs

| MobId | Sprite Sheet | Frame Size | Total Frames | Idle Range | Frame Duration |
|-------|-------------|------------|--------------|------------|----------------|
| `Goblin` | `goblin.png` | 32x32 | 27 | 0-3 | 0.2s |
| `Slime` | `slime.png` | 32x32 | 18 | 0-3 | 0.25s |
| `Dragon` | `dragon.png` | 64x32 | 66 | 0-3 | 0.35s |

### Non-Square Sprites

For sprites with non-square dimensions (like Dragon at 64x32), use `UVec2::new(width, height)` instead of `UVec2::splat()`:

```rust
let dragon_layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 66, 1, None, None);
```

### Sprite Display Sizes

The sprite display containers should be consistent across all locations:

| Location | Container Size | Inner Sprite Size | File |
|----------|----------------|-------------------|------|
| Fight Screen | 224x224 | 192x192 | `src/screens/fight/ui.rs:204-222` |
| MonsterCompendium | 112x112 | 96x96 | `src/screens/monster_compendium.rs:233-249` |
| Dungeon Tab | 48x48 | 48x48 | `src/screens/town/tabs/dungeon.rs` |
| Fight Modal | 128x128 | 128x128 | `src/ui/screens/fight_modal/render.rs` |

## Special Cases

### CompendiumMobSprite

The `CompendiumMobSprite` in `src/ui/screens/monster_compendium/render.rs` does NOT use the `SpriteMarker` trait because it:
- Doesn't remove the marker (sprite updates dynamically on selection change)
- Uses `SpriteAnimation::new(&sheet.animation.clone().into())` directly
