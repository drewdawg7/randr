# Fixed Timestep

## Quick Reference

```rust
// Add systems to FixedUpdate
app.add_systems(FixedUpdate, (
    apply_gravity,
    move_entities,
    check_collisions,
));

// Configure timestep (default is 64 Hz)
use bevy::time::Fixed;
app.insert_resource(Time::<Fixed>::from_hz(120.0)); // 120 Hz
app.insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0)); // 60 Hz

// Access fixed time in systems
fn physics_system(time: Res<Time>) {
    // In FixedUpdate, Time is automatically Time<Fixed>
    let dt = time.delta_secs(); // Always the fixed timestep
}
```

## Overview

`FixedUpdate` runs at a fixed rate independent of frame rate:
- Default: 64 Hz (every ~15.6ms)
- May run 0, 1, or multiple times per frame
- `Time` resource becomes `Time<Fixed>` in FixedUpdate systems

### When to Use FixedUpdate

| Use Case | Why |
|----------|-----|
| Physics simulation | Deterministic, reproducible results |
| AI decision making | Consistent behavior regardless of FPS |
| Networking | Fixed tick rate for sync |
| Game rules/logic | Predictable outcomes |
| Procedural animation | Smooth interpolation |

### When NOT to Use FixedUpdate

| Use Case | Why |
|----------|-----|
| Rendering | Should match display refresh |
| User input | Responsiveness matters |
| Audio triggers | Timing precision needed |
| UI updates | Visual smoothness |

## How Fixed Timestep Works

1. Each frame, real elapsed time accumulates in an "overstep"
2. When overstep >= timestep, FixedUpdate runs and overstep decreases
3. This may happen 0, 1, or many times per frame

```
Frame 1 (16ms): overstep=16ms → runs once (16ms > 15.6ms), overstep=0.4ms
Frame 2 (16ms): overstep=16.4ms → runs once, overstep=0.8ms
Frame 3 (32ms lag): overstep=32.8ms → runs twice, overstep=1.6ms
Frame 4 (8ms): overstep=9.6ms → doesn't run (< 15.6ms)
```

## Configuration

### Setting Timestep

```rust
use bevy::time::Fixed;
use std::time::Duration;

// By frequency (Hz)
app.insert_resource(Time::<Fixed>::from_hz(120.0)); // 120 updates/second

// By duration
app.insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(8))); // ~125 Hz

// By seconds
app.insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0)); // 60 Hz
```

### Choosing a Timestep

| Rate | Timestep | Use Case |
|------|----------|----------|
| 30 Hz | 33.3ms | Simple games, mobile |
| 60 Hz | 16.7ms | Standard games |
| 64 Hz | 15.6ms | Bevy default |
| 120 Hz | 8.3ms | Fast-paced games |
| 144+ Hz | <7ms | Competitive games |

**Trade-offs**:
- Higher rate = more CPU usage, smoother physics
- Lower rate = less CPU, potential jitter

## Time<Fixed> in Systems

In FixedUpdate systems, `Time` is automatically `Time<Fixed>`:

```rust
fn physics_system(time: Res<Time>) {
    // These are fixed values in FixedUpdate:
    let dt = time.delta_secs();        // Always = timestep
    let elapsed = time.elapsed_secs(); // Fixed time since start
}
```

### Time<Fixed> Methods

| Method | Returns |
|--------|---------|
| `timestep()` | The fixed timestep duration |
| `delta()` | Same as timestep() in FixedUpdate |
| `delta_secs()` | Timestep as f32 seconds |
| `elapsed()` | Total fixed time elapsed |
| `elapsed_secs()` | Total elapsed as f32 |
| `overstep()` | Accumulated time toward next step |
| `overstep_fraction()` | Overstep as 0.0-1.0 fraction |

### Accessing Both Time Types

```rust
fn interpolation_system(
    fixed_time: Res<Time<Fixed>>,
    real_time: Res<Time<Real>>,
) {
    let alpha = fixed_time.overstep_fraction();
    // Use alpha for visual interpolation
}
```

## Common Patterns

### Physics Movement

```rust
fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        // dt is constant in FixedUpdate
        transform.translation += velocity.0 * time.delta_secs();
    }
}

app.add_systems(FixedUpdate, apply_velocity);
```

### Accumulator Pattern

For actions that should happen at intervals:

```rust
#[derive(Resource)]
struct SpawnTimer(f32);

fn spawn_enemies(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
) {
    timer.0 += time.delta_secs();

    while timer.0 >= 2.0 {
        timer.0 -= 2.0;
        commands.spawn(EnemyBundle::default());
    }
}

app.add_systems(FixedUpdate, spawn_enemies);
```

