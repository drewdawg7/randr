use bevy::prelude::*;

use crate::input::close_results_modal;
use crate::ui::modal_registry::RegisterModalExt;
use crate::ui::screens::modal::in_results_modal;

use super::state::ResultsModal;

pub struct ResultsModalPlugin;

impl Plugin for ResultsModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<ResultsModal>()
            .add_systems(Update, close_results_modal.run_if(in_results_modal));
    }
}
