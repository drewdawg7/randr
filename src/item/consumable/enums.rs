#[derive(Debug, Clone, PartialEq)]
pub enum ConsumableError {
    /// The item is not a consumable
    NotConsumable,
    /// No effect registered for this item
    NoEffectRegistered,
    /// Target already at full health (for HP potions)
    AlreadyAtFullHealth,
}
