# Room Patterns

Composable tile placement system for dungeon layouts at `src/dungeon/room_patterns.rs`.

## Core Types

### Rect

Rectangular bounds for pattern application:

```rust
use crate::dungeon::Rect;

let bounds = Rect::new(5, 5, 10, 10); // x, y, width, height
bounds.right();      // 15 (x + width)
bounds.bottom();     // 15 (y + height)
bounds.contains(7, 7); // true
```

### RoomPattern Trait

```rust
pub trait RoomPattern {
    fn apply(&self, layout: &mut DungeonLayout, bounds: Rect, rng: &mut impl Rng);
}
```

### RoomPatternKind Enum

Enum wrapper for pattern composition. Add variants here when implementing new patterns:

```rust
#[derive(Clone)]
pub enum RoomPatternKind {
    // Add pattern variants here
}
```

### ComposedPattern

Layer multiple patterns in sequence:

```rust
use crate::dungeon::{ComposedPattern, RoomPatternKind};

let pattern = ComposedPattern::new()
    .add(RoomPatternKind::SomePattern(...))
    .add(RoomPatternKind::AnotherPattern(...));
```

## LayoutBuilder Integration

```rust
use crate::dungeon::{LayoutBuilder, Rect, RoomPatternKind};

let layout = LayoutBuilder::new(40, 30)
    .pattern(RoomPatternKind::SomePattern(...))  // Apply to full layout
    .pattern_at(Rect::new(5, 5, 10, 10), RoomPatternKind::AnotherPattern(...))
    .entrance(20, 28)
    .build();
```

Patterns are applied in order during `build()`, after the default floor/wall setup but before entrance/exit/door/torches/spawns.

## Adding New Patterns

1. Create the pattern struct:
```rust
#[derive(Clone, Copy)]
pub struct MyPattern {
    pub some_config: usize,
}
```

2. Implement `RoomPattern`:
```rust
impl RoomPattern for MyPattern {
    fn apply(&self, layout: &mut DungeonLayout, bounds: Rect, rng: &mut impl Rng) {
        for y in bounds.y..bounds.bottom() {
            for x in bounds.x..bounds.right() {
                // Place tiles based on pattern logic
                layout.set_tile(x, y, Tile::new(TileType::Floor));
            }
        }
    }
}
```

3. Add variant to `RoomPatternKind`:
```rust
pub enum RoomPatternKind {
    MyPattern(MyPattern),
}
```

4. Update the match in `RoomPatternKind::apply()`.

5. Export from `mod.rs` if needed.

## Related

- [mod.md](mod.md) - Dungeon module overview
- [spawn-rules.md](spawn-rules.md) - Entity spawning (similar pattern)
