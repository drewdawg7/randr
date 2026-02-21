use std::fmt::Display;

use crate::{economy::WorthGold, item::{Item, ItemId, ItemRegistry}};

#[derive(Debug, Clone)]
pub struct StoreItem {
    pub item_id: ItemId,
    pub items: Vec<Item>,
    pub max_quantity: i32,
}

impl StoreItem {
    pub fn new(item_id: ItemId, quantity: i32, registry: &ItemRegistry) -> Self {
        let max_quantity = quantity;
        let sample = registry.spawn(item_id);
        let actual_quantity = if sample.item_type.is_equipment() {
            1
        } else {
            quantity
        };
        let items = (0..actual_quantity).map(|_| registry.spawn(item_id)).collect();
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

    pub fn take_item(&mut self) -> Option<Item> {
        self.items.pop()
    }

    pub fn restock(&mut self, registry: &ItemRegistry) {
        self.items.clear();
        let sample = registry.spawn(self.item_id);
        let quantity = if sample.item_type.is_equipment() {
            1
        } else {
            self.max_quantity
        };
        for _ in 0..quantity {
            self.items.push(registry.spawn(self.item_id));
        }
    }

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
