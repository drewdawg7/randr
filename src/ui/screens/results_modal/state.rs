//! Results modal state and components.

use bevy::prelude::*;

use crate::loot::LootDrop;
use crate::mob::MobId;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::sprite_marker::{SpriteData, SpriteMarker};
use crate::ui::MobSpriteSheets;

use super::render::do_spawn_results_modal;

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
        let animation = sheet
            .death_animation
            .clone()
            .unwrap_or_else(|| sheet.animation.clone());
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation,
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

/// Type-safe handle for the results modal.
pub struct ResultsModal;

impl RegisteredModal for ResultsModal {
    type Root = ResultsModalRoot;
    const MODAL_TYPE: ModalType = ModalType::ResultsModal;

    fn spawn(world: &mut World) {
        world.run_system_cached(do_spawn_results_modal).ok();
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<ResultsModalData>();
    }
}
