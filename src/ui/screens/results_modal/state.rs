//! Results modal state and components.

use bevy::prelude::*;

use crate::loot::LootDrop;
use crate::mob::MobId;
use crate::ui::sprite_marker::{SpriteData, SpriteMarker};
use crate::ui::MobSpriteSheets;

/// Marker component for the results modal root entity.
#[derive(Component)]
pub struct ResultsModalRoot;

/// What sprite to display in the results modal.
#[derive(Clone)]
pub enum ResultsSprite {
    Mob(MobId),
}

/// Marker component for the mob sprite in the results modal.
#[derive(Component)]
pub struct ResultsModalMobSprite {
    pub mob_id: MobId,
}

impl SpriteMarker for ResultsModalMobSprite {
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

/// Resource containing results data to display.
#[derive(Resource)]
pub struct ResultsModalData {
    pub title: String,
    pub subtitle: Option<String>,
    pub sprite: Option<ResultsSprite>,
    pub gold_gained: Option<i32>,
    pub xp_gained: Option<i32>,
    pub loot_drops: Vec<LootDrop>,
}

/// Marker resource to trigger spawning the results modal.
#[derive(Resource)]
pub struct SpawnResultsModal;
