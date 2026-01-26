//! Plugin for the anvil modal.

use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::crafting_station::AnvilCraftingState;
use crate::ui::animation::{AnimationConfig, SpriteAnimation};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::dungeon::plugin::AnvilActiveTimer;
use crate::ui::screens::modal::{in_anvil_modal, ActiveModal, ModalType};

use super::input::{
    handle_anvil_modal_navigation, handle_anvil_modal_select, handle_anvil_modal_tab,
    refresh_anvil_recipes,
};
use super::render::{populate_anvil_detail_pane_content, update_anvil_detail_pane_source};
use super::state::{ActiveAnvilEntity, AnvilModal, AnvilModalRoot, CloseAnvilForCrafting};

pub struct AnvilModalPlugin;

impl Plugin for AnvilModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<AnvilModal>()
            .add_systems(
                Update,
                (
                    handle_anvil_close_with_crafting,
                    modal_close_system::<AnvilModal>,
                    (
                        handle_anvil_modal_tab,
                        handle_anvil_modal_navigation,
                        handle_anvil_modal_select,
                        refresh_anvil_recipes,
                        update_anvil_detail_pane_source,
                        populate_anvil_detail_pane_content,
                    )
                        .run_if(in_anvil_modal),
                ),
            );
    }
}

/// Custom close handler that starts crafting animation when closing with a recipe selected.
fn handle_anvil_close_with_crafting(
    mut commands: Commands,
    close_trigger: Option<Res<CloseAnvilForCrafting>>,
    mut active_modal: ResMut<ActiveModal>,
    active_anvil: Option<Res<ActiveAnvilEntity>>,
    game_sprites: Res<GameSprites>,
    anvil_state_query: Query<(Entity, &AnvilCraftingState)>,
    modal_query: Query<Entity, With<AnvilModalRoot>>,
) {
    // Check if we should close for crafting
    if close_trigger.is_none() {
        return;
    }

    commands.remove_resource::<CloseAnvilForCrafting>();

    if active_modal.modal != Some(ModalType::AnvilModal) {
        return;
    }

    let Some(active_anvil) = active_anvil else {
        return;
    };

    let Ok((entity, anvil_state)) = anvil_state_query.get(active_anvil.0) else {
        return;
    };

    if anvil_state.is_crafting && anvil_state.selected_recipe.is_some() {
        // Start anvil animation
        if let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) {
            if let (Some(first), Some(last)) =
                (sheet.get("anvil_active1"), sheet.get("anvil_active_6"))
            {
                let config = AnimationConfig {
                    first_frame: first,
                    last_frame: last,
                    frame_duration: 0.1,
                    looping: true,
                    synchronized: false,
                };
                commands.entity(entity).insert((
                    SpriteAnimation::new(&config),
                    AnvilActiveTimer(Timer::from_seconds(3.0, TimerMode::Once)),
                ));
            }
        }

        // Close the modal by despawning it
        if let Ok(modal_entity) = modal_query.get_single() {
            commands.entity(modal_entity).despawn_recursive();
        }
        active_modal.modal = None;

        // Clean up resources
        commands.remove_resource::<ActiveAnvilEntity>();
    }
}
