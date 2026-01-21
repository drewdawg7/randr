use bevy::prelude::*;

use crate::inventory::Inventory;

use super::input::{handle_inventory_modal_close, handle_inventory_modal_input};
use super::render::{spawn_inventory_modal, update_inventory_display};
use super::state::{InventorySelection, SpawnInventoryModal};

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
                trigger_spawn_inventory_modal.run_if(resource_exists::<SpawnInventoryModal>),
            ),
        );
    }
}

/// System triggered by `SpawnInventoryModal` resource.
fn trigger_spawn_inventory_modal(
    mut commands: Commands,
    inventory: Res<Inventory>,
    mut selection: ResMut<InventorySelection>,
) {
    commands.remove_resource::<SpawnInventoryModal>();
    spawn_inventory_modal(&mut commands, &inventory, &mut selection);
}
