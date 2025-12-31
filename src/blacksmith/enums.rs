use crate::item::enums::ItemError;

pub enum BlacksmithError {
    MaxUpgradesReached,
    NotEnoughGold,
    NoUpgradeStones,
    NotEquipment,
    ItemError(ItemError)
}
