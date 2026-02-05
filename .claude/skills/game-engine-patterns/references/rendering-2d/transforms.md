# Transforms

`Transform` describes an entity's position, rotation, and scale relative to its parent. `GlobalTransform` is the computed world-space position.

## Quick Reference

```rust
// Position only
Transform::from_xyz(100.0, 50.0, 0.0)
Transform::from_translation(Vec3::new(100.0, 50.0, 0.0))

// Rotation only (2D - around Z axis)
Transform::from_rotation(Quat::from_rotation_z(1.57))

// Scale only
Transform::from_scale(Vec3::splat(2.0))

// Combined
Transform::from_xyz(100.0, 50.0, 0.0)
    .with_rotation(Quat::from_rotation_z(0.5))
    .with_scale(Vec3::splat(2.0))
```

## Transform Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `translation` | `Vec3` | `Vec3::ZERO` | Position; Z controls 2D layering |
| `rotation` | `Quat` | `Quat::IDENTITY` | Rotation as quaternion |
| `scale` | `Vec3` | `Vec3::ONE` | Scale on each axis |

## Creating Transforms

### Constructors

```rust
// Position
Transform::from_xyz(x, y, z)
Transform::from_translation(Vec3::new(x, y, z))

// Rotation
Transform::from_rotation(quat)

// Scale
Transform::from_scale(Vec3::splat(2.0))
Transform::from_scale(Vec3::new(2.0, 1.0, 1.0))

// From matrix
Transform::from_matrix(mat4)

// Identity (no transform)
Transform::IDENTITY
Transform::default()
```

### Builder Pattern

```rust
Transform::from_xyz(100.0, 50.0, 0.0)
    .with_rotation(Quat::from_rotation_z(0.5))
    .with_scale(Vec3::splat(2.0))

Transform::IDENTITY
    .with_translation(Vec3::new(100.0, 50.0, 0.0))
    .with_scale(Vec3::ONE * 2.0)
```

## Rotation

### Creating Rotations (Quat)

```rust
// 2D rotation (most common) - around Z axis
Quat::from_rotation_z(radians)

// 3D rotations
Quat::from_rotation_x(radians)
Quat::from_rotation_y(radians)
Quat::from_axis_angle(Vec3::Y, radians)

// From Euler angles
Quat::from_euler(EulerRot::ZYX, z, y, x)

// 2D direction-to-direction
Quat::from_rotation_arc_2d(from_dir, to_dir)
```

### Rotation Methods

```rust
// Mutate in world/parent space
transform.rotate(Quat::from_rotation_z(0.1));
transform.rotate_z(0.1);  // Shorthand for 2D

// Mutate in local space
transform.rotate_local(quat);
transform.rotate_local_z(0.1);

// Rotate around a point
transform.rotate_around(point, rotation);
```

### Rotation Interpolation

```rust
// Spherical linear interpolation (smooth, follows shortest path)
let rotation = quat_a.slerp(quat_b, t);

// Linear interpolation (faster but less accurate)
let rotation = quat_a.lerp(quat_b, t);
```

## Looking at Targets

```rust
// Builder methods
Transform::from_xyz(0.0, 0.0, 0.0)
    .looking_at(target_pos, Vec3::Y)  // Look at position

Transform::from_xyz(0.0, 0.0, 0.0)
    .looking_to(direction, Vec3::Y)   // Look in direction

// Mutating methods
transform.look_at(target_pos, Vec3::Y);
transform.look_to(direction, Vec3::Y);
```

## Direction Vectors

Get the direction the entity is facing:

```rust
// Local axis directions (unit vectors)
transform.forward()  // -Z direction
transform.back()     // +Z direction
transform.right()    // +X direction
transform.left()     // -X direction
transform.up()       // +Y direction
transform.down()     // -Y direction

// Or using local_* methods
transform.local_x()  // Right
transform.local_y()  // Up
transform.local_z()  // Back
```

Note: In Bevy, `-Z` is forward.

## Transform vs GlobalTransform

| Aspect | Transform | GlobalTransform |
|--------|-----------|-----------------|
| Space | Local (relative to parent) | World (absolute) |
| Mutability | Directly mutable | Read-only (computed) |
| Use | Setting position | Reading world position |
| Update | Immediate | After `PostUpdate` |

### Reading GlobalTransform

```rust
fn read_world_position(query: Query<(&Transform, &GlobalTransform)>) {
    for (local, global) in &query {
        let world_pos: Vec3 = global.translation();
        let world_rot: Quat = global.rotation();
        let world_scale: Vec3 = global.scale();

        // Transform point to world space
        let world_point = global.transform_point(local_point);
    }
}
```

### Converting Between Types

```rust
// GlobalTransform to Transform
let transform: Transform = global_transform.compute_transform();

// Transform to GlobalTransform (treating local as world)
let global: GlobalTransform = transform.into();

// Compute local transform for new parent
let new_local: Transform = global_transform.reparented_to(&new_parent_global);
```

