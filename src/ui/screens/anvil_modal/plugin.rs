use bevy::prelude::*;

use crate::input::{craft_anvil_recipe, navigate_anvil_grid, sync_anvil_recipes};
use crate::ui::focus::{tab_toggle_system, FocusPanel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_anvil_modal;
use crate::ui::widgets::{update_detail_pane_source, ItemGridSelection};
use crate::ui::FocusState;

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
                        navigate_anvil_grid,
                        craft_anvil_recipe,
                        sync_anvil_recipes,
                        update_detail_pane_source::<AnvilDetailPane>.run_if(
                            resource_exists::<FocusState>
                                .and(resource_changed::<FocusState>)
                                .or(any_match_filter::<Changed<ItemGridSelection>>),
                        ),
                        populate_anvil_detail_pane_content,
                    )
                        .run_if(in_anvil_modal),
                ),
            );
    }
}
