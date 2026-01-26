//! Input handling for the anvil modal.

use bevy::prelude::*;

use crate::crafting_station::AnvilCraftingState;
use crate::input::GameAction;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::recipe::RecipeId;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::modal::{ActiveModal, ModalType};
use crate::ui::widgets::ItemGrid;

use super::render::{get_player_inventory_entries, get_recipe_entries};
use super::state::{
    ActiveAnvilEntity, AnvilPlayerGrid, AnvilRecipeGrid, AnvilRecipeRefresh,
};

/// Handle Tab key toggling focus between recipe grid and player inventory.
pub fn handle_anvil_modal_tab(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    focus_state: Option<ResMut<FocusState>>,
) {
    if active_modal.modal != Some(ModalType::AnvilModal) {
        return;
    }

    let Some(mut focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if *action == GameAction::NextTab {
            // Toggle between recipe grid and inventory
            if focus_state.is_focused(FocusPanel::RecipeGrid) {
                focus_state.set_focus(FocusPanel::AnvilInventory);
            } else {
                focus_state.set_focus(FocusPanel::RecipeGrid);
            }
        }
    }
}

/// Handle arrow key navigation within the anvil modal.
pub fn handle_anvil_modal_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    focus_state: Option<Res<FocusState>>,
    mut recipe_grids: Query<&mut ItemGrid, (With<AnvilRecipeGrid>, Without<AnvilPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<AnvilPlayerGrid>, Without<AnvilRecipeGrid>)>,
) {
    if active_modal.modal != Some(ModalType::AnvilModal) {
        return;
    }

    let Some(focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if focus_state.is_focused(FocusPanel::RecipeGrid) {
                // Navigate recipe grid
                if let Ok(mut grid) = recipe_grids.get_single_mut() {
                    grid.navigate(*direction);
                }
            } else if focus_state.is_focused(FocusPanel::AnvilInventory) {
                // Navigate player inventory
                if let Ok(mut grid) = player_grids.get_single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

/// Handle Enter key for crafting selected recipe.
pub fn handle_anvil_modal_select(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    focus_state: Option<Res<FocusState>>,
    active_anvil: Option<Res<ActiveAnvilEntity>>,
    mut inventory: ResMut<Inventory>,
    mut anvil_state_query: Query<&mut AnvilCraftingState>,
    recipe_grids: Query<&ItemGrid, (With<AnvilRecipeGrid>, Without<AnvilPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<AnvilPlayerGrid>, Without<AnvilRecipeGrid>)>,
) {
    if active_modal.modal != Some(ModalType::AnvilModal) {
        return;
    }

    let Some(focus_state) = focus_state else { return };

    let Some(active_anvil) = active_anvil else {
        return;
    };

    let Ok(mut anvil_state) = anvil_state_query.get_mut(active_anvil.0) else {
        return;
    };

    for action in action_reader.read() {
        if *action != GameAction::Select {
            continue;
        }

        // Only handle crafting when recipe grid is focused
        if !focus_state.is_focused(FocusPanel::RecipeGrid) {
            continue;
        }

        let Ok(recipe_grid) = recipe_grids.get_single() else {
            continue;
        };

        let recipes = RecipeId::all_forging_recipes();
        let Some(recipe_id) = recipes.get(recipe_grid.selected_index) else {
            continue;
        };

        let spec = recipe_id.spec();

        // Check if player has all ingredients
        let can_craft = spec
            .ingredients
            .iter()
            .all(|(item_id, required)| inventory.count_item(*item_id) >= *required);

        if !can_craft {
            continue;
        }

        // Consume ingredients
        for (item_id, required) in &spec.ingredients {
            inventory.decrease_item_quantity(*item_id, *required);
        }

        // Set anvil to crafting state
        anvil_state.selected_recipe = Some(*recipe_id);
        anvil_state.is_crafting = true;

        // Refresh inventory grid
        if let Ok(mut grid) = player_grids.get_single_mut() {
            grid.items = get_player_inventory_entries(&inventory);
            if !grid.items.is_empty() {
                grid.selected_index = grid.selected_index.min(grid.items.len() - 1);
            } else {
                grid.selected_index = 0;
            }
        }

        // Trigger recipe refresh
        commands.insert_resource(AnvilRecipeRefresh);

        // Close modal to start crafting animation
        commands.insert_resource(crate::ui::screens::anvil_modal::state::CloseAnvilForCrafting);
    }
}

/// Refresh the recipe grid when inventory changes.
pub fn refresh_anvil_recipes(
    mut commands: Commands,
    refresh_trigger: Option<Res<AnvilRecipeRefresh>>,
    inventory: Res<Inventory>,
    mut recipe_grids: Query<&mut ItemGrid, With<AnvilRecipeGrid>>,
) {
    if refresh_trigger.is_none() {
        return;
    }

    commands.remove_resource::<AnvilRecipeRefresh>();

    if let Ok(mut grid) = recipe_grids.get_single_mut() {
        grid.items = get_recipe_entries(&inventory);
    }
}
