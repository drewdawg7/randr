use bevy::prelude::*;

use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_skills_modal;

use super::state::SkillsModal;

pub struct SkillsModalPlugin;

impl Plugin for SkillsModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<SkillsModal>()
            .add_systems(Update, modal_close_system::<SkillsModal>.run_if(in_skills_modal));
    }
}
