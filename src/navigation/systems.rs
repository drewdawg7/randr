use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::AppState;
use crate::ui::screens::modal::{toggle_modal, ActiveModal, ModalAction, ModalType};

use super::table::{NavigationTable, NavigationTarget};

// Import modal root markers and spawn resources
use crate::ui::screens::inventory_modal::state::{InventoryModalRoot, InventorySelection};
use crate::ui::screens::monster_compendium::state::{
    CompendiumListState, CompendiumMonsters, MonsterCompendiumRoot, SpawnMonsterCompendium,
};
use crate::ui::screens::profile_modal::ProfileModalRoot;

use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::player::{Player, PlayerGold, PlayerName};
use crate::stats::StatSheet;

/// Central system that handles all navigation actions.
pub fn handle_navigation(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut active_modal: ResMut<ActiveModal>,
    nav_table: Res<NavigationTable>,
    // Modal queries
    inventory_query: Query<Entity, With<InventoryModalRoot>>,
    profile_query: Query<Entity, With<ProfileModalRoot>>,
    compendium_query: Query<Entity, With<MonsterCompendiumRoot>>,
    // Resources needed for modal spawning
    mut inventory_selection: ResMut<InventorySelection>,
    inventory: Res<Inventory>,
    mut compendium_list_state: ResMut<CompendiumListState>,
    // Profile modal resources
    player_name: Res<PlayerName>,
    player_gold: Res<PlayerGold>,
    progression: Res<Progression>,
    stats: Res<StatSheet>,
) {
    for action in action_reader.read() {
        let Some(target) = nav_table.lookup(**current_state, *action) else {
            continue;
        };

        match target {
            NavigationTarget::State(state) => {
                // Don't transition if we're already in this state
                if **current_state != state {
                    next_state.set(state);
                }
            }
            NavigationTarget::Modal(modal_type) => {
                handle_modal_toggle(
                    &mut commands,
                    &mut active_modal,
                    modal_type,
                    &inventory_query,
                    &profile_query,
                    &compendium_query,
                    &mut inventory_selection,
                    &inventory,
                    &mut compendium_list_state,
                    &player_name,
                    &player_gold,
                    &progression,
                    &stats,
                );
            }
        }
    }
}

/// Handle modal toggle for different modal types.
fn handle_modal_toggle(
    commands: &mut Commands,
    active_modal: &mut ActiveModal,
    modal_type: ModalType,
    inventory_query: &Query<Entity, With<InventoryModalRoot>>,
    profile_query: &Query<Entity, With<ProfileModalRoot>>,
    compendium_query: &Query<Entity, With<MonsterCompendiumRoot>>,
    inventory_selection: &mut InventorySelection,
    inventory: &Inventory,
    compendium_list_state: &mut CompendiumListState,
    player_name: &PlayerName,
    player_gold: &PlayerGold,
    progression: &Progression,
    stats: &StatSheet,
) {
    match modal_type {
        ModalType::Inventory => {
            if let Some(ModalAction::Open) =
                toggle_modal(commands, active_modal, inventory_query, ModalType::Inventory)
            {
                inventory_selection.reset();
                crate::ui::screens::inventory_modal::render::spawn_inventory_modal(
                    commands,
                    inventory,
                    inventory_selection,
                );
            }
        }
        ModalType::Profile => {
            if let Some(ModalAction::Open) =
                toggle_modal(commands, active_modal, profile_query, ModalType::Profile)
            {
                let player = Player::from_resources(player_name, player_gold, progression, inventory, stats);
                crate::ui::screens::profile_modal::spawn_profile_modal(commands, &player);
            }
        }
        ModalType::MonsterCompendium => {
            match toggle_modal(
                commands,
                active_modal,
                compendium_query,
                ModalType::MonsterCompendium,
            ) {
                Some(ModalAction::Closed) => {
                    commands.remove_resource::<CompendiumMonsters>();
                }
                Some(ModalAction::Open) => {
                    compendium_list_state.selected = 0;
                    commands.insert_resource(CompendiumMonsters::from_registry());
                    commands.insert_resource(SpawnMonsterCompendium);
                }
                None => {}
            }
        }
        ModalType::Keybinds | ModalType::FightModal => {
            // These are handled differently (Keybinds is a state transition, FightModal is combat-specific)
        }
    }
}
