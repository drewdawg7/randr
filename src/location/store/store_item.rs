use std::fmt::Display;

use crate::{item::{Item, ItemId}, loot::traits::WorthGold, system::game_state};

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
