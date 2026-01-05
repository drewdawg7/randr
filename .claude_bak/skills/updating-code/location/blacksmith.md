# Blacksmith System

## Overview

The Blacksmith allows players to upgrade equipment stats, improve item quality, and smelt ores into bars using fuel.

## Key Files

| File | Purpose |
|------|---------|
| `src/location/blacksmith/definition.rs` | `Blacksmith` struct and upgrade methods |
| `src/location/blacksmith/enums.rs` | `BlacksmithError`, `BlacksmithUpgradeResult` |
| `src/commands/blacksmith.rs` | Command handlers for UI actions |
| `src/item/definition.rs` | `Item::upgrade()`, `Item::upgrade_quality()` |
| `src/item/enums.rs` | `UpgradeResult`, `ItemQuality`, `ItemError` |

## Return Types

All blacksmith operations return meaningful results instead of `Result<(), Error>`:

### Item Upgrades

```rust
// Item::upgrade() returns stat delta and new level
pub struct UpgradeResult {
    pub new_level: i32,           // The new upgrade level (num_upgrades)
    pub stat_increases: StatSheet, // Delta of stats that increased
}

// Blacksmith::upgrade_item() wraps this with gold info
pub struct BlacksmithUpgradeResult {
    pub upgrade: UpgradeResult,
    pub gold_spent: i32,
}
```

### Quality Upgrades

```rust
// Both return the new quality level
Item::upgrade_quality() -> Result<ItemQuality, ItemError>
Blacksmith::upgrade_item_quality() -> Result<ItemQuality, BlacksmithError>
```

### Other Operations

```rust
// Returns new fuel amount
Blacksmith::add_fuel() -> Result<i32, BlacksmithError>

// Returns the smelted item
Blacksmith::smelt_and_give() -> Result<Item, BlacksmithError>
```

## Upgrade Cost Formula

```rust
cost = (num_upgrades + 1) * base_upgrade_cost * quality_multiplier
```

Quality multipliers: Poor=0.9, Normal=1.0, Improved=1.1, WellForged=1.2, Masterworked=1.3, Mythic=1.4

## Stat Upgrade Formula

Each upgrade increases base stats by 10% (minimum 1 point):

```rust
increase = max(1, round(base_value * 0.1))
```

## Error Types

`BlacksmithError` variants:
- `MaxUpgradesReached` - Item at blacksmith's max_upgrades limit
- `NotEnoughGold` - Player lacks gold for upgrade cost
- `NoUpgradeStones` - Quality upgrade requires QualityUpgradeStone item
- `NotEquipment` - Can only upgrade equipment items
- `NotEnoughFuel` - Smelting requires fuel (from coal)
- `NoFuel` - No coal in inventory to add fuel
- `ItemNotFound` - UUID doesn't match any player item
- `InventoryFull` - Cannot add smelted item to inventory

## Command Integration

Commands in `src/commands/blacksmith.rs` use the return values for messages:

```rust
// Shows: "Upgraded Iron Sword to +3 (-30 gold)"
match blacksmith.upgrade_player_item(&mut player, uuid) {
    Ok(result) => format!("Upgraded {} to +{} (-{} gold)",
        item_name, result.upgrade.new_level, result.gold_spent),
    ...
}

// Shows: "Iron Sword is now Improved quality!"
match blacksmith.upgrade_player_item_quality(&mut player, uuid) {
    Ok(quality) => format!("{} is now {} quality!", item_name, quality.display_name()),
    ...
}
```

## Tests

Comprehensive tests in `src/location/blacksmith/tests.rs` covering:
- Upgrade cost calculations with quality multipliers
- Gold deduction on success
- Stat increases per upgrade
- Error conditions (max upgrades, insufficient gold, non-equipment)
- Blacksmith max_upgrades vs item max_upgrades precedence
