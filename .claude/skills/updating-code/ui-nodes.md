# UI Node Helpers

## Overview
The `src/ui/nodes.rs` module provides helper functions for creating common Bevy UI `Node` layouts, reducing boilerplate when spawning row or column containers.

## Functions

### `row_node(gap: f32) -> Node`
Creates a horizontal row node with centered items.

```rust
use crate::ui::row_node;

// Instead of:
parent.spawn(Node {
    flex_direction: FlexDirection::Row,
    align_items: AlignItems::Center,
    column_gap: Val::Px(4.0),
    ..default()
});

// Use:
parent.spawn(row_node(4.0));
```

### `column_node(gap: f32) -> Node`
Creates a vertical column node.

```rust
use crate::ui::column_node;

// Instead of:
parent.spawn(Node {
    flex_direction: FlexDirection::Column,
    row_gap: Val::Px(15.0),
    ..default()
});

// Use:
parent.spawn(column_node(15.0));
```

## When to Use

Use these helpers when you need a simple row or column container with only a gap setting. If you need additional properties like `padding`, `margin`, `width`, etc., use the full `Node` struct instead.

## Overflow Clipping

**Important:** Bevy UI does NOT clip overflow by default. Child elements can render outside their parent's bounds unless you explicitly set `overflow: Overflow::clip()` on the parent Node.

```rust
// Container that clips children to its bounds
Node {
    width: Val::Px(240.0),
    height: Val::Px(200.0),
    overflow: Overflow::clip(),  // Required for clipping!
    ..default()
}
```

Use `Overflow::clip()` for both axes, or `Overflow::clip_x()` / `Overflow::clip_y()` for single-axis clipping.

Examples in codebase:
- `src/ui/widgets/central_detail_panel.rs` - `Overflow::clip()`
- `src/screens/spell_test_modal.rs` - `Overflow::clip_y()`
- `src/screens/inventory_modal/render.rs` - `Overflow::clip_y()`
- `src/screens/monster_compendium.rs` - `Overflow::clip()`

## Files Using These Helpers

- `src/ui/widgets/gold_display.rs` - gold amount display row
- `src/ui/widgets/player_stats.rs` - HP and gold stat rows
- `src/screens/keybinds.rs` - category containers
- `src/screens/dungeon/rest.rs` - action list
- `src/screens/dungeon/room_entry.rs` - action list
