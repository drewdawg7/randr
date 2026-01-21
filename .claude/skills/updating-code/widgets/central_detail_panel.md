# CentralDetailPanel

Nine-slice panel for item details in buy screen.

**File:** `src/ui/widgets/central_detail_panel.rs`

## Usage

```rust
use crate::ui::widgets::CentralDetailPanel;
use crate::ui::screens::InfoPanelSource;

row.spawn(CentralDetailPanel {
    source: InfoPanelSource::Store { selected_index: 0 },
});
```

The `populate_central_detail_panel` system in `panels.rs` handles rendering item details.

## InfoPanelSource

```rust
pub enum InfoPanelSource {
    Store { selected_index: usize },
    Inventory { selected_index: usize },
}
```
