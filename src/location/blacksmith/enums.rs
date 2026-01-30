use crate::inventory::{FindsItems, Inventory, ManagesItems};
use crate::item::{enums::ItemError, enums::ItemQuality, recipe::RecipeError, Item, ItemId, UpgradeResult};
use crate::player::PlayerGold;

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

#[derive(Debug, Clone)]
pub struct BlacksmithUpgradeResult {
    pub upgrade: UpgradeResult,
    pub gold_spent: i32,
}

#[derive(Debug, Clone)]
pub enum UpgradeOperationResult {
    StatUpgrade(BlacksmithUpgradeResult),
    QualityUpgrade(ItemQuality),
}

#[derive(Clone, Copy)]
pub enum UpgradeOperation {
    Stat { max_upgrades: i32, base_upgrade_cost: i32 },
    Quality,
}

impl UpgradeOperation {
    pub fn execute(
        self,
        gold: &mut PlayerGold,
        inventory: &mut Inventory,
        item: &mut Item,
    ) -> Result<UpgradeOperationResult, BlacksmithError> {
        match self {
            UpgradeOperation::Stat { max_upgrades, base_upgrade_cost } => {
                if !item.item_type.is_equipment() {
                    return Err(BlacksmithError::NotEquipment);
                }
                if item.num_upgrades >= max_upgrades {
                    return Err(BlacksmithError::MaxUpgradesReached);
                }

                let upgrade_cost = Self::calc_upgrade_cost(item, base_upgrade_cost);
                if upgrade_cost > gold.0 {
                    return Err(BlacksmithError::NotEnoughGold);
                }

                match item.upgrade() {
                    Ok(upgrade) => {
                        gold.subtract(upgrade_cost);
                        Ok(UpgradeOperationResult::StatUpgrade(BlacksmithUpgradeResult {
                            upgrade,
                            gold_spent: upgrade_cost,
                        }))
                    }
                    Err(e) => Err(BlacksmithError::ItemError(e)),
                }
            }
            UpgradeOperation::Quality => {
                if !item.item_type.is_equipment() {
                    return Err(BlacksmithError::NotEquipment);
                }

                if inventory.find_item_by_id(ItemId::QualityUpgradeStone).is_none() {
                    return Err(BlacksmithError::NoUpgradeStones);
                }

                let new_quality = item.upgrade_quality()
                    .map_err(BlacksmithError::ItemError)?;

                inventory.decrease_item_quantity(ItemId::QualityUpgradeStone, 1);

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
