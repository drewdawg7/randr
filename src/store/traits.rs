use crate::{item::{ItemKind, definition::ItemRegistry}, store::Store};

impl Default for Store {
    fn default() -> Self {
        let ir = ItemRegistry::new();
        let sword = ir.spawn(ItemKind::Sword);
        let dagger = ir.spawn(ItemKind::Dagger);
        let mut store = Store::new("The Shop");
        store.add_item(&sword);
        store.add_item(&dagger);
        store
    }
}
