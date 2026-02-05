# Cameras

`Camera2d` enables 2D rendering with orthographic projection. In Bevy 0.15+, it uses required components to automatically insert necessary camera components.

## Quick Reference

```rust
// Basic 2D camera
commands.spawn(Camera2d);

// With custom position
commands.spawn((
    Camera2d,
    Transform::from_xyz(100.0, 200.0, 0.0),
));

// With marker for easy querying
#[derive(Component)]
#[require(Camera2d)]
struct MainCamera;

commands.spawn(MainCamera);
```

## Required Components (Auto-Inserted)

When you spawn `Camera2d`, Bevy adds:
- `Camera` - Core configuration (viewport, order, clear color)
- `Projection` - Contains `OrthographicProjection`
- `Frustum` - View frustum for culling
- `Transform` / `GlobalTransform` - Positioning
- `Visibility` - Visibility control

## OrthographicProjection

Controls how the 2D world maps to the screen.

### Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `near` | `f32` | `-1000.0` | Near clipping plane |
| `far` | `f32` | `1000.0` | Far clipping plane |
| `viewport_origin` | `Vec2` | `(0.5, 0.5)` | Normalized pivot (0-1) |
| `scaling_mode` | `ScalingMode` | `WindowSize` | How projection scales |
| `scale` | `f32` | `1.0` | Zoom multiplier |

### Customizing Projection

```rust
use bevy::render::camera::{OrthographicProjection, ScalingMode};

commands.spawn((
    Camera2d,
    Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical { viewport_height: 720.0 },
        scale: 1.0,
        near: -1000.0,
        far: 1000.0,
        ..OrthographicProjection::default_2d()
    }),
));
```

## ScalingMode

Controls how the projection maps to the viewport.

| Variant | Description | Use Case |
|---------|-------------|----------|
| `WindowSize` | 1:1 pixel mapping (default) | Pixel-perfect games |
| `Fixed { width, height }` | Fixed world units, may stretch | Exact game area |
| `FixedVertical { viewport_height }` | Constant height, width adjusts | Side-scrollers |
| `FixedHorizontal { viewport_width }` | Constant width, height adjusts | Top-down games |
| `AutoMin { min_width, min_height }` | Maintains ratio, min visible area | Responsive design |
| `AutoMax { max_width, max_height }` | Maintains ratio, max visible area | Responsive design |

### Examples

```rust
// Pixel art (1:1 mapping)
ScalingMode::WindowSize

// Fixed game world
ScalingMode::Fixed { width: 1920.0, height: 1080.0 }

// Consistent vertical view
ScalingMode::FixedVertical { viewport_height: 720.0 }

// Ensure minimum visible area
ScalingMode::AutoMin { min_width: 800.0, min_height: 600.0 }
```

## Zoom

Use `OrthographicProjection::scale` for zoom:

```rust
fn zoom_camera(
    mut projection: Single<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll: EventReader<MouseWheel>,
) {
    for event in scroll.read() {
        let zoom_speed = 0.1;
        let delta = -event.y * zoom_speed;
        projection.scale = (projection.scale * (1.0 + delta)).clamp(0.1, 10.0);
    }
}
```

- **Lower scale** = zoomed in (objects appear larger)
- **Higher scale** = zoomed out (objects appear smaller)

## Multiple Cameras

Use `Camera::order` to control render sequence:

```rust
use bevy::render::camera::ClearColorConfig;

// Main camera (renders first)
commands.spawn((
    Camera2d,
    Camera {
        order: 0,
        ..default()
    },
));

// Overlay camera (renders on top)
commands.spawn((
    Camera2d,
    Camera {
        order: 1,
        clear_color: ClearColorConfig::None, // Don't clear
        ..default()
    },
));
```

## Viewport

Restrict camera to a rectangular area (split-screen, minimap):

```rust
use bevy::render::camera::Viewport;

// Left half of screen
commands.spawn((
    Camera2d,
    Camera {
        viewport: Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(640, 720),
            ..default()
        }),
        ..default()
    },
));

// Minimap in corner
commands.spawn((
    Camera2d,
    Camera {
        viewport: Some(Viewport {
            physical_position: UVec2::new(1024, 0),
            physical_size: UVec2::new(256, 256),
            ..default()
        }),
        order: 1,
        clear_color: ClearColorConfig::None,
        ..default()
    },
));
```

## Coordinate Conversion

### World to Screen

