use crate::{item::ItemId, store::Store};

impl Default for Store {
    fn default() -> Self {
        let mut store = Store::new("The Shop");
        store.add_stock(ItemId::Sword, 3);
        store.add_stock(ItemId::Dagger, 3);
        store.add_stock(ItemId::BronzePickaxe, 2);
        store
    }
}
