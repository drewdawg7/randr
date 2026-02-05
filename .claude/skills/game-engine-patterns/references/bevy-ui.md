# Bevy UI

Overview of Bevy 0.18 UI system for building game interfaces.

## Quick Reference

```rust
// Basic UI node
commands.spawn(Node {
    width: Val::Percent(100.0),
    height: Val::Px(50.0),
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
});

// Text
commands.spawn((
    Text::new("Hello World"),
    TextFont { font_size: 24.0, ..default() },
    TextColor(Color::WHITE),
));

// Image
commands.spawn(ImageNode::new(texture_handle));

// With children
commands.spawn(Node { ... }).with_children(|parent| {
    parent.spawn(Text::new("Child"));
});
```

## Node & Layout

### Val (Size Values)

| Value | Description |
|-------|-------------|
| `Val::Auto` | Automatic sizing (default) |
| `Val::Px(f32)` | Pixels |
| `Val::Percent(f32)` | Percentage of parent |
| `Val::Vw(f32)` | Viewport width percentage |
| `Val::Vh(f32)` | Viewport height percentage |
| `Val::VMin(f32)` | Min of viewport width/height |
| `Val::VMax(f32)` | Max of viewport width/height |

### Node Fields

```rust
Node {
    // Sizing
    width: Val::Auto,
    height: Val::Auto,
    min_width: Val::Auto,
    max_width: Val::Auto,
    min_height: Val::Auto,
    max_height: Val::Auto,

    // Display
    display: Display::Flex,          // Flex, Grid, Block, None
    position_type: PositionType::Relative,  // Relative, Absolute
    overflow: Overflow::visible(),   // visible(), clip(), scroll()

    // Flexbox
    flex_direction: FlexDirection::Row,
    flex_wrap: FlexWrap::NoWrap,
    justify_content: JustifyContent::FlexStart,
    align_items: AlignItems::Stretch,
    align_content: AlignContent::FlexStart,

    // Individual item alignment
    align_self: AlignSelf::Auto,
    justify_self: JustifySelf::Auto,

    // Spacing
    padding: UiRect::all(Val::Px(0.0)),
    margin: UiRect::all(Val::Px(0.0)),
    row_gap: Val::Px(0.0),
    column_gap: Val::Px(0.0),

    // Positioning (when Absolute)
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,

    ..default()
}
```

### UiRect

```rust
// All sides same
UiRect::all(Val::Px(10.0))

// Horizontal and vertical
UiRect::axes(Val::Px(10.0), Val::Px(5.0))  // x, y

// Individual sides
UiRect {
    left: Val::Px(10.0),
    right: Val::Px(10.0),
    top: Val::Px(5.0),
    bottom: Val::Px(5.0),
}

// Single side
UiRect::left(Val::Px(10.0))
```

## Flexbox

### FlexDirection

| Value | Description |
|-------|-------------|
| `Row` | Left to right (default) |
| `RowReverse` | Right to left |
| `Column` | Top to bottom |
| `ColumnReverse` | Bottom to top |

### JustifyContent (Main Axis)

| Value | Description |
|-------|-------------|
| `FlexStart` | Pack at start |
| `FlexEnd` | Pack at end |
| `Center` | Pack at center |
| `SpaceBetween` | Even space between items |
| `SpaceAround` | Even space around items |
| `SpaceEvenly` | Equal space between and around |

### AlignItems (Cross Axis)

| Value | Description |
|-------|-------------|
| `FlexStart` | Align at start |
| `FlexEnd` | Align at end |
| `Center` | Align at center |
| `Stretch` | Stretch to fill (default) |
| `Baseline` | Align baselines |

### FlexWrap

| Value | Description |
|-------|-------------|
| `NoWrap` | Single line (default) |
| `Wrap` | Multiple lines |
| `WrapReverse` | Multiple lines, reversed |

### Codebase Pattern: Row/Column Helpers

From `src/ui/nodes.rs`:
```rust
pub fn row_node(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(gap),
        ..default()
    }
}

pub fn column_node(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::FlexStart,
        row_gap: Val::Px(gap),
        ..default()
    }
}
```