```rust
fn world_to_screen(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, transform) = *camera;
    let world_pos = Vec3::new(100.0, 50.0, 0.0);

    if let Ok(screen_pos) = camera.world_to_viewport(transform, world_pos) {
        info!("Screen: {:?}", screen_pos);
    }
}
```

### Screen to World (2D)

```rust
fn screen_to_world(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
) {
    let (camera, transform) = *camera;

    if let Some(cursor) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(transform, cursor) {
            info!("World: {:?}", world_pos);
        }
    }
}
```

### Mouse World Position Resource

```rust
#[derive(Resource, Default)]
pub struct MouseWorldPosition(pub Option<Vec2>);

fn update_mouse_position(
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    window: Single<&Window>,
    mut mouse_pos: ResMut<MouseWorldPosition>,
) {
    let (camera, transform) = *camera;
    mouse_pos.0 = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor).ok());
}
```

## Camera Following

### Smooth Follow with smooth_nudge

```rust
use bevy::math::StableInterpolate;

const DECAY_RATE: f32 = 3.0;

fn camera_follow(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let target = Vec3::new(
        player.translation.x,
        player.translation.y,
        camera.translation.z, // Preserve camera Z
    );

    camera.translation.smooth_nudge(&target, DECAY_RATE, time.delta_secs());
}
```

**Decay rate values:**
- `2.0` - Smooth, relaxed
- `5.0` - Snappy, responsive
- `f32::INFINITY` - Instant snap

### Camera with Bounds

```rust
fn camera_follow_bounded(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let (min_x, max_x, min_y, max_y) = (-500.0, 500.0, -300.0, 300.0);

    let target = Vec3::new(
        player.translation.x.clamp(min_x, max_x),
        player.translation.y.clamp(min_y, max_y),
        camera.translation.z,
    );

    camera.translation.smooth_nudge(&target, 3.0, time.delta_secs());
}
```

### Camera with Deadzone

```rust
fn camera_deadzone(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let deadzone = Vec2::new(100.0, 75.0);
    let diff = player.translation.truncate() - camera.translation.truncate();
    let mut target = camera.translation.truncate();

    if diff.x.abs() > deadzone.x {
        target.x = player.translation.x - deadzone.x * diff.x.signum();
    }
    if diff.y.abs() > deadzone.y {
        target.y = player.translation.y - deadzone.y * diff.y.signum();
    }

    let target_3d = target.extend(camera.translation.z);
    camera.translation.smooth_nudge(&target_3d, 5.0, time.delta_secs());
}
```

## Clear Color

```rust
use bevy::render::camera::ClearColorConfig;

// Custom color
Camera {
    clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.15)),
    ..default()
}

// Use global ClearColor resource
Camera {
    clear_color: ClearColorConfig::Default,
    ..default()
}

// Don't clear (for overlays)
Camera {
    clear_color: ClearColorConfig::None,
    ..default()
}

// Set global clear color
commands.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2)));
```

## HDR and Bloom

```rust
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;

commands.spawn((
    Camera2d,
    Camera { hdr: true, ..default() },
    Tonemapping::TonyMcMapface,
    Bloom::NATURAL,
));
```

For bloom to work, use HDR colors (values > 1.0):

```rust
Sprite { color: Color::srgb(3.0, 1.5, 0.5), ..default() } // Will glow
```

## RenderLayers

Control which cameras render which entities:

```rust
use bevy::render::view::RenderLayers;

// Camera sees layers 0 and 1
commands.spawn((Camera2d, RenderLayers::from_layers(&[0, 1])));

// Minimap camera only sees layer 2
commands.spawn((Camera2d, RenderLayers::layer(2)));

// Entity on layer 0 (default)
commands.spawn((Sprite::default(), RenderLayers::layer(0)));

// Entity visible to both cameras
commands.spawn((Sprite::default(), RenderLayers::from_layers(&[0, 2])));
```

Default: Entities without `RenderLayers` are on layer 0. Cameras without it only render layer 0.

## Common Mistakes

### Coordinate Conversion Timing

```rust
// Run after transforms are propagated
app.add_systems(
    PostUpdate,
    update_mouse_position.after(TransformSystem::TransformPropagate)
);
```

### Forgetting Camera Z

```rust
// Wrong: overwrites camera Z
camera.translation = player.translation;

// Correct: preserve camera Z
camera.translation = Vec3::new(
    player.translation.x,
    player.translation.y,
    camera.translation.z,
);
```

### Zoom Direction

```rust
// Counterintuitive: larger scale = zoomed OUT
projection.scale = 2.0; // Objects appear smaller
projection.scale = 0.5; // Objects appear larger
```
