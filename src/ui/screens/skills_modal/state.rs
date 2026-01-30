use bevy::prelude::*;

use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

use super::render::do_spawn_skills_modal;

#[derive(Component)]
pub struct SkillsModalRoot;

pub struct SkillsModal;

impl RegisteredModal for SkillsModal {
    type Root = SkillsModalRoot;
    const MODAL_TYPE: ModalType = ModalType::SkillsModal;

    fn spawn(world: &mut World) {
        world.run_system_cached(do_spawn_skills_modal).ok();
    }
}
