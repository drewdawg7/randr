use crate::assets::SpriteSheetKey;
use crate::mob::MobId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonEntity {
    Chest { variant: u8 },
    Mob { mob_id: MobId },
}

impl DungeonEntity {
    /// Returns the sprite sheet key for static entities.
    /// Panics if called on a Mob entity (use `mob_id()` instead).
    pub fn sprite_sheet_key(&self) -> SpriteSheetKey {
        match self {
            Self::Chest { variant } => match variant % 4 {
                0 => SpriteSheetKey::Chest1,
                1 => SpriteSheetKey::Chest2,
                2 => SpriteSheetKey::Chest3,
                _ => SpriteSheetKey::Chest4,
            },
            Self::Mob { .. } => panic!("Mob entities use DungeonMobSprite marker"),
        }
    }

    /// Returns the sprite name for static entities.
    /// Panics if called on a Mob entity (use `mob_id()` instead).
    pub fn sprite_name(&self) -> &'static str {
        match self {
            Self::Chest { variant } => match variant % 4 {
                0 => "chest_1",
                1 => "chest_2",
                2 => "chest_3",
                _ => "chest_4",
            },
            Self::Mob { .. } => panic!("Mob entities use DungeonMobSprite marker"),
        }
    }

    /// Returns the MobId if this is a Mob entity.
    pub fn mob_id(&self) -> Option<MobId> {
        match self {
            Self::Mob { mob_id } => Some(*mob_id),
            _ => None,
        }
    }
}
