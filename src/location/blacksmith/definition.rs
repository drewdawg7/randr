use uuid::Uuid;

use crate::{
    player::Player,
    inventory::{EquipmentSlot, FindsItems, HasInventory, ManagesEquipment, ManagesItems},
    item::{
        enums::ItemQuality,
        recipe::{Recipe, RecipeId},
        Item, ItemId,
    },
    location::{BlacksmithData, LocationId, LocationSpec},
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
        player: &mut Player,
        recipe_id: &RecipeId,
    ) -> Result<Item, BlacksmithError> {
        if self.fuel_amount <= 0 {
            return Err(BlacksmithError::NotEnoughFuel);
        }

        let recipe = match Recipe::new(*recipe_id) {
            Ok(recipe) => recipe,
            Err(e) => return Err(BlacksmithError::RecipeError(e)),
        };
        let item_id = match recipe.craft(player) {
            Ok(id) => id,
            Err(e) => return Err(BlacksmithError::RecipeError(e)),
        };
        let item = item_id.spawn();
        self.dec_fuel(1);
        Ok(item)
    }

    /// Add fuel to the forge by consuming coal from player inventory.
    /// Returns the new fuel amount on success.
    pub fn add_fuel(&mut self, player: &mut Player) -> Result<i32, BlacksmithError> {
        let coal = player
            .find_item_by_id(ItemId::Coal)
            .ok_or(BlacksmithError::NoFuel)?;

        if coal.quantity == 0 {
            return Err(BlacksmithError::NoFuel);
        }

        player.decrease_item_quantity(ItemId::Coal, 1);
        self.inc_fuel(1);
        Ok(self.fuel_amount)
    }

    /// Process an upgrade operation on a player's item by UUID.
    /// Handles the find-remove-modify-reinsert pattern for both inventory and equipment.
    fn process_player_upgrade(
        player: &mut Player,
        item_uuid: Uuid,
        operation: UpgradeOperation,
    ) -> Result<UpgradeOperationResult, BlacksmithError> {
        // Check equipped items first
        for slot in EquipmentSlot::all() {
            if let Some(equipped) = player.get_equipped_item(*slot) {
                if equipped.item.item_uuid == item_uuid {
                    if let Some(mut inv_item) =
                        player.inventory_mut().equipment_mut().remove(slot)
                    {
                        let result = operation.execute(player, &mut inv_item.item);
                        player
                            .inventory_mut()
                            .equipment_mut()
                            .insert(*slot, inv_item);
                        return result;
                    }
                }
            }
        }

        // Check inventory items
        if let Some(idx) = player.find_item_index_by_uuid(item_uuid) {
            let mut inv_item = player.inventory_mut().items.remove(idx);
            let result = operation.execute(player, &mut inv_item.item);
            player.inventory_mut().items.insert(idx, inv_item);
            return result;
        }

        Err(BlacksmithError::ItemNotFound)
    }

    /// Upgrade an item in player's inventory or equipment by UUID.
    /// Returns the upgrade result with stat increases and gold spent.
    pub fn upgrade_player_item(
        &self,
        player: &mut Player,
        item_uuid: Uuid,
    ) -> Result<BlacksmithUpgradeResult, BlacksmithError> {
        let operation = UpgradeOperation::Stat {
            max_upgrades: self.max_upgrades,
            base_upgrade_cost: self.base_upgrade_cost,
        };
        match Self::process_player_upgrade(player, item_uuid, operation)? {
            UpgradeOperationResult::StatUpgrade(result) => Ok(result),
            UpgradeOperationResult::QualityUpgrade(_) => unreachable!(),
        }
    }

    /// Upgrade item quality in player's inventory or equipment by UUID.
    /// Returns the new quality level.
    pub fn upgrade_player_item_quality(
        &self,
        player: &mut Player,
        item_uuid: Uuid,
    ) -> Result<ItemQuality, BlacksmithError> {
        match Self::process_player_upgrade(player, item_uuid, UpgradeOperation::Quality)? {
            UpgradeOperationResult::QualityUpgrade(quality) => Ok(quality),
            UpgradeOperationResult::StatUpgrade(_) => unreachable!(),
        }
    }

    /// Smelt ore and add result to player inventory.
    /// Returns the smelted item.
    pub fn smelt_and_give(
        &mut self,
        player: &mut Player,
        recipe_id: &RecipeId,
    ) -> Result<Item, BlacksmithError> {
        let item = self.smelt_ore(player, recipe_id)?;
        player
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
