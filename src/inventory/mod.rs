mod definition;
mod enums;
mod equipment;
#[cfg(test)]
mod tests;
mod traits;

pub use definition::{AddItemResult, EquipmentMap, Inventory, InventoryItem};
pub use enums::{EquipmentSlot, InventoryError};
pub use equipment::HasEquipment;
pub use traits::{FindsItems, HasInventory, ManagesEquipment, ManagesItems};
