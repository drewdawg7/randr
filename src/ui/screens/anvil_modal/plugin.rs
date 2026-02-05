use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_anvil_modal;
use crate::ui::widgets::update_detail_pane_source;

use super::input::{handle_anvil_modal_navigation, handle_anvil_modal_select, refresh_anvil_recipes};
use super::render::populate_anvil_detail_pane_content;
use super::state::{AnvilDetailPane, AnvilModal};

pub struct AnvilModalPlugin;

impl Plugin for AnvilModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<AnvilModal>()
            .add_systems(
                Update,
                (
                    modal_close_system::<AnvilModal>.run_if(in_anvil_modal),
                    (
                        tab_toggle_system(FocusPanel::RecipeGrid, FocusPanel::AnvilInventory),
                        handle_anvil_modal_navigation,
                        handle_anvil_modal_select,
                        refresh_anvil_recipes.run_if(resource_changed::<Inventory>),
                        update_detail_pane_source::<AnvilDetailPane>,
                        populate_anvil_detail_pane_content,
                    )
                        .run_if(in_anvil_modal),
                ),
            );
    }
}
