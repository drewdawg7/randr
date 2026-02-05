use avian2d::prelude::*;
use bevy::prelude::*;

use crate::combat::action::{AttackHitbox, HitEntities};
use crate::combat::events::DamageEntity;
use crate::combat::system::{player_attack_value, apply_defense};
use crate::inventory::Inventory;
use crate::mob::components::{CombatStats, MobMarker};
use crate::skills::{SkillType, Skills};
use crate::stats::StatSheet;

pub fn handle_hitbox_collisions(
    mut collisions: MessageReader<CollisionStart>,
    mut damage_writer: MessageWriter<DamageEntity>,
    mut hitboxes: Query<(&AttackHitbox, &mut HitEntities)>,
    mobs: Query<&CombatStats, With<MobMarker>>,
    stats: Res<StatSheet>,
    inventory: Res<Inventory>,
    skills: Res<Skills>,
) {
    let combat_level = skills
        .skill(SkillType::Combat)
        .map(|s| s.level)
        .unwrap_or(1);

    for event in collisions.read() {
        let Some((hitbox_entity, target)) = extract_hitbox_and_mob(event, &hitboxes, &mobs) else {
            continue;
        };

        let Ok((_, mut hit_entities)) = hitboxes.get_mut(hitbox_entity) else {
            continue;
        };

        if !hit_entities.0.insert(target) {
            continue;
        }

        let Ok(mob_combat_stats) = mobs.get(target) else {
            continue;
        };

        let attack = player_attack_value(&stats, &inventory, combat_level);
        let raw_damage = attack.roll_damage();
        let damage = apply_defense(raw_damage, mob_combat_stats.defense);

        damage_writer.write(DamageEntity {
            target,
            amount: damage,
        });
    }
}

fn extract_hitbox_and_mob<'a>(
    event: &CollisionStart,
    hitboxes: &Query<(&AttackHitbox, &mut HitEntities)>,
    mobs: &Query<&CombatStats, With<MobMarker>>,
) -> Option<(Entity, Entity)> {
    if hitboxes.contains(event.collider1) && mobs.contains(event.collider2) {
        Some((event.collider1, event.collider2))
    } else if hitboxes.contains(event.collider2) && mobs.contains(event.collider1) {
        Some((event.collider2, event.collider1))
    } else {
        None
    }
}
