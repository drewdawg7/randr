use bevy::prelude::*;

use super::components::PlayerMarker;
use super::{default_player_stats, PlayerGold, PlayerName};
use crate::entities::Progression;
use crate::game::player::PlayerPreviousLevel;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::{ItemId, ItemRegistry};
use crate::stats::StatSheet;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: PlayerMarker,
    pub name: PlayerName,
    pub gold: PlayerGold,
    pub progression: Progression,
    pub stats: StatSheet,
    pub inventory: Inventory,
    pub previous_level: PlayerPreviousLevel,
}

impl PlayerBundle {
    pub fn new(registry: &ItemRegistry) -> Self {
        let mut inventory = Inventory::new();
        let _ = inventory.add_to_inv(registry.spawn(ItemId::BasicHPPotion));
        let _ = inventory.add_to_inv(registry.spawn(ItemId::Coal));
        let _ = inventory.add_to_inv(registry.spawn(ItemId::IronOre));

        Self {
            marker: PlayerMarker,
            name: PlayerName::default(),
            gold: PlayerGold(100),
            progression: Progression::new(),
            stats: default_player_stats(),
            inventory,
            previous_level: PlayerPreviousLevel(1),
        }
    }
}
