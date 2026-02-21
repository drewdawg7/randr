use bevy::prelude::*;
use tracing::instrument;

use super::events::{DealDamage, EntityDied, PlayerAttackMob};
use super::system::{
    apply_victory_rewards_direct, entity_attacks_player, player_attacks_entity,
    player_effective_magicfind, process_player_defeat,
};

use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::item::ItemRegistry;
use crate::loot::collect_loot_drops;
use crate::mob::{
    CombatStats, DeathProcessed, GoldReward, Health, MobLootTable, MobMarker, XpReward,
};
use crate::player::{PlayerGold, PlayerMarker};
use crate::ui::DyingMob;
use crate::plugins::MobDefeated;
use crate::skills::{SkillType, SkillXpGained, Skills};
use crate::stats::StatSheet;
use crate::states::AppState;

#[derive(Resource)]
pub struct ActiveCombat {
    pub mob_entity: Entity,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerAttackMob>()
            .add_message::<DealDamage>()
            .add_message::<EntityDied>()
            .add_systems(
                Update,
                (
                    process_player_attack.run_if(on_message::<PlayerAttackMob>),
                    handle_mob_death.run_if(on_message::<EntityDied>),
                    handle_player_death.run_if(on_message::<EntityDied>),
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon))
                    .run_if(resource_exists::<ActiveCombat>),
            );
    }
}

#[instrument(level = "debug", skip_all)]
fn process_player_attack(
    mut events: MessageReader<PlayerAttackMob>,
    mut deal_damage_events: MessageWriter<DealDamage>,
    mut entity_died_events: MessageWriter<EntityDied>,
    mut player: Query<(&mut StatSheet, &Inventory), With<PlayerMarker>>,
    skills: Res<Skills>,
    mut mob_query: Query<(&mut Health, &CombatStats)>,
) {
    let Ok((mut stats, inventory)) = player.single_mut() else {
        return;
    };
    let combat_level = skills
        .skill(SkillType::Combat)
        .map(|s| s.level)
        .unwrap_or(1);

    for event in events.read() {
        let Ok((mut mob_health, mob_combat_stats)) = mob_query.get_mut(event.target) else {
            continue;
        };

        let result = player_attacks_entity(
            &stats,
            inventory,
            &mut mob_health,
            mob_combat_stats,
            combat_level,
        );

        deal_damage_events.write(DealDamage {
            target: event.target,
            amount: 0,
            source_name: "Player".to_string(),
        });

        if result.target_died {
            entity_died_events.write(EntityDied {
                entity: event.target,
                is_player: false,
            });
        } else {
            let counter_result =
                entity_attacks_player(mob_combat_stats, &mut stats, inventory, combat_level);

            deal_damage_events.write(DealDamage {
                target: Entity::PLACEHOLDER,
                amount: 0,
                source_name: "Enemy".to_string(),
            });

            if counter_result.target_died {
                entity_died_events.write(EntityDied {
                    entity: Entity::PLACEHOLDER,
                    is_player: true,
                });
            }
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn handle_mob_death(
    mut commands: Commands,
    mut events: MessageReader<EntityDied>,
    mut mob_defeated_events: MessageWriter<MobDefeated>,
    mut skill_xp_events: MessageWriter<SkillXpGained>,
    mut player: Query<
        (&mut StatSheet, &mut Inventory, &mut PlayerGold, &mut Progression),
        With<PlayerMarker>,
    >,
    mut mob_query: Query<(
        &MobMarker,
        &GoldReward,
        &XpReward,
        &MobLootTable,
        &mut DeathProcessed,
    )>,
    registry: Res<ItemRegistry>,
) {
    let Ok((mut stats, mut inventory, mut gold, mut progression)) = player.single_mut() else {
        return;
    };

    for event in events.read() {
        if event.is_player {
            continue;
        }

        let Ok((mob_marker, gold_reward, xp_reward, loot_table, mut death_processed)) =
            mob_query.get_mut(event.entity)
        else {
            continue;
        };

        if death_processed.0 {
            continue;
        }
        death_processed.0 = true;

        let mob_id = mob_marker.0;

        let magic_find = player_effective_magicfind(&stats, &inventory);
        let loot_drops = loot_table.0.roll_drops(magic_find, &registry);

        apply_victory_rewards_direct(
            &mut stats,
            &inventory,
            &mut gold,
            &mut progression,
            gold_reward.0,
            xp_reward.0,
        );

        collect_loot_drops(&mut *inventory, &loot_drops);

        mob_defeated_events.write(MobDefeated { mob_id });

        skill_xp_events.write(SkillXpGained {
            skill: SkillType::Combat,
            amount: xp_reward.0 as u64,
        });

        commands.entity(event.entity).insert(DyingMob);

        commands.remove_resource::<ActiveCombat>();
    }
}

#[instrument(level = "debug", skip_all)]
fn handle_player_death(
    mut commands: Commands,
    mut events: MessageReader<EntityDied>,
    mut player: Query<(&mut StatSheet, &mut PlayerGold), With<PlayerMarker>>,
) {
    let Ok((mut stats, mut player_gold)) = player.single_mut() else {
        return;
    };

    for event in events.read() {
        if !event.is_player {
            continue;
        }

        process_player_defeat(&mut stats, &mut player_gold);
        commands.remove_resource::<ActiveCombat>();
    }
}
