use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_inventory_modal;
use crate::ui::widgets::{update_detail_pane_source, ItemGrid};
use crate::ui::FocusState;

use super::input::{handle_inventory_modal_navigation, handle_inventory_modal_select};
use super::render::{populate_inventory_detail_pane_content, sync_inventory_to_grids};
use super::state::{InventoryDetailPane, InventoryModal};

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
                        tab_toggle_system(FocusPanel::EquipmentGrid, FocusPanel::BackpackGrid),
                        handle_inventory_modal_navigation,
                        handle_inventory_modal_select,
                        sync_inventory_to_grids.run_if(resource_changed::<Inventory>),
                        update_detail_pane_source::<InventoryDetailPane>.run_if(
                            resource_changed::<FocusState>
                                .or(any_match_filter::<Changed<ItemGrid>>),
                        ),
                        populate_inventory_detail_pane_content,
                    )
                        .run_if(in_inventory_modal),
                ),
            );
    }
}
