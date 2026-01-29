# UI Node Helpers

## Overview
The `src/ui/nodes.rs` module provides helper functions for creating common Bevy UI `Node` layouts, reducing boilerplate when spawning row or column containers.

> **Note:** For new code, prefer using the `Row` and `Column` layout components from `src/ui/widgets/`. They provide more flexibility (padding, justify, align, sizing) with a builder API. See [widgets/layout_primitives.md](widgets/layout_primitives.md).

## Functions

| Function | Purpose |
|----------|---------|
| `row_node(gap)` | Horizontal row with centered items |
| `column_node(gap)` | Vertical column |
| `modal_content_row()` | Row for modal content (16px gap, FlexStart) |
| `separator_node()` | Horizontal separator (2px, full width) |
| `screen_root_bundle()` | Full-screen root (Node + BackgroundColor) |
| `screen_root_node()` | Just the node for custom backgrounds |

## When to Use

Use for simple containers. Use full `Node` struct if you need additional properties.

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
- `src/screens/inventory_modal/render.rs` - `Overflow::clip_y()`
- `src/screens/monster_compendium.rs` - `Overflow::clip()`

## Files Using These Helpers

- `src/ui/widgets/gold_display.rs` - gold amount display row
- `src/ui/widgets/player_stats.rs` - HP and gold stat rows
- `src/screens/keybinds.rs` - category containers
- `src/screens/dungeon/rest.rs` - action list
- `src/screens/dungeon/room_entry.rs` - action list

## Framed Widgets with Decorative Borders

Some widgets render decorative borders as absolute-positioned backgrounds (e.g., nine-slice frames). The widget's internal padding is typically small (for general layout), but the decorative border may be much larger.

**The Problem:** If you change `justify_content` to `FlexStart` or add content that starts at the top, it can overlap the decorative border area.

**Wrong approach:** Modifying the widget's internal padding to account for the border. This breaks the widget for other consumers and can cause the frame itself to render incorrectly.

**Correct approach:** Add margin to your content in the consuming code to push it below the decorative border.

```rust
// In render.rs (the consumer), NOT in the widget itself:
parent.spawn((
    Text::new(&item.name),
    TextFont { font_size: 18.0, ..default() },
    Node {
        // Push content below the decorative border
        margin: UiRect { top: Val::Px(36.0), ..default() },
        ..default()
    },
));
```

**Key principle:** Reusable widgets should not be modified for specific use cases. Handle positioning in the code that populates the widget's content.
