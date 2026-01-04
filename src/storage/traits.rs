use crate::{inventory::Inventory, storage::definition::Storage, HasInventory};

impl HasInventory for Storage {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }
    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}
