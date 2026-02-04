# Bevy ECS Core

Overview of Bevy 0.18 ECS fundamentals used in this codebase.

## Quick Navigation

| Topic | Use When | Reference |
|-------|----------|-----------|
| Components | Storing data on entities | [ecs-core/components.md](ecs-core/components.md) |
| Resources | Global shared state | [ecs-core/resources.md](ecs-core/resources.md) |
| Queries | Reading/writing entity data | [ecs-core/queries.md](ecs-core/queries.md) |
| Systems | Game logic functions | [ecs-core/systems.md](ecs-core/systems.md) |
| Commands | Deferred entity/resource changes | [ecs-core/commands.md](ecs-core/commands.md) |
| SystemParam | Grouping system parameters | [ecs-core/system-param.md](ecs-core/system-param.md) |

## ECS Architecture

**Entity**: A unique ID (internally a u64). Entities have no behavior - they're just identifiers that components attach to.

**Component**: Data attached to entities. Components are plain structs with `#[derive(Component)]`. All game state lives in components.

**System**: Functions that operate on components via queries. Systems are where game logic lives. They run each frame (or on specific schedules).

**Resource**: Global singletons not attached to any entity. Use for configuration, caches, or state that doesn't belong to a specific entity.

## This Codebase

This codebase uses several patterns built on ECS:

- **Message trait** for events (see [bevy-events.md](bevy-events.md))
- **Observers** for component lifecycle reactions (see [bevy-events.md](bevy-events.md))
- **State-based run conditions** for screen-specific systems (see [bevy-state.md](bevy-state.md))
- **`#[instrument]`** for tracing on all systems

## Quick Examples

**Marker component for filtering:**
```rust
#[derive(Component)]
struct Player;

// In system: Query<&Health, With<Player>>
```

**Resource for global state:**
```rust
#[derive(Resource, Default)]
struct GameScore(u32);

// In system: Res<GameScore> or ResMut<GameScore>
```

**System with run condition:**
```rust
app.add_systems(Update,
    handle_combat
        .run_if(on_message::<PlayerAttack>)
        .run_if(in_state(AppState::Dungeon))
);
```

**Commands for deferred changes:**
```rust
fn spawn_enemy(mut commands: Commands) {
    commands.spawn((Enemy, Health::new(100)));
}
```