## Text

### Components

| Component | Purpose |
|-----------|---------|
| `Text` | The text content string |
| `TextFont` | Font, size, weight |
| `TextColor` | Text color |
| `TextLayout` | Justify, line breaking |

### Basic Text

```rust
commands.spawn((
    Text::new("Hello World"),
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 24.0,
        ..default()
    },
    TextColor(Color::WHITE),
));
```

### TextLayout

```rust
commands.spawn((
    Text::new("Centered\nMultiline"),
    TextLayout::new_with_justify(Justify::Center),
));
```

**Justify variants**: `Left`, `Center`, `Right`, `Justified`

**LineBreak variants**:
| Value | Description |
|-------|-------------|
| `WordBoundary` | Break at word boundaries (default) |
| `AnyCharacter` | Break at any character |
| `WordOrCharacter` | Word-level, fallback to character |
| `NoWrap` | No soft wrapping |

### Text Spans (Styled Sections)

```rust
commands.spawn(Text::new("Normal ")).with_children(|parent| {
    parent.spawn((
        TextSpan::new("Bold"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::RED),
    ));
    parent.spawn(TextSpan::new(" more normal"));
});
```

### Codebase Pattern: Text Builder

From `src/ui/text.rs`:
```rust
pub struct UiText {
    text: String,
    font_size: f32,
    color: Color,
}

impl UiText {
    pub fn new(text: impl Into<String>) -> Self { ... }
    pub fn size(mut self, size: f32) -> Self { ... }
    pub fn color(mut self, color: Color) -> Self { ... }

    // Presets
    pub fn title(text: impl Into<String>) -> Self {
        Self::new(text).size(48.0).color(CREAM)
    }
    pub fn label(text: impl Into<String>) -> Self {
        Self::new(text).size(16.0).color(GRAY)
    }

    pub fn build(self) -> (Text, TextFont, TextColor) { ... }
}
```

## ImageNode

### Basic Image

```rust
commands.spawn(ImageNode::new(texture_handle));
```

### With Configuration

```rust
commands.spawn(ImageNode {
    image: texture_handle,
    color: Color::WHITE,           // Tint
    flip_x: false,
    flip_y: false,
    rect: None,                    // Optional subregion
    image_mode: NodeImageMode::Auto,
    texture_atlas: None,
    ..default()
});
```

### From Texture Atlas

```rust
commands.spawn(ImageNode::from_atlas_image(
    texture_handle,
    TextureAtlas {
        layout: atlas_layout_handle,
        index: 0,
    },
));
```

### NodeImageMode

| Mode | Description |
|------|-------------|
| `Auto` | Preserve aspect ratio |
| `Stretch` | Stretch to fill |
| `Sliced(TextureSlicer)` | 9-slice scaling |
| `Tiled { ... }` | Tile the image |

### Nine-Slice (Sliced Mode)

```rust
ImageNode {
    image_mode: NodeImageMode::Sliced(TextureSlicer {
        border: BorderRect {
            min_inset: Vec2::new(8.0, 8.0),
            max_inset: Vec2::new(8.0, 8.0),
        },
        ..default()
    }),
    ..default()
}
```

## Z-Ordering

### ZIndex (Local)

Relative to siblings:

```rust
commands.spawn((Node { ... }, ZIndex(1)));  // Above siblings with lower ZIndex
```

### GlobalZIndex

Escape hierarchy ordering:

```rust
commands.spawn((Node { ... }, GlobalZIndex(100)));  // Above most UI
```

**Ordering rules**:
1. Compare `GlobalZIndex` first (regardless of hierarchy)
2. Same `GlobalZIndex`: use `ZIndex` within siblings
3. No explicit index: treated as 0

### Codebase Pattern: Stack Widget

From `src/ui/widgets/stack.rs`:
```rust
// Children stack on top of each other
commands.spawn(Stack::new()).with_children(|stack| {
    stack.spawn(ImageNode::new(background));  // Bottom layer
    stack.spawn((
        ImageNode::new(foreground),
        ZIndex(1),  // On top
    ));
});
```

