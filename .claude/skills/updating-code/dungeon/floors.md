# Floors

Floor system in `src/dungeon/floor/`.

## FloorType (`floor_type.rs`)

Reusable floor templates:

```rust
pub enum FloorType {
    BasicDungeonFloor,  // Standard dungeon
    CaveFloor,          // Cave tileset
    TmxCaveFloor,       // TMX-based cave
}

impl FloorType {
    fn spawn_table(&self, is_final: bool) -> SpawnTable;
    fn layout_id(&self, is_final: bool) -> LayoutId;
    fn tile_scale(&self) -> f32;  // 1.0 for dungeon, 2.0 for cave
}
```

## FloorId (`definitions.rs`)

Predefined floor variants using `define_data!` macro:

```rust
let spec = FloorId::GoblinCave1.spec();
spec.name;        // "Goblin Cave - Floor 1"
spec.layout_id;   // LayoutId for tile grid
spec.spawn_table; // Entity spawns

// FloorId also has floor_type() for rendering
FloorId::HomeFloor.floor_type(); // FloorType::TmxCaveFloor
FloorId::GoblinCave1.floor_type(); // FloorType::BasicDungeonFloor
```

## FloorInstance (`generated.rs`)

Runtime floor representation:

```rust
pub enum FloorInstance {
    Fixed(FloorId),           // Predefined
    Generated(GeneratedFloor), // Runtime-created
}

impl FloorInstance {
    fn layout_id(&self) -> LayoutId;
    fn spawn_table(&self) -> SpawnTable;
    fn name(&self) -> String;
}
```

## WeightedFloorPool (`weighted_pool.rs`)

Random floor selection for generated dungeons:

```rust
let pool = WeightedFloorPool::new()
    .add(FloorType::BasicDungeonFloor, 80)
    .add(FloorType::CaveFloor, 20);

let floor_type = pool.select(&mut rng);
```

## Adding a New Floor

1. Add variant to `FloorId` in `definitions.rs`:
```rust
MyFloor {
    name: "My Floor",
    layout_id: LayoutId::MyLayout,
    spawn_table: SpawnTable::new()
        .mob(MobId::Goblin, 5)
        .chest(1..=2),
}
```

2. Register in `DungeonPlugin` (see [mod.md](mod.md))
