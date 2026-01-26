use bevy::prelude::*;

use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_merchant_modal;

use super::input::{handle_merchant_modal_navigation, handle_merchant_modal_select, handle_merchant_modal_tab};
use super::render::{populate_merchant_detail_pane, sync_merchant_grids};
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
                        handle_merchant_modal_tab,
                        handle_merchant_modal_navigation,
                        handle_merchant_modal_select,
                        sync_merchant_grids,
                        populate_merchant_detail_pane,
                    ).run_if(in_merchant_modal),
                ),
            );
    }
}
