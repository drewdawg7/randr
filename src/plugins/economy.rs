use bevy::prelude::*;

use crate::item::Item;

/// Event fired when loot is dropped from a source (mob, chest, mining, etc.)
#[derive(Message, Debug, Clone)]
pub struct LootDropped {
    /// The items that were dropped
    pub items: Vec<LootDrop>,
    /// Optional source description (e.g., "Goblin", "Chest", "Iron Ore")
    pub source: Option<String>,
}

/// Represents a single item drop with quantity
#[derive(Debug, Clone)]
pub struct LootDrop {
    pub item: Item,
    pub quantity: i32,
}

/// Event fired when a player collects loot
#[derive(Message, Debug, Clone)]
pub struct LootCollected {
    /// The items that were collected
    pub items: Vec<LootDrop>,
    /// Total number of individual items collected
    pub total_items: i32,
}

/// Event fired when a player earns gold
#[derive(Message, Debug, Clone)]
pub struct GoldEarned {
    /// Amount of gold earned
    pub amount: i32,
    /// Optional source description (e.g., "Combat", "Sale", "Quest Reward")
    pub source: Option<String>,
}

/// Event fired when a player spends gold
#[derive(Message, Debug, Clone)]
pub struct GoldSpent {
    /// Amount of gold spent
    pub amount: i32,
    /// Optional description of what it was spent on
    pub reason: Option<String>,
}

/// Event fired when a transaction (purchase/sale) is completed
#[derive(Message, Debug, Clone)]
pub struct TransactionCompleted {
    /// Item involved in the transaction
    pub item: Item,
    /// Price of the transaction
    pub price: i32,
    /// Whether this was a purchase (true) or sale (false)
    pub is_purchase: bool,
}

/// Plugin that registers all economy and loot events for UI feedback
pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<LootDropped>()
            .add_message::<LootCollected>()
            .add_message::<GoldEarned>()
            .add_message::<GoldSpent>()
            .add_message::<TransactionCompleted>();
    }
}
