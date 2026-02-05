# Floors

Floor system in `src/dungeon/floor/`.

## FloorType (`floor_type.rs`)

Floor rendering configuration:

```rust
pub enum FloorType {
    CaveFloor,
}

impl FloorType {
    fn tileset_key(&self) -> SpriteSheetKey;
    fn tile_scale(&self) -> f32;
}
```

All floors use TMX maps for tile definitions. The `tile_scale()` returns `2.0` (cave tiles are 32x32).

## FloorId (`definitions.rs`)

Predefined floor variants using `define_data!` macro:

```rust
let spec = FloorId::MainDungeon1.spec();
spec.name;
spec.path;  // Direct TMX path, e.g., "maps/cave_floor.tmx"
spec.spawn_table;
```

Current floor variants:
- `HomeFloor` - Player home with merchant NPC
- `MainDungeon1`, `MainDungeon2`, `MainDungeon3` - Main dungeon floors

## Adding a New Floor

1. Add variant to `FloorId` in `definitions.rs`:
```rust
MyFloor {
    name: "My Floor",
    path: "maps/cave_floor.tmx",
    spawn_table: SpawnTable::new()
        .mob(MobId::Goblin, 5)
        .chest(1..=2),
}
```

2. Register in `DungeonPlugin`:
```rust
DungeonPlugin::new()
    .location(LocationId::MyLocation)
        .floor(FloorId::MyFloor)
    .build()
```

## DungeonConfig (`config.rs`)

Simple floor list configuration:

```rust
pub struct DungeonConfig {
    floors: Vec<FloorId>,
}

impl DungeonConfig {
    pub fn new(floors: Vec<FloorId>) -> Self;
    pub fn floors(&self) -> &[FloorId];
    pub fn floor_count(&self) -> usize;
}
```

## DungeonState (`state.rs`)

Tracks current dungeon progress:

```rust
pub struct DungeonState {
    pub current_location: Option<LocationId>,
    pub floor_index: usize,
    pub floor_sequence: Vec<FloorId>,
    pub layout: Option<DungeonLayout>,
    pub player_pos: GridPosition,
    pub player_size: GridSize,
}

state.enter_dungeon(location, &registry);
state.current_floor();
state.advance_floor(&registry);
state.load_floor_layout();
state.exit_dungeon();
```
