use bevy::{ecs::system::SystemParam, prelude::*};
use tracing::instrument;

use super::events::{DealDamage, EntityDied, PlayerAttackMob, VictoryAchieved};
use super::system::{
    apply_victory_rewards_direct, entity_attacks_player, player_attacks_entity,
    player_effective_magicfind, process_player_defeat,
};
use crate::dungeon::{GridOccupancy, GridSize};
use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::loot::collect_loot_drops;
use crate::mob::{
    CombatStats, DeathProcessed, GoldReward, Health, MobLootTable, MobMarker, XpReward,
};
use crate::player::PlayerGold;
use crate::plugins::MobDefeated;
use crate::skills::{SkillType, SkillXpGained, Skills};
use crate::stats::StatSheet;
use crate::states::AppState;
use crate::ui::screens::FightModalMob;

#[derive(SystemParam)]
struct PlayerResources<'w> {
    gold: ResMut<'w, PlayerGold>,
    progression: ResMut<'w, Progression>,
    inventory: ResMut<'w, Inventory>,
    stats: ResMut<'w, StatSheet>,
}

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
            .add_message::<VictoryAchieved>()
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
    mut stats: ResMut<StatSheet>,
    inventory: Res<Inventory>,
    skills: Res<Skills>,
    mut mob_query: Query<(&mut Health, &CombatStats)>,
) {
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
            &inventory,
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
                entity_attacks_player(mob_combat_stats, &mut stats, &inventory, combat_level);

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
    mut victory_events: MessageWriter<VictoryAchieved>,
    mut occupancy: ResMut<GridOccupancy>,
    mut player: PlayerResources,
    fight_mob: Option<Res<FightModalMob>>,
    mut mob_query: Query<(
        &MobMarker,
        &GoldReward,
        &XpReward,
        &MobLootTable,
        &mut DeathProcessed,
    )>,
) {
    for event in events.read() {
        if event.is_player {
            continue;
        }

        let Some(ref fight_mob) = fight_mob else {
            continue;
        };

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
        let mob_name = mob_id.spec().name.clone();

        let magic_find = player_effective_magicfind(&player.stats, &player.inventory);
        let loot_drops = loot_table.0.roll_drops(magic_find);

        let rewards = apply_victory_rewards_direct(
            &mut player.stats,
            &player.inventory,
            &mut player.gold,
            &mut player.progression,
            gold_reward.0,
            xp_reward.0,
        );

        collect_loot_drops(&mut *player.inventory, &loot_drops);

        mob_defeated_events.write(MobDefeated { mob_id });

        skill_xp_events.write(SkillXpGained {
            skill: SkillType::Combat,
            amount: xp_reward.0 as u64,
        });

        occupancy.vacate(fight_mob.pos, GridSize::single());
        commands.entity(event.entity).despawn();

        victory_events.write(VictoryAchieved {
            mob_id,
            mob_name,
            gold_gained: rewards.gold_gained,
            xp_gained: rewards.xp_gained,
            loot_drops,
        });

        commands.remove_resource::<ActiveCombat>();
    }
}

#[instrument(level = "debug", skip_all)]
fn handle_player_death(
    mut commands: Commands,
    mut events: MessageReader<EntityDied>,
    mut stats: ResMut<StatSheet>,
    mut player_gold: ResMut<PlayerGold>,
) {
    for event in events.read() {
        if !event.is_player {
            continue;
        }

        process_player_defeat(&mut stats, &mut player_gold);
        commands.remove_resource::<ActiveCombat>();
    }
}
