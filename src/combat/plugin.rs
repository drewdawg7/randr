use bevy::{ecs::system::SystemParam, prelude::*};

use super::events::{DealDamage, EntityDied, PlayerAttackMob};
use super::system::{
    apply_victory_rewards_direct, entity_attacks_player, player_attacks_entity,
    player_effective_magicfind, process_player_defeat,
};
use crate::dungeon::{GridOccupancy, GridSize};
use crate::entities::Progression;
use crate::inventory::Inventory;
use crate::loot::{collect_loot_drops, LootDrop};
use crate::mob::{
    CombatStats, DeathProcessed, GoldReward, Health, MobId, MobLootTable, MobMarker, XpReward,
};
use crate::player::PlayerGold;
use crate::plugins::MobDefeated;
use crate::skills::{SkillType, SkillXpGained};
use crate::stats::StatSheet;
use crate::ui::screens::FightModalMob;

#[derive(SystemParam)]
struct PlayerResources<'w> {
    gold: ResMut<'w, PlayerGold>,
    progression: ResMut<'w, Progression>,
    inventory: ResMut<'w, Inventory>,
    stats: ResMut<'w, StatSheet>,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttackMob>()
            .add_event::<DealDamage>()
            .add_event::<EntityDied>()
            .add_systems(
                Update,
                (
                    process_player_attack.run_if(on_event::<PlayerAttackMob>),
                    handle_mob_death.run_if(on_event::<EntityDied>),
                    handle_player_death.run_if(on_event::<EntityDied>),
                ),
            );
    }
}

#[derive(Resource)]
pub struct PendingVictory {
    pub mob_id: MobId,
    pub mob_name: String,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}

fn process_player_attack(
    mut events: EventReader<PlayerAttackMob>,
    mut deal_damage_events: EventWriter<DealDamage>,
    mut entity_died_events: EventWriter<EntityDied>,
    mut stats: ResMut<StatSheet>,
    inventory: Res<Inventory>,
    mut mob_query: Query<(&mut Health, &CombatStats)>,
) {
    for event in events.read() {
        let Ok((mut mob_health, mob_combat_stats)) = mob_query.get_mut(event.target) else {
            continue;
        };

        let result = player_attacks_entity(&stats, &inventory, &mut mob_health, mob_combat_stats);

        deal_damage_events.send(DealDamage {
            target: event.target,
            amount: 0,
            source_name: "Player".to_string(),
        });

        if result.target_died {
            entity_died_events.send(EntityDied {
                entity: event.target,
                is_player: false,
            });
        } else {
            let counter_result = entity_attacks_player(mob_combat_stats, &mut stats, &inventory);

            deal_damage_events.send(DealDamage {
                target: Entity::PLACEHOLDER,
                amount: 0,
                source_name: "Enemy".to_string(),
            });

            if counter_result.target_died {
                entity_died_events.send(EntityDied {
                    entity: Entity::PLACEHOLDER,
                    is_player: true,
                });
            }
        }
    }
}

fn handle_mob_death(
    mut commands: Commands,
    mut events: EventReader<EntityDied>,
    mut mob_defeated_events: EventWriter<MobDefeated>,
    mut skill_xp_events: EventWriter<SkillXpGained>,
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

        mob_defeated_events.send(MobDefeated { mob_id });

        skill_xp_events.send(SkillXpGained {
            skill: SkillType::Combat,
            amount: xp_reward.0 as u64,
        });

        occupancy.vacate(fight_mob.pos, GridSize::single());
        commands.entity(event.entity).despawn_recursive();

        commands.insert_resource(PendingVictory {
            mob_id,
            mob_name,
            gold_gained: rewards.gold_gained,
            xp_gained: rewards.xp_gained,
            loot_drops,
        });
    }
}

fn handle_player_death(
    mut events: EventReader<EntityDied>,
    mut stats: ResMut<StatSheet>,
    mut player_gold: ResMut<PlayerGold>,
) {
    for event in events.read() {
        if !event.is_player {
            continue;
        }

        process_player_defeat(&mut stats, &mut player_gold);
    }
}
