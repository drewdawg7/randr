# Combat ECS System

## Overview

Combat uses direct action-based attacks. The player attacks mobs in real-time by pressing a key, which spawns a hitbox that damages nearby enemies.

Key events:
- `DealDamage` - sent for observation (damage numbers, combat log)
- `EntityDied` - sent when mob or player dies

## Key Files

| File | Purpose |
|------|---------|
| `src/combat/action_combat.rs` | ActionCombatPlugin with attack systems |
| `src/combat/action.rs` | Attack hitbox components |
| `src/combat/systems/` | Attack input, collision, damage, death rewards, hitbox cleanup |
| `src/combat/events.rs` | Combat events (DealDamage, EntityDied) |
| `src/combat/system.rs` | Combat helper functions |
| `src/mob/components.rs` | ECS components for mob combat data |
| `src/mob/bundle.rs` | MobCombatBundle for spawning mobs |
| `src/dungeon/systems/mob_health_bar.rs` | Health bars displayed above dungeon mobs |

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
- `Health { current, max }` - Health tracking with `take_damage()` and `is_alive()`
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

// Process player defeat (5% gold penalty, restore health)
process_player_defeat(stats: &mut StatSheet, gold: &mut PlayerGold)

// Apply victory rewards (gold with goldfind, XP)
apply_victory_rewards_direct(
    stats: &StatSheet,
    inventory: &Inventory,
    gold: &mut PlayerGold,
    progression: &mut Progression,
    base_gold: i32,
    base_xp: i32,
) -> VictoryRewards
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

## Combat Flow (Action-Based)

1. Player presses attack key in dungeon
2. `handle_attack_input` spawns `AttackHitbox` entity at player position
3. `check_attack_collisions` detects overlap with mob entities
4. `apply_attack_damage` applies damage to mobs, sends `DealDamage` event
5. If mob health reaches 0:
   - `handle_death_rewards` applies rewards (gold, XP, loot)
   - `PendingVictory` resource inserted for results modal
   - Mob entity despawned
6. `cleanup_expired_hitboxes` removes hitbox after duration

## Combat Events

Located in `src/combat/events.rs`:

```rust
#[derive(Event)]
pub struct DealDamage {
    pub target: Entity,
    pub amount: i32,
    pub source_name: String,
}

#[derive(Event)]
pub struct EntityDied {
    pub entity: Entity,
    pub is_player: bool,
}
```

- `DealDamage` - observation event for damage numbers, combat log
- `EntityDied` - triggers death handling and UI transitions

## Defense Formula

Combat uses diminishing returns for defense:
- Formula: `reduction = defense / (defense + 50)`
- 50 defense = 50% damage reduction
- 100 defense = 67% damage reduction
- Defense never reaches 100% reduction

```rust
// In system.rs
pub fn calculate_damage_reduction(defense: i32) -> f64
pub fn apply_defense(raw_damage: i32, defense: i32) -> i32
```
