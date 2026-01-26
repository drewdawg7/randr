//! Composable tile variant selection strategies for visual variety.
//!
//! The `VariantStrategy` trait enables configurable tile variant selection.
//! Different strategies can create uniform looks, weighted random distribution,
//! organic clusters, or deterministic patterns.

use rand::Rng;

use super::tile::TileType;

/// How to choose tile variants for visual variety.
pub trait VariantStrategy: Send + Sync {
    /// Choose a variant for a tile at the given position.
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        tile_type: TileType,
        rng: &mut impl Rng,
    ) -> u8;
}

/// Enum-based variant strategy for composability without dyn.
#[derive(Clone)]
pub enum VariantStrategyKind {
    Uniform(UniformVariant),
    Percentage(PercentageVariant),
    Pattern(PatternVariant),
    Checkerboard(CheckerboardVariant),
    Clustered(ClusteredVariant),
    TileType(TileTypeVariant),
}

impl VariantStrategy for VariantStrategyKind {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        tile_type: TileType,
        rng: &mut impl Rng,
    ) -> u8 {
        match self {
            Self::Uniform(s) => s.choose_variant(x, y, tile_type, rng),
            Self::Percentage(s) => s.choose_variant(x, y, tile_type, rng),
            Self::Pattern(s) => s.choose_variant(x, y, tile_type, rng),
            Self::Checkerboard(s) => s.choose_variant(x, y, tile_type, rng),
            Self::Clustered(s) => s.choose_variant(x, y, tile_type, rng),
            Self::TileType(s) => s.choose_variant(x, y, tile_type, rng),
        }
    }
}

/// Single variant only (uniform look).
#[derive(Clone, Copy)]
pub struct UniformVariant(pub u8);

impl VariantStrategy for UniformVariant {
    fn choose_variant(
        &self,
        _x: usize,
        _y: usize,
        _tile_type: TileType,
        _rng: &mut impl Rng,
    ) -> u8 {
        self.0
    }
}

/// Weighted random selection (current default behavior).
///
/// Each entry is `(variant_id, weight)`. The probability of selecting
/// a variant is its weight divided by the total weight.
#[derive(Clone)]
pub struct PercentageVariant {
    /// (variant_id, weight) pairs
    pub weights: Vec<(u8, u32)>,
}

impl PercentageVariant {
    /// Creates the default variant distribution (75% variant 0, 25% split among 1-4).
    pub fn default_floor() -> Self {
        Self {
            weights: vec![(0, 75), (1, 6), (2, 6), (3, 6), (4, 7)],
        }
    }
}

impl VariantStrategy for PercentageVariant {
    fn choose_variant(
        &self,
        _x: usize,
        _y: usize,
        _tile_type: TileType,
        rng: &mut impl Rng,
    ) -> u8 {
        if self.weights.is_empty() {
            return 0;
        }

        let total_weight: u32 = self.weights.iter().map(|(_, w)| w).sum();
        if total_weight == 0 {
            return 0;
        }

        let roll = rng.gen_range(0..total_weight);
        let mut cumulative = 0;

        for (variant, weight) in &self.weights {
            cumulative += weight;
            if roll < cumulative {
                return *variant;
            }
        }

        self.weights[0].0
    }
}

/// Deterministic pattern based on position.
///
/// The pattern function receives (x, y) and returns the variant to use.
#[derive(Clone, Copy)]
pub struct PatternVariant {
    pattern: fn(x: usize, y: usize) -> u8,
}

impl PatternVariant {
    pub fn new(pattern: fn(x: usize, y: usize) -> u8) -> Self {
        Self { pattern }
    }
}

impl VariantStrategy for PatternVariant {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        _tile_type: TileType,
        _rng: &mut impl Rng,
    ) -> u8 {
        (self.pattern)(x, y)
    }
}

/// Checkerboard pattern alternating between two variants.
#[derive(Clone, Copy)]
pub struct CheckerboardVariant {
    pub variant_a: u8,
    pub variant_b: u8,
}

impl CheckerboardVariant {
    pub fn new(variant_a: u8, variant_b: u8) -> Self {
        Self { variant_a, variant_b }
    }
}

impl VariantStrategy for CheckerboardVariant {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        _tile_type: TileType,
        _rng: &mut impl Rng,
    ) -> u8 {
        if (x + y) % 2 == 0 {
            self.variant_a
        } else {
            self.variant_b
        }
    }
}

/// Clustered variants (same variant in nearby tiles) for organic appearance.
///
/// Uses position-based hashing to create deterministic clusters that
/// look organic without requiring actual randomness storage.
#[derive(Clone, Copy)]
pub struct ClusteredVariant {
    /// Size of clusters in tiles (larger = bigger clusters)
    pub cluster_size: usize,
    /// Seed for deterministic clustering
    pub seed: u64,
    /// Number of variant options (0..variant_count)
    pub variant_count: u8,
}

