use uuid::Uuid;

use crate::{
    inventory::{EquipmentSlot, FindsItems, Inventory, ManagesItems},
    item::{
        enums::ItemQuality,
        recipe::{Recipe, RecipeId},
        Item, ItemId,
    },
    location::{BlacksmithData, LocationId, LocationSpec},
    player::PlayerGold,
};

use super::enums::{BlacksmithError, BlacksmithUpgradeResult, UpgradeOperation, UpgradeOperationResult};

pub struct Blacksmith {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub max_upgrades: i32,
    pub base_upgrade_cost: i32,
    pub fuel_amount: i32,
}

impl Blacksmith {
    /// Create a Blacksmith from a LocationSpec
    pub fn from_spec(location_id: LocationId, spec: &LocationSpec, data: &BlacksmithData) -> Self {
        Blacksmith {
            location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            max_upgrades: data.max_upgrades,
            base_upgrade_cost: data.base_upgrade_cost,
            fuel_amount: 0,
        }
    }

    pub fn new(name: String, max_upgrades: i32, base_upgrade_cost: i32) -> Self {
        Self {
            location_id: LocationId::VillageBlacksmith,
            name,
            description: String::new(),
            max_upgrades,
            base_upgrade_cost,
            fuel_amount: 0,
        }
    }

    pub fn inc_fuel(&mut self, amount: i32) {
        self.fuel_amount = (self.fuel_amount + amount).min(100)
    }

    pub fn dec_fuel(&mut self, amount: i32) {
        self.fuel_amount = (self.fuel_amount - amount).max(0)
    }

    pub fn calc_upgrade_cost(&self, item: &Item) -> i32 {
        let multiplier = item.quality.upgrade_cost_multiplier();
        let base = (item.num_upgrades + 1) * self.base_upgrade_cost;
        ((base as f64) * multiplier).round() as i32
    }

    pub fn smelt_ore(
        &mut self,
        inventory: &mut Inventory,
        recipe_id: &RecipeId,
    ) -> Result<Item, BlacksmithError> {
        if self.fuel_amount <= 0 {
            return Err(BlacksmithError::NotEnoughFuel);
        }

        let recipe = match Recipe::new(*recipe_id) {
            Ok(recipe) => recipe,
            Err(e) => return Err(BlacksmithError::RecipeError(e)),
        };
        let item_id = match recipe.craft(inventory) {
            Ok(id) => id,
            Err(e) => return Err(BlacksmithError::RecipeError(e)),
        };
        let item = item_id.spawn();
        self.dec_fuel(1);
        Ok(item)
    }

    pub fn add_fuel(&mut self, inventory: &mut Inventory) -> Result<i32, BlacksmithError> {
        let coal = inventory
            .find_item_by_id(ItemId::Coal)
            .ok_or(BlacksmithError::NoFuel)?;

        if coal.quantity == 0 {
            return Err(BlacksmithError::NoFuel);
        }

        inventory.decrease_item_quantity(ItemId::Coal, 1);
        self.inc_fuel(1);
        Ok(self.fuel_amount)
    }

    fn process_player_upgrade(
        gold: &mut PlayerGold,
        inventory: &mut Inventory,
        item_uuid: Uuid,
        operation: UpgradeOperation,
    ) -> Result<UpgradeOperationResult, BlacksmithError> {
        for slot in EquipmentSlot::all() {
            if let Some(equipped) = inventory.equipment().get(slot) {
                if equipped.item.item_uuid == item_uuid {
                    if let Some(mut inv_item) = inventory.equipment_mut().remove(slot) {
                        let result = operation.execute(gold, inventory, &mut inv_item.item);
                        inventory.equipment_mut().insert(*slot, inv_item);
                        return result;
                    }
                }
            }
        }

        if let Some(idx) = inventory.find_item_index_by_uuid(item_uuid) {
            let mut inv_item = inventory.items.remove(idx);
            let result = operation.execute(gold, inventory, &mut inv_item.item);
            inventory.items.insert(idx, inv_item);
            return result;
        }

        Err(BlacksmithError::ItemNotFound)
    }

    pub fn upgrade_player_item(
        &self,
        gold: &mut PlayerGold,
        inventory: &mut Inventory,
        item_uuid: Uuid,
    ) -> Result<BlacksmithUpgradeResult, BlacksmithError> {
        let operation = UpgradeOperation::Stat {
            max_upgrades: self.max_upgrades,
            base_upgrade_cost: self.base_upgrade_cost,
        };
        match Self::process_player_upgrade(gold, inventory, item_uuid, operation)? {
            UpgradeOperationResult::StatUpgrade(result) => Ok(result),
            UpgradeOperationResult::QualityUpgrade(_) => unreachable!(),
        }
    }

    pub fn upgrade_player_item_quality(
        &self,
        gold: &mut PlayerGold,
        inventory: &mut Inventory,
        item_uuid: Uuid,
    ) -> Result<ItemQuality, BlacksmithError> {
        match Self::process_player_upgrade(gold, inventory, item_uuid, UpgradeOperation::Quality)? {
            UpgradeOperationResult::QualityUpgrade(quality) => Ok(quality),
            UpgradeOperationResult::StatUpgrade(_) => unreachable!(),
        }
    }

    pub fn smelt_and_give(
        &mut self,
        inventory: &mut Inventory,
        recipe_id: &RecipeId,
    ) -> Result<Item, BlacksmithError> {
        let item = self.smelt_ore(inventory, recipe_id)?;
        inventory
            .add_to_inv(item.clone())
            .map_err(|_| BlacksmithError::InventoryFull)?;
        Ok(item)
    }

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