### Visual Interpolation

Smooth rendering between fixed updates:

```rust
// In FixedUpdate: update physics position
fn physics_update(mut query: Query<(&Velocity, &mut PhysicsPosition)>) {
    for (vel, mut pos) in &mut query {
        pos.previous = pos.current;
        pos.current += vel.0 * TIMESTEP;
    }
}

// In Update: interpolate for rendering
fn interpolate_rendering(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&PhysicsPosition, &mut Transform)>,
) {
    let alpha = fixed_time.overstep_fraction();

    for (physics, mut transform) in &mut query {
        transform.translation = physics.previous.lerp(physics.current, alpha);
    }
}

app.add_systems(FixedUpdate, physics_update);
app.add_systems(Update, interpolate_rendering);
```

## Fixed Timestep Schedules

Like frame schedules, fixed timestep has multiple schedules:

| Schedule | Purpose |
|----------|---------|
| `FixedFirst` | Before all fixed systems |
| `FixedPreUpdate` | Engine prep |
| `FixedUpdate` | **Main fixed logic** |
| `FixedPostUpdate` | Engine cleanup |
| `FixedLast` | After all fixed systems |

```rust
app.add_systems(FixedPreUpdate, prepare_physics);
app.add_systems(FixedUpdate, run_physics);
app.add_systems(FixedPostUpdate, sync_transforms);
```

## Important Caveats

### Not Real-Time

Fixed timestep does NOT run at actual fixed intervals:
- May run multiple times in one frame (lag catch-up)
- May not run at all in a frame (fast frames)
- Audio/network packets won't be evenly spaced in real time

### Respects Virtual Time

Fixed timestep respects time scaling and pausing:
- `time.set_relative_speed(0.5)` slows fixed updates
- Pausing virtual time pauses FixedUpdate

For real-time behavior, use `Time<Real>` instead.

### Input Lag

Reading input in FixedUpdate can feel laggy:
- Input may be processed up to one timestep late
- Consider reading input in Update, queuing for FixedUpdate

```rust
// Better pattern: queue input in Update
fn queue_input(input: Res<ButtonInput<KeyCode>>, mut queue: ResMut<InputQueue>) {
    if input.just_pressed(KeyCode::Space) {
        queue.push(InputAction::Jump);
    }
}

// Process in FixedUpdate
fn process_input(mut queue: ResMut<InputQueue>, mut player: Query<&mut Velocity>) {
    for action in queue.drain() {
        if let InputAction::Jump = action {
            // Apply jump
        }
    }
}

app.add_systems(Update, queue_input);
app.add_systems(FixedUpdate, process_input);
```

## Common Mistakes

### Frame-rate dependent physics
```rust
// Wrong: in Update, dt varies with frame rate
app.add_systems(Update, |time: Res<Time>, mut q: Query<&mut Transform>| {
    for mut t in &mut q {
        t.translation.y -= 9.8 * time.delta_secs(); // Inconsistent!
    }
});

// Correct: in FixedUpdate, dt is constant
app.add_systems(FixedUpdate, |time: Res<Time>, mut q: Query<&mut Transform>| {
    for mut t in &mut q {
        t.translation.y -= 9.8 * time.delta_secs(); // Consistent
    }
});
```

### Assuming single execution per frame
```rust
// Wrong: assumes FixedUpdate runs exactly once
fn count_frames(mut counter: ResMut<FrameCounter>) {
    counter.0 += 1; // May increment multiple times per frame!
}

// Correct: count fixed ticks, not frames
fn count_ticks(mut counter: ResMut<TickCounter>) {
    counter.0 += 1; // This is intentional per-tick counting
}
```

### Visual updates in FixedUpdate
```rust
// Wrong: jerky visuals at low FPS
app.add_systems(FixedUpdate, update_sprite_animation);

// Correct: smooth visuals in Update
app.add_systems(Update, update_sprite_animation);
```

### Ignoring overstep for interpolation
```rust
// Wrong: snappy movement between fixed positions
fn render(query: Query<(&PhysicsPos, &mut Transform)>) {
    for (physics, mut transform) in &query {
        transform.translation = physics.current; // Jerky!
    }
}

// Correct: interpolate using overstep
fn render(
    fixed_time: Res<Time<Fixed>>,
    query: Query<(&PhysicsPos, &mut Transform)>,
) {
    let alpha = fixed_time.overstep_fraction();
    for (physics, mut transform) in &query {
        transform.translation = physics.previous.lerp(physics.current, alpha);
    }
}
```
