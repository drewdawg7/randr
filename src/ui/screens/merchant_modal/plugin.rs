use bevy::prelude::*;

use crate::ui::modal_registry::modal_close_system;
use crate::ui::screens::modal::in_merchant_modal;

use super::input::{handle_merchant_modal_navigation, handle_merchant_modal_select, handle_merchant_modal_tab};
use super::render::{populate_merchant_detail_pane, spawn_merchant_modal, sync_merchant_grids};
use super::state::{MerchantModal, SpawnMerchantModal};

/// Plugin that manages the merchant modal system.
pub struct MerchantModalPlugin;

impl Plugin for MerchantModalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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
                spawn_merchant_modal.run_if(resource_exists::<SpawnMerchantModal>),
            ),
        );
    }
}
