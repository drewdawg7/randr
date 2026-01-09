pub mod consumable;
pub mod definition;
pub mod definitions;
pub mod enums;
pub mod modifier;
pub mod recipe;
mod traits;

pub use definition::Item;
// ItemId, ItemSpec, ItemRegistry now come from definitions (macro-generated)
pub use definitions::ItemId;
pub use enums::ItemType;
pub use enums::UpgradeResult;
