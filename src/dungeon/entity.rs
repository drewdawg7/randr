use crate::assets::SpriteSheetKey;
use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;
use crate::rock::RockType;

use super::grid::GridSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonEntity {
    Chest { variant: u8, size: GridSize },
    Mob { mob_id: MobId, size: GridSize },
    Npc { mob_id: MobId, size: GridSize },
    Stairs { size: GridSize },
    Rock { rock_type: RockType, size: GridSize },
    CraftingStation { station_type: CraftingStationType, size: GridSize },
}

/// Describes how a DungeonEntity should be rendered in the grid.
pub enum EntityRenderData {
    /// Static sprite from a named sprite sheet.
    SpriteSheet {
        sheet_key: SpriteSheetKey,
        sprite_name: &'static str,
    },
    /// Animated mob sprite using the SpriteMarker observer system.
    AnimatedMob { mob_id: MobId },
}

impl DungeonEntity {
    /// Returns the grid size for this entity.
    pub fn size(&self) -> GridSize {
        match self {
            Self::Chest { size, .. } => *size,
            Self::Mob { size, .. } => *size,
            Self::Npc { size, .. } => *size,
            Self::Stairs { size } => *size,
            Self::Rock { size, .. } => *size,
            Self::CraftingStation { size, .. } => *size,
        }
    }

    /// Returns rendering data for this entity.
    pub fn render_data(&self) -> EntityRenderData {
        match self {
            Self::Chest { .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::Chests,
                sprite_name: "Slice_1",
            },
            Self::Rock { rock_type, .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::Rocks,
                sprite_name: rock_type.sprite_name(),
            },
            Self::Stairs { .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::DungeonTileset,
                sprite_name: "stairs",
            },
            Self::CraftingStation { station_type, .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::Forge,
                sprite_name: station_type.sprite_name(),
            },
            Self::Mob { mob_id, .. } => EntityRenderData::AnimatedMob { mob_id: *mob_id },
            Self::Npc { mob_id, .. } => EntityRenderData::AnimatedMob { mob_id: *mob_id },
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
