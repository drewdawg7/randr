# Inventory Modal

Modal displaying the player's inventory items as an `ItemGrid` with an `ItemDetailPane` showing selected item details.

## Files

```
src/ui/screens/inventory_modal/
├── mod.rs      # Module declarations, re-exports InventoryModalPlugin + InventoryModal
├── plugin.rs   # InventoryModalPlugin: close, navigation, populate, and spawn trigger
├── state.rs    # InventoryModalRoot, SpawnInventoryModal, InventoryModal (RegisteredModal)
├── input.rs    # handle_inventory_modal_close + handle_inventory_modal_navigation
└── render.rs   # spawn_inventory_modal, populate_item_detail_pane, get_ordered_items
```

## Behavior

- **Open**: Press `I` in town → shows a 4x4 `ItemGrid` (left) and `ItemDetailPane` (right) in a horizontal row
- **Close**: Press `Escape` or `I` again
- **Selection**: Animated selector sprite highlights the currently selected cell (`is_focused: true`)
- **Navigation**: Arrow keys move selection within the grid; detail pane updates reactively
- **No modal container**: Uses `spawn_modal_overlay` directly (no Modal builder)

## Layout Structure

```
InventoryModalRoot (overlay)
└── Row (flex_direction: Row, column_gap: 16px)
    ├── ItemGrid (4x4, 320x320px)
    └── ItemDetailPane (240x288px, nine-slice background)
        ├── Nine-slice panels (DetailPanelSlice, 48px borders)
        └── ItemDetailPaneContent (absolute, inset 48px, 144x192px)
            ├── Item name (16px, quality-colored)
            ├── Item type (14px, gray)
            ├── Quality label (14px, quality-colored)
            └── ItemStatsDisplay (14px, icon+value mode)
```

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

The `get_ordered_items` helper (in `render.rs`) returns items in display order:

1. Equipped items first (iterates all `EquipmentSlot`s)
2. Backpack items second (from `inventory.get_inventory_items()`)

Each item is converted to `ItemGridEntry { sprite_name }` using `item.item_id.sprite_name()`.

## Detail Pane Population

The `populate_item_detail_pane` system runs every frame and checks if the pane needs updating:

1. Reads `ItemGrid.selected_index`
2. Compares against current `ItemDetailPane.source` index
3. On mismatch (or first frame with no children), updates content:
   - Despawns existing `ItemDetailPaneContent` children
   - Looks up item via `get_ordered_items` at the selected index
   - Spawns: item name, item type, quality label, and `ItemStatsDisplay`
4. Guards: if `selected_index >= ordered_items.len()`, clears content (no crash on empty cells)

## Key Types

| Type | Role |
|------|------|
| `InventoryModal` | `RegisteredModal` impl, used with `commands.toggle_modal::<InventoryModal>()` |
| `InventoryModalRoot` | Marker component on the modal overlay entity |
| `SpawnInventoryModal` | Trigger resource inserted by `InventoryModal::spawn()` |

## Render Pattern

```rust
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    let ordered = get_ordered_items(inventory);
    let items: Vec<ItemGridEntry> = ordered.iter().map(|inv_item| ItemGridEntry {
        sprite_name: inv_item.item.item_id.sprite_name().to_string(),
    }).collect();

    let overlay = spawn_modal_overlay(commands);
    commands.entity(overlay).insert(InventoryModalRoot).with_children(|parent| {
        parent.spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            align_items: AlignItems::FlexStart,
            ..default()
        }).with_children(|row| {
            row.spawn(ItemGrid { items, selected_index: 0, is_focused: true });
            row.spawn(ItemDetailPane {
                source: InfoPanelSource::Inventory { selected_index: 0 },
            });
        });
    });
}
```

## Plugin Systems

| System | Schedule | Run Condition |
|--------|----------|---------------|
| `handle_inventory_modal_close` | Update | Always (guards on ActiveModal) |
| `handle_inventory_modal_navigation` | Update | Always (guards on ActiveModal) |
| `populate_item_detail_pane` | Update | Always (guards internally on query results) |
| `trigger_spawn_inventory_modal` | Update | `resource_exists::<SpawnInventoryModal>` |
