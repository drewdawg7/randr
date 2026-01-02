# Stats System

## Overview

The stats system provides a generic way to track numerical attributes (health, attack, defense, etc.) for entities like Player, Mob, Item, and Rock.

## Key Files

| File | Purpose |
|------|---------|
| `src/stats/definition.rs` | Core structures: `StatSheet`, `StatInstance`, `StatType` enum |
| `src/stats/traits.rs` | `HasStats` trait with convenience methods |
| `src/stats/mod.rs` | Re-exports |

## StatType Enum

Five stat types defined in `src/stats/definition.rs`:

| Variant | Usage |
|---------|-------|
| `Health` | HP for combat entities (Player, Mob, Rock) |
| `Attack` | Damage dealing (Player, Mob, Equipment) |
| `Defense` | Damage reduction (Player, Mob, Equipment) |
| `GoldFind` | Equipment bonus for gold drops |
| `Mining` | Equipment bonus for mining damage |

## StatSheet

A `HashMap<StatType, StatInstance>` wrapper providing safe accessors.

Key methods:
- `with(stat_type, value)` - Builder pattern
- `value(stat_type)` - Get current value (returns 0 if missing)
- `max_value(stat_type)` - Get max value
- `increase_stat(stat_type, amount)` - Increase current
- `decrease_stat(stat_type, amount)` - Decrease current (floors at 0)
- `increase_stat_max(stat_type, amount)` - Increase max
- `decrease_stat_max(stat_type, amount)` - Decrease max

## StatInstance

Stores a single stat with both current and max values:

```rust
pub struct StatInstance {
    pub stat_type: StatType,
    pub current_value: i32,
    pub max_value: i32,
}
```

Created via `StatType::instance(base_value)` which sets both current and max to base_value.

## HasStats Trait

Trait for entities with stats. Provides convenience methods:

| Method | Description |
|--------|-------------|
| `stats()` / `stats_mut()` | Access StatSheet |
| `value(stat_type)` | Get current value |
| `max_value(stat_type)` | Get max value |
| `inc(stat_type, amount)` | Increase current |
| `dec(stat_type, amount)` | Decrease current |
| `inc_max(stat_type, amount)` | Increase max |
| `dec_max(stat_type, amount)` | Decrease max |
| `hp()` / `max_hp()` | Health shortcuts |
| `attack()` | Attack shortcut |
| `defense()` | Defense shortcut |
| `goldfind()` | GoldFind shortcut |
| `mining()` | Mining shortcut |

## Implementations

| Entity | Stats Used | Notes |
|--------|------------|-------|
| Player | Health, Attack, Defense, GoldFind, Mining | All stats, equipment adds bonuses |
| Mob | Health, Attack, Defense | Defense now included |
| Item | Attack, Defense, GoldFind, Mining | Equipment bonuses only |
| Rock | Health | Only HP for mining |

## Player Effective Stats

Player has methods that combine base stats with equipment bonuses:

| Method | Calculation |
|--------|-------------|
| `effective_attack()` | Via `DealsDamage` trait, includes equipment |
| `effective_defense()` | Via `Combatant` trait, `defense() + equipment` |
| `effective_mining()` | `mining() + equipment mining` |
| `effective_goldfind()` | `goldfind() + equipment goldfind` |

## Combat Integration

Stats flow into combat via traits in `src/combat/traits.rs`:
- `DealsDamage` - Uses `attack()` to calculate damage range
- `Combatant` - Uses `defense()` for damage reduction
- `IsKillable` - Uses `hp()` for health tracking

Defense reduction formula: `reduction = defense / (defense + 50)`
