use bevy::prelude::*;

use crate::game::{
    ForgeRecipeEvent, Player, SmeltRecipeEvent, UpgradeItemEvent, UpgradeQualityEvent,
};
use crate::input::{GameAction, NavigationDirection};
use crate::item::recipe::RecipeId;

use super::state::{BlacksmithMode, BlacksmithModeKind, BlacksmithSelections};

/// Handle input for the Blacksmith tab.
pub fn handle_blacksmith_input(
    mut blacksmith_mode: ResMut<BlacksmithMode>,
    mut blacksmith_selections: ResMut<BlacksmithSelections>,
    player: Res<Player>,
    mut action_events: EventReader<GameAction>,
    mut upgrade_events: EventWriter<UpgradeItemEvent>,
    mut quality_events: EventWriter<UpgradeQualityEvent>,
    mut smelt_events: EventWriter<SmeltRecipeEvent>,
    mut forge_events: EventWriter<ForgeRecipeEvent>,
) {
    for action in action_events.read() {
        match blacksmith_mode.mode {
            BlacksmithModeKind::Menu => {
                handle_menu_input(&mut blacksmith_mode, &mut blacksmith_selections, action)
            }
            BlacksmithModeKind::Upgrade => handle_upgrade_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                &player,
                action,
                &mut upgrade_events,
            ),
            BlacksmithModeKind::Quality => handle_quality_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                &player,
                action,
                &mut quality_events,
            ),
            BlacksmithModeKind::Smelt => handle_smelt_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                action,
                &mut smelt_events,
            ),
            BlacksmithModeKind::Forge => handle_forge_input(
                &mut blacksmith_mode,
                &mut blacksmith_selections,
                action,
                &mut forge_events,
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
    player: &Player,
    action: &GameAction,
    upgrade_events: &mut EventWriter<UpgradeItemEvent>,
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
                // Emit event - game logic handled by event system
                upgrade_events.send(UpgradeItemEvent {
                    item_uuid: inv_item.item.item_uuid,
                });
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
    player: &Player,
    action: &GameAction,
    quality_events: &mut EventWriter<UpgradeQualityEvent>,
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
                // Emit event - game logic handled by event system
                quality_events.send(UpgradeQualityEvent {
                    item_uuid: inv_item.item.item_uuid,
                });
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
    action: &GameAction,
    smelt_events: &mut EventWriter<SmeltRecipeEvent>,
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
        GameAction::Select => {
            let recipes = RecipeId::all_smelting_recipes();
            if let Some(recipe_id) = recipes.get(blacksmith_selections.smelt.selected) {
                // Emit event - game logic handled by event system
                smelt_events.send(SmeltRecipeEvent {
                    recipe_id: *recipe_id,
                });
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
    action: &GameAction,
    forge_events: &mut EventWriter<ForgeRecipeEvent>,
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
        GameAction::Select => {
            let recipes = RecipeId::all_forging_recipes();
            if let Some(recipe_id) = recipes.get(blacksmith_selections.forge.selected) {
                // Emit event - game logic handled by event system
                forge_events.send(ForgeRecipeEvent {
                    recipe_id: *recipe_id,
                });
            }
        }
        GameAction::Back => {
            blacksmith_mode.mode = BlacksmithModeKind::Menu;
            blacksmith_selections.forge.reset();
        }
        _ => {}
    }
}