## Transform Hierarchy

When entities have parent-child relationships:

```rust
// Parent at (100, 0, 0)
commands.spawn(Transform::from_xyz(100.0, 0.0, 0.0))
    .with_children(|parent| {
        // Child at local (10, 0, 0) = world (110, 0, 0)
        parent.spawn(Transform::from_xyz(10.0, 0.0, 0.0));
    });
```

**Propagation timing:** `GlobalTransform` updates during `PostUpdate`. Changes after that have a one-frame delay.

### TransformHelper

For mid-frame world transform calculations:

```rust
fn system(helper: TransformHelper, query: Query<Entity>) {
    for entity in &query {
        if let Ok(global) = helper.compute_global_transform(entity) {
            // Use computed global transform
        }
    }
}
```

## 2D Patterns

### Z-Ordering (Depth)

In 2D, the Z component controls draw order:

```rust
Transform::from_xyz(0.0, 0.0, 0.0)   // Background
Transform::from_xyz(0.0, 0.0, 1.0)   // Ground
Transform::from_xyz(0.0, 0.0, 10.0)  // Entities
Transform::from_xyz(0.0, 0.0, 100.0) // UI overlay
```

Higher Z = rendered on top.

### 2D Rotation

Only rotate around Z axis:

```rust
// Create rotated
Transform::from_xyz(0.0, 0.0, 0.0)
    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4))

// Rotate over time
transform.rotate_z(angular_velocity * time.delta_secs());
```

### Common 2D Spawn Pattern

```rust
commands.spawn((
    Sprite::from_image(texture),
    Transform::from_xyz(x, y, z_layer),
));
```

## Interpolation

### Translation/Scale (Vec3)

```rust
// Linear interpolation (t: 0.0 to 1.0)
let pos = start_pos.lerp(end_pos, t);
```

### Rotation (Quat)

```rust
// Spherical linear interpolation (preferred)
let rot = start_rot.slerp(end_rot, t);
```

### Full Transform

```rust
fn interpolate(a: &Transform, b: &Transform, t: f32) -> Transform {
    Transform {
        translation: a.translation.lerp(b.translation, t),
        rotation: a.rotation.slerp(b.rotation, t),
        scale: a.scale.lerp(b.scale, t),
    }
}
```

### Smooth Movement (smooth_nudge)

```rust
use bevy::math::StableInterpolate;

// Exponential decay toward target
position.smooth_nudge(&target, decay_rate, time.delta_secs());
```

## Transform Math

### Point Transformation

```rust
// Local to world
let world_point = transform.transform_point(local_point);
let world_point = global_transform.transform_point(local_point);

// Multiply transforms
let combined = parent.mul_transform(child);
```

### Matrix Conversion

```rust
// To matrix
let matrix: Mat4 = transform.to_matrix();
let affine: Affine3A = transform.compute_affine();

// From matrix
let transform = Transform::from_matrix(matrix);
```

## Vec3 Constants

```rust
Vec3::ZERO      // (0, 0, 0)
Vec3::ONE       // (1, 1, 1)
Vec3::X         // (1, 0, 0)
Vec3::Y         // (0, 1, 0)
Vec3::Z         // (0, 0, 1)
Vec3::NEG_X     // (-1, 0, 0)
Vec3::NEG_Y     // (0, -1, 0)
Vec3::NEG_Z     // (0, 0, -1)

// Constructors
Vec3::new(x, y, z)
Vec3::splat(v)  // (v, v, v)
```

## This Codebase

### DepthSorting System

From `src/dungeon/state.rs` - automatic Y-based z-ordering:

```rust
#[derive(Resource)]
pub struct DepthSorting {
    pub factor: f32,    // 1.0 / max_world_y
    pub camera_z: f32,
}

impl DepthSorting {
    pub fn entity_z(&self, y: f32) -> f32 {
        y * self.factor
    }
}
```

Usage:
```rust
let z = depth.entity_z(pos.y);
let world_pos = Vec3::new(pos.x, pos.y, z);
```

## Common Mistakes

### Modifying GlobalTransform

```rust
// Wrong: GlobalTransform is computed, not directly mutable
global_transform.translation = ...;

// Correct: modify Transform instead
transform.translation = ...;
```

### Forgetting Hierarchy Effects

```rust
// Child transform is relative to parent
// Setting child to (0, 0, 0) puts it at parent's position, not world origin
```

### Scale Affects Children

```rust
// Parent scale affects all children
// Scale parent by 2.0 = children appear 2x larger and twice as far from parent
```

### Rotation Units

```rust
// Quat methods use radians, not degrees
Quat::from_rotation_z(90.0)  // Wrong: 90 radians!
Quat::from_rotation_z(90.0_f32.to_radians())  // Correct
Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)  // Also correct
```
