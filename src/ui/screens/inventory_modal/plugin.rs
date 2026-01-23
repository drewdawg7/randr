use bevy::prelude::*;

use crate::inventory::Inventory;

use super::input::{handle_inventory_modal_close, handle_inventory_modal_navigation};
use super::render::{populate_item_detail_pane, spawn_inventory_modal};
use super::state::SpawnInventoryModal;

/// Plugin that manages the inventory modal system.
pub struct InventoryModalPlugin;

impl Plugin for InventoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_inventory_modal_close,
                handle_inventory_modal_navigation,
                populate_item_detail_pane,
                trigger_spawn_inventory_modal.run_if(resource_exists::<SpawnInventoryModal>),
            ),
        );
    }
}

/// System triggered by `SpawnInventoryModal` resource.
fn trigger_spawn_inventory_modal(mut commands: Commands, inventory: Res<Inventory>) {
    commands.remove_resource::<SpawnInventoryModal>();
    spawn_inventory_modal(&mut commands, &inventory);
}
