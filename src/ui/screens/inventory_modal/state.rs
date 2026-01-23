use bevy::prelude::*;

use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Component marker for the inventory modal UI.
#[derive(Component)]
pub struct InventoryModalRoot;

/// Marker for the equipment grid (3x3).
#[derive(Component)]
pub struct EquipmentGrid;

/// Marker for the backpack grid (4x4).
#[derive(Component)]
pub struct BackpackGrid;

/// Marker resource to trigger spawning the inventory modal.
#[derive(Resource)]
pub struct SpawnInventoryModal;

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
        world.insert_resource(SpawnInventoryModal);
    }
}
