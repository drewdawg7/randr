use bevy::prelude::*;

use crate::chest::Chest;
use crate::dungeon::events::{MineEntity, MineableEntityType, MiningResult};
use crate::dungeon::DungeonCommands;
use crate::inventory::Inventory;
use crate::loot::{collect_loot_drops, HasLoot};
use crate::rock::Rock;
use crate::skills::{SkillType, SkillXpGained};
use crate::stats::{StatSheet, StatType};

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

        let loot_drops = match &event.mineable_type {
            MineableEntityType::Chest => Chest::default().roll_drops(magic_find),
            MineableEntityType::Rock { rock_type } => {
                xp_events.write(SkillXpGained {
                    skill: SkillType::Mining,
                    amount: rock_type.mining_xp(),
                });
                Rock::new(*rock_type).roll_drops(magic_find)
            }
        };

        collect_loot_drops(&mut *inventory, &loot_drops);
        commands.despawn_dungeon_entity(event.entity);

        result_events.write(MiningResult {
            mineable_type: event.mineable_type,
            loot_drops,
        });
    }
}
