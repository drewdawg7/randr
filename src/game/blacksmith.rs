use bevy::prelude::*;
use uuid::Uuid;

use crate::entities::Progression;
use crate::inventory::{FindsItems, Inventory, ManagesItems};
use crate::item::recipe::{Recipe, RecipeId};
use crate::player::{Player, PlayerGold, PlayerName};
use crate::stats::StatSheet;

/// Event sent when player attempts to upgrade an item's stats.
#[derive(Event, Debug, Clone)]
pub struct UpgradeItemEvent {
    pub item_uuid: Uuid,
}

/// Event sent when player attempts to upgrade an item's quality.
#[derive(Event, Debug, Clone)]
pub struct UpgradeQualityEvent {
    pub item_uuid: Uuid,
}

/// Event sent when player attempts to smelt ore into bars.
#[derive(Event, Debug, Clone)]
pub struct SmeltRecipeEvent {
    pub recipe_id: RecipeId,
}

/// Event sent when player attempts to forge equipment.
#[derive(Event, Debug, Clone)]
pub struct ForgeRecipeEvent {
    pub recipe_id: RecipeId,
}

/// Result event for blacksmith operations.
#[derive(Event, Debug, Clone)]
pub enum BlacksmithResult {
    UpgradeSuccess {
        item_name: String,
        new_level: i32,
        gold_spent: i32,
    },
    UpgradeFailedNotEnoughGold {
        need: i32,
        have: i32,
    },
    UpgradeFailedMaxLevel {
        item_name: String,
    },
    QualityUpgradeSuccess {
        item_name: String,
        new_quality: String,
    },
    QualityUpgradeFailedNoStone,
    QualityUpgradeFailedMaxQuality {
        item_name: String,
    },
    SmeltSuccess {
        item_name: String,
    },
    SmeltFailedInsufficientIngredients {
        recipe_name: String,
    },
    SmeltFailedInventoryFull {
        item_name: String,
    },
    ForgeSuccess {
        item_name: String,
    },
    ForgeFailedInsufficientIngredients {
        recipe_name: String,
    },
    ForgeFailedInventoryFull {
        item_name: String,
    },
}

/// Plugin for blacksmith-related events and systems.
pub struct BlacksmithPlugin;

impl Plugin for BlacksmithPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpgradeItemEvent>()
            .add_event::<UpgradeQualityEvent>()
            .add_event::<SmeltRecipeEvent>()
            .add_event::<ForgeRecipeEvent>()
            .add_event::<BlacksmithResult>()
            .add_systems(
                Update,
                (
                    handle_upgrade_item,
                    handle_upgrade_quality,
                    handle_smelt_recipe,
                    handle_forge_recipe,
                ),
            );
    }
}

/// Calculate the upgrade cost for an item.
pub fn calculate_upgrade_cost(item: &crate::item::Item) -> i32 {
    let base_cost = 100;
    let quality_multiplier = item.quality.upgrade_cost_multiplier();
    (base_cost as f64 * (item.num_upgrades + 1) as f64 * quality_multiplier) as i32
}

/// Handle upgrade item events by executing the upgrade logic.
fn handle_upgrade_item(
    mut upgrade_events: EventReader<UpgradeItemEvent>,
    mut result_events: EventWriter<BlacksmithResult>,
    mut gold: ResMut<PlayerGold>,
    mut inventory: ResMut<Inventory>,
) {
    for event in upgrade_events.read() {
        // Get item info first (immutable borrow)
        let Some(inv_item) = inventory.find_item_by_uuid(event.item_uuid) else {
            continue;
        };

        let item_name = inv_item.item.name.clone();
        let upgrade_cost = calculate_upgrade_cost(&inv_item.item);
        let can_upgrade = inv_item.item.num_upgrades < inv_item.item.max_upgrades;

        // Check if item can be upgraded
        if !can_upgrade {
            result_events.send(BlacksmithResult::UpgradeFailedMaxLevel { item_name });
            info!("Item is already at max upgrade level");
            continue;
        }

        // Check if player has enough gold
        if gold.0 < upgrade_cost {
            result_events.send(BlacksmithResult::UpgradeFailedNotEnoughGold {
                need: upgrade_cost,
                have: gold.0,
            });
            info!("Not enough gold to upgrade");
            continue;
        }

        // Deduct gold
        gold.0 -= upgrade_cost;

        // Perform upgrade (mutable borrow)
        if let Some(inv_item_mut) = inventory.find_item_by_uuid_mut(event.item_uuid) {
            if let Ok(result) = inv_item_mut.item.upgrade() {
                result_events.send(BlacksmithResult::UpgradeSuccess {
                    item_name: inv_item_mut.item.name.clone(),
                    new_level: result.new_level,
                    gold_spent: upgrade_cost,
                });
                info!(
                    "Upgraded {} to level {}",
                    inv_item_mut.item.name, result.new_level
                );
            }
        }
    }
}

