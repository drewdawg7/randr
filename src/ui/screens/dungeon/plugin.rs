use bevy::prelude::*;

use crate::crafting_station::{AnvilCraftingStarted, ForgeCraftingStarted};
use crate::dungeon::{CraftingStationInteraction, FloorReady, MiningResult};
use crate::input::{emit_move_intent, request_menu_transition, GameAction};
use crate::states::AppState;
use crate::ui::screens::modal::ActiveModal;

use super::components::PendingPlayerSpawn;
use super::crafting_animation::{
    handle_anvil_crafting_started, handle_forge_crafting_started, on_anvil_timer_finished,
    on_forge_timer_finished,
};
use super::systems::update_player_sprite_direction;
use super::interaction::{
    handle_crafting_station_interaction, handle_interact_action, handle_mining_result,
};
use super::lifecycle::{
    enter_dungeon, handle_floor_ready, on_map_created_queue_player_spawn, spawn_player_when_ready,
};
use super::spawn::add_entity_visuals;
use super::systems::cleanup_dungeon;

pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_entity_visuals)
            .add_observer(on_map_created_queue_player_spawn)
            .add_observer(on_forge_timer_finished)
            .add_observer(on_anvil_timer_finished)
            .add_systems(OnEnter(AppState::Dungeon), enter_dungeon)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                FixedFirst,
                emit_move_intent
                    .run_if(on_message::<GameAction>)
                    .run_if(|modal: Res<ActiveModal>| modal.modal.is_none())
                    .run_if(in_state(AppState::Dungeon)),
            )
            .add_systems(
                Update,
                (
                    handle_floor_ready.run_if(on_message::<FloorReady>),
                    spawn_player_when_ready.run_if(resource_exists::<PendingPlayerSpawn>),
                    update_player_sprite_direction,
                    handle_interact_action
                        .run_if(on_message::<GameAction>)
                        .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
                    handle_crafting_station_interaction.run_if(on_message::<CraftingStationInteraction>),
                    handle_mining_result.run_if(on_message::<MiningResult>),
                    request_menu_transition,
                    handle_forge_crafting_started.run_if(on_message::<ForgeCraftingStarted>),
                    handle_anvil_crafting_started.run_if(on_message::<AnvilCraftingStarted>),
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}
