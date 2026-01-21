use bevy::prelude::*;

use crate::inventory::Inventory;

use super::input::{handle_inventory_modal_close, handle_inventory_modal_input};
use super::render::update_inventory_display;
use super::state::InventorySelection;

/// Plugin that manages the inventory modal system.
pub struct InventoryModalPlugin;

impl Plugin for InventoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventorySelection>().add_systems(
            Update,
            (
                handle_inventory_modal_close,
                handle_inventory_modal_input,
                update_inventory_display
                    .run_if(resource_changed::<Inventory>.or(resource_changed::<InventorySelection>)),
            ),
        );
    }
}
