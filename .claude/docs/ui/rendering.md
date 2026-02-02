# Rendering

## Camera

Camera spawned in `src/plugins/game.rs` with 2x zoom:

```rust
commands.spawn((
    Camera2d,
    Projection::Orthographic(OrthographicProjection {
        scale: 0.5,
        ..OrthographicProjection::default_2d()
    }),
));
```

- `scale: 0.5` = 2x zoom (world appears twice as large)
- Coordinate systems unaffected (physics, tile positions, entity spawning)
- Bevy UI uses separate camera, so HUD/modals are not zoomed

## Window

Window configured in `src/main.rs`:
- Resolution: 1280x720
- `ImagePlugin::default_nearest()` for pixel-perfect sprites
