//! Victory modal state and components.

use bevy::prelude::*;

use crate::loot::LootDrop;
use crate::mob::MobId;
use crate::ui::sprite_marker::{SpriteData, SpriteMarker};
use crate::ui::MobSpriteSheets;

/// Marker component for the victory modal root entity.
#[derive(Component)]
pub struct VictoryModalRoot;

/// Marker component for the mob sprite in the victory modal.
#[derive(Component)]
pub struct VictoryModalMobSprite {
    pub mob_id: MobId,
}

impl SpriteMarker for VictoryModalMobSprite {
    type Resources = Res<'static, MobSpriteSheets>;

    fn resolve(&self, sheets: &Res<MobSpriteSheets>) -> Option<SpriteData> {
        let sheet = sheets.get(self.mob_id)?;
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation: sheet.animation.clone().into(),
            flip_x: false,
        })
    }
}

/// Resource containing victory data to display.
#[derive(Resource)]
pub struct VictoryModalData {
    pub mob_name: String,
    pub mob_id: MobId,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}

/// Marker resource to trigger spawning the victory modal.
#[derive(Resource)]
pub struct SpawnVictoryModal;
