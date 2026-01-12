# Mob Sprites

## Overview

Mob sprites are displayed during combat on the fight screen and in the monster book popup. Each `MobId` can have an associated animated sprite sheet with idle animation support.

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
| `src/ui/mob_animation.rs` | `MobAnimationPlugin`, `MobSpriteSheets` resource, `MobAnimation` component, animation system |
| `src/screens/fight/ui.rs` | `populate_mob_sprite()` - displays animated sprite in combat |
| `src/screens/book_popup.rs` | `update_book_mob_sprite()` - displays animated sprite in monster book |
| `assets/sprites/mobs/` | Sprite sheet PNGs and JSON metadata |

## How It Works

### Animation System (`MobAnimationPlugin`)

1. **Loading**: `load_mob_sprite_sheets()` runs at `PreStartup`, loading textures and creating `TextureAtlasLayout` for each mob
2. **Populating**: When a `NeedsMobSprite` or `BookMobSprite` entity is detected, the system inserts:
   - `ImageNode` with the texture atlas
   - `MobAnimation` component with timer and frame config
3. **Animating**: `animate_mob_sprites()` runs every frame, ticking timers and updating atlas indices

### Components

```rust
/// Animation configuration for a mob's idle animation.
pub struct MobAnimationConfig {
    pub first_frame: usize,    // First frame index
    pub last_frame: usize,     // Last frame index (inclusive)
    pub frame_duration: f32,   // Seconds per frame
}

/// Component for animated mob sprites.
pub struct MobAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub first_frame: usize,
    pub last_frame: usize,
}
```

## Currently Supported Mobs

| MobId | Sprite Sheet | Total Frames | Idle Range | Frame Duration |
|-------|-------------|--------------|------------|----------------|
| `Goblin` | `goblin.png` | 27 | 0-3 | 0.2s |
| `Slime` | `slime.png` | 18 | 0-3 | 0.25s |

## Legacy Note

The old `SpriteAssets::mob_sprite()` method and static sprite loading in `src/assets/sprites.rs` is no longer used for mobs with animations. The `MobSpriteSheets` resource in `src/ui/mob_animation.rs` is the new source of truth.
