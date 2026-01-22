use crate::assets::SpriteSheetKey;
use crate::mob::MobId;

use super::grid::GridSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonEntity {
    Chest { variant: u8, size: GridSize },
    Mob { mob_id: MobId, size: GridSize },
    Stairs { size: GridSize },
}

impl DungeonEntity {
    /// Returns the grid size for this entity.
    pub fn size(&self) -> GridSize {
        match self {
            Self::Chest { size, .. } => *size,
            Self::Mob { size, .. } => *size,
            Self::Stairs { size } => *size,
        }
    }

    /// Returns the sprite sheet key for static entities.
    /// Panics if called on a Mob entity (use `mob_id()` instead).
    pub fn sprite_sheet_key(&self) -> SpriteSheetKey {
        match self {
            Self::Chest { variant, .. } => match variant % 4 {
                0 => SpriteSheetKey::Chest1,
                1 => SpriteSheetKey::Chest2,
                2 => SpriteSheetKey::Chest3,
                _ => SpriteSheetKey::Chest4,
            },
            Self::Mob { .. } => panic!("Mob entities use DungeonMobSprite marker"),
            Self::Stairs { .. } => SpriteSheetKey::DungeonTileset,
        }
    }

    /// Returns the sprite name for static entities.
    /// Panics if called on a Mob entity (use `mob_id()` instead).
    pub fn sprite_name(&self) -> &'static str {
        match self {
            Self::Chest { variant, .. } => match variant % 4 {
                0 => "chest_1",
                1 => "chest_2",
                2 => "chest_3",
                _ => "chest_4",
            },
            Self::Mob { .. } => panic!("Mob entities use DungeonMobSprite marker"),
            Self::Stairs { .. } => "stairs",
        }
    }

    /// Returns the MobId if this is a Mob entity.
    pub fn mob_id(&self) -> Option<MobId> {
        match self {
            Self::Mob { mob_id, .. } => Some(*mob_id),
            _ => None,
        }
    }
}
