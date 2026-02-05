use std::fmt::Display;

use bevy::prelude::Resource;

use crate::{
    economy::WorthGold,
    inventory::{Inventory, ManagesItems},
    item::{Item, ItemId},
    location::{LocationId, LocationSpec, StoreData},
    player::PlayerGold,
};

use super::store_item::StoreItem;

#[derive(Debug, Clone, Resource)]
pub struct Store {
    location_id: LocationId,
    pub name: String,
    description: String,
    pub inventory: Vec<StoreItem>,
}

impl Store {
    pub fn from_spec(location_id: LocationId, spec: &LocationSpec, data: &StoreData) -> Self {
        let inventory = data
            .initial_stock
            .iter()
            .map(|(item_id, quantity)| StoreItem::new(*item_id, *quantity))
            .collect();
        Store {
            location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            inventory,
        }
    }

    pub fn new(name: &str, initial_stock: Vec<(ItemId, i32)>) -> Self {
        let inventory = initial_stock
            .into_iter()
            .map(|(item_id, quantity)| StoreItem::new(item_id, quantity))
            .collect();
        Store {
            location_id: LocationId::VillageStore,
            name: name.to_string(),
            description: String::new(),
            inventory,
        }
    }

    pub fn get_store_item_by_id(&self, item_id: ItemId) -> Option<&StoreItem> {
        self.inventory.iter().find(|si| si.item_id == item_id)
    }

    pub fn get_store_item_by_id_mut(&mut self, item_id: ItemId) -> Option<&mut StoreItem> {
        self.inventory.iter_mut().find(|si| si.item_id == item_id)
    }

    pub fn add_item(&mut self, item: Item) {
        let item_id = item.item_id;
        match self.get_store_item_by_id_mut(item_id) {
            Some(store_item) => {
                store_item.items.push(item);
            }
            None => {
                let store_item = StoreItem {
                    item_id,
                    items: vec![item],
                    max_quantity: 1,
                };
                self.inventory.push(store_item);
            }
        };
    }

    pub fn purchase_item(
        &mut self,
        gold: &mut PlayerGold,
        inventory: &mut Inventory,
        index: usize,
    ) -> Result<Item, super::StoreError> {
        use super::StoreError;

        if index >= self.inventory.len() {
            return Err(StoreError::InvalidIndex);
        }

        let item = self.inventory[index].take_item().ok_or(StoreError::OutOfStock)?;
        let cost = item.purchase_price();

        if gold.0 < cost {
            self.inventory[index].items.push(item);
            return Err(StoreError::NotEnoughGold);
        }

        if inventory.add_to_inv(item.clone()).is_err() {
            self.inventory[index].items.push(item);
            return Err(StoreError::InventoryFull);
        }

        gold.subtract(cost);
        Ok(item)
    }

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

pub fn sell_player_item(gold: &mut PlayerGold, inventory: &mut Inventory, item: &Item) -> i32 {
    if item.is_locked {
        return 0;
    }
    let sell_price = item.sell_price();
    gold.add(sell_price);
    inventory.decrease_item_quantity(item.item_id, 1);
    sell_price
}
