# Combat System

## Overview

The combat system handles damage calculation between combatants (Player, Mob). It uses variable damage with percentage-based defense.

## Key Files

| File | Purpose |
|------|---------|
| `src/combat/attack.rs` | `Attack` struct with min/max damage range |
| `src/combat/traits.rs` | Combat traits: `Combatant`, `DealsDamage`, `IsKillable`, `Named`, `HasGold`, `DropsGold` |
| `src/combat/system.rs` | Core `attack()` function, defense calculations, combat flow functions |
| `src/combat/result.rs` | `AttackResult`, `CombatRounds`, death result structs |
| `src/combat/state.rs` | `ActiveCombat`, `CombatPhase` for UI state |
| `src/combat/tests.rs` | Combat system tests |

## Damage Calculation

### Formula
```
raw_damage = random(attack.min_damage, attack.max_damage)
reduction = defense / (defense + 50)
final_damage = raw_damage * (1 - reduction)
```

### Attack Variance
- Attack range derived from Attack stat with ±25% variance
- Example: 20 Attack = 15-25 damage range

### Defense (Diminishing Returns)
| Defense | Reduction |
|---------|-----------|
| 0       | 0%        |
| 25      | 33%       |
| 50      | 50%       |
| 100     | 67%       |
| 200     | 80%       |

Defense constant `K=50` in `src/combat/system.rs:10`

## Traits

### DealsDamage (`src/combat/traits.rs:28-48`)
```rust
pub trait DealsDamage: HasStats {
    const ATTACK_VARIANCE: f64 = 0.25;
    fn get_attack(&self) -> Attack;       // Returns damage range
    fn effective_attack(&self) -> i32;    // Average for display
}
```

### Combatant (`src/combat/traits.rs:50-64`)
Extends `Named + IsKillable + DealsDamage`. Required for entities in combat.
- `effective_defense()` - defense value for damage reduction
- `effective_health()` - current health
- `increase_health()` - heal

### IsKillable (`src/combat/traits.rs:4-26`)
For entities that can die.
- `take_damage(amount)` - applies damage
- `is_alive()` - health > 0
- `on_death()` - returns death result (gold, xp, loot for mobs)

## Implementations

### Player (`src/entities/player/traits.rs`)
- `DealsDamage`: Attack range from (base + equipment) with ±25% variance
- `Combatant`: Defense includes equipment bonus

### Mob (`src/entities/mob/traits.rs`)
- `DealsDamage`: Uses default implementation (stat-based with variance)
- `Combatant`: Uses base stats only

## Combat Flow Functions

| Function | File:Line | Purpose |
|----------|-----------|---------|
| `attack()` | `system.rs:28` | Single attack: roll damage, apply defense, deal damage |
| `player_attack_step()` | `system.rs:118` | Player attacks mob, updates combat phase |
| `enemy_attack_step()` | `system.rs:134` | Mob attacks player, updates combat phase |
| `process_victory()` | `system.rs:150` | Award gold (with goldfind), XP, loot |
| `process_defeat()` | `system.rs:170` | Player loses 5% gold, health restored |

## Tuning Parameters

| Parameter | Location | Default | Effect |
|-----------|----------|---------|--------|
| `DEFENSE_CONSTANT` | `system.rs:10` | 50.0 | Higher = more defense needed for same reduction |
| `ATTACK_VARIANCE` | `traits.rs:32` | 0.25 | ±25% damage variance |
