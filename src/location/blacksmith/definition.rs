use std::time::Instant;

use uuid::Uuid;

use crate::{
    combat::HasGold,
    player::Player,
    inventory::{EquipmentSlot, FindsItems, HasInventory, ManagesEquipment, ManagesItems},
    item::{
        enums::ItemQuality,
        recipe::{Recipe, RecipeId},
        Item, ItemId,
    },
    location::{BlacksmithData, LocationId, LocationSpec},
    magic::effect::PassiveEffect,
};

use super::enums::{BlacksmithError, BlacksmithUpgradeResult};

pub struct Blacksmith {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub max_upgrades: i32,
    pub base_upgrade_cost: i32,
    pub fuel_amount: i32,
    /// Last time fuel regeneration was applied
    pub(crate) last_fuel_regen: Option<Instant>,
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
            last_fuel_regen: None,
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
            last_fuel_regen: None,
        }
    }

    /// Apply fuel regeneration based on passive effects and elapsed time
    pub fn apply_fuel_regen(&mut self, player: &Player) {
        // Calculate total fuel regen per minute from passive effects
        let fuel_regen_per_min: i32 = player
            .tome_passive_effects()
            .iter()
            .filter_map(|e| {
                if let PassiveEffect::FurnaceFuelRegen(amt) = e {
                    Some(*amt)
                } else {
                    None
                }
            })
            .sum();

        if fuel_regen_per_min <= 0 {
            return;
        }

        let now = Instant::now();
        let elapsed_secs = match self.last_fuel_regen {
            Some(last) => now.duration_since(last).as_secs(),
            None => {
                // First time - just set the timestamp, no retroactive regen
                self.last_fuel_regen = Some(now);
                return;
            }
        };

        // Calculate fuel to add (1 per minute = 1 per 60 seconds)
        let minutes_elapsed = elapsed_secs / 60;
        if minutes_elapsed > 0 {
            let fuel_to_add = (minutes_elapsed as i32 * fuel_regen_per_min).min(100);
            self.inc_fuel(fuel_to_add);
            self.last_fuel_regen = Some(now);
        }
    }

    pub fn upgrade_item(
        &self,
        player: &mut Player,
        item: &mut Item,
    ) -> Result<BlacksmithUpgradeResult, BlacksmithError> {
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
            Ok(upgrade) => {
                player.dec_gold(upgrade_cost);
                Ok(BlacksmithUpgradeResult {
                    upgrade,
                    gold_spent: upgrade_cost,
                })
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
    ) -> Result<ItemQuality, BlacksmithError> {
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
        let new_quality = item.upgrade_quality()
            .map_err(BlacksmithError::ItemError)?;

        // Decrease the upgrade stone quantity
        player.decrease_item_quantity(&stone, 1);
        Ok(new_quality)
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
            .cloned()
            .ok_or(BlacksmithError::NoFuel)?;

        if coal.quantity == 0 {
            return Err(BlacksmithError::NoFuel);
        }

        player.decrease_item_quantity(&coal, 1);
        self.inc_fuel(1);
        Ok(self.fuel_amount)
    }

    /// Upgrade an item in player's inventory or equipment by UUID.
    /// Returns the upgrade result with stat increases and gold spent.
    pub fn upgrade_player_item(
        &self,
        player: &mut Player,
        item_uuid: Uuid,
    ) -> Result<BlacksmithUpgradeResult, BlacksmithError> {
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

    /// Upgrade item quality in player's inventory or equipment by UUID.
    /// Returns the new quality level.
    pub fn upgrade_player_item_quality(
        &self,
        player: &mut Player,
        item_uuid: Uuid,
    ) -> Result<ItemQuality, BlacksmithError> {
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