impl ClusteredVariant {
    pub fn new(cluster_size: usize, seed: u64, variant_count: u8) -> Self {
        Self {
            cluster_size: cluster_size.max(1),
            seed,
            variant_count: variant_count.max(1),
        }
    }
}

impl VariantStrategy for ClusteredVariant {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        _tile_type: TileType,
        _rng: &mut impl Rng,
    ) -> u8 {
        // Divide into clusters
        let cluster_x = x / self.cluster_size;
        let cluster_y = y / self.cluster_size;

        // Simple position-based hash for deterministic clusters
        let hash = self
            .seed
            .wrapping_mul(31)
            .wrapping_add(cluster_x as u64)
            .wrapping_mul(31)
            .wrapping_add(cluster_y as u64);

        (hash % self.variant_count as u64) as u8
    }
}

/// Different strategy per tile type.
///
/// Allows configuring different variant selection for walls vs floors.
#[derive(Clone)]
pub struct TileTypeVariant {
    pub floor_strategy: Box<VariantStrategyKind>,
    pub wall_strategy: Box<VariantStrategyKind>,
}

impl TileTypeVariant {
    pub fn new(floor: VariantStrategyKind, wall: VariantStrategyKind) -> Self {
        Self {
            floor_strategy: Box::new(floor),
            wall_strategy: Box::new(wall),
        }
    }
}

impl VariantStrategy for TileTypeVariant {
    fn choose_variant(
        &self,
        x: usize,
        y: usize,
        tile_type: TileType,
        rng: &mut impl Rng,
    ) -> u8 {
        match tile_type {
            TileType::Wall | TileType::TorchWall | TileType::Door | TileType::DoorOpen => {
                self.wall_strategy.choose_variant(x, y, tile_type, rng)
            }
            _ => self.floor_strategy.choose_variant(x, y, tile_type, rng),
        }
    }
}

/// Default strategy matching current behavior (75% variant 0, 25% others for floors).
pub fn default_strategy() -> VariantStrategyKind {
    VariantStrategyKind::Percentage(PercentageVariant::default_floor())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_always_returns_same_variant() {
        let strategy = UniformVariant(2);
        let mut rng = rand::thread_rng();

        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(strategy.choose_variant(x, y, TileType::Floor, &mut rng), 2);
            }
        }
    }

    #[test]
    fn percentage_respects_weights() {
        let strategy = PercentageVariant {
            weights: vec![(0, 100)], // 100% variant 0
        };
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            assert_eq!(strategy.choose_variant(0, 0, TileType::Floor, &mut rng), 0);
        }
    }

    #[test]
    fn percentage_empty_returns_zero() {
        let strategy = PercentageVariant { weights: vec![] };
        let mut rng = rand::thread_rng();

        assert_eq!(strategy.choose_variant(0, 0, TileType::Floor, &mut rng), 0);
    }

    #[test]
    fn checkerboard_alternates() {
        let strategy = CheckerboardVariant::new(0, 1);
        let mut rng = rand::thread_rng();

        assert_eq!(strategy.choose_variant(0, 0, TileType::Floor, &mut rng), 0);
        assert_eq!(strategy.choose_variant(1, 0, TileType::Floor, &mut rng), 1);
        assert_eq!(strategy.choose_variant(0, 1, TileType::Floor, &mut rng), 1);
        assert_eq!(strategy.choose_variant(1, 1, TileType::Floor, &mut rng), 0);
    }

    #[test]
    fn clustered_same_cluster_same_variant() {
        let strategy = ClusteredVariant::new(4, 12345, 5);
        let mut rng = rand::thread_rng();

        // All tiles in the same 4x4 cluster should have same variant
        let cluster_variant = strategy.choose_variant(0, 0, TileType::Floor, &mut rng);
        for x in 0..4 {
            for y in 0..4 {
                assert_eq!(
                    strategy.choose_variant(x, y, TileType::Floor, &mut rng),
                    cluster_variant
                );
            }
        }
    }

    #[test]
    fn tile_type_uses_correct_strategy() {
        let strategy = TileTypeVariant::new(
            VariantStrategyKind::Uniform(UniformVariant(1)),
            VariantStrategyKind::Uniform(UniformVariant(2)),
        );
        let mut rng = rand::thread_rng();

        assert_eq!(strategy.choose_variant(0, 0, TileType::Floor, &mut rng), 1);
        assert_eq!(strategy.choose_variant(0, 0, TileType::Wall, &mut rng), 2);
    }

    #[test]
    fn variant_strategy_kind_delegates() {
        let kind = VariantStrategyKind::Uniform(UniformVariant(3));
        let mut rng = rand::thread_rng();

        assert_eq!(kind.choose_variant(0, 0, TileType::Floor, &mut rng), 3);
    }
}
