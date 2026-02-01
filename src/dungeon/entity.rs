use bevy::prelude::*;

use crate::assets::SpriteSheetKey;
use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;
use crate::rock::RockType;

use super::grid::EntitySize;

#[derive(Component)]
pub struct DungeonEntityMarker {
    pub pos: Vec2,
    pub entity_type: DungeonEntity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DungeonEntity {
    Chest { variant: u8, size: EntitySize },
    Mob { mob_id: MobId, size: EntitySize },
    Npc { mob_id: MobId, size: EntitySize },
    Stairs { size: EntitySize },
    Rock { rock_type: RockType, sprite_variant: u8, size: EntitySize },
    CraftingStation { station_type: CraftingStationType, size: EntitySize },
    Door { size: EntitySize },
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
    /// Returns the size for this entity.
    pub fn size(&self) -> EntitySize {
        match self {
            Self::Chest { size, .. } => *size,
            Self::Mob { size, .. } => *size,
            Self::Npc { size, .. } => *size,
            Self::Stairs { size } => *size,
            Self::Rock { size, .. } => *size,
            Self::CraftingStation { size, .. } => *size,
            Self::Door { size } => *size,
        }
    }

    /// Returns rendering data for this entity.
    pub fn render_data(&self) -> EntityRenderData {
        match self {
            Self::Chest { .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::Chests,
                sprite_name: "Slice_1",
            },
            Self::Rock { rock_type, sprite_variant, .. } => {
                let (sheet_key, sprite_name) = rock_type.sprite_data(*sprite_variant);
                EntityRenderData::SpriteSheet { sheet_key, sprite_name }
            }
            Self::Stairs { .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::DungeonTileset,
                sprite_name: "stairs",
            },
            Self::CraftingStation { station_type, .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::CraftingStations,
                sprite_name: station_type.sprite_name(),
            },
            Self::Door { .. } => EntityRenderData::SpriteSheet {
                sheet_key: SpriteSheetKey::DungeonTileset,
                sprite_name: "gate",
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
