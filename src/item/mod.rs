pub(crate) mod consumable;
pub(crate) mod definition;
pub mod definitions;
pub(crate) mod enums;
pub mod modifier;
pub(crate) mod recipe;
mod traits;

pub(crate) use definition::Item;
// ItemId, ItemSpec, ItemRegistry now come from definitions (macro-generated)
pub use definitions::ItemId;
pub(crate) use enums::ItemType;
pub(crate) use enums::UpgradeResult;
pub(crate) use definitions::{ItemRegistry, ItemSpec};
