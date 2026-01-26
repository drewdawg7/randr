# Variant Strategy

Composable tile variant selection strategies for visual variety at `src/dungeon/variant_strategy.rs`.

## Core Concepts

The `VariantStrategy` trait enables configurable tile variant selection. Different strategies create uniform looks, weighted random distribution, organic clusters, or deterministic patterns.

### VariantStrategy Trait

```rust
pub trait VariantStrategy: Send + Sync {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        tile_type: TileType,
        rng: &mut impl Rng,
    ) -> u8;
}
```

### VariantStrategyKind Enum

Enum-based wrapper for type-safe composition without `dyn`:

```rust
pub enum VariantStrategyKind {
    Uniform(UniformVariant),
    Percentage(PercentageVariant),
    Pattern(PatternVariant),
    Checkerboard(CheckerboardVariant),
    Clustered(ClusteredVariant),
    TileType(TileTypeVariant),
}
```

## Strategy Types

### UniformVariant

Single variant only (uniform look):

```rust
use crate::dungeon::{VariantStrategyKind, UniformVariant};

// All tiles use variant 0
let strategy = VariantStrategyKind::Uniform(UniformVariant(0));
```

### PercentageVariant

Weighted random selection (default behavior):

```rust
use crate::dungeon::{VariantStrategyKind, PercentageVariant};

// 75% variant 0, 25% split among 1-4
let strategy = VariantStrategyKind::Percentage(PercentageVariant::default_floor());

// Custom weights
let custom = VariantStrategyKind::Percentage(PercentageVariant {
    weights: vec![(0, 50), (1, 25), (2, 25)],  // 50/25/25 split
});
```

### PatternVariant

Deterministic pattern based on position:

```rust
use crate::dungeon::{VariantStrategyKind, PatternVariant};

// Custom pattern function
let strategy = VariantStrategyKind::Pattern(PatternVariant::new(|x, y| {
    ((x + y) % 4) as u8  // Diagonal stripes
}));
```

### CheckerboardVariant

Alternating pattern between two variants:

```rust
use crate::dungeon::{VariantStrategyKind, CheckerboardVariant};

// Alternates between variant 0 and 1
let strategy = VariantStrategyKind::Checkerboard(CheckerboardVariant::new(0, 1));
```

### ClusteredVariant

Grouped variants for organic appearance:

```rust
use crate::dungeon::{VariantStrategyKind, ClusteredVariant};

// 4x4 clusters, 5 variants (0-4)
let strategy = VariantStrategyKind::Clustered(ClusteredVariant::new(
    4,      // cluster_size
    12345,  // seed for determinism
    5,      // variant_count
));
```

Uses position-based hashing for deterministic clusters without storing randomness.

### TileTypeVariant

Different strategy per tile type:

```rust
use crate::dungeon::{VariantStrategyKind, TileTypeVariant, UniformVariant, PercentageVariant};

// Floors use percentage, walls use uniform
let strategy = VariantStrategyKind::TileType(TileTypeVariant::new(
    VariantStrategyKind::Percentage(PercentageVariant::default_floor()),
    VariantStrategyKind::Uniform(UniformVariant(0)),
));
```

## LayoutBuilder Integration

```rust
use crate::dungeon::{LayoutBuilder, VariantStrategyKind, ClusteredVariant};

let layout = LayoutBuilder::new(40, 21)
    .variant_strategy(VariantStrategyKind::Clustered(ClusteredVariant::new(3, 12345, 5)))
    .entrance(20, 1)
    .door(20, 0)
    .build();
```

The default strategy is `PercentageVariant::default_floor()` which matches the original 75/25 behavior.

## File Structure

```
src/dungeon/
    variant_strategy.rs  # VariantStrategy trait + implementations
    layout_builder.rs    # Uses variant_strategy field
```

## Adding New Strategies

1. Create the strategy struct:
```rust
#[derive(Clone, Copy)]
pub struct MyStrategy {
    pub some_config: usize,
}
```

2. Implement `VariantStrategy`:
```rust
impl VariantStrategy for MyStrategy {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        tile_type: TileType,
        rng: &mut impl Rng,
    ) -> u8 {
        // Your logic here
        0
    }
}
```

3. Add variant to `VariantStrategyKind`:
```rust
pub enum VariantStrategyKind {
    // ... existing variants
    My(MyStrategy),
}
```

4. Update the match in `VariantStrategyKind` impl.

5. Export from `mod.rs`.

## Usage Examples

### Starting Room (Default)

```rust
// Uses default PercentageVariant::default_floor()
LayoutBuilder::new(w, h)
    .entrance(w / 2, 1)
    .door(w / 2, 0)
    .build()
```

### Clustered Floor

```rust
// Organic appearance with 3x3 clusters
LayoutBuilder::new(w, h)
    .variant_strategy(VariantStrategyKind::Clustered(ClusteredVariant::new(3, 54321, 5)))
    .entrance(w / 2, 1)
    .door(w / 2, 0)
    .build()
```

### Temple Floor (Uniform)

```rust
// Clean, uniform appearance
LayoutBuilder::new(w, h)
    .variant_strategy(VariantStrategyKind::Uniform(UniformVariant(0)))
    .entrance(w / 2, 1)
    .build()
```

## Related

- [mod.md](mod.md) - Dungeon module overview
- [room-patterns.md](room-patterns.md) - Similar trait pattern for room patterns
- [spawn-rules.md](spawn-rules.md) - Similar trait pattern for entity spawning
