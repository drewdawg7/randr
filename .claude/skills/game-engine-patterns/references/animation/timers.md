# Timers

`Timer` tracks elapsed time and triggers when a duration is reached. Essential for frame timing in animations and time-based game logic.

## Quick Reference

```rust
// Create timers
Timer::from_seconds(0.5, TimerMode::Repeating)
Timer::from_seconds(2.0, TimerMode::Once)

// Advance time (call every frame)
timer.tick(time.delta());

// Check completion
if timer.just_finished() {
    // Triggers only on the frame it completes
}

// Query state
timer.elapsed_secs()      // Time passed
timer.remaining_secs()    // Time left
timer.fraction()          // Progress 0.0-1.0
timer.finished()          // Is complete (stays true for Once)

// Control
timer.pause();
timer.unpause();
timer.reset();
```

## TimerMode

### Repeating

Timer automatically resets when finished:

```rust
let mut timer = Timer::from_seconds(0.1, TimerMode::Repeating);

// In system
timer.tick(time.delta());
if timer.just_finished() {
    // Triggers every 0.1 seconds
    // Timer auto-resets
}
```

Use for: looping animations, periodic spawning, continuous effects.

### Once

Timer stops when finished:

```rust
let mut timer = Timer::from_seconds(2.0, TimerMode::Once);

// In system
timer.tick(time.delta());
if timer.just_finished() {
    // Triggers once after 2 seconds
    // Must manually reset() if needed again
}
```

Use for: one-shot animations, cooldowns, delays.

## Creating Timers

```rust
// From seconds (most common)
Timer::from_seconds(0.5, TimerMode::Repeating)

// From Duration
Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once)
Timer::new(Duration::from_millis(100), TimerMode::Repeating)
```

## Advancing Time

**Always call `tick()` before checking status:**

```rust
fn my_system(time: Res<Time>, mut timer: ResMut<MyTimer>) {
    timer.tick(time.delta());  // Always first!

    if timer.just_finished() {
        // Now check
    }
}
```

The `tick()` method takes a `Duration` - usually `time.delta()` for real time.

## Status Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `just_finished()` | `bool` | True only on tick when duration reached |
| `finished()` | `bool` | True if timer reached duration (stays true for Once) |
| `paused()` | `bool` | True if timer is paused |
| `elapsed()` | `Duration` | Time elapsed since last reset |
| `elapsed_secs()` | `f32` | Elapsed as float seconds |
| `remaining()` | `Duration` | Time remaining |
| `remaining_secs()` | `f32` | Remaining as float seconds |
| `fraction()` | `f32` | Progress 0.0 to 1.0 |
| `fraction_remaining()` | `f32` | Remaining progress 1.0 to 0.0 |
| `duration()` | `Duration` | Configured duration |
| `times_finished_this_tick()` | `u32` | Completions this tick (Repeating) |

### Choosing just_finished vs finished

```rust
// just_finished - triggers once
if timer.just_finished() {
    spawn_projectile();  // Spawns exactly once
}

// finished - stays true (Once mode)
if timer.finished() {
    hide_cooldown_indicator();  // Continuous while finished
}
```

## Control Methods

```rust
// Pause/unpause (tick does nothing while paused)
timer.pause();
timer.unpause();

// Reset elapsed time to zero
timer.reset();

// Immediately mark as finished
timer.finish();
```

## Configuration Methods

```rust
// Change duration
timer.set_duration(Duration::from_secs(2.0));

// Set elapsed time
timer.set_elapsed(Duration::from_secs(0.5));

// Change mode
timer.set_mode(TimerMode::Once);
```

## Timer as Component

Wrap in a newtype for type safety:

```rust
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct CooldownTimer(Timer);

// Deref allows direct access to Timer methods
fn system(mut q: Query<&mut AnimationTimer>) {
    for mut timer in &mut q {
        timer.tick(time.delta());  // Works via Deref
        if timer.just_finished() { ... }
    }
}
```

## Timer as Resource

For global timers:

```rust
#[derive(Resource, Deref, DerefMut)]
struct SpawnTimer(Timer);

fn setup(mut commands: Commands) {
    commands.insert_resource(SpawnTimer(
        Timer::from_seconds(5.0, TimerMode::Repeating)
    ));
}

fn spawn_enemies(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        // Spawn enemy every 5 seconds
    }
}
```

## Frame Timing for Animation

