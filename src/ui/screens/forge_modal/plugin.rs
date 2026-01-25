use bevy::prelude::*;

use crate::ui::modal_registry::modal_close_system;

use super::input::{handle_forge_modal_navigation, handle_forge_modal_select, handle_forge_modal_tab};
use super::render::{
    animate_forge_slot_selector, populate_forge_item_detail_pane, refresh_forge_slots,
    spawn_forge_modal, update_forge_slot_selector,
};
use super::state::{ForgeModal, ForgeSlotRefresh, SpawnForgeModal};

pub struct ForgeModalPlugin;

impl Plugin for ForgeModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_forge_close_with_crafting,
                modal_close_system::<ForgeModal>,
                handle_forge_modal_tab,
                handle_forge_modal_navigation,
                handle_forge_modal_select,
                spawn_forge_modal.run_if(resource_exists::<SpawnForgeModal>),
                refresh_forge_slots.run_if(resource_exists::<ForgeSlotRefresh>),
                populate_forge_item_detail_pane,
            ),
        )
        .add_systems(
            PostUpdate,
            (update_forge_slot_selector, animate_forge_slot_selector).chain(),
        );
    }
}

/// Custom close handler that starts crafting when closing the modal with ingredients.
fn handle_forge_close_with_crafting(
    mut commands: Commands,
    mut action_reader: EventReader<crate::input::GameAction>,
    active_modal: Res<crate::ui::screens::modal::ActiveModal>,
    active_forge: Option<Res<super::state::ActiveForgeEntity>>,
    game_sprites: Res<crate::assets::GameSprites>,
    mut forge_state_query: Query<(Entity, &mut crate::crafting_station::ForgeCraftingState)>,
) {
    use crate::assets::SpriteSheetKey;
    use crate::input::GameAction;
    use crate::ui::animation::{AnimationConfig, SpriteAnimation};
    use crate::ui::screens::dungeon::plugin::ForgeActiveTimer as DungeonForgeActiveTimer;
    use crate::ui::screens::modal::ModalType;

    if active_modal.modal != Some(ModalType::ForgeModal) {
        return;
    }

    let Some(active_forge) = active_forge else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::CloseModal {
            continue;
        }

        // Check if we should start crafting
        let Ok((entity, mut forge_state)) = forge_state_query.get_mut(active_forge.0) else {
            continue;
        };

        if forge_state.can_start_crafting() {
            // Start crafting
            forge_state.is_crafting = true;

            // Start forge animation
            if let Some(sheet) = game_sprites.get(SpriteSheetKey::Forge) {
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
                    commands.entity(entity).insert((
                        SpriteAnimation::new(&config),
                        DungeonForgeActiveTimer(Timer::from_seconds(5.0, TimerMode::Once)),
                    ));
                }
            }
        }
    }
}
