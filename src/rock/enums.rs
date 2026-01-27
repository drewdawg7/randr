use crate::assets::SpriteSheetKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockType {
    Iron,
    Coal,
    Gold,
}

impl RockType {
    /// Returns the sprite sheet key and slice name for this rock type.
    pub fn sprite_data(&self) -> (SpriteSheetKey, &'static str) {
        match self {
            Self::Iron => (SpriteSheetKey::CraftingMaterials, "iron_rock"),
            Self::Gold => (SpriteSheetKey::CraftingMaterials, "gold_rock"),
            Self::Coal => (SpriteSheetKey::Rocks, "coal_gold_rock"),
        }
    }
}
