pub(crate) mod consumable;
pub(crate) mod definition;
pub(crate) mod enums;
pub mod spec;
mod traits;

pub(crate) use definition::Item;
pub use enums::ItemId;
pub(crate) use enums::ItemType;
pub(crate) use spec::ItemRegistry;
