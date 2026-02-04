---
name: game-engine-patterns
description: Best practices for Bevy ECS, bevy_ecs_tiled tilemaps, and Avian2d physics. Use when working with components, resources, queries, systems, events, observers, triggers, state management, UI nodes, sprites, texture atlases, animations, asset loading, plugins, bundles, Tiled maps, tile properties, physics colliders, rigid bodies, collision layers, sensors, or spatial queries.
---

# Game Engine Patterns

Best practices for Bevy 0.18, bevy_ecs_tiled, and Avian2d based on patterns in this codebase.

## Bevy

### ECS Core
Components, resources, queries, systems, commands, SystemParam.
See [references/bevy-ecs-core.md](references/bevy-ecs-core.md)

### Events & Communication
Events, observers, triggers.
See [references/bevy-events.md](references/bevy-events.md)

### Scheduling
Run conditions, system ordering.
See [references/bevy-scheduling.md](references/bevy-scheduling.md)

### State Management
States, state transitions.
See [references/bevy-state.md](references/bevy-state.md)

### UI
Nodes & layout, flexbox, text, images.
See [references/bevy-ui.md](references/bevy-ui.md)

### Rendering 2D
Sprites, texture atlases, cameras, transforms, z-ordering.
See [references/bevy-rendering.md](references/bevy-rendering.md)

### Animation
Sprite animation, timers.
See [references/bevy-animation.md](references/bevy-animation.md)

### Assets
Asset loading, handles, custom loaders.
See [references/bevy-assets.md](references/bevy-assets.md)

### Input
Keyboard input.
See [references/bevy-input.md](references/bevy-input.md)

### Plugins
Plugin structure, configuration.
See [references/bevy-plugins.md](references/bevy-plugins.md)

### Bundles
Custom bundles, required components.
See [references/bevy-bundles.md](references/bevy-bundles.md)

## Tiled (bevy_ecs_tiled)

[Main Book](https://adrien-bon.github.io/bevy_ecs_tiled/) | [API Docs](https://docs.rs/bevy_ecs_tiled/latest/bevy_ecs_tiled/)

### Map Loading & Events
TiledMapAsset, spawning maps, MapCreated, ColliderCreated.
See [references/tiled-loading.md](references/tiled-loading.md)

### Tile Properties & Data
Custom properties, property-to-component mapping, tilemap components, coordinate conversion.
See [references/tiled-properties.md](references/tiled-properties.md)

### Physics Integration
TiledPhysicsPlugin setup.
See [references/tiled-physics.md](references/tiled-physics.md)

## Avian Physics (avian2d)

[API Docs](https://docs.rs/avian2d/latest/avian2d/)

### Rigid Bodies
RigidBody types, locked axes.
See [references/avian-rigid-bodies.md](references/avian-rigid-bodies.md)

### Colliders
Collider shapes, compound colliders.
See [references/avian-colliders.md](references/avian-colliders.md)

### Collision Detection
Collision layers, collision events, sensors.
See [references/avian-collision.md](references/avian-collision.md)

### Spatial Queries
Point queries.
See [references/avian-queries.md](references/avian-queries.md)

### Movement
Linear velocity.
See [references/avian-movement.md](references/avian-movement.md)

### Configuration
Physics plugins setup, gravity, length units, debug plugin.
See [references/avian-config.md](references/avian-config.md)
