# Sprite Animation

Sprite animation in Bevy works by changing the `index` field of a `TextureAtlas` component over time. This document covers the standard pattern and advanced techniques.

## Quick Reference

```rust
// Components
#[derive(Component)]
struct AnimationIndices { first: usize, last: usize }

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Spawn
commands.spawn((
    Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 }),
    AnimationIndices { first: 0, last: 5 },
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
));

// System
fn animate(time: Res<Time>, mut q: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>) {
    for (indices, mut timer, mut sprite) in &mut q {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last { indices.first } else { atlas.index + 1 };
            }
        }
    }
}
```

## Core Components

### AnimationIndices

Defines the frame range for an animation:

```rust
#[derive(Component)]
struct AnimationIndices {
    first: usize,  // Starting frame index
    last: usize,   // Ending frame index (inclusive)
}
```

### AnimationTimer

Wrapper around `Timer` for frame timing:

```rust
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
```

The `Deref`/`DerefMut` derives allow direct access to `Timer` methods.

## Setup

### Create TextureAtlasLayout

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/character.png");

    // 6 frames in a row, 32x32 each
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), 6, 1, None, None
    );
    let layout_handle = layouts.add(layout);

    let animation = AnimationIndices { first: 0, last: 5 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas { layout: layout_handle, index: animation.first },
        ),
        animation,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
```

## Animation System

### Looping Animation

```rust
fn animate_looping(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first  // Loop back
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
```

### One-Shot Animation

For animations that play once and stop (attacks, deaths):

```rust
#[derive(Component)]
struct OneShotAnimation {
    first: usize,
    last: usize,
    finished: bool,
}

fn animate_oneshot(
    time: Res<Time>,
    mut query: Query<(&mut OneShotAnimation, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut anim, mut timer, mut sprite) in &mut query {
        if anim.finished { continue; }

        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == anim.last {
                    anim.finished = true;
                } else {
                    atlas.index += 1;
                    timer.reset();  // Reset for next frame
                }
            }
        }
    }
}
```

## Timer Modes

### Repeating (Looping Animations)

```rust
// Timer auto-resets when finished
Timer::from_seconds(0.1, TimerMode::Repeating)
```

### Once (One-Shot Animations)

```rust
// Timer stops when finished - manually reset for next frame
Timer::from_seconds(0.1, TimerMode::Once)
```

## FPS-Based Configuration

Convert FPS to duration:

```rust
let fps = 12.0;
let duration = 1.0 / fps;  // 0.0833 seconds per frame

Timer::from_seconds(duration, TimerMode::Repeating)
```

### FPS Config Pattern

```rust
#[derive(Component)]
struct AnimationConfig {
    first: usize,
    last: usize,
    fps: f32,
}

impl AnimationConfig {
    fn frame_duration(&self) -> f32 {
        1.0 / self.fps
    }
}
```

## Pausing and Resuming

```rust
#[derive(Component)]
struct AnimationPaused;

fn pause_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationTimer), Without<AnimationPaused>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (entity, mut timer) in &mut query {
            timer.pause();
            commands.entity(entity).insert(AnimationPaused);
        }
    }
}

fn resume_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationTimer), With<AnimationPaused>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for (entity, mut timer) in &mut query {
            timer.unpause();
            commands.entity(entity).remove::<AnimationPaused>();
        }
    }
}
```

## Animation Events

### Event-Based Completion

```rust
#[derive(Event)]
struct AnimationComplete {
    entity: Entity,
}

