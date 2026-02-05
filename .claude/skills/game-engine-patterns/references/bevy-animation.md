# Bevy Animation

Overview of Bevy 0.18 sprite animation using texture atlases and timers.

## Quick Navigation

| Topic | Use When | Reference |
|-------|----------|-----------|
| Sprite Animation | Frame-based animation, atlas cycling | [animation/sprite-animation.md](animation/sprite-animation.md) |
| Timers | Frame timing, duration control | [animation/timers.md](animation/timers.md) |

## Quick Reference

```rust
// Animation components
#[derive(Component)]
struct AnimationIndices { first: usize, last: usize }

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Spawn animated sprite
commands.spawn((
    Sprite::from_atlas_image(
        texture,
        TextureAtlas { layout: layout_handle, index: 0 },
    ),
    AnimationIndices { first: 0, last: 5 },
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
));

// Animation system
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
```

## Key Types

| Type | Module | Purpose |
|------|--------|---------|
| `Timer` | `bevy::time` | Time tracking with modes |
| `TimerMode` | `bevy::time` | `Repeating` or `Once` |
| `TextureAtlas` | `bevy::sprite` | Frame index + layout reference |

## This Codebase

This codebase has a comprehensive animation system:

- **AnimationConfig** - defines frame range, duration, looping, sync mode
- **SpriteAnimation** - per-entity animation state component
- **AnimationClock** - global timer for synchronized animations
- **SpriteMarker trait** - generic sprite population with animation

Key files:
- `src/ui/animation.rs` - core animation system
- `src/ui/sprite_marker.rs` - trait-based sprite population
- `src/ui/mob_animation.rs` - mob sprite sheets and animations
- `src/ui/player_sprite.rs` - player animation with state switching

## Animation Timing Guidelines

| Animation Type | Frame Duration | Notes |
|---------------|----------------|-------|
| Idle (normal) | 0.15-0.25s | Synchronized across entities |
| Idle (large) | 0.35s | Slower for big creatures |
| Walk | 0.08s | Fast, responsive |
| Attack | 0.06s | Very fast for impact |
| Death | 0.15s | Deliberate timing |
| Crafting | 0.1s | Medium speed |
