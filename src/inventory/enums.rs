#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    OffHand
}

pub enum InventoryError {
    Full
}
