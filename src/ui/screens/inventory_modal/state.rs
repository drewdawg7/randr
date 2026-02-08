use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::player::PlayerMarker;
use crate::ui::focus::FocusPanel;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::widgets::{DetailPaneContext, ItemGrid};
use crate::ui::InfoPanelSource;

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

pub struct InventoryDetailPane;

impl DetailPaneContext for InventoryDetailPane {
    type LeftGridMarker = EquipmentGrid;
    type RightGridMarker = BackpackGrid;

    const LEFT_FOCUS: FocusPanel = FocusPanel::EquipmentGrid;
    const RIGHT_FOCUS: FocusPanel = FocusPanel::BackpackGrid;

    fn source_from_left_grid(grid: &ItemGrid) -> InfoPanelSource {
        InfoPanelSource::Equipment {
            selected_index: grid.selected_index,
        }
    }

    fn source_from_right_grid(grid: &ItemGrid) -> InfoPanelSource {
        InfoPanelSource::Inventory {
            selected_index: grid.selected_index,
        }
    }
}

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

fn do_spawn_inventory_modal(
    mut commands: Commands,
    player: Query<&Inventory, With<PlayerMarker>>,
) {
    let Ok(inventory) = player.single() else {
        return;
    };
    spawn_inventory_modal(&mut commands, inventory);
}
