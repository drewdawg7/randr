use crate::item::{enums::ItemError, enums::ItemQuality, recipe::RecipeError, Item, UpgradeResult};
use crate::player::Player;
use crate::combat::HasGold;

#[derive(Debug)]
pub enum BlacksmithError {
    MaxUpgradesReached,
    NotEnoughGold,
    NoUpgradeStones,
    NotEquipment,
    InvalidBlacksmithRecipe,
    ItemError(ItemError),
    RecipeError(RecipeError),
    NotEnoughFuel,
    NoFuel,
    ItemNotFound,
    InventoryFull,
}

/// Result of a blacksmith upgrade operation, including the item upgrade details and gold spent
#[derive(Debug, Clone)]
pub struct BlacksmithUpgradeResult {
    /// The upgrade result from the item (new level and stat increases)
    pub upgrade: UpgradeResult,
    /// The amount of gold spent on this upgrade
    pub gold_spent: i32,
}

/// Unified result for all upgrade operations.
#[derive(Debug, Clone)]
pub enum UpgradeOperationResult {
    /// Stat upgrade completed
    StatUpgrade(BlacksmithUpgradeResult),
    /// Quality upgrade completed
    QualityUpgrade(ItemQuality),
}

/// The type of upgrade operation to perform on an item.
/// Used by the upgrade helper to reduce code duplication between stat and quality upgrades.
#[derive(Clone, Copy)]
pub enum UpgradeOperation {
    /// Upgrade item stats (costs gold)
    Stat { max_upgrades: i32, base_upgrade_cost: i32 },
    /// Upgrade item quality (costs QualityUpgradeStone)
    Quality,
}

impl UpgradeOperation {
    /// Execute the upgrade operation on an item.
    /// Handles validation, resource consumption, and the actual upgrade.
    pub fn execute(
        self,
        player: &mut Player,
        item: &mut Item,
    ) -> Result<UpgradeOperationResult, BlacksmithError> {
        match self {
            UpgradeOperation::Stat { max_upgrades, base_upgrade_cost } => {
                // Only equipment can be upgraded
                if !item.item_type.is_equipment() {
                    return Err(BlacksmithError::NotEquipment);
                }
                if item.num_upgrades >= max_upgrades {
                    return Err(BlacksmithError::MaxUpgradesReached);
                }

                let upgrade_cost = Self::calc_upgrade_cost(item, base_upgrade_cost);
                if upgrade_cost > player.gold {
                    return Err(BlacksmithError::NotEnoughGold);
                }

                match item.upgrade() {
                    Ok(upgrade) => {
                        player.dec_gold(upgrade_cost);
                        Ok(UpgradeOperationResult::StatUpgrade(BlacksmithUpgradeResult {
                            upgrade,
                            gold_spent: upgrade_cost,
                        }))
                    }
                    Err(e) => Err(BlacksmithError::ItemError(e)),
                }
            }
            UpgradeOperation::Quality => {
                // Only equipment can have quality upgraded
                if !item.item_type.is_equipment() {
                    return Err(BlacksmithError::NotEquipment);
                }

                // Check if player has an upgrade stone
                use crate::inventory::FindsItems;
                use crate::item::ItemId;
                if player.find_item_by_id(ItemId::QualityUpgradeStone).is_none() {
                    return Err(BlacksmithError::NoUpgradeStones);
                }

                // Upgrade the item quality
                let new_quality = item.upgrade_quality()
                    .map_err(BlacksmithError::ItemError)?;

                // Consume the upgrade stone
                use crate::inventory::ManagesItems;
                player.decrease_item_quantity(ItemId::QualityUpgradeStone, 1);

                Ok(UpgradeOperationResult::QualityUpgrade(new_quality))
            }
        }
    }

    fn calc_upgrade_cost(item: &Item, base_upgrade_cost: i32) -> i32 {
        let multiplier = item.quality.upgrade_cost_multiplier();
        let base = (item.num_upgrades + 1) * base_upgrade_cost;
        ((base as f64) * multiplier).round() as i32
    }
}
