# Item & Equipment System

## Overview

Items follow the registry pattern with specs and spawned instances. Equipment items can be equipped to slots and provide stat bonuses.

## Key Files

| File | Purpose |
|------|---------|
| `src/item/item_id.rs` | `ItemId` enum - all item identifiers |
| `src/item/enums.rs` | `ItemType`, `EquipmentType`, `ItemQuality` enums |
| `src/item/definition.rs` | `Item` struct - spawned item instances |
| `src/item/spec/definition.rs` | `ItemSpec` struct - static item definitions |
| `src/item/spec/specs.rs` | Static `Lazy<ItemSpec>` definitions |
| `src/item/spec/traits.rs` | `SpawnFromSpec` and `RegistryDefaults` impls |
| `src/inventory/enums.rs` | `EquipmentSlot` enum |

## Equipment Slots

Defined in `src/inventory/enums.rs`:

```rust
pub enum EquipmentSlot {
    Weapon,   // Primary weapon (swords, daggers)
    OffHand,  // Secondary (shields)
    Ring,     // Accessories
    Tool,     // Tools (pickaxes)
    Head,     // Helmets
    Chest,    // Chestplates
    Hands,    // Gauntlets
    Feet,     // Greaves
    Legs,     // Leggings
}
```

## Equipment Types

Defined in `src/item/enums.rs`:

```rust
pub enum EquipmentType {
    Weapon,
    Shield,
    Ring,
    Tool(ToolKind),
    Armor(EquipmentSlot),  // For Head/Chest/Hands/Feet/Legs
}
```

The `slot()` method maps `EquipmentType` to `EquipmentSlot`.

## Adding a New Item

1. Add variant to `ItemId` in `src/item/item_id.rs`
2. Create static spec in `src/item/spec/specs.rs`:
   ```rust
   pub static MY_ITEM: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
       name: "My Item",
       item_type: ItemType::Equipment(EquipmentType::Weapon),
       quality: None,  // None = random roll, Some(x) = fixed
       stats: StatSheet::new().with(StatType::Attack, 10),
       max_upgrades: 5,
       max_stack_quantity: 1,
       gold_value: 15,
   });
   ```
3. Import and register in `src/item/spec/traits.rs`:
   ```rust
   (ItemId::MyItem, MY_ITEM.clone()),
   ```

## Adding New Armor

For armor, use `EquipmentType::Armor(EquipmentSlot::X)`:

```rust
pub static COPPER_HELMET: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Helmet",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 36),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 180,
});
```

## Current Armor Items

| Metal | Defense Scale | Max Upgrades |
|-------|---------------|--------------|
| Copper | 3 def/ingot | 5 |
| Tin | 3 def/ingot | 5 |
| Bronze | 4 def/ingot | 7 |

| Slot | Ingot Cost | Copper/Tin Def | Bronze Def |
|------|------------|----------------|------------|
| Head (Helmet) | 12 | 36 | 48 |
| Chest (Chestplate) | 20 | 60 | 80 |
| Hands (Gauntlets) | 8 | 24 | 32 |
| Feet (Greaves) | 10 | 30 | 40 |
| Legs (Leggings) | 18 | 54 | 72 |

## Item Quality

Items can have 6 quality levels affecting stats via multiplier:

| Quality | Multiplier |
|---------|------------|
| Poor | 0.80 |
| Normal | 1.0 |
| Improved | 1.2 |
| WellForged | 1.4 |
| Masterworked | 1.6 |
| Mythic | 1.8 |

## Related Modules

- `src/item/recipe/` - Crafting recipes
- `src/inventory/` - Inventory and equipment management
- `src/location/blacksmith/` - Item upgrades
