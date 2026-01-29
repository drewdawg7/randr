# Combat ECS System

This document describes the ECS-based combat system that replaces the trait-based approach.

## Overview

Combat operates on Bevy ECS components and direct resource access instead of:
- Player struct reconstruction (cloning resources into a struct, operating, writing back)
- PlayerGuard RAII pattern (which defeats Bevy's change detection)
- Owned Mob structs in combat (instead of Entity references)
- Generic trait-bound functions (instead of Bevy systems)

## Key Files

| File | Purpose |
|------|---------|
| `src/combat/system.rs` | Combat helper functions for direct resource access |
| `src/mob/components.rs` | ECS components for mob combat data |
| `src/mob/bundle.rs` | MobCombatBundle for spawning mobs |
| `src/ui/screens/fight_modal/input.rs` | Combat input handling with entity queries |
| `src/ui/screens/fight_modal/render.rs` | Health bar updates from entity components |

## Mob ECS Components

When spawning dungeon mobs, include `MobCombatBundle::from_mob_id(mob_id)`:

```rust
use crate::mob::MobCombatBundle;

// In spawn_entities:
EntityRenderData::AnimatedMob { mob_id } => {
    layer.spawn((
        marker,
        DungeonMobSprite { mob_id },
        MobCombatBundle::from_mob_id(mob_id),
        ZIndex(pos.y as i32),
        entity_node,
    ));
}
```

The bundle includes:
- `MobMarker(MobId)` - Identifies the mob type
- `Health { current, max }` - Health tracking
- `CombatStats { attack, defense }` - Combat values
- `GoldReward(i32)` - Gold dropped on death
- `XpReward(i32)` - XP given on death
- `MobLootTable(LootTable)` - Loot table for drops
- `DeathProcessed(bool)` - Guard against double rewards

## Combat Helper Functions

Located in `src/combat/system.rs`:

### Direct Resource Helpers
```rust
// Get player's attack range from resources
player_attack_value(stats: &StatSheet, inventory: &Inventory) -> Attack

// Get player's effective defense from resources
player_effective_defense(stats: &StatSheet, inventory: &Inventory) -> i32

// Apply damage directly to StatSheet
player_take_damage(stats: &mut StatSheet, amount: i32)

// Get magic/gold find bonuses
player_effective_magicfind(stats: &StatSheet, inventory: &Inventory) -> i32
player_effective_goldfind(stats: &StatSheet, inventory: &Inventory) -> i32
```

### Entity Component Combat Functions
```rust
// Player attacks a mob entity
player_attacks_entity(
    player_name: &str,
    player_stats: &StatSheet,
    player_inventory: &Inventory,
    mob_name: &str,
    mob_health: &mut Health,
    mob_combat_stats: &CombatStats,
) -> AttackResult

// Mob entity attacks the player
entity_attacks_player(
    mob_name: &str,
    mob_combat_stats: &CombatStats,
    player_name: &str,
    player_stats: &mut StatSheet,
    player_inventory: &Inventory,
) -> AttackResult
```

## Combat Flow (Fight Modal)

1. Player walks into mob, triggering `FightModalMob` resource insertion
2. `FightModalMob` stores `mob_id`, `entity`, `pos` (no owned Mob struct)
3. Fight modal input system queries mob entity's components:
   ```rust
   mob_query: Query<(
       &MobMarker,
       &mut Health,
       &CombatStats,
       &GoldReward,
       &XpReward,
       &MobLootTable,
       &mut DeathProcessed,
   )>
   ```
4. Combat uses `player_attacks_entity` / `entity_attacks_player`
5. On victory, rewards applied with `apply_victory_rewards_direct`
6. On defeat, gold penalty with `process_player_defeat`

## FightModalMob Resource

The resource no longer holds a `Mob` struct:

```rust
#[derive(Resource)]
pub struct FightModalMob {
    pub mob_id: MobId,       // For sprite lookup
    pub pos: GridPosition,   // For occupancy cleanup
    pub entity: Entity,      // For despawning and component queries
}
```

