use uuid::Uuid;

use crate::{
    combat::HasGold,
    entities::Player,
    inventory::EquipmentSlot,
    item::{
        recipe::{Recipe, RecipeId},
        Item, ItemId,
    },
    location::{BlacksmithData, LocationId, LocationSpec},
    HasInventory,
};

use super::enums::BlacksmithError;

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
    pub fn from_spec(spec: &LocationSpec, data: &BlacksmithData) -> Self {
        Blacksmith {
            location_id: spec.location_id,
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

    pub fn upgrade_item(
        &self,
        player: &mut Player,
        item: &mut Item,
    ) -> Result<(), BlacksmithError> {
        // Only equipment can be upgraded
        if !item.item_type.is_equipment() {
            return Err(BlacksmithError::NotEquipment);
        }
        if item.num_upgrades >= self.max_upgrades {
            return Err(BlacksmithError::MaxUpgradesReached);
        }
        let upgrade_cost = self.calc_upgrade_cost(item);
        if upgrade_cost > player.gold {
            return Err(BlacksmithError::NotEnoughGold);
        }

        match item.upgrade() {
            Ok(_) => {
                player.dec_gold(upgrade_cost);
                Ok(())
            }
            Err(e) => Err(BlacksmithError::ItemError(e)),
        }
    }

    pub fn inc_fuel(&mut self, amount: i32) {
        self.fuel_amount = (self.fuel_amount + amount).min(100)
    }

    pub fn dec_fuel(&mut self, amount: i32) {
        self.fuel_amount = (self.fuel_amount - amount).max(0)
    }

    pub fn upgrade_item_quality(
        &self,
        player: &mut Player,
        item: &mut Item,
    ) -> Result<(), BlacksmithError> {
        // Only equipment can have quality upgraded
        if !item.item_type.is_equipment() {
            return Err(BlacksmithError::NotEquipment);
        }
        // Check if player has an upgrade stone
        let stone = player
            .find_item_by_id(ItemId::QualityUpgradeStone)
            .ok_or(BlacksmithError::NoUpgradeStones)?
            .clone();

        // Upgrade the item quality
        item.upgrade_quality()
            .map_err(BlacksmithError::ItemError)?;

        // Decrease the upgrade stone quantity
        player.decrease_item_quantity(&stone, 1);
        Ok(())
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
        let item = match recipe.craft(player) {
            Ok(item) => item,
            Err(e) => return Err(BlacksmithError::RecipeError(e)),
        };
        self.dec_fuel(1);
        Ok(item)
    }

    /// Add fuel to the forge by consuming coal from player inventory
    pub fn add_fuel(&mut self, player: &mut Player) -> Result<(), BlacksmithError> {
        let coal = player
            .find_item_by_id(ItemId::Coal)
            .cloned()
            .ok_or(BlacksmithError::NoFuel)?;

        if coal.quantity == 0 {
            return Err(BlacksmithError::NoFuel);
        }

        player.decrease_item_quantity(&coal, 1);
        self.inc_fuel(1);
        Ok(())
    }

    /// Upgrade an item in player's inventory or equipment by UUID
    pub fn upgrade_player_item(
        &self,
        player: &mut Player,
        item_uuid: Uuid,
    ) -> Result<(), BlacksmithError> {
        // Check equipped items first
        for slot in EquipmentSlot::all() {
            if let Some(equipped) = player.get_equipped_item(*slot) {
                if equipped.item.item_uuid == item_uuid {
                    if let Some(mut inv_item) =
                        player.inventory_mut().equipment_mut().remove(slot)
                    {
                        let result = self.upgrade_item(player, &mut inv_item.item);
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
            let result = self.upgrade_item(player, &mut inv_item.item);
            player.inventory_mut().items.insert(idx, inv_item);
            return result;
        }

        Err(BlacksmithError::ItemNotFound)
    }

    /// Upgrade item quality in player's inventory or equipment by UUID
    pub fn upgrade_player_item_quality(
        &self,
        player: &mut Player,
        item_uuid: Uuid,
    ) -> Result<(), BlacksmithError> {
        // Check equipped items first
        for slot in EquipmentSlot::all() {
            if let Some(equipped) = player.get_equipped_item(*slot) {
                if equipped.item.item_uuid == item_uuid {
                    if let Some(mut inv_item) =
                        player.inventory_mut().equipment_mut().remove(slot)
                    {
                        let result = self.upgrade_item_quality(player, &mut inv_item.item);
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
            let result = self.upgrade_item_quality(player, &mut inv_item.item);
            player.inventory_mut().items.insert(idx, inv_item);
            return result;
        }

        Err(BlacksmithError::ItemNotFound)
    }

    /// Smelt ore and add result to player inventory
    pub fn smelt_and_give(
        &mut self,
        player: &mut Player,
        recipe_id: &RecipeId,
    ) -> Result<(), BlacksmithError> {
        let item = self.smelt_ore(player, recipe_id)?;
        player
            .add_to_inv(item)
            .map_err(|_| BlacksmithError::InventoryFull)?;
        Ok(())
    }

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
