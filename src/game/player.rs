use bevy::prelude::*;

use crate::entities::Progression;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::ItemId;
use crate::player::{default_player_stats, PlayerGold, PlayerName};
use crate::stats::StatSheet;

/// Event fired when the player takes damage
#[derive(Event, Debug, Clone)]
pub struct PlayerDamaged {
    pub amount: i32,
    pub current_hp: i32,
    pub max_hp: i32,
}

/// Event fired when the player is healed
#[derive(Event, Debug, Clone)]
pub struct PlayerHealed {
    pub amount: i32,
    pub current_hp: i32,
    pub max_hp: i32,
}

/// Event fired when the player levels up
#[derive(Event, Debug, Clone)]
pub struct PlayerLeveledUp {
    pub new_level: u32,
    pub old_level: u32,
}

/// Event fired when the player's gold changes
#[derive(Event, Debug, Clone)]
pub struct GoldChanged {
    pub amount: i32,
    pub new_total: i32,
}

/// Resource to track player's previous level for level-up detection
#[derive(Resource, Default)]
pub struct PlayerPreviousLevel(pub i32);

/// Plugin that initializes player resources and registers player-related events
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Create inventory with initial items for testing
        let mut inventory = Inventory::new();
        let _ = inventory.add_to_inv(ItemId::BasicHPPotion.spawn());
        let _ = inventory.add_to_inv(ItemId::Coal.spawn());
        let _ = inventory.add_to_inv(ItemId::IronOre.spawn());

        app.init_resource::<PlayerName>()
            .insert_resource(PlayerGold(100))
            .insert_resource(Progression::new())
            .insert_resource(inventory)
            .insert_resource(default_player_stats())
            .insert_resource(PlayerPreviousLevel(1))
            .add_event::<PlayerDamaged>()
            .add_event::<PlayerHealed>()
            .add_event::<PlayerLeveledUp>()
            .add_event::<GoldChanged>()
            .add_systems(Update, handle_level_up);
    }
}

/// System that detects level-ups and applies stat bonuses
fn handle_level_up(
    progression: Res<Progression>,
    mut stats: ResMut<StatSheet>,
    mut prev_level: ResMut<PlayerPreviousLevel>,
    mut level_up_events: EventWriter<PlayerLeveledUp>,
) {
    let current_level = progression.level;
    if current_level > prev_level.0 {
        // Apply stat bonuses for each level gained
        for level in (prev_level.0 + 1)..=current_level {
            // Defense bonus every 10 levels
            if level % 10 == 0 {
                stats.increase_stat(crate::stats::StatType::Defense, 1);
            }
            // Health +5, Attack +1 every level
            stats.increase_stat(crate::stats::StatType::Health, 5);
            stats.increase_stat_max(crate::stats::StatType::Health, 5);
            stats.increase_stat(crate::stats::StatType::Attack, 1);

            level_up_events.send(PlayerLeveledUp {
                new_level: level as u32,
                old_level: (level - 1) as u32,
            });
        }
        prev_level.0 = current_level;
    }
}
