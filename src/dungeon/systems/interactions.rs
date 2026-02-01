use bevy::prelude::*;

use crate::chest::Chest;
use crate::dungeon::events::{MineEntity, MiningResult};
use crate::dungeon::{DungeonCommands, DungeonEntity, GridSize};
use crate::inventory::Inventory;
use crate::loot::{collect_loot_drops, HasLoot};
use crate::rock::Rock;
use crate::skills::{SkillType, SkillXpGained};
use crate::stats::{StatSheet, StatType};

/// Handles mining interactions (chests and rocks).
pub fn handle_mine_entity(
    mut commands: Commands,
    mut events: MessageReader<MineEntity>,
    mut result_events: MessageWriter<MiningResult>,
    mut xp_events: MessageWriter<SkillXpGained>,
    stats: Res<StatSheet>,
    mut inventory: ResMut<Inventory>,
) {
    for event in events.read() {
        let magic_find = stats.value(StatType::MagicFind);

        let loot_drops = match &event.entity_type {
            DungeonEntity::Chest { .. } => Chest::default().roll_drops(magic_find),
            DungeonEntity::Rock { rock_type, .. } => {
                xp_events.write(SkillXpGained {
                    skill: SkillType::Mining,
                    amount: rock_type.mining_xp(),
                });
                Rock::new(*rock_type).roll_drops(magic_find)
            }
            _ => continue,
        };

        collect_loot_drops(&mut *inventory, &loot_drops);
        commands.despawn_dungeon_entity(event.entity, event.pos, GridSize::single());

        result_events.write(MiningResult {
            entity_type: event.entity_type.clone(),
            loot_drops,
        });
    }
}
