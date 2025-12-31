use crate::{item::ItemId, store::Store};

impl Default for Store {
    fn default() -> Self {
        let mut store = Store::new("The Shop");
        store.add_stock(ItemId::Sword, 1);
        store.add_stock(ItemId::Dagger, 1);
        store.add_stock(ItemId::BronzePickaxe, 1);
        store.add_stock(ItemId::BasicHPPotion, 7);
        store
    }
}
