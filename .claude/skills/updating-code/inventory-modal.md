# Inventory Modal

Minimal modal displaying the player's inventory items as an `ItemGrid`.

## Files

```
src/ui/screens/inventory_modal/
├── mod.rs      # Module declarations, re-exports InventoryModalPlugin + InventoryModal
├── plugin.rs   # InventoryModalPlugin: close handler + spawn trigger
├── state.rs    # InventoryModalRoot, SpawnInventoryModal, InventoryModal (RegisteredModal)
├── input.rs    # handle_inventory_modal_close (Escape/CloseModal)
└── render.rs   # spawn_inventory_modal - builds ItemGrid from Inventory
```

## Behavior

- **Open**: Press `I` in town → shows a 4x4 `ItemGrid` (320x320px) with all inventory items
- **Close**: Press `Escape` or `I` again
- **No interaction**: No selection, no navigation within the grid (`is_focused: false`)
- **No modal container**: Uses `spawn_modal_overlay` directly (no Modal builder) so only the ItemGrid's own nine-slice background is visible

## Item Display Order

1. Equipped items first (iterates all `EquipmentSlot`s)
2. Backpack items second (from `inventory.get_inventory_items()`)

Each item is converted to `ItemGridEntry { sprite_name }` using `item.item_id.sprite_name()`.

## Key Types

| Type | Role |
|------|------|
| `InventoryModal` | `RegisteredModal` impl, used with `commands.toggle_modal::<InventoryModal>()` |
| `InventoryModalRoot` | Marker component on the modal overlay entity |
| `SpawnInventoryModal` | Trigger resource inserted by `InventoryModal::spawn()` |

## Render Pattern

```rust
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    let items: Vec<ItemGridEntry> = /* equipped + backpack items mapped to sprite names */;

    let overlay = spawn_modal_overlay(commands);
    commands
        .entity(overlay)
        .insert(InventoryModalRoot)
        .with_children(|parent| {
            parent.spawn(ItemGrid {
                items,
                selected_index: 0,
                is_focused: false,
            });
        });
}
```

## Plugin Systems

| System | Schedule | Run Condition |
|--------|----------|---------------|
| `handle_inventory_modal_close` | Update | Always (guards on ActiveModal) |
| `trigger_spawn_inventory_modal` | Update | `resource_exists::<SpawnInventoryModal>` |