/// Handle upgrade quality events by executing the quality upgrade logic.
fn handle_upgrade_quality(
    mut quality_events: EventReader<UpgradeQualityEvent>,
    mut result_events: EventWriter<BlacksmithResult>,
    mut inventory: ResMut<Inventory>,
) {
    for event in quality_events.read() {
        // Get item info first
        let Some(inv_item) = inventory.find_item_by_uuid(event.item_uuid) else {
            continue;
        };
        let item_name = inv_item.item.name.clone();

        // Check if player has QualityUpgradeStone
        if inventory
            .find_item_by_id(crate::item::ItemId::QualityUpgradeStone)
            .is_none()
        {
            result_events.send(BlacksmithResult::QualityUpgradeFailedNoStone);
            info!("You need a Magic Rock (Quality Upgrade Stone) to improve quality");
            continue;
        }

        // Perform quality upgrade
        if let Some(inv_item_mut) = inventory.find_item_by_uuid_mut(event.item_uuid) {
            match inv_item_mut.item.upgrade_quality() {
                Ok(new_quality) => {
                    let quality_name = format!("{:?}", new_quality);

                    // Consume the stone
                    inventory.decrease_item_quantity(crate::item::ItemId::QualityUpgradeStone, 1);

                    result_events.send(BlacksmithResult::QualityUpgradeSuccess {
                        item_name: item_name.clone(),
                        new_quality: quality_name.clone(),
                    });
                    info!("Upgraded {} to {} quality", item_name, quality_name);
                }
                Err(_) => {
                    result_events.send(BlacksmithResult::QualityUpgradeFailedMaxQuality {
                        item_name,
                    });
                    info!("Item is already at max quality");
                }
            }
        }
    }
}

/// Handle smelt recipe events by executing the smelting logic.
fn handle_smelt_recipe(
    mut smelt_events: EventReader<SmeltRecipeEvent>,
    mut result_events: EventWriter<BlacksmithResult>,
    name: Res<PlayerName>,
    mut gold: ResMut<PlayerGold>,
    mut progression: ResMut<Progression>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<StatSheet>,
) {
    for event in smelt_events.read() {
        let Ok(recipe) = Recipe::new(event.recipe_id) else {
            continue;
        };

        let recipe_name = recipe.name().to_string();

        // Build Player view for Recipe API
        let mut player = Player::from_resources(&name, &gold, &progression, &inventory, &stats);

        // Check ingredients
        if !recipe.can_craft(&player) {
            result_events.send(BlacksmithResult::SmeltFailedInsufficientIngredients { recipe_name });
            info!("Not enough ingredients to smelt");
            continue;
        }

        // Craft (consumes ingredients)
        match recipe.craft(&mut player) {
            Ok(item_id) => {
                let item = item_id.spawn();
                let item_name = recipe.name().to_string();

                match player.add_to_inv(item) {
                    Ok(_) => {
                        // Write changes back
                        player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
                        result_events.send(BlacksmithResult::SmeltSuccess {
                            item_name: item_name.clone(),
                        });
                        info!("Smelted {}", item_name);
                    }
                    Err(_) => {
                        result_events.send(BlacksmithResult::SmeltFailedInventoryFull { item_name });
                        info!("Inventory full!");
                    }
                }
            }
            Err(_) => {
                // Crafting failed for some reason
                continue;
            }
        }
    }
}

/// Handle forge recipe events by executing the forging logic.
fn handle_forge_recipe(
    mut forge_events: EventReader<ForgeRecipeEvent>,
    mut result_events: EventWriter<BlacksmithResult>,
    name: Res<PlayerName>,
    mut gold: ResMut<PlayerGold>,
    mut progression: ResMut<Progression>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<StatSheet>,
) {
    for event in forge_events.read() {
        let Ok(recipe) = Recipe::new(event.recipe_id) else {
            continue;
        };

        let recipe_name = recipe.name().to_string();

        // Build Player view for Recipe API
        let mut player = Player::from_resources(&name, &gold, &progression, &inventory, &stats);

        // Check ingredients
        if !recipe.can_craft(&player) {
            result_events.send(BlacksmithResult::ForgeFailedInsufficientIngredients { recipe_name });
            info!("Not enough ingredients to forge");
            continue;
        }

        // Craft (consumes ingredients)
        match recipe.craft(&mut player) {
            Ok(item_id) => {
                let item = item_id.spawn();
                let item_name = recipe.name().to_string();

                match player.add_to_inv(item) {
                    Ok(_) => {
                        // Write changes back
                        player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
                        result_events.send(BlacksmithResult::ForgeSuccess {
                            item_name: item_name.clone(),
                        });
                        info!("Forged {}", item_name);
                    }
                    Err(_) => {
                        result_events.send(BlacksmithResult::ForgeFailedInventoryFull { item_name });
                        info!("Inventory full!");
                    }
                }
            }
            Err(_) => {
                // Crafting failed for some reason
                continue;
            }
        }
    }
}
