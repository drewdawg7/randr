# Forge Modal

The forge modal allows players to smelt ores into ingots using coal as fuel.

## Overview

- Opens when player interacts with a forge in a dungeon (Space key)
- Left side: 3 horizontal crafting slots (Coal, Ore, Product)
- Right side: Player inventory grid (5x5)
- Far right: Item detail pane showing selected item info
- Tab switches focus between crafting slots and inventory
- Enter transfers items between inventory and slots

## Key Files

| File | Purpose |
|------|---------|
| `src/ui/screens/forge_modal/mod.rs` | Module exports |
| `src/ui/screens/forge_modal/state.rs` | State types and `RegisteredModal` impl |
| `src/ui/screens/forge_modal/render.rs` | UI spawning and detail pane population |
| `src/ui/screens/forge_modal/input.rs` | Navigation and item transfer logic |
| `src/ui/screens/forge_modal/plugin.rs` | System registration |
| `src/crafting_station/mod.rs` | `ForgeCraftingState` component |

## State Types

```rust
// Modal root marker
pub struct ForgeModalRoot;

// Grid markers
pub struct ForgeSlotsGrid;      // 1x3 crafting slots
pub struct ForgePlayerGrid;     // 5x5 inventory

// Slot selection (when crafting_focused)
pub enum ForgeSlotIndex { Coal = 0, Ore = 1, Product = 2 }

// Modal state resource
pub struct ForgeModalState {
    pub crafting_focused: bool,      // true = slots, false = inventory
    pub selected_slot: ForgeSlotIndex,
}

// Tracks which forge entity is open
pub struct ActiveForgeEntity(pub Entity);

// Trigger resources
pub struct SpawnForgeModal;
pub struct ForgeSlotRefresh;  // Triggers slot display update
```

## ForgeCraftingState Component

Attached to forge entities to track crafting state:

```rust
#[derive(Component, Default, Clone)]
pub struct ForgeCraftingState {
    pub coal_slot: Option<(ItemId, u32)>,    // (item_id, quantity)
    pub ore_slot: Option<(ItemId, u32)>,
    pub product_slot: Option<(ItemId, u32)>,
    pub is_crafting: bool,
}

impl ForgeCraftingState {
    pub fn can_start_crafting(&self) -> bool;  // Both coal and ore present
    pub fn get_output_item(&self) -> Option<ItemId>;  // CopperOre→CopperIngot, TinOre→TinIngot
    pub fn complete_crafting(&mut self);  // Consumes inputs, produces ingots
}
```

## Crafting Flow

1. Player opens forge modal (Space near forge)
2. Navigate inventory, press Enter on coal → moves to coal slot
3. Press Enter on ore → moves to ore slot
4. Press Esc to close modal
5. If both slots filled, forge animation plays for 5 seconds
6. Reopen modal to find ingots in product slot
7. Tab to crafting slots, navigate to product, Enter to collect

## Item Validation

| Slot | Valid Items |
|------|-------------|
| Coal | `ItemId::Coal` only |
| Ore | `ItemId::CopperOre`, `ItemId::TinOre` |
| Product | Output only (crafted ingots) |

## Key Systems

### `spawn_forge_modal`
Builds the modal UI with crafting slots, inventory grid, and detail pane.

### `handle_forge_modal_tab`
Toggles `crafting_focused` and updates `ItemGrid.is_focused`.

### `handle_forge_modal_navigation`
- Crafting focused: Left/Right between slots
- Inventory focused: Arrow keys navigate grid

### `handle_forge_modal_select`
Handles Enter key:
- Inventory focused + coal selected → transfer to coal slot
- Inventory focused + ore selected → transfer to ore slot
- Crafting focused + any slot → return items to inventory

### `refresh_forge_slots`
Updates slot visuals when `ForgeSlotRefresh` resource exists.

### `update_forge_detail_pane_source`
Updates `pane.source` based on focus, modal state, and grid selection. Only runs when:
- `FocusState` changes (tab between crafting slots and inventory)
- `ForgeModalState` changes (slot selection)
- `ItemGrid.selected_index` changes (navigation in inventory)

### `populate_forge_detail_pane_content`
Renders content when source or data changes. Only runs when:
- `pane.source` changed (via source update system)
- `inventory.is_changed()` or `ForgeCraftingState.is_changed()` (data changed)

Shows item details for selected inventory item or forge slot contents.

### `handle_forge_close_with_crafting`
When modal closes with both slots filled:
- Sets `is_crafting = true`
- Starts forge animation
- Adds `ForgeActiveTimer(5 seconds)`

### `revert_forge_idle` (in dungeon plugin)
When timer expires:
- Calls `forge_state.complete_crafting()`
- Reverts to idle sprite

## Slot Display Components

```rust
// Markers for reactive updates
pub struct ForgeSlotCell { pub slot_type: ForgeSlotIndex }
pub struct ForgeSlotItemSprite;
pub struct ForgeSlotQuantityText;
pub struct ForgeSlotSelector { pub timer, pub frame, pub frame_indices }
```

### Quantity Text

Quantity text uses the shared `spawn_outlined_quantity_text` function from `src/ui/widgets/outlined_text.rs`. The `ForgeSlotQuantityText` marker component is passed to identify these text entities for updates.

## Opening the Modal

From dungeon interaction (`handle_mine_interaction`):

```rust
// Check forge isn't already crafting
if forge_state.map(|s| s.is_crafting).unwrap_or(false) {
    return; // Can't open while crafting
}

commands.insert_resource(ActiveForgeEntity(entity_id));
commands.insert_resource(ForgeModalState::default());
commands.insert_resource(SpawnForgeModal);
```

## Crafting Recipe

- 1 Coal + 1 CopperOre = 1 CopperIngot
- 1 Coal + 1 TinOre = 1 TinIngot
- Crafts `min(coal_qty, ore_qty)` ingots
- All coal and ore consumed when crafting starts
