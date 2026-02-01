use bevy::prelude::*;

use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_merchant_modal;

use super::input::{handle_merchant_modal_navigation, handle_merchant_modal_select};
use super::render::{
    populate_merchant_detail_pane_content, sync_merchant_grids, update_merchant_detail_pane_source,
};
use super::state::MerchantModal;

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
                        sync_merchant_grids,
                        update_merchant_detail_pane_source,
                        populate_merchant_detail_pane_content,
                    )
                        .run_if(in_merchant_modal),
                ),
            );
    }
}
