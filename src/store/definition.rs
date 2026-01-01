
use std::fmt::Display;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL_SECS: u64 = 60;


use crate::{combat::HasGold, entities::Player, inventory::HasInventory, item::{Item, ItemId}, loot::traits::WorthGold, system::game_state};

#[derive(Debug)]
pub struct Store {
    pub name: String,
    pub inventory: Vec<StoreItem>,
    last_refresh: Instant,
    refresh_interval: Duration,
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            inventory: self.inventory.clone(),
            last_refresh: Instant::now(),
            refresh_interval: self.refresh_interval,
        }
    }
}

impl Store {
    pub fn new(name: &str) -> Self {
        Store {
            name: name.to_string(),
            inventory: Vec::new(),
            last_refresh: Instant::now(),
            refresh_interval: Duration::from_secs(REFRESH_INTERVAL_SECS),
        }
    }

    /// Check if refresh interval elapsed and restock if needed.
    /// Cheap to call every frame.
    pub fn check_and_restock(&mut self) {
        if self.last_refresh.elapsed() >= self.refresh_interval {
            self.restock();
            self.last_refresh = Instant::now();
        }
    }

    /// Respawn all items in the store with fresh qualities
    pub fn restock(&mut self) {
        for store_item in &mut self.inventory {
            store_item.restock();
        }
    }

    /// Returns seconds until next restock
    pub fn time_until_restock(&self) -> u64 {
        let elapsed = self.last_refresh.elapsed();
        if elapsed >= self.refresh_interval {
            0
        } else {
            (self.refresh_interval - elapsed).as_secs()
        }
    }
}


#[derive(Debug, Clone)]
pub struct StoreItem {
    pub item_id: ItemId,
    pub items: Vec<Item>,
    pub max_quantity: i32,
}

impl StoreItem {
    pub fn new(item_id: ItemId, max_quantity: i32) -> Self {
        // Don't spawn items here - game_state() may not be initialized yet
        // Call restock() after GameState is fully initialized
        Self {
            item_id,
            items: Vec::new(),
            max_quantity,
        }
    }

    pub fn quantity(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn is_in_stock(&self) -> bool {
        !self.items.is_empty()
    }

    /// Take an item from stock (for purchasing)
    pub fn take_item(&mut self) -> Option<Item> {
        self.items.pop()
    }

    /// Respawn items up to max_quantity (equipment is capped at 1)
    pub fn restock(&mut self) {
        self.items.clear();
        // Equipment only stocks 1 at a time
        let quantity = if game_state().is_item_equipment(self.item_id) {
            1
        } else {
            self.max_quantity
        };
        for _ in 0..quantity {
            let item = game_state().spawn_item(self.item_id);
            self.items.push(item);
        }
    }

    /// Get a reference to the first item (for display purposes)
    pub fn display_item(&self) -> Option<&Item> {
        self.items.first()
    }
}

impl Display for StoreItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(item) = self.display_item() {
            write!(f, "{:<10} |{:>4}g |{:>3}", item.name, item.purchase_price(), self.quantity())
        } else {
            write!(f, "{:<10} |{:>4} |{:>3}", "Out of stock", "-", 0)
        }
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{:<10}  {:>4}  {:>3}", "Item", "Price", "Qty")?;
        writeln!(f,
            "{:-<10}-+-{:-<4}-+-{:-<3}",
            "", "", ""
        )?;
        for item in &self.inventory {
            writeln!(f, "{}", item)?
        }
        Ok(())
    }
}
impl Store {
    pub fn get_store_item_by_id(&self, item_id: ItemId) -> Option<&StoreItem> {
        self.inventory
            .iter()
            .find(|si| si.item_id == item_id)
    }

    pub fn get_store_item_by_id_mut(&mut self, item_id: ItemId) -> Option<&mut StoreItem> {
        self.inventory
            .iter_mut()
            .find(|si| si.item_id == item_id)
    }

    /// Add a stock slot for an item type with the given quantity
    pub fn add_stock(&mut self, item_id: ItemId, quantity: i32) {
        match self.get_store_item_by_id_mut(item_id) {
            Some(store_item) => {
                // Add more stock to existing slot
                for _ in 0..quantity {
                    let item = game_state().spawn_item(item_id);
                    store_item.items.push(item);
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
        match self.get_store_item_by_id_mut(item.item_id) {
            Some(store_item) => {
                store_item.items.push(item);
            }
            None => {
                // Create new slot for this item type
                let store_item = StoreItem {
                    item_id: item.item_id,
                    items: vec![item],
                    max_quantity: 1,
                };
                self.inventory.push(store_item);
            }
        };
    }

    /// Attempt to purchase an item at the given index.
    /// Returns Some(item) on success, None on failure (out of stock, insufficient gold, inventory full).
    pub fn purchase_item(&mut self, player: &mut Player, index: usize) -> Option<Item> {
        if index >= self.inventory.len() {
            return None;
        }

        // Take item from store
        let item = self.inventory[index].take_item()?;
        let cost = item.purchase_price();

        // Check gold
        if player.gold() < cost {
            // Not enough gold - put item back
            self.inventory[index].items.push(item);
            return None;
        }

        // Try to add to inventory
        if player.add_to_inv(item.clone()).is_err() {
            // Inventory full - put item back
            self.inventory[index].items.push(item);
            return None;
        }

        // Deduct gold
        player.dec_gold(cost);
        Some(item)
    }
}

pub fn sell_player_item(player: &mut Player, item: &Item) -> i32 {
    if item.is_locked {
        return 0; // Cannot sell locked items
    }
    let sell_price = item.sell_price();
    player.add_gold(sell_price);
    if let Some(inv_item) = player.find_item_by_id(item.item_id) {
        let inv_item = inv_item.clone();
        player.decrease_item_quantity(&inv_item, 1);
    }
    sell_price
}
