use serde::Deserialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    OffHand,
    Ring,
    Tool,
    Head,
    Chest,
    Hands,
    Feet,
    Legs,
}

impl EquipmentSlot {
    pub fn all() -> &'static [EquipmentSlot] {
        &[
            EquipmentSlot::Weapon,
            EquipmentSlot::OffHand,
            EquipmentSlot::Ring,
            EquipmentSlot::Tool,
            EquipmentSlot::Head,
            EquipmentSlot::Chest,
            EquipmentSlot::Hands,
            EquipmentSlot::Feet,
            EquipmentSlot::Legs,
        ]
    }
}

#[derive(Debug)]
pub enum InventoryError {
    Full
}
