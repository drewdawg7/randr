# Item & Equipment System

## Overview

Items use the `define_entity!` macro system for declarative definitions with direct spawning via `ItemId::spawn()`.

## Key Files

| File | Purpose |
|------|---------|
| `src/item/definitions.rs` | All items defined via `define_entity!` macro |
| `src/item/enums.rs` | `ItemType`, `EquipmentType`, `ItemQuality` enums |
| `src/item/definition.rs` | `Item` struct - spawned item instances |
| `src/inventory/enums.rs` | `EquipmentSlot` enum |
| `entity_macros/src/lib.rs` | Proc macro implementation |

## Entity Macro System

Items are defined using the `define_entity!` macro which generates:
- `ItemId` enum with all variants
- `ItemId::spec(&self) -> &'static ItemSpec` method
- `ItemId::spawn(&self) -> Item` convenience method
- `ItemId::ALL: &[ItemId]` for iteration

## Spawning Items

```rust
// Direct spawning (preferred)
let sword = ItemId::BonkStick.spawn();

// Spec lookup
let spec = ItemId::BonkStick.spec();
println!("Name: {}", spec.name);
```

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

## Adding a New Item

1. Add to `src/item/definitions.rs` inside the `define_entity!` block:
   ```rust
   MyItem => ItemSpec {
       name: "My Item",
       item_type: ItemType::Equipment(EquipmentType::Weapon),
       quality: None,  // None = random roll, Some(x) = fixed
       stats: StatSheet::new().with(StatType::Attack, 10),
       max_upgrades: 5,
       max_stack_quantity: 1,
       gold_value: 15,
   },
   ```

2. Use the new item:
   ```rust
   let item = ItemId::MyItem.spawn();
   ```

## Adding New Armor

For armor, use `EquipmentType::Armor(EquipmentSlot::X)`:

```rust
CopperHelmet => ItemSpec {
    name: "Copper Helmet",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 36),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 180,
},
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
