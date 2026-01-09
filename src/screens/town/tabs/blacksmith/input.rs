use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemId;
use crate::{FindsItems, ManagesItems};

use super::state::{BlacksmithMode, BlacksmithModeKind, BlacksmithSelections};

/// Handle input for the Blacksmith tab.
pub fn handle_blacksmith_input(
    mut blacksmith_mode: ResMut<BlacksmithMode>,
    mut blacksmith_selections: ResMut<BlacksmithSelections>,
    mut player: ResMut<Player>,
    mut action_events: EventReader<GameAction>,
) {
    for action in action_events.read() {
        match blacksmith_mode.mode {
            BlacksmithModeKind::Menu => {
                handle_menu_input(&mut blacksmith_mode, &mut blacksmith_selections, action)
            }
            BlacksmithModeKind::Upgrade => handle_upgrade_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                &mut player,
                action,
            ),
            BlacksmithModeKind::Quality => handle_quality_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                &mut player,
                action,
            ),
            BlacksmithModeKind::Smelt => handle_smelt_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                &mut player,
                action,
            ),
            BlacksmithModeKind::Forge => handle_forge_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                &mut player,
                action,
            ),
        }
    }
}

/// Handle input for the main menu.
fn handle_menu_input(
    blacksmith_mode: &mut BlacksmithMode,
    blacksmith_selections: &mut BlacksmithSelections,
    action: &GameAction,
) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_selections.menu.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_selections.menu.move_down();
        }
        GameAction::Select => {
            blacksmith_mode.mode = match blacksmith_selections.menu.selected {
                0 => BlacksmithModeKind::Upgrade,
                1 => BlacksmithModeKind::Quality,
                2 => BlacksmithModeKind::Smelt,
                3 => BlacksmithModeKind::Forge,
                _ => BlacksmithModeKind::Menu,
            };
        }
        _ => {}
    }
}

/// Handle input for Upgrade mode.
fn handle_upgrade_input(
    blacksmith_mode: &mut BlacksmithMode,
    blacksmith_selections: &mut BlacksmithSelections,
    player: &mut Player,
    action: &GameAction,
) {
    // Get equipment items and update selection count
    let equipment_count = player
        .inventory
        .items
        .iter()
        .filter(|inv_item| inv_item.item.item_type.is_equipment())
        .count();
    blacksmith_selections.upgrade.set_count(equipment_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_selections.upgrade.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_selections.upgrade.move_down();
        }
        GameAction::Select => {
            // Get equipment items
            let equipment_items: Vec<_> = player
                .inventory
                .items
                .iter()
                .filter(|inv_item| inv_item.item.item_type.is_equipment())
                .collect();

            if let Some(inv_item) = equipment_items.get(blacksmith_selections.upgrade.selected) {
                let item_uuid = inv_item.item.item_uuid;

                // Calculate upgrade cost first
                let upgrade_cost = calculate_upgrade_cost(&inv_item.item);
                let can_upgrade = inv_item.item.num_upgrades < inv_item.item.max_upgrades;

                // Check if player has enough gold and item can be upgraded
                if player.gold >= upgrade_cost && can_upgrade {
                    // Deduct gold first
                    player.gold -= upgrade_cost;

                    // Find the mutable item and upgrade
                    if let Some(inv_item_mut) = player.find_item_by_uuid_mut(item_uuid) {
                        // Upgrade the item
                        if let Ok(result) = inv_item_mut.item.upgrade() {
                            info!(
                                "Upgraded {} to level {}",
                                inv_item_mut.item.name, result.new_level
                            );
                        }
                    }
                } else if player.gold < upgrade_cost {
                    info!("Not enough gold to upgrade");
                } else {
                    info!("Item is already at max upgrade level");
                }
            }
        }
        GameAction::Back => {
            blacksmith_mode.mode = BlacksmithModeKind::Menu;
            blacksmith_selections.upgrade.reset();
        }
        _ => {}
    }
}

