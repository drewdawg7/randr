#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    OffHand,
    Ring,
    Tool,
}

impl EquipmentSlot {
    pub fn all() -> &'static [EquipmentSlot] {
        &[EquipmentSlot::Weapon, EquipmentSlot::OffHand, EquipmentSlot::Ring, EquipmentSlot::Tool]
    }
}

pub enum InventoryError {
    Full
}
