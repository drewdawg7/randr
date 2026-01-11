use std::fmt::Display;
use std::time::Duration;

use bevy::time::{Timer, TimerMode};

use crate::{
    combat::HasGold,
    economy::WorthGold,
    player::Player,
    inventory::ManagesItems,
    item::{Item, ItemId},
    location::{LocationId, LocationSpec, StoreData},
    magic::effect::PassiveEffect,
};

use super::store_item::StoreItem;

#[derive(Debug)]
pub struct Store {
    location_id: LocationId,
    pub name: String,
    description: String,
    pub inventory: Vec<StoreItem>,
    refresh_timer: Timer,
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Self {
            location_id: self.location_id,
            name: self.name.clone(),
            description: self.description.clone(),
            inventory: self.inventory.clone(),
            refresh_timer: self.refresh_timer.clone(),
        }
    }
}

impl Store {
    /// Create a Store from a LocationSpec
    pub fn from_spec(location_id: LocationId, spec: &LocationSpec, data: &StoreData) -> Self {
        let refresh_interval = spec.refresh_interval.unwrap_or(Duration::from_secs(60));
        let mut store = Store {
            location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            inventory: Vec::new(),
            refresh_timer: Timer::new(refresh_interval, TimerMode::Repeating),
        };

        // Initialize stock from spec
        for (item_id, quantity) in &data.initial_stock {
            store.add_stock(*item_id, *quantity);
        }

        store
    }

    pub fn new(name: &str) -> Self {
        Store {
            location_id: LocationId::VillageStore,
            name: name.to_string(),
            description: String::new(),
            inventory: Vec::new(),
            refresh_timer: Timer::new(Duration::from_secs(60), TimerMode::Repeating),
        }
    }

    /// Check if refresh timer finished and restock if needed.
    /// Call tick_timer() first to advance the timer.
    pub fn check_and_restock(&mut self) {
        if self.refresh_timer.just_finished() {
            self.restock();
        }
    }

    /// Tick the refresh timer with the given delta time.
    /// Should be called from the Refreshable::tick implementation.
    pub fn tick_timer(&mut self, elapsed: Duration) {
        self.refresh_timer.tick(elapsed);
    }

    /// Respawn all items in the store with fresh qualities
    pub fn restock(&mut self) {
        for store_item in &mut self.inventory {
            store_item.restock();
        }
    }

    /// Returns seconds until next restock
    pub fn time_until_restock(&self) -> u64 {
        self.refresh_timer.remaining_secs() as u64
    }

    pub fn get_store_item_by_id(&self, item_id: ItemId) -> Option<&StoreItem> {
        self.inventory.iter().find(|si| si.item_id == item_id)
    }

    pub fn get_store_item_by_id_mut(&mut self, item_id: ItemId) -> Option<&mut StoreItem> {
        self.inventory.iter_mut().find(|si| si.item_id == item_id)
    }

    /// Add a stock slot for an item type with the given quantity
    pub fn add_stock(&mut self, item_id: ItemId, quantity: i32) {
        match self.get_store_item_by_id_mut(item_id) {
            Some(store_item) => {
                // Add more stock to existing slot
                for _ in 0..quantity {
                    store_item.items.push(item_id.spawn());
                }
                store_item.max_quantity += quantity;
            }
            None => {
                // Create new stock slot
                let store_item = StoreItem::new(item_id, quantity);
                self.inventory.push(store_item);
            }
        };
    }

    /// Add a specific item to the store (e.g., when player sells)
    pub fn add_item(&mut self, item: Item) {
        let item_id = item.item_id;
        match self.get_store_item_by_id_mut(item_id) {
            Some(store_item) => {
                store_item.items.push(item);
            }
            None => {
                // Create new slot for this item type
                let store_item = StoreItem {
                    item_id,
                    items: vec![item],
                    max_quantity: 1,
                };
                self.inventory.push(store_item);
            }
        };
    }

    /// Attempt to purchase an item at the given index.
    /// Returns Ok(item) on success, Err on failure.
    pub fn purchase_item(&mut self, player: &mut Player, index: usize) -> Result<Item, super::StoreError> {
        use super::StoreError;

        if index >= self.inventory.len() {
            return Err(StoreError::InvalidIndex);
        }

        // Take item from store
        let item = self.inventory[index].take_item().ok_or(StoreError::OutOfStock)?;
        let base_cost = item.purchase_price();

        // Apply store discount from passive effects
        let discount_pct: i32 = player
            .tome_passive_effects()
            .iter()
            .filter_map(|e| {
                if let PassiveEffect::StoreDiscount(pct) = e {
                    Some(*pct)
                } else {
                    None
                }
            })
            .sum();
        let discount_mult = 1.0 - (discount_pct.min(100) as f64 / 100.0);
        let cost = (base_cost as f64 * discount_mult).round() as i32;

        // Check gold
        if player.gold() < cost {
            // Not enough gold - put item back
            self.inventory[index].items.push(item);
            return Err(StoreError::NotEnoughGold);
        }

        // Try to add to inventory
        if player.add_to_inv(item.clone()).is_err() {
            // Inventory full - put item back
            self.inventory[index].items.push(item);
            return Err(StoreError::InventoryFull);
        }

        // Deduct gold
        player.dec_gold(cost);
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

impl Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{:<10}  {:>4}  {:>3}", "Item", "Price", "Qty")?;
        writeln!(f, "{:-<10}-+-{:-<4}-+-{:-<3}", "", "", "")?;
        for item in &self.inventory {
            writeln!(f, "{}", item)?
        }
        Ok(())
    }
}

pub fn sell_player_item(player: &mut Player, item: &Item) -> i32 {
    if item.is_locked {
        return 0; // Cannot sell locked items
    }
    let sell_price = item.sell_price();
    player.add_gold(sell_price);
    player.decrease_item_quantity(item.item_id, 1);
    sell_price
}
