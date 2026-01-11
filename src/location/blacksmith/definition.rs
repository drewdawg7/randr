use std::time::Duration;

use bevy::time::{Timer, TimerMode};
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

/// Fuel regeneration interval (1 minute)
pub const FUEL_REGEN_INTERVAL: Duration = Duration::from_secs(60);

pub struct Blacksmith {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub max_upgrades: i32,
    pub base_upgrade_cost: i32,
    pub fuel_amount: i32,
    /// Timer for fuel regeneration (fires every minute)
    pub(crate) fuel_regen_timer: Timer,
    /// Cached fuel regeneration rate (per minute) from player passive effects
    pub(crate) fuel_regen_per_min: i32,
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
            fuel_regen_timer: Timer::new(FUEL_REGEN_INTERVAL, TimerMode::Repeating),
            fuel_regen_per_min: 0,
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
            fuel_regen_timer: Timer::new(FUEL_REGEN_INTERVAL, TimerMode::Repeating),
            fuel_regen_per_min: 0,
        }
    }

    /// Update fuel regeneration rate from player passive effects.
    /// This should be called when the player enters the blacksmith or equips/unequips tomes.
    pub fn apply_fuel_regen(&mut self, player: &Player) {
        // Calculate and cache total fuel regen per minute from passive effects
        self.fuel_regen_per_min = player
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
    }

    /// Tick the fuel regeneration timer and apply fuel if finished.
    /// Should be called from Refreshable::tick with the elapsed duration.
    pub fn tick_fuel_regen(&mut self, elapsed: Duration) {
        if self.fuel_regen_per_min <= 0 {
            return;
        }

        // Advance the timer
        self.fuel_regen_timer.tick(elapsed);

        // Add fuel for each time the timer completes (handles multiple completions)
        let times_finished = self.fuel_regen_timer.times_finished_this_tick();
        if times_finished > 0 {
            let fuel_to_add = (times_finished as i32 * self.fuel_regen_per_min).min(100);
            self.inc_fuel(fuel_to_add);
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
        if player.find_item_by_id(ItemId::QualityUpgradeStone).is_none() {
            return Err(BlacksmithError::NoUpgradeStones);
        }

        // Upgrade the item quality
        let new_quality = item.upgrade_quality()
            .map_err(BlacksmithError::ItemError)?;

        // Decrease the upgrade stone quantity
        player.decrease_item_quantity(ItemId::QualityUpgradeStone, 1);
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
            .ok_or(BlacksmithError::NoFuel)?;

        if coal.quantity == 0 {
            return Err(BlacksmithError::NoFuel);
        }

        player.decrease_item_quantity(ItemId::Coal, 1);
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
