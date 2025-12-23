
use crate::{combat::HasGold, item::Item};

#[derive(Debug)]
pub struct Store {
    name: String,
    inventory: Vec<StoreItem>

}

impl Store {
    pub fn new(name: &str) -> Self {
        Store {
            name: name.to_string(),
            inventory: Vec::new()
        }
    }
}


#[derive(Debug)]
pub struct StoreItem {
    item: Item,
    quantity: i32,
    price: i32,
}

impl StoreItem {


    pub fn inc_quantity(&mut self, amount: i32) {
        self.quantity += amount;
    }
    pub fn dec_quantity(&mut self, amount: i32) {
        self.quantity = (self.quantity - amount).max(0);
    }

    pub fn purchase_price(&self) -> i32 {
        self.price
    }

    pub fn sell_price(&self) -> i32 {
        self.price / 2
    }
}

impl Store {

    pub fn purchase_item<P: HasGold>(&mut self, item: &mut StoreItem, player: &mut P) 
    -> Result<Item, StoreError>{
        if item.quantity <= 0 {
            return Err(StoreError::OutOfStock)
        };
        let player_gold = player.gold();
        let item_cost = item.purchase_price();
        if player_gold < item_cost {
            return Err(StoreError::NotEnoughGold)
        };
        item.dec_quantity(1);
        player.dec_gold(item_cost);
        Ok(item.item)
    }

    pub fn sell_item<P: HasGold>(&mut self, item: &mut StoreItem, player: &mut P)
    -> Result<i32, StoreError> {
        let sell_price = item.sell_price();
        item.inc_quantity(1);
        player.add_gold(sell_price);
        Ok(sell_price)

    }

    pub fn get_store_item(&self, item: Item) -> Option<&StoreItem> {
        self
            .inventory
            .iter()
            .find(|si| si.item == item)

    }



    pub fn get_store_item_mut(&mut self, item: Item) -> Option<&mut StoreItem> {
        self.inventory
            .iter_mut()
            .find(|si| si.item == item)

    }
    pub fn add_item(&mut self, item: &Item) {
       match self.get_store_item_mut(*item) {
            Some(store_item) => store_item.inc_quantity(1),
            None                  => {
                let store_item = StoreItem {
                    item: *item,
                    quantity: 1,
                    price: 5,
                };
                self.inventory.push(store_item);
            }
       }; 
    }
}

pub enum StoreError {
    NotEnoughGold,
    OutOfStock
}
