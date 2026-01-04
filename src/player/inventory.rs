//! Inventory-related trait implementations for Player

use crate::inventory::{HasEquipment, HasInventory, Inventory};

use super::Player;

impl HasInventory for Player {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

impl HasEquipment for Player {}