## Positioning

### PositionType

| Type | Description |
|------|-------------|
| `Relative` | Normal flow (default) |
| `Absolute` | Positioned relative to parent |

### Absolute Positioning

```rust
// Parent establishes positioning context
commands.spawn(Node {
    position_type: PositionType::Relative,
    width: Val::Px(200.0),
    height: Val::Px(200.0),
    ..default()
}).with_children(|parent| {
    // Child positioned relative to parent
    parent.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        top: Val::Px(10.0),
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        ..default()
    });
});
```

## Overflow

```rust
Node {
    overflow: Overflow::visible(),   // Content can overflow
    overflow: Overflow::clip(),      // Content clipped to bounds
    overflow: Overflow::scroll(),    // Scrollable (future)

    // Per-axis
    overflow: Overflow {
        x: OverflowAxis::Clip,
        y: OverflowAxis::Visible,
    },
    ..default()
}
```

## Interaction

### Interaction Component

```rust
commands.spawn((
    Button,
    Node { ... },
    BackgroundColor(Color::GRAY),
));

fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Pressed => *color = Color::RED.into(),
            Interaction::Hovered => *color = Color::LIGHT_GRAY.into(),
            Interaction::None => *color = Color::GRAY.into(),
        }
    }
}
```

### FocusPolicy

| Policy | Description |
|--------|-------------|
| `Block` | Blocks input to children |
| `Pass` | Passes input to children (default) |

## This Codebase

### Pattern: Widget Components + Observers

From `src/ui/widgets/column.rs`:
```rust
#[derive(Component)]
pub struct Column {
    pub gap: f32,
    pub align: AlignItems,
    pub justify: JustifyContent,
}

// Observer transforms component into Node
fn on_add_column(trigger: On<Add, Column>, mut commands: Commands, query: Query<&Column>) {
    let entity = trigger.entity;
    let col = query.get(entity).unwrap();

    commands.entity(entity).remove::<Column>().insert(Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(col.gap),
        align_items: col.align,
        justify_content: col.justify,
        ..default()
    });
}

// Usage: declarative API
commands.spawn(Column { gap: 10.0, ..default() });
```

### Pattern: Screen Root Node

From `src/ui/nodes.rs`:
```rust
pub fn screen_root_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }
}
```

### Pattern: Marker Components for Queries

```rust
#[derive(Component)]
struct GridContainer;

#[derive(Component)]
struct GridCell { index: usize }

// Efficient filtering
fn update_grid(
    container: Query<&Children, With<GridContainer>>,
    cells: Query<&GridCell>,
) { ... }
```

## Common Mistakes

### Percentage width on text container
```rust
// Wrong: text wrapping may not work with percent widths
Node { width: Val::Percent(50.0), ... }

// Correct: use pixel width for reliable text wrapping
Node { width: Val::Px(200.0), ... }
```

### Forgetting parent size for percentages
```rust
// Wrong: child percentage has no reference
commands.spawn(Node::default()).with_children(|p| {
    p.spawn(Node { width: Val::Percent(50.0), ... }); // 50% of what?
});

// Correct: parent has explicit size
commands.spawn(Node { width: Val::Px(400.0), ... }).with_children(|p| {
    p.spawn(Node { width: Val::Percent(50.0), ... }); // 200px
});
```

### Absolute without relative parent
```rust
// Wrong: absolute child without positioning context
commands.spawn(Node::default()).with_children(|p| {
    p.spawn(Node { position_type: PositionType::Absolute, ... });
});

// Correct: parent establishes context
commands.spawn(Node { position_type: PositionType::Relative, ... }).with_children(|p| {
    p.spawn(Node { position_type: PositionType::Absolute, ... });
});
```

### Using justify_self in flexbox
```rust
// Wrong: justify_self has no effect in flexbox
Node { justify_self: JustifySelf::Center, ... }

// Correct: use justify_content on parent or margin: auto
```
