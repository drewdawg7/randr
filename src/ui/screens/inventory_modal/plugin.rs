use bevy::prelude::*;

use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_inventory_modal;

use super::input::{handle_inventory_modal_navigation, handle_inventory_modal_select, handle_inventory_modal_tab};
use super::render::{populate_item_detail_pane, sync_inventory_to_grids};
use super::state::InventoryModal;

/// Plugin that manages the inventory modal system.
pub struct InventoryModalPlugin;

impl Plugin for InventoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<InventoryModal>()
            .add_systems(
                Update,
                (
                    modal_close_system::<InventoryModal>,
                    (
                        handle_inventory_modal_tab,
                        handle_inventory_modal_navigation,
                        handle_inventory_modal_select,
                        sync_inventory_to_grids,
                        populate_item_detail_pane,
                    ).run_if(in_inventory_modal),
                ),
            );
    }
}
