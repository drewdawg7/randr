#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    OffHand,
    Ring
}

impl EquipmentSlot {
    pub fn all() -> &'static [EquipmentSlot] {
        &[EquipmentSlot::Weapon, EquipmentSlot::OffHand, EquipmentSlot::Ring]
    }
}

pub enum InventoryError {
    Full
}
