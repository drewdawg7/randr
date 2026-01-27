use crate::assets::SpriteSheetKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockType {
    Coal,
    Copper,
    Iron,
    Gold,
}

impl RockType {
    /// All rock types for iteration.
    pub const ALL: [RockType; 4] = [
        RockType::Coal,
        RockType::Copper,
        RockType::Iron,
        RockType::Gold,
    ];

    /// Returns the sprite sheet key and slice name for this rock type.
    /// The `variant` parameter (0 or 1) selects between the two sprite variants.
    pub fn sprite_data(&self, variant: u8) -> (SpriteSheetKey, &'static str) {
        match self {
            Self::Coal => (
                SpriteSheetKey::CaveTileset,
                if variant == 0 { "rock_1" } else { "rock_2" },
            ),
            Self::Copper => (
                SpriteSheetKey::CaveTileset,
                if variant == 0 { "copper_rock_1" } else { "copper_rock_2" },
            ),
            Self::Iron => (
                SpriteSheetKey::CaveTileset,
                if variant == 0 { "iron_rock_1" } else { "iron_rock_2" },
            ),
            Self::Gold => (
                SpriteSheetKey::CaveTileset,
                if variant == 0 { "gold_rock_1" } else { "gold_rock_2" },
            ),
        }
    }
}
