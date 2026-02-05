# ItemGrid

Configurable NxN grid for displaying items with animated selection highlight and quantity text.

**Module:** `src/ui/widgets/item_grid/`

| File | Contents |
|------|----------|
| `mod.rs` | Plugin, constants, re-exports |
| `components.rs` | ItemGrid, ItemGridEntry, ItemGridFocusPanel, markers |
| `cell.rs` | GridContainer, GridCell, GridCellBundle |
| `systems.rs` | on_add_item_grid, update_grid_items, update_grid_selector |

## Component Fields

```rust
#[derive(Component)]
pub struct ItemGrid {
    pub items: Vec<ItemGridEntry>,   // Items to display (sprites + quantities)
    pub selected_index: usize,       // Currently selected cell index
    pub grid_size: usize,            // Columns/rows (default: 4, e.g. 3 for 3x3)
}
```

## ItemGridFocusPanel

Associates an ItemGrid with a FocusPanel for selector visibility:

```rust
#[derive(Component)]
pub struct ItemGridFocusPanel(pub FocusPanel);
```

When the associated FocusPanel is focused, the selector is visible.

## ItemGridEntry

```rust
#[derive(Clone)]
pub struct ItemGridEntry {
    pub sprite_sheet_key: SpriteSheetKey,  // Sprite sheet containing the icon
    pub sprite_name: String,               // Slice name in sprite sheet
    pub quantity: u32,                     // Quantity to display (only shown if > 1)
}

impl ItemGridEntry {
    /// Create a grid entry from an inventory item.
    pub fn from_inventory_item(inv_item: &InventoryItem) -> Self;

    /// Create grid entries from all items in an inventory.
    pub fn from_inventory(inventory: &Inventory) -> Vec<Self>;
}
```

### Creating Entries from Inventory

Use the helper methods to avoid duplicating conversion code:

```rust
// Convert all inventory items to grid entries
let entries = ItemGridEntry::from_inventory(&inventory);

// Convert equipped items to grid entries
let entries: Vec<ItemGridEntry> = get_equipment_items(&inventory)
    .iter()
    .map(|inv_item| ItemGridEntry::from_inventory_item(inv_item))
    .collect();
```

## Usage

```rust
use crate::ui::widgets::{ItemGrid, ItemGridEntry, ItemGridFocusPanel};
use crate::ui::focus::FocusPanel;

// 4x4 grid with focus panel
parent.spawn((
    ItemGrid {
        items: vec![ItemGridEntry {
            sprite_sheet_key: SpriteSheetKey::Items,
            sprite_name: "sword".to_string(),
            quantity: 1,  // Won't show quantity text
        }],
        selected_index: 0,
        grid_size: 4,
    },
    ItemGridFocusPanel(FocusPanel::Left),
));

// Stackable items show quantity in bottom-right corner
parent.spawn((
    ItemGrid {
        items: vec![ItemGridEntry {
            sprite_sheet_key: SpriteSheetKey::Items,
            sprite_name: "potion".to_string(),
            quantity: 5,  // Shows "5" with black outline
        }],
        selected_index: 0,
        grid_size: 4,
    },
    ItemGridFocusPanel(FocusPanel::Right),
));
```

## Quantity Display

- Quantities > 1 displayed as white text (14px) with black outline in bottom-right corner
- Uses shared `spawn_outlined_quantity_text` function from `src/ui/widgets/outlined_text.rs`
- Creates 8 shadow layers for the outline effect
- Uses `GridItemQuantityText` as the marker component for reactive updates
- Positioned at `right: 2px, bottom: 0px`

## Size Calculation

Grid container dimensions are computed from `grid_size`:

```rust
const CELL_SIZE: f32 = 48.0;
const GAP: f32 = 4.0;
const NINE_SLICE_INSET: f32 = 58.0;

// content = grid_size * CELL_SIZE + (grid_size - 1) * GAP
// total = content + 2 * NINE_SLICE_INSET
// 3x3 → 268px, 4x4 → 320px
```

## Selector Behavior

- Uses shared `AnimatedSelector` from `src/ui/widgets/selector.rs`
- Selector visible when `ItemGridFocusPanel` is associated and its FocusPanel is focused
- Changing `selected_index` or FocusState triggers `update_grid_selector`
- Selector animates between two frames (0.5s interval)

## Multiple Grids

When using multiple `ItemGrid` instances in the same screen, add marker components and use `Without<>` filters on queries to avoid Bevy's query conflict panic:

```rust
// Marker components
#[derive(Component)]
pub struct EquipmentGrid;
#[derive(Component)]
pub struct BackpackGrid;

// Disjoint queries
mut eq: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
mut bp: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
```

## Reactive Item Updates

Changing the `items` field on an existing `ItemGrid` triggers the `update_grid_items` system (runs in `PostUpdate` on `Changed<ItemGrid>`). This system:
1. Despawns all existing `GridItemSprite` and `GridItemQuantityText` children from each cell
2. Re-spawns item sprites and quantity text matching the current `items` vector

This enables live updates (e.g., equip/unequip, buy/sell) without rebuilding the entire grid.

## Internal Components (private)

| Component | Role |
|-----------|------|
| `GridContainer` | Marker on the CSS Grid container child |
| `GridCell { index }` | Marker on each cell with its position |
| `GridItemSprite` | Marker on item icon sprites (for update/despawn) |
| `GridItemQuantityText` | Marker on quantity text container (for update/despawn) |

## Plugin

`ItemGridPlugin` — registers the `on_add_item_grid` observer and `PostUpdate` systems: `update_grid_items`, `update_grid_selector` (chained in that order).
