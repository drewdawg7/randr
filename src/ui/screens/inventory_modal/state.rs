use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

use super::render::spawn_inventory_modal;

/// Component marker for the inventory modal UI.
#[derive(Component)]
pub struct InventoryModalRoot;

/// Marker for the equipment grid (3x3).
#[derive(Component)]
pub struct EquipmentGrid;

/// Marker for the backpack grid (4x4).
#[derive(Component)]
pub struct BackpackGrid;

/// Type-safe handle for the inventory modal.
///
/// Used with `ModalCommands`:
/// ```ignore
/// commands.toggle_modal::<InventoryModal>();
/// commands.close_modal::<InventoryModal>();
/// ```
pub struct InventoryModal;

impl RegisteredModal for InventoryModal {
    type Root = InventoryModalRoot;
    const MODAL_TYPE: ModalType = ModalType::Inventory;

    fn spawn(world: &mut World) {
        world.run_system_cached(do_spawn_inventory_modal).ok();
    }
}

/// System that spawns the inventory modal UI.
fn do_spawn_inventory_modal(mut commands: Commands, inventory: Res<Inventory>) {
    spawn_inventory_modal(&mut commands, &inventory);
}
