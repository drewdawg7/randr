# ItemGrid

5x5 grid for displaying items with selection highlight.

**File:** `src/ui/widgets/item_grid.rs`

## Usage

```rust
use crate::ui::widgets::{ItemGrid, ItemGridEntry};

parent.spawn(ItemGrid {
    items: vec![ItemGridEntry { sprite_name: "sword".to_string() }],
    selected_index: 0,
    is_focused: true,
});
```
