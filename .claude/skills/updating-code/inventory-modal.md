# Inventory Modal

Modal displaying the player's equipment and backpack items as two `ItemGrid` widgets with an `ItemDetailPane` showing selected item details.

## Files

```
src/ui/screens/inventory_modal/
├── mod.rs      # Module declarations, re-exports InventoryModalPlugin + InventoryModal
├── plugin.rs   # InventoryModalPlugin: close, tab, navigation, sync, populate, and spawn trigger
├── state.rs    # InventoryModalRoot, EquipmentGrid, BackpackGrid, SpawnInventoryModal, InventoryModal
├── input.rs    # handle_inventory_modal_tab, handle_inventory_modal_navigation, handle_inventory_modal_select
└── render.rs   # spawn_inventory_modal, sync_inventory_to_grids, populate_item_detail_pane, get_equipment_items, get_backpack_items
```

## Behavior

- **Open**: Press `I` in town → shows equipment grid (3x3, left), backpack grid (4x4, middle), and detail pane (right)
- **Close**: Press `Escape` or `I` again
- **Tab**: Toggles focus between equipment and backpack grids
- **Selection**: Animated selector sprite highlights the currently selected cell in the focused grid
- **Navigation**: Arrow keys move selection within the focused grid; detail pane updates reactively
- **Equip/Unequip**: Enter key equips (backpack) or unequips (equipment) the selected item
- **No modal container**: Uses `spawn_modal_overlay` directly (no Modal builder)

## Layout Structure

```
InventoryModalRoot (overlay)
└── Row (flex_direction: Row, column_gap: 16px)
    ├── ItemGrid (3x3, 268x268px) + EquipmentGrid marker — focused by default
    ├── ItemGrid (4x4, 320x320px) + BackpackGrid marker
    └── ItemDetailPane (240x288px, nine-slice background)
        ├── Nine-slice panels (DetailPanelSlice, 48px borders)
        └── ItemDetailPaneContent (absolute, inset 48px, 144x192px)
            ├── Item name (16px, quality-colored)
            ├── Item type (14px, gray)
            ├── Quality label (14px, quality-colored)
            ├── Quantity "Qty: X" (14px, green, only if qty > 1)
            └── ItemStatsDisplay (14px, icon+value mode)
```

## Dual Grid System

The inventory modal uses two separate `ItemGrid` instances distinguished by marker components:

- **`EquipmentGrid`** (3x3): Shows equipped items in slot order (Weapon, OffHand, Ring, Tool, Head, Chest, Hands, Feet, Legs). Only populated slots have item sprites.
- **`BackpackGrid`** (4x4): Shows non-equipped backpack items.

### Tab Switching

`handle_inventory_modal_tab` listens for `GameAction::NextTab` and toggles `is_focused` between the two grids. The `update_grid_selector` system in `ItemGridPlugin` reactively shows/hides the animated selector.

### Query Patterns

Because both grids have `&mut ItemGrid`, queries must use `Without<>` filters to avoid Bevy's query conflict panic:

```rust
mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
```

## Grid Navigation

The `handle_inventory_modal_navigation` system handles arrow key input on the focused grid:

```rust
// Navigation uses grid.grid_size for row/col calculations:
// row = current / grid_size
// col = current % grid_size
// Moves clamped to item_count (cannot navigate to empty cells)

fn navigate_grid(grid: &mut ItemGrid, direction: NavigationDirection) {
    let gs = grid.grid_size;
    // Left:  col > 0 → index - 1
    // Right: col < gs-1 → index + 1
    // Up:    row > 0 → index - gs
    // Down:  row < gs-1 → index + gs
}
```

## Equip/Unequip (Enter Key)

`handle_inventory_modal_select` handles `GameAction::Select`:

- **Equipment grid focused**: Unequips the selected item back to backpack
  - Maps `selected_index` to the Nth *populated* `EquipmentSlot` (since empty slots are skipped by `get_equipment_items`)
  - Calls `inventory.unequip_item(slot)` — silently fails if backpack is full
- **Backpack grid focused**: Equips the selected item
  - Only acts on items where `item_type.equipment_slot()` returns `Some(slot)`
  - Calls `inventory.equip_from_inventory(uuid, slot)` — automatically swaps if slot is occupied
  - Non-equipment items (materials, consumables) are ignored

Grids update automatically via `sync_inventory_to_grids` which uses Bevy's change detection (`inventory.is_changed()`).

### Slot Mapping for Equipment Grid

Since `get_equipment_items()` only returns populated slots, the index-to-slot mapping requires filtering:

