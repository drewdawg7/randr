# Storage System

## Overview

Storage is a shared inventory container accessible from the Store screen. It allows players to deposit items from their inventory for safekeeping and withdraw them later.

**Key distinction**: Storage is NOT a Location (doesn't implement the Location trait). It's a simple inventory wrapper integrated with Town.

## Module Structure

```
src/storage/
├── mod.rs          # Exports
├── definition.rs   # Storage struct, new(), Default
├── traits.rs       # HasInventory impl
└── enums.rs        # (empty, reserved for future errors)
```

## Key Types

### Storage (definition.rs)

Simple wrapper around an unlimited Inventory:

```rust
pub struct Storage {
    pub inventory: Inventory,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new_unlimited(),
        }
    }
}
```

### HasInventory Trait (traits.rs)

Storage implements `HasInventory`, giving access to all inventory operations:
- `find_item_by_uuid()`
- `add_to_inv()`
- `remove_item()`
- `get_inventory_items()`

## Integration Points

### Town (town/definition.rs)

Storage is a field on Town:
```rust
pub struct Town {
    // ... other fields ...
    pub storage: Storage,
}
```

### GameState (system.rs)

Accessors for storage:
```rust
game_state().storage()      // &Storage
game_state().storage_mut()  // &mut Storage
```

### Commands (commands/storage.rs)

Two commands for item transfer:

| Command | Purpose |
|---------|---------|
| `DepositItem { item_uuid }` | Move item from player inventory to storage |
| `WithdrawItem { item_uuid }` | Move item from storage to player inventory |

**Deposit restrictions**:
- Cannot deposit locked items
- Cannot deposit equipped items

**Withdraw restrictions**:
- Player inventory must have room

## UI Components

### Store Menu (ui/components/store/menu.rs)

Storage is the 3rd option in the store menu (Buy, Sell, **Storage**, Back).

### Storage Screen (ui/components/store/storage.rs)

Dual-panel interface:

| Panel | Content | Back Button |
|-------|---------|-------------|
| Left (Player) | Player inventory items | Yes |
| Right (Storage) | Storage inventory items | No |

**Controls**:
- `Tab` - Switch focus between panels
- `Enter` - Transfer selected item to other panel
- `Esc` - Return to store menu
- `F` - Cycle inventory filter

**Focus indicator**: Yellow `>` prefix on focused panel title.

### Item Wrappers (ui/components/widgets/item_list/impls.rs)

| Wrapper | Usage | Selectable Rule |
|---------|-------|-----------------|
| `DepositableItem` | Player items in storage view | Not locked AND not equipped |
| `StoredItem` | Storage items | Always selectable |

## Files Summary

| File | Purpose |
|------|---------|
| `src/storage/definition.rs` | Storage struct and constructor |
| `src/storage/traits.rs` | HasInventory implementation |
| `src/storage/mod.rs` | Module exports |
| `src/town/definition.rs` | Town.storage field |
| `src/system.rs` | GameState storage accessors |
| `src/commands/storage.rs` | DepositItem/WithdrawItem handlers |
| `src/ui/components/store/storage.rs` | Storage screen UI |
| `src/ui/components/store/menu.rs` | Store menu with Storage option |
| `src/ui/components/store/tab.rs` | StoreTab with Storage state |
| `src/ui/components/widgets/item_list/impls.rs` | DepositableItem, StoredItem wrappers |
