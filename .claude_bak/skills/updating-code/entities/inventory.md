# Inventory System

> Player inventory and equipment management system.
> Files: `src/inventory/`

## Module Structure

- `mod.rs` - Module exports
- `definition.rs` - `InventoryItem` and `Inventory` structs
- `enums.rs` - `EquipmentSlot` and `InventoryError` enums
- `traits.rs` - `HasInventory` trait with inventory operations
- `equipment.rs` - `HasEquipment` trait
- `tests.rs` - Comprehensive test coverage

## Key Types

### InventoryItem (`definition.rs:12-34`)
Wrapper around `Item` with quantity tracking:
```rust
pub struct InventoryItem {
    pub item: Item,
    pub quantity: u32,
}
```
- `new(item)` - Creates with quantity 1
- `uuid()` - Returns item's UUID
- `decrease_quantity(amount)` - Uses `saturating_sub` to floor at 0
- `increase_quantity(amount)` - Adds to quantity

### Inventory (`definition.rs:36-108`)
Container for items and equipment:
```rust
pub struct Inventory {
    pub items: Vec<InventoryItem>,     // Unequipped items
    max_slots: usize,                   // Default: 15
    equipment: HashMap<EquipmentSlot, InventoryItem>,
}
```
- `new()` - Standard inventory (15 slots)
- `new_unlimited()` - For storage (usize::MAX slots)
- `sum_equipment_stats(stat_type)` - Sums stat values across all equipped items
- `equipped_tome()` / `equipped_tome_mut()` - Access to tome in OffHand slot

### EquipmentSlot (`enums.rs:1-28`)
9 equipment slots:
- Weapon, OffHand, Ring, Tool
- Head, Chest, Hands, Feet, Legs

### InventoryError (`enums.rs:30-33`)
```rust
#[derive(Debug)]
pub enum InventoryError {
    Full
}
```

### AddItemResult (`definition.rs:12-21`)
Returned by `add_to_inv` with details about the operation:
```rust
pub struct AddItemResult {
    pub was_stacked: bool,     // Whether item was stacked with existing
    pub total_quantity: u32,   // Total quantity after adding
    pub slot_index: usize,     // Index where item was placed
}
```

## HasInventory Trait (`traits.rs`)

Core trait for inventory management. Requires:
- `inventory(&self) -> &Inventory`
- `inventory_mut(&mut self) -> &mut Inventory`

### Key Methods

| Method | Returns | Description |
|--------|---------|-------------|
| `add_to_inv(item)` | `Result<AddItemResult, InventoryError>` | Adds item, stacks non-equipment (up to `max_stack_quantity`) |
| `find_item_by_uuid(uuid)` | `Option<&InventoryItem>` | Find in inventory items only |
| `find_item_by_id(item_id)` | `Option<&InventoryItem>` | Find in inventory OR equipment |
| `decrease_item_quantity(item, amount)` | `()` | Decrease, remove if quantity=0 |
| `remove_item_from_inventory(item)` | `()` | Remove from items vec by UUID |
| `equip_item(item, slot)` | `()` | Equip item (unequips existing, sets `is_equipped`) |
| `unequip_item(slot)` | `Result<Option<Item>, InventoryError>` | Move to inventory, returns unequipped item |
| `equip_from_inventory(uuid, slot)` | `()` | Move from inventory to equipment |
| `get_equipped_item(slot)` | `Option<&InventoryItem>` | Get reference to equipped item |
| `remove_item(uuid)` | `Option<InventoryItem>` | Remove from equipment or inventory |

### Stacking Behavior
- Only non-equipment items stack (`!item.item_type.is_equipment()`)
- Stacks up to `item.max_stack_quantity`
- Equipment items always take separate slots

### Error Handling
- `add_to_inv` returns `Err(InventoryError::Full)` when at max_slots
- `unequip_item` returns `Err(InventoryError::Full)` when inventory full

## Testing Patterns

Tests use mock structs implementing `HasInventory`:
```rust
#[cfg(test)]
struct MockInventoryHolder {
    inventory: Inventory,
}

#[cfg(test)]
impl HasInventory for MockInventoryHolder { ... }
```

Helper functions create test items:
- `create_test_weapon(id, attack)` - Equipment item
- `create_test_shield(id, defense)` - Equipment item
- `create_test_material(id)` - Stackable material (max_stack: 99)

## Common Patterns

### Adding Item to Player
```rust
player.add_to_inv(item)?;
```

### Equipping from Inventory
```rust
let uuid = item.item_uuid;
player.add_to_inv(item)?;
player.equip_from_inventory(uuid, EquipmentSlot::Weapon);
```

### Checking Equipment Stats
```rust
let total_attack = player.inventory().sum_equipment_stats(StatType::Attack);
```