```rust
let equipped_slots: Vec<EquipmentSlot> = EquipmentSlot::all()
    .iter()
    .copied()
    .filter(|slot| inventory.get_equipped_item(*slot).is_some())
    .collect();
// equipped_slots[selected_index] gives the actual EquipmentSlot
```

## Item Helpers

Two helpers in `render.rs` provide items for each grid:

```rust
/// Equipment items in slot order (only populated slots).
pub fn get_equipment_items(inventory: &Inventory) -> Vec<&InventoryItem>

/// Backpack (non-equipped) items.
pub fn get_backpack_items(inventory: &Inventory) -> Vec<&InventoryItem>
```

## Stat Comparison

The `ManagesEquipment` trait provides a `get_comparison_stats` method for comparing an item's stats against the currently equipped item in the same slot:

```rust
// Returns None if item is not equipment, Some(empty vec) if slot is empty
let comparison = inventory.get_comparison_stats(&item);
if let Some(stats) = comparison {
    display = display.with_comparison(stats);
}
```

This method is defined in `src/inventory/traits.rs` and is used by inventory, merchant, and other modals to show stat differences when viewing items.

## Reactive Grid Sync

The `sync_inventory_to_grids` system uses Bevy's native change detection to automatically update grids when inventory changes:

```rust
pub fn sync_inventory_to_grids(
    inventory: Res<Inventory>,
    mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    if !inventory.is_changed() {
        return;
    }
    // Rebuild grid items and clamp selected_index...
}
```

This replaces manual `refresh_grids()` calls after equip/unequip operations.

## Detail Pane Systems

Detail pane logic is split into two systems for efficient change detection:

### `update_inventory_detail_pane_source`
Updates `pane.source` based on focus and grid selection. Only runs when:
- `FocusState` changes (tab between grids)
- `ItemGrid.selected_index` changes (navigation)

Uses `Ref<ItemGrid>` to check `is_changed()` on each grid without query filters.

### `populate_inventory_detail_pane_content`
Renders content when source or inventory changes. Only runs when:
- `pane.source` changed (via source update system)
- `inventory.is_changed()` (item at index may have changed after equip/unequip)

Uses `Ref<ItemDetailPane>` to check `is_changed()` for pane updates.

**Content rendered:**
- Item name (quality-colored with black outline)
- Item type (gray)
- Quality label (quality-colored)
- Quantity "Qty: X" (green, only if qty > 1)
- `ItemStatsDisplay` with stat comparison for backpack items

**Guards:** if `selected_index >= items.len()`, content is cleared (no crash on empty cells)

## Key Types

| Type | Role |
|------|------|
| `InventoryModal` | `RegisteredModal` impl, used with `commands.toggle_modal::<InventoryModal>()` |
| `InventoryModalRoot` | Marker component on the modal overlay entity |
| `EquipmentGrid` | Marker component on the 3x3 equipment grid entity |
| `BackpackGrid` | Marker component on the 4x4 backpack grid entity |
| `SpawnInventoryModal` | Trigger resource inserted by `InventoryModal::spawn()` |

## Render Pattern

```rust
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    let equipment_entries = get_equipment_items(inventory).iter().map(/* ... */).collect();
    let backpack_entries = get_backpack_items(inventory).iter().map(/* ... */).collect();

    let overlay = spawn_modal_overlay(commands);
    commands.entity(overlay).insert(InventoryModalRoot).with_children(|parent| {
        parent.spawn(Node { flex_direction: Row, column_gap: 16px, ... }).with_children(|row| {
            row.spawn((EquipmentGrid, ItemGrid { items: equipment_entries, grid_size: 3, is_focused: true, .. }));
            row.spawn((BackpackGrid, ItemGrid { items: backpack_entries, grid_size: 4, is_focused: false, .. }));
            row.spawn(ItemDetailPane { source: InfoPanelSource::Inventory { selected_index: 0 } });
        });
    });
}
```

## Plugin Systems

| System | Schedule | Run Condition |
|--------|----------|---------------|
| `modal_close_system::<InventoryModal>` | Update | Always |
| `handle_inventory_modal_tab` | Update | `in_inventory_modal` |
| `handle_inventory_modal_navigation` | Update | `in_inventory_modal` |
| `handle_inventory_modal_select` | Update | `in_inventory_modal` |
| `sync_inventory_to_grids` | Update | `in_inventory_modal` |
| `update_inventory_detail_pane_source` | Update | `in_inventory_modal` |
| `populate_inventory_detail_pane_content` | Update | `in_inventory_modal` |
| `trigger_spawn_inventory_modal` | Update | `resource_exists::<SpawnInventoryModal>` |
