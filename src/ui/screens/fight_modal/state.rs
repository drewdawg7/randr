//! Fight modal state and components.

use bevy::prelude::*;

use crate::mob::MobId;
use crate::ui::sprite_marker::{SpriteData, SpriteMarker};
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet};

/// Marker component for the fight modal root entity.
#[derive(Component)]
pub struct FightModalRoot;

/// Marker component for the player sprite in the fight modal.
#[derive(Component)]
pub struct FightModalPlayerSprite;

impl SpriteMarker for FightModalPlayerSprite {
    type Resources = Res<'static, PlayerSpriteSheet>;

    fn resolve(&self, sheet: &Res<PlayerSpriteSheet>) -> Option<SpriteData> {
        if !sheet.is_loaded() {
            return None;
        }
        Some(SpriteData {
            texture: sheet.texture.as_ref()?.clone(),
            layout: sheet.layout.as_ref()?.clone(),
            animation: sheet.animation.clone().into(),
            flip_x: false,
        })
    }
}

/// Marker component for the mob sprite in the fight modal.
#[derive(Component)]
pub struct FightModalMobSprite {
    pub mob_id: MobId,
}

impl SpriteMarker for FightModalMobSprite {
    type Resources = Res<'static, MobSpriteSheets>;

    fn resolve(&self, sheets: &Res<MobSpriteSheets>) -> Option<SpriteData> {
        let sheet = sheets.get(self.mob_id)?;
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation: sheet.animation.clone().into(),
            flip_x: true, // Flip to face left toward the player
        })
    }
}

/// Resource storing the mob encountered for the fight modal.
#[derive(Resource)]
pub struct FightModalMob {
    pub mob_id: MobId,
}

/// Marker resource to trigger spawning the fight modal.
#[derive(Resource)]
pub struct SpawnFightModal;
