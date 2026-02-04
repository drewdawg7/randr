use bevy::prelude::*;

use crate::crafting_station::TryStartForgeCrafting;
use crate::input::GameAction;
use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{ModalCommands, RegisterModalExt};
use crate::ui::screens::modal::in_forge_modal;

use super::input::{handle_forge_modal_navigation, handle_forge_modal_select};
use super::render::{
    animate_forge_slot_selector, populate_forge_detail_pane_content, refresh_forge_slots,
    update_forge_detail_pane_source, update_forge_slot_selector,
};
use super::state::{ActiveForgeEntity, ForgeModal};

pub struct ForgeModalPlugin;

impl Plugin for ForgeModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<ForgeModal>()
            .add_systems(
                Update,
                (
                    handle_forge_close.run_if(in_forge_modal),
                    (
                        tab_toggle_system(FocusPanel::ForgeCraftingSlots, FocusPanel::ForgeInventory),
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

fn handle_forge_close(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    mut try_start_events: MessageWriter<TryStartForgeCrafting>,
    active_forge: Option<Res<ActiveForgeEntity>>,
) {
    for action in action_reader.read() {
        if *action != GameAction::CloseModal {
            continue;
        }

        if let Some(ref active_forge) = active_forge {
            try_start_events.write(TryStartForgeCrafting {
                entity: active_forge.0,
            });
        }

        commands.close_modal::<ForgeModal>();
    }
}
