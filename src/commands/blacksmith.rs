//! Blacksmith-related game commands.
//!
//! Handles item upgrades, quality improvements, smelting, and forging.

use uuid::Uuid;

use crate::inventory::HasInventory;
use crate::item::recipe::{Recipe, RecipeId};
use crate::location::BlacksmithError;
use crate::system::game_state;

use super::CommandResult;

/// Upgrade an item's stats at the blacksmith.
pub fn upgrade_item(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    // Get item name before upgrade attempt
    let item_name = gs
        .player
        .find_item_by_uuid(item_uuid)
        .map(|inv| inv.item.name.to_string())
        .unwrap_or_else(|| "Item".to_string());

    match gs
        .town
        .blacksmith
        .upgrade_player_item(&mut gs.player, item_uuid)
    {
        Ok(_) => CommandResult::success(format!("Upgraded {}!", item_name)),
        Err(e) => {
            let msg = match e {
                BlacksmithError::MaxUpgradesReached => "Max upgrades reached",
                BlacksmithError::NotEnoughGold => "Not enough gold",
                BlacksmithError::NoUpgradeStones => "No upgrade stones",
                BlacksmithError::NotEquipment => "Cannot upgrade this item",
                BlacksmithError::ItemNotFound => "Item not found",
                BlacksmithError::InventoryFull => "Inventory is full",
                _ => "Upgrade failed",
            };
            CommandResult::error(msg)
        }
    }
}

/// Upgrade an item's quality at the blacksmith.
pub fn upgrade_quality(item_uuid: Uuid) -> CommandResult {
    let gs = game_state();

    // Get item name before upgrade attempt
    let item_name = gs
        .player
        .find_item_by_uuid(item_uuid)
        .map(|inv| inv.item.name.to_string())
        .unwrap_or_else(|| "Item".to_string());

    match gs
        .town
        .blacksmith
        .upgrade_player_item_quality(&mut gs.player, item_uuid)
    {
        Ok(_) => CommandResult::success(format!("Improved {} quality!", item_name)),
        Err(e) => {
            let msg = match e {
                BlacksmithError::MaxUpgradesReached => "Max quality reached",
                BlacksmithError::NotEnoughGold => "Not enough gold",
                BlacksmithError::NoUpgradeStones => "No upgrade stones",
                BlacksmithError::NotEquipment => "Cannot upgrade this item",
                BlacksmithError::ItemNotFound => "Item not found",
                _ => "Quality upgrade failed",
            };
            CommandResult::error(msg)
        }
    }
}

/// Add fuel to the blacksmith forge.
pub fn add_fuel() -> CommandResult {
    let gs = game_state();

    match gs.town.blacksmith.add_fuel(&mut gs.player) {
        Ok(_) => CommandResult::success("Added fuel"),
        Err(e) => {
            let msg = match e {
                BlacksmithError::NoFuel => "No coal to add",
                _ => "Failed to add fuel",
            };
            CommandResult::error(msg)
        }
    }
}

/// Smelt a recipe at the blacksmith forge.
pub fn smelt_recipe(recipe_id: RecipeId) -> CommandResult {
    let gs = game_state();

    match gs
        .town
        .blacksmith
        .smelt_and_give(&mut gs.player, &recipe_id)
    {
        Ok(_) => {
            let name = Recipe::new(recipe_id)
                .map(|r| r.name().to_string())
                .unwrap_or_else(|_| "item".to_string());
            CommandResult::success(format!("Smelted {}!", name))
        }
        Err(e) => {
            let msg = match e {
                BlacksmithError::NotEnoughFuel => "Not enough fuel",
                BlacksmithError::RecipeError(_) => "Missing ingredients",
                BlacksmithError::InventoryFull => "Inventory is full",
                _ => "Smelting failed",
            };
            CommandResult::error(msg)
        }
    }
}

/// Forge an item from a recipe at the blacksmith.
pub fn forge_recipe(recipe_id: RecipeId) -> CommandResult {
    let gs = game_state();

    match Recipe::new(recipe_id) {
        Ok(recipe) => match recipe.craft(&mut gs.player) {
            Ok(item_id) => {
                let item = item_id.spawn();
                let item_name = item.name.clone();
                match gs.player.add_to_inv(item) {
                    Ok(_) => CommandResult::success(format!("Forged {}!", item_name)),
                    Err(_) => CommandResult::error("Inventory is full"),
                }
            }
            Err(_) => CommandResult::error("Missing ingredients"),
        },
        Err(_) => CommandResult::error("Invalid recipe"),
    }
}
