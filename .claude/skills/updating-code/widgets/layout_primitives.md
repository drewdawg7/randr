# Layout Primitives (Row, Column, Stack)

Declarative layout components for building UI hierarchies.

**Files:**
- `src/ui/widgets/row.rs`
- `src/ui/widgets/column.rs`
- `src/ui/widgets/stack.rs`

## Overview

These components use the observer pattern (like `StatRow`) to provide a declarative API for common layout containers. When spawned, they're automatically replaced with properly configured `Node` components.

## Row

Horizontal layout container.

```rust
use crate::ui::widgets::Row;

// Basic row with gap
parent.spawn(Row::new().gap(10.0));

// Row with centered items (default alignment)
parent.spawn(Row::new().gap(8.0).align_center());

// Row with space between items
parent.spawn(
    Row::new()
        .gap(10.0)
        .justify(JustifyContent::SpaceBetween)
        .padding(UiRect::all(Val::Px(10.0)))
);

// Row with specific size
parent.spawn(Row::new().gap(5.0).width(Val::Percent(100.0)));
```

### Builder Methods

| Method | Default | Description |
|--------|---------|-------------|
| `new()` | - | Creates row with default settings |
| `gap(f32)` | 0.0 | Gap between child elements |
| `justify(JustifyContent)` | FlexStart | Main axis alignment |
| `align(AlignItems)` | Center | Cross axis alignment |
| `align_center()` | - | Convenience for `align(AlignItems::Center)` |
| `padding(UiRect)` | default | Inner padding |
| `width(Val)` | Auto | Row width |
| `height(Val)` | Auto | Row height |

**Note:** Default `align: AlignItems::Center` matches the behavior of `row_node()` helper.

## Column

Vertical layout container.

```rust
use crate::ui::widgets::Column;

// Basic column with gap
parent.spawn(Column::new().gap(5.0));

// Column with centered items
parent.spawn(Column::new().gap(10.0).align_center());

// Column with specific width
parent.spawn(
    Column::new()
        .gap(8.0)
        .width(Val::Px(200.0))
        .padding(UiRect::all(Val::Px(10.0)))
);
```

### Builder Methods

| Method | Default | Description |
|--------|---------|-------------|
| `new()` | - | Creates column with default settings |
| `gap(f32)` | 0.0 | Gap between child elements |
| `justify(JustifyContent)` | FlexStart | Main axis alignment |
| `align(AlignItems)` | Stretch | Cross axis alignment |
| `align_center()` | - | Convenience for `align(AlignItems::Center)` |
| `padding(UiRect)` | default | Inner padding |
| `width(Val)` | Auto | Column width |
| `height(Val)` | Auto | Column height |

**Note:** Default `align: AlignItems::Stretch` means children expand to fill column width.

## Stack

Z-layered container for overlapping elements.

```rust
use crate::ui::widgets::Stack;

// Basic stack container
parent.spawn(Stack::new())
    .with_children(|stack| {
        // Background layer (first child = bottom)
        stack.spawn((
            ImageNode::new(background_image),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ));
        // Foreground layer (renders on top)
        stack.spawn((
            ImageNode::new(foreground_image),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ZIndex(1),
        ));
    });

// Stack with specific size
parent.spawn(Stack::new().width(Val::Px(200.0)).height(Val::Px(150.0)));
```

### Builder Methods

| Method | Default | Description |
|--------|---------|-------------|
| `new()` | - | Creates stack with default settings |
| `width(Val)` | Auto | Stack width |
| `height(Val)` | Auto | Stack height |
| `justify(JustifyContent)` | Center | Main axis (for non-absolute children) |
| `align(AlignItems)` | Center | Cross axis (for non-absolute children) |

### Important Notes

- Children should use `PositionType::Absolute` to stack properly
- Later children render on top of earlier children (natural z-ordering)
- Use `ZIndex` component for explicit z-order control
- The Stack itself uses `PositionType::Relative` so absolute children position relative to it

## Migration from Helper Functions

The layout primitives supersede the simpler `row_node()` and `column_node()` helpers:

```rust
// Before
parent.spawn(row_node(4.0));

// After (equivalent)
parent.spawn(Row::new().gap(4.0));

// Before
parent.spawn(column_node(15.0));

// After (equivalent)
parent.spawn(Column::new().gap(15.0));
```

### When to Use Each

| Use Case | Recommendation |
|----------|----------------|
| Simple gap-only layout | Either works, `Row`/`Column` preferred for consistency |
| Need padding, justify, or sizing | Use `Row`/`Column` |
| Z-layered overlapping elements | Use `Stack` |

## Plugin Registration

All three plugins are registered in `src/plugins/game.rs`:

```rust
use crate::ui::widgets::{ColumnPlugin, RowPlugin, StackPlugin};

app.add_plugins((
    // ... other plugins ...
    RowPlugin,
    ColumnPlugin,
    StackPlugin,
));
```

## Pattern: Observer-Based Widgets

These components follow the same observer pattern as `StatRow`:

1. Component struct with builder methods
2. Plugin registers `OnAdd` observer
3. Observer captures config values
4. Observer removes component and inserts expanded `Node`

This pattern provides a declarative API while still resulting in standard Bevy UI nodes.
