# Z-Ordering

In 2D games, z-ordering determines which sprites appear in front of others. Bevy uses the Z component of `Transform::translation` for sprite layering.

## Quick Reference

```rust
// Layer via translation Z
Transform::from_xyz(x, y, 0.0)   // Background
Transform::from_xyz(x, y, 10.0)  // Entities
Transform::from_xyz(x, y, 100.0) // Foreground

// Higher Z = rendered on top
```

## How Z-Ordering Works

In Bevy's 2D rendering:
- Sprites are sorted by their `GlobalTransform.translation.z` value
- **Higher Z values** render **on top** of lower values
- Sprites with the same Z value have undefined order (may flicker)

```rust
// Background layer (drawn first)
commands.spawn((
    Sprite::from_image(background_texture),
    Transform::from_xyz(0.0, 0.0, 0.0),
));

// Entity layer (drawn after background)
commands.spawn((
    Sprite::from_image(player_texture),
    Transform::from_xyz(100.0, 50.0, 10.0),
));

// UI/foreground layer (drawn last)
commands.spawn((
    Sprite::from_image(ui_overlay),
    Transform::from_xyz(0.0, 0.0, 100.0),
));
```

## Layer Constants Pattern

Define layer constants for consistency:

```rust
pub mod layers {
    pub const BACKGROUND: f32 = 0.0;
    pub const GROUND: f32 = 1.0;
    pub const SHADOWS: f32 = 5.0;
    pub const ENTITIES: f32 = 10.0;
    pub const EFFECTS: f32 = 15.0;
    pub const UI_WORLD: f32 = 50.0;
}

// Usage
Transform::from_xyz(x, y, layers::ENTITIES)
```

## Y-Based Sorting (Isometric/Top-Down)

For games where entities lower on screen should appear in front:

### Simple Y-to-Z Conversion

```rust
fn update_z_from_y(mut query: Query<&mut Transform, With<YSortedSprite>>) {
    for mut transform in &mut query {
        // Invert Y so lower Y = higher Z (in front)
        transform.translation.z = -transform.translation.y;
    }
}
```

### Normalized Y-to-Z (This Codebase Pattern)

From `src/dungeon/state.rs`:

```rust
#[derive(Resource, Clone, Copy)]
pub struct DepthSorting {
    pub factor: f32,    // 1.0 / max_world_y
    pub camera_z: f32,
}

impl DepthSorting {
    pub fn from_map(map_height_tiles: f32, tile_size: f32) -> Self {
        let max_world_y = (map_height_tiles * tile_size).max(1.0);
        Self {
            factor: 1.0 / max_world_y,
            camera_z: 10.0,
        }
    }

    #[inline]
    pub fn entity_z(&self, y: f32) -> f32 {
        y * self.factor
    }
}
```

Usage:
```rust
fn spawn_entity(depth: Res<DepthSorting>) {
    let z = depth.entity_z(pos.y);
    let world_pos = Vec3::new(pos.x, pos.y, z);
    commands.spawn((sprite, Transform::from_translation(world_pos)));
}
```

Benefits:
- Z values stay in predictable range (0.0 to 1.0)
- Works regardless of map size
- Entities at same Y have same Z (no fighting)

## Sub-Layers Within a Layer

For entities that need ordering within the same Y position:

```rust
const ENTITY_BASE_Z: f32 = 10.0;
const ENTITY_SHADOW_OFFSET: f32 = -0.1;
const ENTITY_BODY_OFFSET: f32 = 0.0;
const ENTITY_WEAPON_OFFSET: f32 = 0.1;

fn spawn_character(y_based_z: f32) {
    // Shadow slightly behind body
    let shadow_z = ENTITY_BASE_Z + y_based_z + ENTITY_SHADOW_OFFSET;
    // Body at base layer
    let body_z = ENTITY_BASE_Z + y_based_z + ENTITY_BODY_OFFSET;
    // Weapon slightly in front
    let weapon_z = ENTITY_BASE_Z + y_based_z + ENTITY_WEAPON_OFFSET;
}
```

