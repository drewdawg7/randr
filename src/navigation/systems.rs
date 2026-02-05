use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::{AppState, StateTransitionRequest};
use crate::ui::screens::modal::ModalType;
use crate::ui::ModalCommands;

use super::table::{NavigationTable, NavigationTarget};

use crate::ui::screens::forge_modal::ForgeModal;
use crate::ui::screens::inventory_modal::InventoryModal;
use crate::ui::screens::merchant_modal::MerchantModal;
use crate::ui::screens::monster_compendium::MonsterCompendiumModal;
use crate::ui::screens::skills_modal::SkillsModal;

pub fn handle_navigation(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    current_state: Res<State<AppState>>,
    mut state_requests: MessageWriter<StateTransitionRequest>,
    nav_table: Res<NavigationTable>,
) {
    for action in action_reader.read() {
        let Some(target) = nav_table.lookup(**current_state, *action) else {
            continue;
        };

        match target {
            NavigationTarget::State(state) => {
                if **current_state != state {
                    state_requests.write(state.into());
                }
            }
            NavigationTarget::Modal(modal_type) => {
                handle_modal_toggle(&mut commands, modal_type);
            }
        }
    }
}

fn handle_modal_toggle(commands: &mut Commands, modal_type: ModalType) {
    match modal_type {
        ModalType::Inventory => commands.toggle_modal::<InventoryModal>(),
        ModalType::MonsterCompendium => commands.toggle_modal::<MonsterCompendiumModal>(),
        ModalType::MerchantModal => commands.toggle_modal::<MerchantModal>(),
        ModalType::ForgeModal => commands.toggle_modal::<ForgeModal>(),
        ModalType::SkillsModal => commands.toggle_modal::<SkillsModal>(),
        ModalType::Profile | ModalType::Keybinds | ModalType::ResultsModal | ModalType::AnvilModal => {
        }
    }
}
