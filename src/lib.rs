pub mod assets;
pub mod camera;
pub mod chest;
pub mod crafting_station;
pub mod rock;
pub mod storage;
pub mod economy;
pub mod location;
pub mod loot;
pub mod mob;
pub mod navigation;
pub mod player;
pub mod ui;
pub mod entities;
pub mod combat;
pub mod dungeon;
pub mod item;
pub mod registry;
pub mod inventory;
pub mod skills;
pub mod stats;
pub mod game;
pub mod input;
pub mod states;
pub mod plugins;

#[cfg(test)]
mod entity_test;

// Re-exports for main.rs
pub use inventory::{EquipmentSlot, FindsItems, HasInventory, ManagesEquipment, ManagesItems};
pub use item::ItemId;
