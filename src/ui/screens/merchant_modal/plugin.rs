use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_merchant_modal;
use crate::ui::widgets::update_detail_pane_source;

use super::input::{handle_merchant_modal_navigation, handle_merchant_modal_select};
use super::render::{
    populate_merchant_detail_pane_content, sync_merchant_player_grid, sync_merchant_stock_grid,
};
use super::state::{MerchantDetailPane, MerchantModal, MerchantStock};

/// Plugin that manages the merchant modal system.
pub struct MerchantModalPlugin;

impl Plugin for MerchantModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<MerchantModal>()
            .add_systems(
                Update,
                (
                    modal_close_system::<MerchantModal>,
                    (
                        tab_toggle_system(FocusPanel::MerchantStock, FocusPanel::PlayerInventory),
                        handle_merchant_modal_navigation,
                        handle_merchant_modal_select,
                        sync_merchant_stock_grid.run_if(resource_changed::<MerchantStock>),
                        sync_merchant_player_grid.run_if(resource_changed::<Inventory>),
                        update_detail_pane_source::<MerchantDetailPane>,
                        populate_merchant_detail_pane_content,
                    )
                        .run_if(in_merchant_modal),
                ),
            );
    }
}
