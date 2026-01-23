# Generic Sprite Marker System

## Overview

The sprite marker system provides a generic, trait-based approach to sprite population, eliminating repeated marker-to-sprite conversion code across the codebase.

## Key Files

| File | Purpose |
|------|---------|
| `src/ui/animation.rs` | `SpriteAnimation`, `AnimationConfig`, `animate_sprites()` |
| `src/ui/sprite_marker.rs` | `SpriteMarker` trait, `SpriteData`, `populate_sprite_markers<M>()` |

## Core Concepts

### SpriteMarker Trait

```rust
pub trait SpriteMarker: Component + Sized {
    /// The resource type(s) needed for sprite lookup.
    type Resources: SystemParam;

    /// Resolve sprite data from resources. Returns None if not ready.
    fn resolve(&self, resources: &<Self::Resources as SystemParam>::Item<'_, '_>) -> Option<SpriteData>;
}
```

### SpriteData

```rust
pub struct SpriteData {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation: AnimationConfig,
    pub flip_x: bool,
}
```

### SpriteAnimation

Unified animation component for all animated sprites:

```rust
pub struct SpriteAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub first_frame: usize,
    pub last_frame: usize,
}
```

## Existing Implementations

### Player Sprites

| Marker | File | Resource | flip_x |
|--------|------|----------|--------|
| `DungeonPlayerSprite` | `player_sprite.rs` | `PlayerSpriteSheet` | false |
| `FightModalPlayerSprite` | `fight_modal/state.rs` | `PlayerSpriteSheet` | false |

### Mob Sprites

| Marker | File | Resource | flip_x |
|--------|------|----------|--------|
| `DungeonMobSprite` | `mob_animation.rs` | `MobSpriteSheets` | false |
| `FightModalMobSprite` | `fight_modal/state.rs` | `MobSpriteSheets` | true |

## Adding a New Sprite Marker

### 1. Define the Marker Component

```rust
#[derive(Component)]
pub struct MySprite {
    // Optional data fields (e.g., mob_id)
}
```

### 2. Implement SpriteMarker

```rust
impl SpriteMarker for MySprite {
    type Resources = Res<'static, MySpriteSheet>;

    fn resolve(&self, sheet: &Res<MySpriteSheet>) -> Option<SpriteData> {
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation: sheet.animation.clone(),
            flip_x: false,
        })
    }
}
```

### 3. Register in Plugin

```rust
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.register_sprite_marker::<MySprite>();
    }
}
```

## How It Works

1. **Spawn marker**: Spawn entity with marker component + `Node` for sizing
2. **Detection**: `populate_sprite_markers<M>()` detects `Added<M>` entities
3. **Resolution**: Calls `marker.resolve()` to get sprite data from resources
4. **Population**: Removes marker, inserts `ImageNode` + `SpriteAnimation`
5. **Animation**: `animate_sprites()` updates all `SpriteAnimation` components

## Runtime Animation Switching

Sprites can switch between animations at runtime by mutating `SpriteAnimation` fields directly. This is used for the player walk animation:

### Pattern: Timer-Based Animation Switch

1. **Add alternate `AnimationConfig`** to the sprite sheet resource (e.g., `PlayerSpriteSheet::walk_animation`)
2. **Add a timer component** (e.g., `PlayerWalkTimer(Timer)`) to the entity
3. **On trigger** (e.g., successful movement), update `SpriteAnimation` fields and reset the timer:
   ```rust
   anim.first_frame = sheet.walk_animation.first_frame;
   anim.last_frame = sheet.walk_animation.last_frame;
   anim.current_frame = sheet.walk_animation.first_frame;
   walk_timer.0.reset();
   ```
4. **Revert system** ticks the timer and switches back to idle when it expires:
   ```rust
   fn revert_player_idle(time, sheet, query: Query<(&mut PlayerWalkTimer, &mut SpriteAnimation)>) {
       timer.0.tick(time.delta());
       if timer.0.just_finished() {
           anim.first_frame = sheet.animation.first_frame;
           // ...
       }
   }
   ```

### Key Files
- `src/ui/player_sprite.rs` — `PlayerWalkTimer`, `revert_player_idle`, `walk_animation` field
- `src/ui/screens/dungeon/plugin.rs` — animation switch in `handle_dungeon_movement`

## Special Cases

### Dynamic Sprites (CompendiumMobSprite)

Some sprites need to update dynamically without removing the marker. These should NOT use the `SpriteMarker` trait:

```rust
// Instead, handle directly with SpriteAnimation::new()
commands.entity(entity).insert((
    ImageNode::from_atlas_image(texture, atlas),
    SpriteAnimation::new(&sheet.animation),
));
```

## Benefits

- **Single animation system**: One `animate_sprites()` for all sprites
- **DRY**: No repeated populate functions
- **Type-safe**: Compile-time resource requirements
- **Extensible**: New sprite types just implement the trait