## Hierarchy and Z-Ordering

Child Z values are **added** to parent Z:

```rust
// Parent at Z=10
commands.spawn(Transform::from_xyz(0.0, 0.0, 10.0))
    .with_children(|parent| {
        // Child at local Z=1 = world Z=11
        parent.spawn(Transform::from_xyz(0.0, 0.0, 1.0));
        // Child at local Z=-1 = world Z=9
        parent.spawn(Transform::from_xyz(0.0, 0.0, -1.0));
    });
```

## UI Z-Index (Different System)

For Bevy UI (`Node`), use `ZIndex` instead of Transform:

```rust
// Relative to siblings
commands.spawn((Node { ... }, ZIndex(1)));

// Global, escapes hierarchy
commands.spawn((Node { ... }, GlobalZIndex(100)));
```

See [bevy-ui.md](../bevy-ui.md) for UI-specific z-ordering.

## Dynamic Z Updates

Update Z when Y changes:

```rust
fn update_entity_depth(
    depth: Res<DepthSorting>,
    mut query: Query<&mut Transform, (With<DynamicEntity>, Changed<Transform>)>,
) {
    for mut transform in &mut query {
        let base_z = depth.entity_z(transform.translation.y);
        transform.translation.z = base_z;
    }
}
```

**Caution:** `Changed<Transform>` triggers on any Transform change, including Z changes. Guard against infinite loops or use a separate Y-position component.

## RenderLayers (Alternative)

For completely separate rendering passes (not Z-based):

```rust
use bevy::render::view::RenderLayers;

// Main camera sees layer 0
commands.spawn((Camera2d, RenderLayers::layer(0)));

// UI camera sees layer 1, renders after
commands.spawn((
    Camera2d,
    Camera { order: 1, clear_color: ClearColorConfig::None, ..default() },
    RenderLayers::layer(1),
));

// Entity on layer 0 (game world)
commands.spawn((Sprite::default(), RenderLayers::layer(0)));

// Entity on layer 1 (UI overlay)
commands.spawn((Sprite::default(), RenderLayers::layer(1)));
```

## Best Practices

1. **Use constants** for layer values to maintain consistency

2. **Leave gaps** between layers for flexibility:
   ```rust
   const BACKGROUND: f32 = 0.0;
   const ENTITIES: f32 = 10.0;  // Gap of 10 for sub-layers
   const UI: f32 = 100.0;
   ```

3. **Normalize Y-to-Z** to keep values predictable

4. **Don't rely on undefined order** - sprites at same Z may flicker

5. **Use hierarchy** for related sprites (character + shadow)

## Common Mistakes

### Same Z for Different Entities

```rust
// Wrong: undefined rendering order
Transform::from_xyz(0.0, 0.0, 10.0)  // Player
Transform::from_xyz(50.0, 0.0, 10.0) // Enemy (same Z!)

// Correct: use Y-based or offset
Transform::from_xyz(0.0, player_y, depth.entity_z(player_y))
Transform::from_xyz(50.0, enemy_y, depth.entity_z(enemy_y))
```

### Z Values Too Close

```rust
// Wrong: floating point imprecision
Transform::from_xyz(0.0, 0.0, 10.0)
Transform::from_xyz(0.0, 0.0, 10.0001)  // May not reliably sort

// Correct: use meaningful offsets
Transform::from_xyz(0.0, 0.0, 10.0)
Transform::from_xyz(0.0, 0.0, 10.1)
```

### Forgetting Camera Z

```rust
// If camera is at Z=0, negative Z sprites won't render
// OrthographicProjection default: near=-1000, far=1000

// Safe: keep sprites within camera's frustum
// Sprites with Z < near or Z > far won't render
```

### Mixing Transform Z and ZIndex

```rust
// ZIndex is for UI (Node), not world sprites
// Don't use both on same entity

// For world sprites: use Transform.translation.z
// For UI nodes: use ZIndex
```
