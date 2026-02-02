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
    pub looping: bool,
    pub frame_duration: f32,
    pub synchronized: bool,
}
```

- `synchronized: true` — frame derived from global `AnimationClock` (all same-config animations stay in phase)
- `synchronized: false` — per-entity timer (for triggered animations like walk/attack/death)

The `AnimationClock` ticks every frame unconditionally to maintain consistent timing, even when no `SpriteAnimation` components exist. The animation application systems only run when animations are present.

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
2. **Add a timer component** (e.g., `PlayerWalkTimer(Timer::from_seconds(0.3, TimerMode::Once))`) to the entity
3. **On trigger** (e.g., successful movement), switch animation only if not already walking, then reset timer:
   ```rust
   let already_walking = anim.first_frame == sheet.walk_animation.first_frame;
   if !already_walking {
       anim.first_frame = sheet.walk_animation.first_frame;
       anim.last_frame = sheet.walk_animation.last_frame;
       anim.current_frame = sheet.walk_animation.first_frame;
       anim.frame_duration = sheet.walk_animation.frame_duration;
       anim.synchronized = false;
       anim.timer = Timer::from_seconds(sheet.walk_animation.frame_duration, TimerMode::Repeating);
   }
   walk_timer.0.reset();  // Always reset to keep animation alive during continuous movement
   ```
   **Important**: Don't reset `current_frame` if already walking — this prevents choppy animation during held-key movement.
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

### Walk Animation Config
- Frames 13-18, `frame_duration: 0.08`, looping, not synchronized
- `PlayerWalkTimer`: 0.3s — slightly longer than one tile movement (~0.167s at 6 tiles/sec) to prevent idle flicker between consecutive moves

### Player Attack Animation (Fight Modal)

Same timer pattern but for combat:
1. `PlayerSpriteSheet::attack_animation` — frames 39-47, 0.08s/frame, non-looping
2. `PlayerAttackTimer(Timer)` — spawned on fight modal player sprite (0.72s duration)
3. In `handle_fight_modal_select` (input.rs): after `attack()`, switch `SpriteAnimation` to attack frames with `looping = false`, reset timer
4. `revert_attack_idle` system: when timer expires, switch back to idle and set `looping = true`

### Key Files
- `src/ui/player_sprite.rs` — `PlayerWalkTimer`, `PlayerAttackTimer`, revert systems, animation fields
- `src/ui/screens/dungeon/plugin.rs` — walk animation switch in `handle_dungeon_movement`
- `src/ui/screens/fight_modal/input.rs` — attack animation switch in `handle_fight_modal_select`
- `src/ui/screens/fight_modal/render.rs` — spawns `PlayerAttackTimer` on player sprite

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
