use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::{FocusPanel, FocusState, SelectionState};

use super::state::{CompendiumListState, CompendiumMonsters, DropsListState};

pub fn handle_compendium_tab(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut focus_state: Option<ResMut<FocusState>>,
) {
    if active_modal.modal != Some(ModalType::MonsterCompendium) {
        return;
    }

    let Some(ref mut focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            focus_state.toggle_between(
                FocusPanel::CompendiumMonsterList,
                FocusPanel::CompendiumDropsList,
            );
        }
    }
}

pub fn handle_compendium_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    focus_state: Option<Res<FocusState>>,
    mut list_state: ResMut<CompendiumListState>,
    mut drops_state: Option<ResMut<DropsListState>>,
    monsters: Option<Res<CompendiumMonsters>>,
) {
    if active_modal.modal != Some(ModalType::MonsterCompendium) {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(focus_state) = focus_state else { return };

    if list_state.count != monsters.len() {
        list_state.count = monsters.len();
    }

    for action in action_reader.read() {
        if let GameAction::Navigate(dir) = action {
            match dir {
                NavigationDirection::Up | NavigationDirection::Down => {
                    if focus_state.is_focused(FocusPanel::CompendiumMonsterList) {
                        match dir {
                            NavigationDirection::Up => list_state.up(),
                            NavigationDirection::Down => list_state.down(),
                            _ => {}
                        }
                    } else if focus_state.is_focused(FocusPanel::CompendiumDropsList) {
                        if let Some(ref mut drops) = drops_state {
                            match dir {
                                NavigationDirection::Up => drops.up(),
                                NavigationDirection::Down => drops.down(),
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
