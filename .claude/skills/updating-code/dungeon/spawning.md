# Entity Spawning

Spawn system in `src/dungeon/spawn.rs` and `spawn_rules.rs`.

## SpawnTable (`spawn.rs`)

High-level declarative API:

```rust
let spawns = SpawnTable::new()
    .mob(MobId::Goblin, 5)    // Weight 5
    .mob(MobId::Slime, 3)     // Weight 3
    .mob_count(3..=5)         // Total mobs
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
| `chest(range)` | Chest count (1×1) |
| `stairs(range)` | Stairs count (1×1) |
| `rock(range)` | Rock count (random RockType) |
| `forge(range)` / `forge_chance(f64)` | Forge spawning |
| `anvil(range)` / `anvil_chance(f64)` | Anvil spawning |
| `npc(id, range)` / `npc_chance(id, f64)` | NPC spawning |
| `guaranteed_mob(id, count)` | Always spawn these |

## Spawn Order

1. Chests
2. Stairs
3. Rocks
4. Forges (count, then probability)
5. Anvils (count, then probability)
6. NPCs (count, then probability)
7. Guaranteed mobs
8. Weighted mobs

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