/// Handle input for Quality mode.
fn handle_quality_input(
    blacksmith_mode: &mut BlacksmithMode,
    blacksmith_selections: &mut BlacksmithSelections,
    player: &mut Player,
    action: &GameAction,
) {
    // Get equipment items and update selection count
    let equipment_count = player
        .inventory
        .items
        .iter()
        .filter(|inv_item| inv_item.item.item_type.is_equipment())
        .count();
    blacksmith_selections.quality.set_count(equipment_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_selections.quality.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_selections.quality.move_down();
        }
        GameAction::Select => {
            // Get equipment items
            let equipment_items: Vec<_> = player
                .inventory
                .items
                .iter()
                .filter(|inv_item| inv_item.item.item_type.is_equipment())
                .collect();

            if let Some(inv_item) = equipment_items.get(blacksmith_selections.quality.selected) {
                let item_uuid = inv_item.item.item_uuid;

                // Check if player has QualityUpgradeStone
                if let Some(stone_inv) = player.find_item_by_id(ItemId::QualityUpgradeStone).cloned()
                {
                    // Find the mutable item to upgrade
                    if let Some(inv_item_mut) = player.find_item_by_uuid_mut(item_uuid) {
                        // Upgrade quality
                        if let Ok(new_quality) = inv_item_mut.item.upgrade_quality() {
                            let item_name = inv_item_mut.item.name.clone();

                            // Consume the stone
                            player.decrease_item_quantity(&stone_inv, 1);
                            info!("Upgraded {} to {:?} quality", item_name, new_quality);
                        } else {
                            info!("Item is already at max quality");
                        }
                    }
                } else {
                    info!("You need a Magic Rock (Quality Upgrade Stone) to improve quality");
                }
            }
        }
        GameAction::Back => {
            blacksmith_mode.mode = BlacksmithModeKind::Menu;
            blacksmith_selections.quality.reset();
        }
        _ => {}
    }
}

/// Handle input for Smelt mode.
fn handle_smelt_input(
    blacksmith_mode: &mut BlacksmithMode,
    blacksmith_selections: &mut BlacksmithSelections,
    player: &mut Player,
    action: &GameAction,
) {
    // Update selection count based on available recipes
    let recipe_count = RecipeId::all_smelting_recipes().len();
    blacksmith_selections.smelt.set_count(recipe_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_selections.smelt.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_selections.smelt.move_down();
        }
        GameAction::Select => 'select: {
            let recipes = RecipeId::all_smelting_recipes();
            let Some(recipe_id) = recipes.get(blacksmith_selections.smelt.selected) else {
                break 'select;
            };
            let Ok(recipe) = Recipe::new(*recipe_id) else {
                break 'select;
            };
            if !recipe.can_craft(&player) {
                info!("Not enough ingredients to smelt {}", recipe.name());
                break 'select;
            }
            let Ok(output_item_id) = recipe.craft(player) else {
                break 'select;
            };
            let new_item = output_item_id.spawn();
            if player.add_to_inv(new_item).is_ok() {
                info!("Smelted {}", recipe.name());
            }
        }
        GameAction::Back => {
            blacksmith_mode.mode = BlacksmithModeKind::Menu;
            blacksmith_selections.smelt.reset();
        }
        _ => {}
    }
}

/// Handle input for Forge mode.
fn handle_forge_input(
    blacksmith_mode: &mut BlacksmithMode,
    blacksmith_selections: &mut BlacksmithSelections,
    player: &mut Player,
    action: &GameAction,
) {
    // Update selection count based on available recipes
    let recipe_count = RecipeId::all_forging_recipes().len();
    blacksmith_selections.forge.set_count(recipe_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_selections.forge.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_selections.forge.move_down();
        }
        GameAction::Select => 'select: {
            let recipes = RecipeId::all_forging_recipes();
            let Some(recipe_id) = recipes.get(blacksmith_selections.forge.selected) else {
                break 'select;
            };
            let Ok(recipe) = Recipe::new(*recipe_id) else {
                break 'select;
            };
            if !recipe.can_craft(&player) {
                info!("Not enough ingredients to forge {}", recipe.name());
                break 'select;
            }
            let Ok(output_item_id) = recipe.craft(player) else {
                break 'select;
            };
            let new_item = output_item_id.spawn();
            if player.add_to_inv(new_item).is_ok() {
                info!("Forged {}", recipe.name());
            }
        }
        GameAction::Back => {
            blacksmith_mode.mode = BlacksmithModeKind::Menu;
            blacksmith_selections.forge.reset();
        }
        _ => {}
    }
}

/// Calculate the upgrade cost for an item.
pub fn calculate_upgrade_cost(item: &crate::item::Item) -> i32 {
    let base_cost = 100; // Base upgrade cost
    let quality_multiplier = item.quality.upgrade_cost_multiplier();
    (base_cost as f64 * (item.num_upgrades + 1) as f64 * quality_multiplier) as i32
}
