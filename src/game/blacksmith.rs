use bevy::prelude::*;
use uuid::Uuid;

use crate::inventory::{FindsItems, Inventory, ManagesItems};
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemRegistry;
use crate::player::{PlayerGold, PlayerMarker};

#[derive(Message, Debug, Clone)]
pub struct UpgradeItemEvent {
    pub item_uuid: Uuid,
}

#[derive(Message, Debug, Clone)]
pub struct UpgradeQualityEvent {
    pub item_uuid: Uuid,
}

#[derive(Message, Debug, Clone)]
pub struct SmeltRecipeEvent {
    pub recipe_id: RecipeId,
}

#[derive(Message, Debug, Clone)]
pub struct ForgeRecipeEvent {
    pub recipe_id: RecipeId,
}

#[derive(Message, Debug, Clone)]
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

pub struct BlacksmithPlugin;

impl Plugin for BlacksmithPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UpgradeItemEvent>()
            .add_message::<UpgradeQualityEvent>()
            .add_message::<SmeltRecipeEvent>()
            .add_message::<ForgeRecipeEvent>()
            .add_message::<BlacksmithResult>()
            .add_systems(
                Update,
                (
                    handle_upgrade_item.run_if(on_message::<UpgradeItemEvent>),
                    handle_upgrade_quality.run_if(on_message::<UpgradeQualityEvent>),
                    handle_smelt_recipe.run_if(on_message::<SmeltRecipeEvent>),
                    handle_forge_recipe.run_if(on_message::<ForgeRecipeEvent>),
                ),
            );
    }
}

#[derive(Clone, Copy)]
enum CraftingOperation {
    Smelt,
    Forge,
}

impl CraftingOperation {
    fn fail_ingredients_result(self, recipe_name: String) -> BlacksmithResult {
        match self {
            CraftingOperation::Smelt => {
                BlacksmithResult::SmeltFailedInsufficientIngredients { recipe_name }
            }
            CraftingOperation::Forge => {
                BlacksmithResult::ForgeFailedInsufficientIngredients { recipe_name }
            }
        }
    }

    fn success_result(self, item_name: String) -> BlacksmithResult {
        match self {
            CraftingOperation::Smelt => BlacksmithResult::SmeltSuccess { item_name },
            CraftingOperation::Forge => BlacksmithResult::ForgeSuccess { item_name },
        }
    }

    fn fail_full_result(self, item_name: String) -> BlacksmithResult {
        match self {
            CraftingOperation::Smelt => BlacksmithResult::SmeltFailedInventoryFull { item_name },
            CraftingOperation::Forge => BlacksmithResult::ForgeFailedInventoryFull { item_name },
        }
    }

    fn verb(self) -> &'static str {
        match self {
            CraftingOperation::Smelt => "smelt",
            CraftingOperation::Forge => "forge",
        }
    }

    fn past_verb(self) -> &'static str {
        match self {
            CraftingOperation::Smelt => "Smelted",
            CraftingOperation::Forge => "Forged",
        }
    }
}

fn process_crafting_recipe(
    recipe_id: RecipeId,
    operation: CraftingOperation,
    result_events: &mut MessageWriter<BlacksmithResult>,
    inventory: &mut Inventory,
    registry: &ItemRegistry,
) -> bool {
    let Ok(recipe) = Recipe::new(recipe_id) else {
        return false;
    };

    let recipe_name = recipe.name().to_string();

    if !recipe.can_craft(inventory) {
        result_events.write(operation.fail_ingredients_result(recipe_name));
        info!("Not enough ingredients to {}", operation.verb());
        return false;
    }

    match recipe.craft(inventory) {
        Ok(item_id) => {
            let item = registry.spawn(item_id);
            let item_name = recipe.name().to_string();

            match inventory.add_to_inv(item) {
                Ok(_) => {
                    result_events.write(operation.success_result(item_name.clone()));
                    info!("{} {}", operation.past_verb(), item_name);
                    true
                }
                Err(_) => {
                    result_events.write(operation.fail_full_result(item_name));
                    info!("Inventory full!");
                    false
                }
            }
        }
        Err(_) => false,
    }
}

