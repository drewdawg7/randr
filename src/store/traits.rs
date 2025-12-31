use crate::{item::{ItemId, ItemRegistry}, store::Store};

impl Default for Store {
    fn default() -> Self {
        let ir = ItemRegistry::new();
        let sword  = ir.spawn(ItemId::Sword);
        let dagger = ir.spawn(ItemId::Dagger);
        let pick   = ir.spawn(ItemId::BronzePickaxe);
        let mut store = Store::new("The Shop");
        store.add_item(&sword);
        store.add_item(&dagger);
        store.add_item(&pick);
        store
    }
}
