pub(crate) mod definition;
pub(crate) mod enums;
mod traits;

pub(crate) use definition::{Item, ItemSpec};
pub use enums::ItemId;  // Used by main.rs
pub(crate) use enums::{
    ItemType, EquipmentType, MaterialType, ConsumableType, ToolKind
};