fn animate_with_events(
    time: Res<Time>,
    mut events: EventWriter<AnimationComplete>,
    mut query: Query<(Entity, &AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (entity, indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == indices.last {
                    events.send(AnimationComplete { entity });
                    atlas.index = indices.first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}

fn on_animation_complete(mut events: EventReader<AnimationComplete>) {
    for event in events.read() {
        info!("Animation finished on {:?}", event.entity);
    }
}
```

## Variable Frame Durations

For animations where each frame has different timing:

```rust
#[derive(Component)]
struct VariableAnimation {
    frames: Vec<usize>,      // Frame indices
    durations: Vec<f32>,     // Duration per frame
    current: usize,          // Current index into frames/durations
    looping: bool,
}

fn animate_variable(
    time: Res<Time>,
    mut query: Query<(&mut VariableAnimation, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut anim, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            // Advance to next frame
            let can_advance = anim.current + 1 < anim.frames.len();

            if can_advance {
                anim.current += 1;
            } else if anim.looping {
                anim.current = 0;
            } else {
                continue;  // Animation finished
            }

            // Update sprite and timer
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = anim.frames[anim.current];
            }
            timer.set_duration(Duration::from_secs_f32(anim.durations[anim.current]));
            timer.reset();
        }
    }
}
```

## Animation State Switching

Change animation based on game state:

```rust
#[derive(Component)]
struct PlayerAnimations {
    idle: AnimationIndices,
    walk: AnimationIndices,
    attack: AnimationIndices,
}

fn switch_to_walk(
    mut query: Query<(&PlayerAnimations, &mut AnimationIndices, &mut Sprite)>,
) {
    for (anims, mut current, mut sprite) in &mut query {
        *current = anims.walk.clone();
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = current.first;
        }
    }
}
```

## This Codebase

### AnimationConfig

From `src/ui/animation.rs`:

```rust
pub struct AnimationConfig {
    pub first_frame: usize,
    pub last_frame: usize,
    pub frame_duration: f32,
    pub looping: bool,
    pub synchronized: bool,  // Use global clock
}
```

### SpriteAnimation Component

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

### AnimationClock (Synchronized Animations)

Global timer for animations that should stay in phase:

```rust
#[derive(Resource, Default)]
pub struct AnimationClock {
    pub elapsed: f32,
}

fn tick_clock(time: Res<Time>, mut clock: ResMut<AnimationClock>) {
    clock.elapsed += time.delta_secs();
}
```

Synchronized animations (like idle) use the global clock so all entities animate together. Non-synchronized animations (walk, attack) use per-entity timers.

### SpriteMarker Trait

Generic pattern for declarative sprite population:

```rust
pub trait SpriteMarker: Component {
    type Resources: SystemParam;
    fn resolve(&self, resources: &...) -> Option<SpriteData>;
}

// Usage: spawn marker, system auto-populates sprite
commands.spawn(DungeonMobSprite { mob_id: MobId::Goblin });
```

## Timing Guidelines

| Animation Type | Frame Duration | FPS | Notes |
|---------------|----------------|-----|-------|
| Idle (normal) | 0.15-0.25s | 4-7 | Synchronized |
| Idle (large) | 0.35s | ~3 | Slower for big creatures |
| Walk | 0.08s | 12.5 | Fast, responsive |
| Attack | 0.06s | ~17 | Very fast for impact |
| Death | 0.15s | ~7 | Deliberate pacing |
| Crafting | 0.1s | 10 | Medium speed |

## Complete Plugin Example

```rust
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationComplete>()
            .add_systems(Update, (
                animate_looping_sprites,
                animate_oneshot_sprites,
            ));
    }
}
```

## Common Mistakes

### Forgetting to Tick Timer

```rust
// Wrong: timer never advances
if timer.just_finished() { ... }

// Correct: tick first
timer.tick(time.delta());
if timer.just_finished() { ... }
```

### Using finished() Instead of just_finished()

```rust
// Wrong: triggers every frame after completion
if timer.finished() { ... }

// Correct: triggers only on the tick it finishes
if timer.just_finished() { ... }
```

### Not Resetting Timer for One-Shot

```rust
// For TimerMode::Once, manually reset for next frame
if timer.just_finished() {
    atlas.index += 1;
    timer.reset();  // Don't forget!
}
```

### Animation Desync

```rust
// Problem: spawning at random times causes visual chaos
// Solution: use synchronized animations for idle/ambient
pub synchronized: bool,
```
