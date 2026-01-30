# Spawn Rules

Composable entity placement system for dungeon layouts.

## Core Concepts

The `SpawnRule` trait enables modular entity spawning. Each rule encapsulates specific placement logic, and rules can be composed via `ComposedSpawnRules`.

### SpawnRule Trait (`src/dungeon/spawn_rules.rs`)

```rust
pub trait SpawnRule {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32;
}
```

Returns the count of entities spawned.

### SpawnRuleKind Enum

Enum-based wrapper for type-safe composition without `dyn`:

```rust
pub enum SpawnRuleKind {
    Chest(ChestSpawner),
    Stairs(StairsSpawner),
    Rock(RockSpawner),
    CraftingStation(CraftingStationSpawner),
    ProbabilityCraftingStation(ProbabilityCraftingStationSpawner),
    Npc(NpcSpawner),
    ProbabilityNpc(ProbabilityNpcSpawner),
    GuaranteedMob(GuaranteedMobSpawner),
    WeightedMob(WeightedMobSpawner),
    FixedPosition(FixedPositionSpawner),
}
```

### ComposedSpawnRules

Applies rules in sequence, summing spawn counts:

```rust
use crate::dungeon::{ComposedSpawnRules, SpawnRuleKind, ChestSpawner, StairsSpawner};

let rules = ComposedSpawnRules::new()
    .add(SpawnRuleKind::Chest(ChestSpawner::new(1..=2)))
    .add(SpawnRuleKind::Stairs(StairsSpawner::new(1..=1)));

let total = rules.apply(&mut layout, &mut rng);
```

## Individual Spawners

### ChestSpawner
Spawns chests with random variants (0-3).

```rust
ChestSpawner::new(count: RangeInclusive<u32>)
```

### StairsSpawner
Spawns stairs that advance the player to the next floor.

```rust
StairsSpawner::new(count: RangeInclusive<u32>)
```

### RockSpawner
Spawns rocks with random types (Copper, Coal, Tin).

```rust
RockSpawner::new(count: RangeInclusive<u32>)
```

### CraftingStationSpawner
Spawns crafting stations (Forge, Anvil) with a count range.

```rust
CraftingStationSpawner::new(station_type: CraftingStationType, count: RangeInclusive<u32>)
```

### ProbabilityCraftingStationSpawner
Spawns 1 crafting station with a given probability (0.0 to 1.0).

```rust
ProbabilityCraftingStationSpawner::new(station_type: CraftingStationType, probability: f64)
```

### NpcSpawner
Spawns NPCs (non-combat, blocks movement) with a count range.

```rust
NpcSpawner::new(mob_id: MobId, count: RangeInclusive<u32>)
```

### ProbabilityNpcSpawner
Spawns 1 NPC with a given probability (0.0 to 1.0).

```rust
ProbabilityNpcSpawner::new(mob_id: MobId, probability: f64)
```

### GuaranteedMobSpawner
Spawns an exact count of a specific mob type.

```rust
GuaranteedMobSpawner::new(mob_id: MobId, count: u32)
```

### WeightedMobSpawner
Spawns mobs using weighted random selection.

```rust
WeightedMobSpawner::new()
    .mob(MobId::Goblin, 5)   // Weight 5
    .mob(MobId::Slime, 3)    // Weight 3
    .count(3..=5)            // Spawn 3-5 total
```

### FixedPositionSpawner
Spawns an entity at a specific grid position.

```rust
FixedPositionSpawner::new(pos: GridPosition, entity: DungeonEntity)
```

## SpawnTable Integration

`SpawnTable` uses `ComposedSpawnRules` internally:

```rust
// SpawnTable builds rules internally
let table = SpawnTable::new()
    .chest(1..=2)
    .stairs(1..=1)
    .mob(MobId::Goblin, 3)
    .mob_count(2..=4);

table.apply(&mut layout, &mut rng);
```

### Probability-Based Spawning

For spawning with exact probability control (e.g., "33% chance to spawn 1"), use probability methods:

```rust
SpawnTable::new()
    .rock(0..=4)                        // 0-4 rocks (uniform random)
    .forge_chance(0.33)                 // 33% chance of 1 forge
    .anvil_chance(0.33)                 // 33% chance of 1 anvil
    .npc_chance(MobId::Merchant, 0.33)  // 33% chance of 1 merchant
```

**When to use count ranges vs probability:**
- Count range (`0..=4`): Each value in range has equal probability (e.g., 20% for 0, 20% for 1, etc.)
- Probability (`0.33`): Exactly 33% chance to spawn 1, 67% chance to spawn 0

The `SpawnTable` maintains backward compatibility while delegating to individual spawners.

## Adding New Spawner Types

1. Create the spawner struct with configuration:
```rust
#[derive(Clone)]
pub struct TrapSpawner {
    count: RangeInclusive<u32>,
    trap_type: TrapType,
}
```

2. Implement `SpawnRule`:
```rust
impl SpawnRule for TrapSpawner {
    fn apply(&self, layout: &mut DungeonLayout, rng: &mut impl Rng) -> u32 {
        let count = rng.gen_range(self.count.clone());
        // ... spawn logic
        spawned
    }
}
```

3. Add variant to `SpawnRuleKind`:
```rust
pub enum SpawnRuleKind {
    // ... existing variants
    Trap(TrapSpawner),
}
```

4. Update `SpawnRuleKind::apply()` match arm.

5. Export from `mod.rs`.

## File Structure

```
src/dungeon/
    spawn.rs        # SpawnTable (high-level API)
    spawn_rules.rs  # SpawnRule trait + spawner implementations
```

## Spawn Order

When using `SpawnTable`, entities spawn in this order:
1. Chests (count range)
2. Stairs (count range)
3. Rocks (count range)
4. Forges (count range)
5. Forges (probability)
6. Anvils (count range)
7. Anvils (probability)
8. NPCs (count range)
9. NPCs (probability)
10. Guaranteed mobs
11. Weighted mobs

Each spawner uses `layout.spawn_areas(size)` to find valid positions, ensuring entities never overlap.

## Related

- [mod.md](mod.md) - Dungeon module overview
- [entities.md](entities.md) - DungeonEntity enum and spawning
