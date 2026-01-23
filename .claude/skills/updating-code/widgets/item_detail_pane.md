# ItemDetailPane

Nine-slice panel for displaying item details. Used in the inventory modal and store buy screen.

**File:** `src/ui/widgets/item_detail_pane.rs`

## Components

| Component | Role |
|-----------|------|
| `ItemDetailPane` | Main widget component with `source: InfoPanelSource` |
| `ItemDetailPaneContent` | Marker on the inner content container (used by populate systems) |

## Usage

```rust
use crate::ui::widgets::ItemDetailPane;
use crate::ui::screens::InfoPanelSource;

row.spawn(ItemDetailPane {
    source: InfoPanelSource::Inventory { selected_index: 0 },
});
```

## Layout

- Panel size: 240x288px
- Nine-slice border: 48px (`DetailPanelSlice::SLICE_SIZE`)
- Content area: 144x192px (absolutely positioned inside the nine-slice inner area)
- Content flex: column, align start, row_gap 4px, overflow clipped

## InfoPanelSource

```rust
pub enum InfoPanelSource {
    Store { selected_index: usize },
    Inventory { selected_index: usize },
}
```

## Population Pattern

The pane's content is populated by external systems (not the widget itself):

- **Inventory modal**: `populate_item_detail_pane` in `render.rs` reacts to `ItemGrid` selection changes
- **Store screen**: `populate_central_detail_panel` in `panels.rs` handles store item details

These systems:
1. Query `ItemDetailPaneContent` entity
2. Despawn existing children
3. Spawn new content (item name, type, quality, stats)

## Plugin

`ItemDetailPanePlugin` — registers the `on_add_item_detail_pane` observer that builds the nine-slice background and content container.
