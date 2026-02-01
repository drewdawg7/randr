# Floor Entity Spawning

ECS-based entity spawning system that queries tile components to place dungeon entities.

## Architecture

### FloorSpawnConfig Resource (`src/dungeon/systems/spawning.rs`)

Holds spawn parameters for the current floor:

```rust
#[derive(Resource)]
pub struct FloorSpawnConfig {
    pub chest: RangeInclusive<u32>,
    pub stairs: RangeInclusive<u32>,
    pub rock: RangeInclusive<u32>,
    pub forge: RangeInclusive<u32>,
    pub anvil: RangeInclusive<u32>,
    pub forge_chance: Option<f64>,
    pub anvil_chance: Option<f64>,
    pub weighted_mobs: Vec<MobSpawnEntry>,
    pub mob_count: RangeInclusive<u32>,
    pub guaranteed_mobs: Vec<(MobId, u32)>,
    pub npc_spawns: Vec<(MobId, RangeInclusive<u32>)>,
    pub npc_chances: Vec<(MobId, f64)>,
}
```

### SpawnTable (`src/dungeon/spawn.rs`)

Builder pattern for defining spawn rules in floor definitions:

```rust
SpawnTable::new()
    .chest(1..=2)
    .stairs(1..=1)
    .mob(MobId::Goblin, 5)
    .mob(MobId::Slime, 3)
    .mob_count(3..=4)
    .guaranteed_mob(MobId::DwarfKing, 1)
    .forge_chance(0.33)
    .npc(MobId::Merchant, 1..=1)
    .build()
```

Convert to config with `.to_config()` or `FloorSpawnConfig::from(&table)`.

## Spawning Flow

1. **Floor Load**: `DungeonState::load_floor_layout()` returns `FloorSpawnConfig`
2. **Resource Insert**: Caller inserts `FloorSpawnConfig` as resource
3. **Tilemap Load**: bevy_ecs_tiled loads TMX, creates tile entities with `can_have_entity` component
4. **Map Created Event**: `TiledEvent<MapCreated>` fires when map finishes loading
5. **Entity Spawn**: `on_map_created` observer queries tiles, spawns entities with `DungeonEntityMarker`
6. **Visual Setup**: `add_entity_visuals` observer adds sprites/transforms when `DungeonEntityMarker` is added
7. **Occupancy Track**: `track_entity_occupancy` observer marks grid cells as occupied

## Key Systems

### on_map_created (`src/dungeon/systems/spawning.rs`)

Observer triggered by `TiledEvent<MapCreated>`:

```rust
fn on_map_created(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    spawn_tiles: Query<&TilePos, With<can_have_entity>>,
    config: Option<Res<FloorSpawnConfig>>,
    occupancy: Option<ResMut<GridOccupancy>>,
)
```

Queries tiles with `can_have_entity`, spawns entities with `DungeonEntityMarker`.

### add_entity_visuals (`src/ui/screens/dungeon/spawn.rs`)

Observer triggered by `Add<DungeonEntityMarker>`:

```rust
fn add_entity_visuals(
    trigger: On<Add, DungeonEntityMarker>,
    mut commands: Commands,
    query: Query<&DungeonEntityMarker>,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
    tile_sizes: Option<Res<TileSizes>>,
)
```

Adds visual components (sprites, transforms) based on entity type.

## Spawn Order

Entities spawn in this order (in on_map_created):
1. Chests
2. Stairs
3. Rocks
4. Forges (count range, then probability)
5. Anvils (count range, then probability)
6. NPCs (count range, then probability)
7. Guaranteed mobs
8. Weighted mobs

## Tile Components

Spawning queries tiles with these components (from Tiled custom properties):

- `can_have_entity` - Tile can have a dungeon entity spawned on it
- `can_spawn_player` - Tile can be player spawn point
- `is_solid` - Tile blocks movement
- `is_door` - Tile is a door

## File Structure

```
src/dungeon/
    spawn.rs              # SpawnTable builder
    systems/spawning.rs   # FloorSpawnConfig, on_map_created observer
src/ui/screens/dungeon/
    spawn.rs              # add_entity_visuals observer
```

## Related

- [mod.md](mod.md) - Dungeon module overview
- [entities.md](entities.md) - DungeonEntity enum
- [spawning.md](spawning.md) - SpawnTable usage in floor definitions
