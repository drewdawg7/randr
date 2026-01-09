pub mod assets;
pub mod chest;
pub mod dungeon;
pub mod storage;
pub mod economy;
pub mod location;
pub mod loot;
pub mod magic;
pub mod mob;
pub mod player;
pub mod town;
pub mod ui;
pub mod entities;
pub mod combat;
pub mod item;
pub mod registry;
pub mod inventory;
pub mod utilities;
pub mod stats;
pub mod utils;
pub mod game;
pub mod input;
pub mod states;
pub mod screens;
pub mod save_load;
pub mod plugins;

#[cfg(test)]
mod entity_test;

// Re-exports for main.rs
pub use inventory::{EquipmentSlot, FindsItems, HasInventory, ManagesEquipment, ManagesItems};
pub use item::ItemId;
