mod input;
mod render;
mod state;
mod utils;

use bevy::prelude::*;

use crate::game::Player;

use input::{handle_inventory_modal_input, handle_inventory_modal_toggle};
use render::update_inventory_display;
use state::InventorySelection;

/// Plugin that manages the inventory modal system.
pub struct InventoryModalPlugin;

impl Plugin for InventoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventorySelection>().add_systems(
            Update,
            (
                handle_inventory_modal_toggle,
                handle_inventory_modal_input,
                update_inventory_display
                    .run_if(resource_changed::<Player>.or(resource_changed::<InventorySelection>)),
            ),
        );
    }
}
