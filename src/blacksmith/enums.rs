use crate::item::enums::ItemError;

pub enum BlacksmithError {
    MaxUpgradesReached,
    NotEnoughGold,
    ItemError(ItemError)
}
