use avian2d::prelude::Collider;
use bevy::prelude::Vec2;

pub const DEFAULT_TILE_SIZE: f32 = 32.0;

/// Multiplier for interaction radius relative to tile size.
pub const INTERACTION_RADIUS_MULTIPLIER: f32 = 0.3;
pub const CHEST_SPRITE_NAME: &str = "Slice_1";

/// Configuration for creating entity colliders from sprite dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColliderConfig {
    /// Scale factor for width (0.0 to 1.0+)
    pub scale_x: f32,
    /// Scale factor for height (0.0 to 1.0+)
    pub scale_y: f32,
    /// Y-axis offset for the collider center (negative = lower)
    pub offset_y: f32,
}

impl ColliderConfig {
    /// Uniform scale with no offset (most common case).
    pub const fn uniform(scale: f32) -> Self {
        Self {
            scale_x: scale,
            scale_y: scale,
            offset_y: 0.0,
        }
    }

    /// Uniform scale with Y offset.
    pub const fn uniform_with_offset(scale: f32, offset_y: f32) -> Self {
        Self {
            scale_x: scale,
            scale_y: scale,
            offset_y,
        }
    }

    /// Asymmetric scaling (e.g., forge).
    pub const fn asymmetric(scale_x: f32, scale_y: f32, offset_y: f32) -> Self {
        Self {
            scale_x,
            scale_y,
            offset_y,
        }
    }

    /// Create a collider from sprite dimensions and this config.
    pub fn create_collider(&self, sprite_size: Vec2) -> Collider {
        let width = sprite_size.x * self.scale_x;
        let height = sprite_size.y * self.scale_y;

        if self.offset_y == 0.0 {
            Collider::rectangle(width, height)
        } else {
            Collider::compound(vec![(
                Vec2::new(0.0, self.offset_y),
                0.0,
                Collider::rectangle(width, height),
            )])
        }
    }
}

impl Default for ColliderConfig {
    fn default() -> Self {
        STATIC_COLLIDER
    }
}

/// Default collider for static entities (chests, rocks, anvil, doors).
pub const STATIC_COLLIDER: ColliderConfig = ColliderConfig::uniform(0.9);

/// Collider for stairs (smaller hitbox).
pub const STAIRS_COLLIDER: ColliderConfig = ColliderConfig::uniform(0.6);

/// Collider for forge (asymmetric with offset).
pub const FORGE_COLLIDER: ColliderConfig = ColliderConfig::asymmetric(0.75, 1.0, -8.0);

/// Collider for mobs (half sprite size with offset for grounded feet).
pub const MOB_COLLIDER: ColliderConfig = ColliderConfig::uniform_with_offset(0.5, -8.0);

/// Collider for player (half sprite size with offset for grounded feet).
pub const PLAYER_COLLIDER: ColliderConfig = ColliderConfig::uniform_with_offset(0.5, -8.0);
