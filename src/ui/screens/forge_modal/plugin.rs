use bevy::prelude::*;

use crate::ui::modal_registry::RegisterModalExt;
use crate::ui::screens::modal::in_forge_modal;

use super::input::{handle_forge_modal_navigation, handle_forge_modal_select, handle_forge_modal_tab};
use super::render::{
    animate_forge_slot_selector, populate_forge_detail_pane_content, refresh_forge_slots,
    update_forge_detail_pane_source, update_forge_slot_selector,
};
use super::state::ForgeModal;

pub struct ForgeModalPlugin;

impl Plugin for ForgeModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<ForgeModal>()
            .add_systems(
                Update,
                (
                    // Custom close handler replaces modal_close_system - handles both crafting and normal close
                    handle_forge_close,
                    (
                        handle_forge_modal_tab,
                        handle_forge_modal_navigation,
                        handle_forge_modal_select,
                        refresh_forge_slots,
                        update_forge_detail_pane_source,
                        populate_forge_detail_pane_content,
                    )
                        .run_if(in_forge_modal),
                ),
            )
            .add_systems(
                PostUpdate,
                (update_forge_slot_selector, animate_forge_slot_selector)
                    .chain()
                    .run_if(in_forge_modal),
            );
    }
}

/// Unified close handler for forge modal.
/// Handles CloseModal action: starts crafting if ready, then closes the modal.
fn handle_forge_close(
    mut commands: Commands,
    mut action_reader: EventReader<crate::input::GameAction>,
    mut active_modal: ResMut<crate::ui::screens::modal::ActiveModal>,
    active_forge: Option<Res<super::state::ActiveForgeEntity>>,
    game_sprites: Res<crate::assets::GameSprites>,
    skills: Res<crate::skills::Skills>,
    mut forge_state_query: Query<(Entity, &mut crate::crafting_station::ForgeCraftingState)>,
    modal_query: Query<Entity, With<super::state::ForgeModalRoot>>,
) {
    use crate::assets::SpriteSheetKey;
    use crate::input::GameAction;
    use crate::ui::animation::{AnimationConfig, SpriteAnimation};
    use crate::crafting_station::ForgeActiveTimer;
    use crate::ui::screens::modal::ModalType;
    use crate::skills::{blacksmith_speed_multiplier, SkillType};

    if active_modal.modal != Some(ModalType::ForgeModal) {
        return;
    }

    let blacksmith_level = skills
        .skill(SkillType::Blacksmith)
        .map(|s| s.level)
        .unwrap_or(1);
    let speed_mult = blacksmith_speed_multiplier(blacksmith_level);

    for action in action_reader.read() {
        if *action != GameAction::CloseModal {
            continue;
        }

        // Try to start crafting if we have a forge and it's ready
        if let Some(ref active_forge) = active_forge {
            if let Ok((entity, mut forge_state)) = forge_state_query.get_mut(active_forge.0) {
                if forge_state.can_start_crafting() {
                    // Start crafting
                    forge_state.is_crafting = true;

                    // Start forge animation
                    if let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) {
                        if let (Some(first), Some(last)) =
                            (sheet.get("forge_1_active1"), sheet.get("forge_1_active3"))
                        {
                            let config = AnimationConfig {
                                first_frame: first,
                                last_frame: last,
                                frame_duration: 0.1,
                                looping: true,
                                synchronized: false,
                            };
                            let duration = 5.0 * speed_mult;
                            commands.entity(entity).insert((
                                SpriteAnimation::new(&config),
                                ForgeActiveTimer(Timer::from_seconds(duration, TimerMode::Once)),
                            ));
                        }
                    }
                }
            }
        }

        // Always close the modal
        if let Ok(modal_entity) = modal_query.get_single() {
            commands.entity(modal_entity).despawn_recursive();
        }
        active_modal.modal = None;

        // Clean up resources
        commands.remove_resource::<super::state::ForgeModalState>();
        commands.remove_resource::<super::state::ActiveForgeEntity>();
    }
}
