use bevy::prelude::*;

use crate::input::{navigate_inventory_grid, toggle_equipment};
use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_inventory_modal;
use crate::ui::widgets::{update_detail_pane_source, ItemGridSelection};
use crate::ui::FocusState;

use super::render::{populate_inventory_detail_pane_content, sync_inventory_to_grids};
use super::state::{InventoryDetailPane, InventoryModal};

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
                        navigate_inventory_grid,
                        toggle_equipment,
                        sync_inventory_to_grids,
                        update_detail_pane_source::<InventoryDetailPane>.run_if(
                            resource_exists::<FocusState>
                                .and(resource_changed::<FocusState>)
                                .or(any_match_filter::<Changed<ItemGridSelection>>),
                        ),
                        populate_inventory_detail_pane_content,
                    )
                        .run_if(in_inventory_modal),
                ),
            );
    }
}
