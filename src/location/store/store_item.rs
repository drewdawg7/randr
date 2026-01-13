use std::fmt::Display;

use crate::{economy::WorthGold, item::{Item, ItemId}};

#[derive(Debug, Clone)]
pub struct StoreItem {
    pub item_id: ItemId,
    pub items: Vec<Item>,
    pub max_quantity: i32,
}

impl StoreItem {
    pub fn new(item_id: ItemId, quantity: i32) -> Self {
        let max_quantity = quantity;
        // Equipment only stocks 1 at a time
        let actual_quantity = if item_id.spec().item_type.is_equipment() {
            1
        } else {
            quantity
        };
        let items = (0..actual_quantity).map(|_| item_id.spawn()).collect();
        Self {
            item_id,
            items,
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
        let quantity = if self.item_id.spec().item_type.is_equipment() {
            1
        } else {
            self.max_quantity
        };
        for _ in 0..quantity {
            self.items.push(self.item_id.spawn());
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
