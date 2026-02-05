use bevy::prelude::*;

use crate::combat::events::{DamageEntity, EntityDied};
use crate::mob::components::Health;

pub fn apply_damage(
    mut events: MessageReader<DamageEntity>,
    mut death_writer: MessageWriter<EntityDied>,
    mut targets: Query<&mut Health>,
    mut already_dead: Local<Vec<Entity>>,
) {
    already_dead.clear();

    for event in events.read() {
        if already_dead.contains(&event.target) {
            continue;
        }

        let Ok(mut health) = targets.get_mut(event.target) else {
            continue;
        };

        health.take_damage(event.amount);

        if !health.is_alive() {
            already_dead.push(event.target);
            death_writer.write(EntityDied {
                entity: event.target,
                is_player: false,
            });
        }
    }
}
