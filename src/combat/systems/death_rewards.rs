use bevy::prelude::*;

use crate::combat::events::{EntityDied, GoldGained, LootDropped, XpGained};
use crate::combat::system::{apply_goldfind, player_effective_goldfind, player_effective_magicfind};
use crate::inventory::Inventory;
use crate::loot::collect_loot_drops;
use crate::mob::components::{DeathProcessed, GoldReward, MobLootTable, MobMarker, XpReward};
use crate::player::PlayerMarker;
use crate::stats::StatSheet;

pub fn grant_kill_gold(
    mut events: MessageReader<EntityDied>,
    mut gold_writer: MessageWriter<GoldGained>,
    mobs: Query<(&MobMarker, &GoldReward, &DeathProcessed)>,
    player: Query<(&StatSheet, &Inventory), With<PlayerMarker>>,
) {
    let Ok((stats, inventory)) = player.single() else {
        return;
    };
    let goldfind = player_effective_goldfind(stats, inventory);

    for event in events.read() {
        if event.is_player {
            continue;
        }

        let Ok((marker, reward, death_processed)) = mobs.get(event.entity) else {
            continue;
        };

        if death_processed.0 {
            continue;
        }

        let amount = apply_goldfind(reward.0, goldfind);
        gold_writer.write(GoldGained {
            amount,
            source: marker.0.spec().name.clone(),
        });
    }
}

pub fn grant_kill_xp(
    mut events: MessageReader<EntityDied>,
    mut xp_writer: MessageWriter<XpGained>,
    mobs: Query<(&MobMarker, &XpReward, &DeathProcessed)>,
) {
    for event in events.read() {
        if event.is_player {
            continue;
        }

        let Ok((marker, reward, death_processed)) = mobs.get(event.entity) else {
            continue;
        };

        if death_processed.0 {
            continue;
        }

        xp_writer.write(XpGained {
            amount: reward.0,
            source: marker.0.spec().name.clone(),
        });
    }
}

pub fn roll_kill_loot(
    mut events: MessageReader<EntityDied>,
    mut loot_writer: MessageWriter<LootDropped>,
    mut player: Query<(&StatSheet, &mut Inventory), With<PlayerMarker>>,
    mobs: Query<(&MobLootTable, &DeathProcessed)>,
) {
    let Ok((stats, mut inventory)) = player.single_mut() else {
        return;
    };
    let magic_find = player_effective_magicfind(stats, &inventory);

    for event in events.read() {
        if event.is_player {
            continue;
        }

        let Ok((loot_table, death_processed)) = mobs.get(event.entity) else {
            continue;
        };

        if death_processed.0 {
            continue;
        }

        let drops = loot_table.0.roll_drops(magic_find);
        for drop in &drops {
            loot_writer.write(LootDropped {
                item_name: drop.item.name.clone(),
            });
        }
        collect_loot_drops(&mut *inventory, &drops);
    }
}

pub fn mark_death_processed(
    mut events: MessageReader<EntityDied>,
    mut mobs: Query<&mut DeathProcessed>,
) {
    for event in events.read() {
        if event.is_player {
            continue;
        }

        let Ok(mut death_processed) = mobs.get_mut(event.entity) else {
            continue;
        };

        death_processed.0 = true;
    }
}

pub fn despawn_dead_entity(mut commands: Commands, mut events: MessageReader<EntityDied>) {
    for event in events.read() {
        if event.is_player {
            continue;
        }

        commands.entity(event.entity).despawn();
    }
}