pub fn calculate_upgrade_cost(item: &crate::item::Item) -> i32 {
    let base_cost = 100;
    let quality_multiplier = item.quality.upgrade_cost_multiplier();
    (base_cost as f64 * (item.num_upgrades + 1) as f64 * quality_multiplier) as i32
}

fn handle_upgrade_item(
    mut upgrade_events: MessageReader<UpgradeItemEvent>,
    mut result_events: MessageWriter<BlacksmithResult>,
    mut player: Query<(&mut PlayerGold, &mut Inventory), With<PlayerMarker>>,
) {
    let Ok((mut gold, mut inventory)) = player.single_mut() else {
        return;
    };

    for event in upgrade_events.read() {
        let Some(inv_item) = inventory.find_item_by_uuid(event.item_uuid) else {
            continue;
        };

        let item_name = inv_item.item.name.clone();
        let upgrade_cost = calculate_upgrade_cost(&inv_item.item);
        let can_upgrade = inv_item.item.num_upgrades < inv_item.item.max_upgrades;

        if !can_upgrade {
            result_events.write(BlacksmithResult::UpgradeFailedMaxLevel { item_name });
            info!("Item is already at max upgrade level");
            continue;
        }

        if gold.0 < upgrade_cost {
            result_events.write(BlacksmithResult::UpgradeFailedNotEnoughGold {
                need: upgrade_cost,
                have: gold.0,
            });
            info!("Not enough gold to upgrade");
            continue;
        }

        gold.0 -= upgrade_cost;

        if let Some(inv_item_mut) = inventory.find_item_by_uuid_mut(event.item_uuid) {
            if let Ok(result) = inv_item_mut.item.upgrade() {
                result_events.write(BlacksmithResult::UpgradeSuccess {
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

fn handle_upgrade_quality(
    mut quality_events: MessageReader<UpgradeQualityEvent>,
    mut result_events: MessageWriter<BlacksmithResult>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
) {
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for event in quality_events.read() {
        let Some(inv_item) = inventory.find_item_by_uuid(event.item_uuid) else {
            continue;
        };
        let item_name = inv_item.item.name.clone();

        if inventory
            .find_item_by_id(crate::item::ItemId::QualityUpgradeStone)
            .is_none()
        {
            result_events.write(BlacksmithResult::QualityUpgradeFailedNoStone);
            info!("You need a Magic Rock (Quality Upgrade Stone) to improve quality");
            continue;
        }

        if let Some(inv_item_mut) = inventory.find_item_by_uuid_mut(event.item_uuid) {
            match inv_item_mut.item.upgrade_quality() {
                Ok(new_quality) => {
                    let quality_name = format!("{:?}", new_quality);

                    inventory.decrease_item_quantity(crate::item::ItemId::QualityUpgradeStone, 1);

                    result_events.write(BlacksmithResult::QualityUpgradeSuccess {
                        item_name: item_name.clone(),
                        new_quality: quality_name.clone(),
                    });
                    info!("Upgraded {} to {} quality", item_name, quality_name);
                }
                Err(_) => {
                    result_events.write(BlacksmithResult::QualityUpgradeFailedMaxQuality {
                        item_name,
                    });
                    info!("Item is already at max quality");
                }
            }
        }
    }
}

fn handle_smelt_recipe(
    mut smelt_events: MessageReader<SmeltRecipeEvent>,
    mut result_events: MessageWriter<BlacksmithResult>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    registry: Res<ItemRegistry>,
) {
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for event in smelt_events.read() {
        process_crafting_recipe(
            event.recipe_id,
            CraftingOperation::Smelt,
            &mut result_events,
            &mut inventory,
            &registry,
        );
    }
}

fn handle_forge_recipe(
    mut forge_events: MessageReader<ForgeRecipeEvent>,
    mut result_events: MessageWriter<BlacksmithResult>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
    registry: Res<ItemRegistry>,
) {
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for event in forge_events.read() {
        process_crafting_recipe(
            event.recipe_id,
            CraftingOperation::Forge,
            &mut result_events,
            &mut inventory,
            &registry,
        );
    }
}
