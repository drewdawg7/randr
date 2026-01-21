mod collection;
pub(crate) mod definition;
pub(crate) mod enums;
pub(crate) mod traits;
#[cfg(test)]
mod tests;

pub use collection::collect_loot_drops;
pub(crate) use definition::{LootDrop, LootTable};
pub(crate) use traits::HasLoot;
