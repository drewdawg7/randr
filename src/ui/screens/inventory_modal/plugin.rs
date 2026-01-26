use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::ui::modal_registry::modal_close_system;
use crate::ui::screens::modal::in_inventory_modal;

use super::input::{handle_inventory_modal_navigation, handle_inventory_modal_select, handle_inventory_modal_tab};
use super::render::{populate_item_detail_pane, spawn_inventory_modal, sync_inventory_to_grids};
use super::state::{InventoryModal, SpawnInventoryModal};

/// Plugin that manages the inventory modal system.
pub struct InventoryModalPlugin;

impl Plugin for InventoryModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
