use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::{ModalCommands, SelectionState};

use super::state::{CompendiumListState, CompendiumMonsters, MonsterCompendiumModal};

/// System to handle closing the monster compendium with Escape.
pub fn handle_compendium_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal != Some(ModalType::MonsterCompendium) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            commands.close_modal::<MonsterCompendiumModal>();
        }
    }
}

/// System to handle up/down navigation in the monster list.
pub fn handle_compendium_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut list_state: ResMut<CompendiumListState>,
    monsters: Option<Res<CompendiumMonsters>>,
) {
    if active_modal.modal != Some(ModalType::MonsterCompendium) {
        return;
    }

    let Some(monsters) = monsters else { return };

    // Update count from monsters resource
    if list_state.count != monsters.len() {
        list_state.count = monsters.len();
    }

    for action in action_reader.read() {
        if let GameAction::Navigate(dir) = action {
            match dir {
                NavigationDirection::Up => list_state.up(),
                NavigationDirection::Down => list_state.down(),
                _ => {}
            }
        }
    }
}
