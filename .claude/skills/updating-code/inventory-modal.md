# Inventory Modal

Modal displaying the player's inventory items as an `ItemGrid` with arrow key navigation.

## Files

```
src/ui/screens/inventory_modal/
├── mod.rs      # Module declarations, re-exports InventoryModalPlugin + InventoryModal
├── plugin.rs   # InventoryModalPlugin: close, navigation, and spawn trigger
├── state.rs    # InventoryModalRoot, SpawnInventoryModal, InventoryModal (RegisteredModal)
├── input.rs    # handle_inventory_modal_close + handle_inventory_modal_navigation
└── render.rs   # spawn_inventory_modal - builds ItemGrid from Inventory
```

## Behavior

- **Open**: Press `I` in town → shows a 4x4 `ItemGrid` (320x320px) with all inventory items
- **Close**: Press `Escape` or `I` again
- **Selection**: Animated selector sprite highlights the currently selected cell (`is_focused: true`)
- **Navigation**: Arrow keys move selection within the grid using 2D grid math (clamped at boundaries and item count)
- **No modal container**: Uses `spawn_modal_overlay` directly (no Modal builder) so only the ItemGrid's own nine-slice background is visible

## Grid Navigation

The `handle_inventory_modal_navigation` system handles arrow key input:

```rust
// 4x4 grid layout (indices):
// 0  1  2  3
// 4  5  6  7
// 8  9 10 11
// 12 13 14 15

const GRID_SIZE: usize = 4;

// Navigation: row/col derived from selected_index
// Left:  col > 0 → index - 1
// Right: col < 3 → index + 1
// Up:    row > 0 → index - 4
// Down:  row < 3 → index + 4
// All moves clamped to item_count (cannot navigate to empty cells)
```

The system directly mutates `ItemGrid.selected_index`, which triggers the `update_grid_selector` system in `ItemGridPlugin` to move the animated selector sprite reactively.

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
                is_focused: true,
            });
        });
}
```

## Plugin Systems

| System | Schedule | Run Condition |
|--------|----------|---------------|
| `handle_inventory_modal_close` | Update | Always (guards on ActiveModal) |
| `handle_inventory_modal_navigation` | Update | Always (guards on ActiveModal) |
| `trigger_spawn_inventory_modal` | Update | `resource_exists::<SpawnInventoryModal>` |
