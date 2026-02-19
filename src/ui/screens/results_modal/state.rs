use bevy::prelude::*;

use crate::loot::LootDrop;
use crate::mob::MobId;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

use super::render::do_spawn_results_modal;

#[derive(Component)]
pub struct ResultsModalRoot;

#[derive(Clone)]
pub enum ResultsSprite {
    Mob(MobId),
}

#[derive(Resource)]
pub struct ResultsModalData {
    pub title: String,
    pub subtitle: Option<String>,
    pub sprite: Option<ResultsSprite>,
    pub gold_gained: Option<i32>,
    pub xp_gained: Option<i32>,
    pub loot_drops: Vec<LootDrop>,
}

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
