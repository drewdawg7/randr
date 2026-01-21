use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use super::super::modal::{close_modal, toggle_modal, ActiveModal, ModalAction, ModalType};

use super::state::{
    CompendiumListState, CompendiumMonsters, MonsterCompendiumRoot, SpawnMonsterCompendium,
};

/// System to handle opening the monster compendium with 'b' key.
pub fn handle_compendium_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    mut list_state: ResMut<CompendiumListState>,
    existing_compendium: Query<Entity, With<MonsterCompendiumRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenCompendium {
            match toggle_modal(
                &mut commands,
                &mut active_modal,
                &existing_compendium,
                ModalType::MonsterCompendium,
            ) {
                Some(ModalAction::Closed) => {
                    commands.remove_resource::<CompendiumMonsters>();
                }
                Some(ModalAction::Open) => {
                    list_state.selected = 0;
                    commands.insert_resource(CompendiumMonsters::from_registry());
                    commands.insert_resource(SpawnMonsterCompendium);
                }
                None => {}
            }
        }
    }
}

/// System to handle closing the monster compendium with Escape.
pub fn handle_compendium_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    compendium_query: Query<Entity, With<MonsterCompendiumRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal
            && close_modal(
                &mut commands,
                &mut active_modal,
                &compendium_query,
                ModalType::MonsterCompendium,
            )
        {
            commands.remove_resource::<CompendiumMonsters>();
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
    let count = monsters.len();
    for action in action_reader.read() {
        if let GameAction::Navigate(dir) = action {
            match dir {
                NavigationDirection::Up => {
                    if list_state.selected > 0 {
                        list_state.selected -= 1;
                    }
                }
                NavigationDirection::Down => {
                    if list_state.selected < count.saturating_sub(1) {
                        list_state.selected += 1;
                    }
                }
                _ => {}
            }
        }
    }
}
