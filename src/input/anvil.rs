use bevy::prelude::*;

use crate::crafting_station::{AnvilCraftingState, TryStartAnvilCrafting};
use crate::input::GameAction;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::recipe::RecipeId;
use crate::player::PlayerMarker;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::modal_registry::ModalCommands;
use crate::ui::screens::anvil_modal::render::get_recipe_entries;
use crate::ui::screens::anvil_modal::{ActiveAnvilEntity, AnvilModal, AnvilPlayerGrid, AnvilRecipeGrid};
use crate::ui::widgets::{ItemGrid, ItemGridEntry};

pub fn navigate_anvil_grid(
    mut action_reader: MessageReader<GameAction>,
    focus_state: Option<Res<FocusState>>,
    mut recipe_grids: Query<&mut ItemGrid, (With<AnvilRecipeGrid>, Without<AnvilPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<AnvilPlayerGrid>, Without<AnvilRecipeGrid>)>,
) {
    let Some(focus_state) = focus_state else { return };

    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            if focus_state.is_focused(FocusPanel::RecipeGrid) {
                if let Ok(mut grid) = recipe_grids.single_mut() {
                    grid.navigate(*direction);
                }
            } else if focus_state.is_focused(FocusPanel::AnvilInventory) {
                if let Ok(mut grid) = player_grids.single_mut() {
                    grid.navigate(*direction);
                }
            }
        }
    }
}

pub fn craft_anvil_recipe(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    mut try_start_events: MessageWriter<TryStartAnvilCrafting>,
    focus_state: Option<Res<FocusState>>,
    active_anvil: Option<Res<ActiveAnvilEntity>>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    mut anvil_state_query: Query<&mut AnvilCraftingState>,
    recipe_grids: Query<&ItemGrid, (With<AnvilRecipeGrid>, Without<AnvilPlayerGrid>)>,
    mut player_grids: Query<&mut ItemGrid, (With<AnvilPlayerGrid>, Without<AnvilRecipeGrid>)>,
) {
    let Some(focus_state) = focus_state else { return };
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

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

        if !focus_state.is_focused(FocusPanel::RecipeGrid) {
            continue;
        }

        let Ok(recipe_grid) = recipe_grids.single() else {
            continue;
        };

        let recipes = RecipeId::all_forging_recipes();
        let Some(recipe_id) = recipes.get(recipe_grid.selected_index) else {
            continue;
        };

        let spec = recipe_id.spec();

        let can_craft = spec
            .ingredients
            .iter()
            .all(|(item_id, required)| inventory.count_item(*item_id) >= *required);

        if !can_craft {
            continue;
        }

        for (item_id, required) in &spec.ingredients {
            inventory.decrease_item_quantity(*item_id, *required);
        }

        anvil_state.selected_recipe = Some(*recipe_id);

        if let Ok(mut grid) = player_grids.single_mut() {
            grid.items = ItemGridEntry::from_inventory(&inventory);
            grid.clamp_selection();
        }

        try_start_events.write(TryStartAnvilCrafting {
            entity: active_anvil.0,
        });
        commands.close_modal::<AnvilModal>();
    }
}

pub fn sync_anvil_recipes(
    player: Query<&Inventory, (With<PlayerMarker>, Changed<Inventory>)>,
    mut recipe_grids: Query<&mut ItemGrid, With<AnvilRecipeGrid>>,
) {
    let Ok(inventory) = player.single() else {
        return;
    };

    if let Ok(mut grid) = recipe_grids.single_mut() {
        grid.items = get_recipe_entries(inventory);
    }
}
