# Entity Spawning

Spawn system in `src/dungeon/systems/spawning.rs`.

## Observer Registration

The `on_map_created` observer is attached directly to the TiledMap entity using `.observe()`:

```rust
commands
    .spawn((TiledMap(map_handle), ChildOf(floor_root)))
    .observe(on_map_created);
```

**Important**: Use entity-scoped observers (`.observe()`) not global observers (`.add_observer()`). Global observers receive propagated events from the entire hierarchy, causing duplicate spawning when TiledMap is a child of FloorRoot.

## SpawnTable (`src/dungeon/spawn.rs`)

High-level declarative API:

```rust
let spawns = SpawnTable::new()
    .mob(MobId::Goblin, 5)
    .mob(MobId::Slime, 3)
    .mob_count(3..=5)
    .chest(1..=2)
    .stairs(1..=1)
    .rock(0..=4)
    .forge_chance(0.33)
    .anvil_chance(0.33)
    .npc_chance(MobId::Merchant, 0.33)
    .guaranteed_mob(MobId::Boss, 1);

spawns.apply(&mut layout, &mut rng);
```

## Methods

| Method | Effect |
|--------|--------|
| `mob(id, weight)` | Add weighted mob (size from MobSpec) |
| `mob_count(range)` | Set mob spawn count |
| `chest(range)` | Chest count (1x1) |
| `stairs(range)` | Stairs count (1x1) |
| `rock(range)` | Rock count (random RockType) |
| `forge(range)` / `forge_chance(f64)` | Forge spawning |
| `anvil(range)` / `anvil_chance(f64)` | Anvil spawning |
| `npc(id, range)` / `npc_chance(id, f64)` | NPC spawning |
| `guaranteed_mob(id, count)` | Always spawn these |

## Spawn Order

1. **Doors** (automatic, from `is_door` tiles)
2. Chests
3. Stairs
4. Rocks
5. Forges (count, then probability)
6. Anvils (count, then probability)
7. NPCs (count, then probability)
8. Guaranteed mobs
9. Weighted mobs

## Door Spawning

Doors are spawned automatically from tiles with `is_door` property:
- `spawn_doors()` runs first in `on_map_created`
- Queries tiles with `is_door` component
- Spawns invisible `DungeonEntity::Door` entities with Sensor colliders
- Door visual comes from the tilemap itself (cave opening tile)

## Position Selection

Each spawner uses `layout.spawn_areas(size)`:
- Checks all cells are walkable floor
- Checks no overlap with existing entities
- Entities never overlap

## Custom Spawners (`spawn_rules.rs`)

For advanced control, implement `SpawnRule`:

```rust
pub trait SpawnRule {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32;
}
```

Built-in spawners:
- `ChestSpawner`, `StairsSpawner`, `RockSpawner`
- `WeightedMobSpawner`, `GuaranteedMobSpawner`
- `CraftingStationSpawner`, `NpcSpawner`
- `FixedPositionSpawner` (exact coordinates)

See [spawn-rules.md](spawn-rules.md) for implementation details.
