# Bevy Rendering 2D

Overview of Bevy 0.18 2D rendering for sprites, texture atlases, cameras, transforms, and z-ordering.

## Quick Navigation

| Topic | Use When | Reference |
|-------|----------|-----------|
| Sprites | Rendering 2D images/textures | [rendering-2d/sprites.md](rendering-2d/sprites.md) |
| Texture Atlases | Sprite sheets, frame-based sprites | [rendering-2d/texture-atlases.md](rendering-2d/texture-atlases.md) |
| Cameras | 2D camera setup, viewport, zoom | [rendering-2d/cameras.md](rendering-2d/cameras.md) |
| Transforms | Position, rotation, scale | [rendering-2d/transforms.md](rendering-2d/transforms.md) |
| Z-Ordering | Layering, depth sorting | [rendering-2d/z-ordering.md](rendering-2d/z-ordering.md) |

## Quick Reference

```rust
// Basic sprite
commands.spawn(Sprite::from_image(asset_server.load("sprite.png")));

// Sprite from texture atlas
commands.spawn(Sprite::from_atlas_image(
    texture,
    TextureAtlas { layout: layout_handle, index: 0 },
));

// 2D Camera
commands.spawn(Camera2d);

// Positioned sprite with z-ordering
commands.spawn((
    Sprite::from_image(texture),
    Transform::from_xyz(100.0, 50.0, 10.0), // z=10 for layering
));
```

## Key Types

| Type | Module | Purpose |
|------|--------|---------|
| `Sprite` | `bevy::sprite` | 2D image rendering component |
| `TextureAtlas` | `bevy::sprite` | Sprite sheet frame selection |
| `TextureAtlasLayout` | `bevy::image` | Defines sprite sheet regions |
| `Camera2d` | `bevy::core_pipeline` | 2D camera marker |
| `Transform` | `bevy::transform` | Local position/rotation/scale |
| `GlobalTransform` | `bevy::transform` | World-space transform (computed) |
| `Anchor` | `bevy::sprite` | Sprite origin point |

## This Codebase

This codebase uses several rendering patterns:

- **SpriteSheet wrapper** for ergonomic atlas access (see `src/assets/sprites.rs`)
- **DepthSorting resource** for Y-based z-ordering (see `src/dungeon/state.rs`)
- **SpriteMarker trait** for declarative sprite population (see `src/ui/sprite_marker.rs`)
- **GameSprites resource** for centralized sprite sheet access

## Migration Notes (Bevy 0.15+)

- `SpriteBundle` removed - spawn `Sprite` directly
- `TextureAtlasSprite` removed - use `Sprite::from_atlas_image()`
- Required components (`Transform`, `Visibility`) are auto-inserted
