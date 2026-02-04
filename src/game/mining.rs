use bevy::prelude::*;

use crate::chest::Chest;
use crate::dungeon::events::{ChestMined, MineableEntityType, MiningResult, RockMined};
use crate::dungeon::DungeonCommands;
use crate::inventory::Inventory;
use crate::loot::{collect_loot_drops, HasLoot};
use crate::rock::Rock;
use crate::skills::{SkillType, SkillXpGained};
use crate::stats::{StatSheet, StatType};

pub struct MiningPlugin;

impl Plugin for MiningPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_chest_mined)
            .add_observer(on_rock_mined);
    }
}

fn on_chest_mined(
    trigger: On<ChestMined>,
    mut commands: Commands,
    mut result_events: MessageWriter<MiningResult>,
    stats: Res<StatSheet>,
    mut inventory: ResMut<Inventory>,
) {
    let event = trigger.event();
    let magic_find = stats.value(StatType::MagicFind);

    let loot_drops = Chest::default().roll_drops(magic_find);

    collect_loot_drops(&mut *inventory, &loot_drops);
    commands.despawn_dungeon_entity(event.entity);

    result_events.write(MiningResult {
        mineable_type: MineableEntityType::Chest,
        loot_drops,
    });
}

fn on_rock_mined(
    trigger: On<RockMined>,
    mut commands: Commands,
    mut result_events: MessageWriter<MiningResult>,
    mut xp_events: MessageWriter<SkillXpGained>,
    stats: Res<StatSheet>,
    mut inventory: ResMut<Inventory>,
) {
    let event = trigger.event();
    let magic_find = stats.value(StatType::MagicFind);

    xp_events.write(SkillXpGained {
        skill: SkillType::Mining,
        amount: event.rock_type.mining_xp(),
    });

    let loot_drops = Rock::new(event.rock_type).roll_drops(magic_find);

    collect_loot_drops(&mut *inventory, &loot_drops);
    commands.despawn_dungeon_entity(event.entity);

    result_events.write(MiningResult {
        mineable_type: MineableEntityType::Rock { rock_type: event.rock_type },
        loot_drops,
    });
}