Convert FPS to timer duration:

```rust
let fps = 12.0;
let frame_duration = 1.0 / fps;  // ~0.083 seconds

Timer::from_seconds(frame_duration, TimerMode::Repeating)
```

### Common Animation Durations

| Animation | FPS | Duration |
|-----------|-----|----------|
| Idle | 4-7 | 0.14-0.25s |
| Walk | 10-15 | 0.067-0.1s |
| Attack | 15-20 | 0.05-0.067s |
| UI pulse | 2-4 | 0.25-0.5s |

## Multiple Completions Per Tick

For very short durations or lag spikes, timer may complete multiple times:

```rust
// Repeating timer that might complete multiple times
timer.tick(time.delta());

// Handle each completion
for _ in 0..timer.times_finished_this_tick() {
    spawn_particle();
}

// Or just once
if timer.just_finished() {
    // Only runs once even if multiple completions
}
```

## Cooldown Pattern

```rust
#[derive(Component)]
struct AttackCooldown(Timer);

fn attack_system(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AttackCooldown>,
) {
    for mut cooldown in &mut query {
        cooldown.tick(time.delta());

        if input.just_pressed(KeyCode::Space) && cooldown.finished() {
            perform_attack();
            cooldown.reset();
        }
    }
}
```

## Delay/Countdown Pattern

```rust
#[derive(Component)]
struct DespawnDelay(Timer);

fn despawn_after_delay(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnDelay)>,
) {
    for (entity, mut delay) in &mut query {
        delay.tick(time.delta());

        if delay.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

## Progress-Based Effects

Use `fraction()` for smooth interpolation:

```rust
fn fade_out(
    time: Res<Time>,
    mut query: Query<(&mut FadeTimer, &mut Sprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        // fraction goes 0.0 -> 1.0
        let alpha = 1.0 - timer.fraction();  // 1.0 -> 0.0
        sprite.color.set_alpha(alpha);
    }
}
```

## This Codebase

### Animation Timer Usage

From `src/ui/animation.rs`:

```rust
pub struct SpriteAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    // ...
}

fn advance_animation(anim: &mut SpriteAnimation, delta: Duration) {
    anim.timer.tick(delta);
    if anim.timer.just_finished() {
        // Advance frame
    }
}
```

### State-Based Timer Switching

From `src/ui/player_sprite.rs` - timers for animation state:

```rust
#[derive(Component)]
struct PlayerWalkTimer(Timer);

#[derive(Component)]
struct PlayerAttackTimer(Timer);

// Reset walk timer on movement
fn on_move(mut timer: Query<&mut PlayerWalkTimer>) {
    for mut t in &mut timer {
        t.reset();
    }
}

// Revert to idle when timer expires
fn check_idle(time: Res<Time>, mut timer: Query<&mut PlayerWalkTimer>) {
    for mut t in &mut timer {
        t.tick(time.delta());
        if t.just_finished() {
            // Switch back to idle animation
        }
    }
}
```

## Common Mistakes

### Not Ticking Before Checking

```rust
// Wrong: timer never advances
if timer.just_finished() { ... }

// Correct
timer.tick(time.delta());
if timer.just_finished() { ... }
```

### Using wrong check method

```rust
// just_finished: true for one frame only
// finished: stays true (for Once mode)

// For triggering actions once:
if timer.just_finished() { spawn(); }

// For checking if cooldown is ready:
if timer.finished() { allow_action(); }
```

### Forgetting to Reset One-Shot Timers

```rust
// Once mode doesn't auto-reset
let mut timer = Timer::from_seconds(1.0, TimerMode::Once);

// After it fires, must manually reset
if timer.just_finished() {
    do_thing();
    timer.reset();  // If you want it to fire again
}
```

### Hardcoding frame rates

```rust
// Wrong: assumes 60fps
frame += 1;
if frame % 6 == 0 { ... }  // Every 6 frames

// Correct: use actual time
timer.tick(time.delta());
if timer.just_finished() { ... }
```

## Time Resource Reference

```rust
// Real time (affected by pause)
time.delta()           // Duration since last frame
time.delta_secs()      // As f32
time.elapsed()         // Total time since startup
time.elapsed_secs()    // As f32

// Fixed time (for physics)
let fixed_time: Res<Time<Fixed>>;
fixed_time.delta()
fixed_time.overstep_fraction()  // For interpolation
```
