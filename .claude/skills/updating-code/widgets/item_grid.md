# ItemGrid

Configurable NxN grid for displaying items with animated selection highlight.

**File:** `src/ui/widgets/item_grid.rs`

## Component Fields

```rust
#[derive(Component)]
pub struct ItemGrid {
    pub items: Vec<ItemGridEntry>,   // Items to display (sprites)
    pub selected_index: usize,       // Currently selected cell index
    pub is_focused: bool,            // Whether selector is visible (default: true)
    pub grid_size: usize,            // Columns/rows (default: 4, e.g. 3 for 3x3)
}
```

## Usage

```rust
use crate::ui::widgets::{ItemGrid, ItemGridEntry};

// 4x4 grid (default)
parent.spawn(ItemGrid {
    items: vec![ItemGridEntry { sprite_name: "sword".to_string() }],
    selected_index: 0,
    is_focused: true,
    grid_size: 4,
});

// 3x3 grid (e.g. equipment)
parent.spawn(ItemGrid {
    items: vec![],
    selected_index: 0,
    is_focused: false,
    grid_size: 3,
});
```

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

- `is_focused: true` → animated selector sprite visible on `selected_index` cell
- `is_focused: false` → no selector visible
- Changing either field triggers `update_grid_selector` (runs in `PostUpdate`, reacts to `Changed<ItemGrid>`)
- Selector animates between two frames (0.5s interval) using `GridSelector` component

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
1. Despawns all existing `GridItemSprite` children from each cell
2. Re-spawns item sprites matching the current `items` vector

This enables live updates (e.g., equip/unequip) without rebuilding the entire grid.

## Internal Components (private)

| Component | Role |
|-----------|------|
| `GridContainer` | Marker on the CSS Grid container child |
| `GridCell { index }` | Marker on each cell with its position |
| `GridSelector` | Animation state on the selector sprite |
| `GridItemSprite` | Marker on item icon sprites (for update/despawn) |

## Plugin

`ItemGridPlugin` — registers the `on_add_item_grid` observer and `PostUpdate` systems: `update_grid_items`, `update_grid_selector`, `animate_grid_selector` (chained in that order).
