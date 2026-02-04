use bevy::prelude::*;

use crate::game::{AnvilCraftingCompleteEvent, ForgeCraftingCompleteEvent};

use super::{AnvilActiveTimer, AnvilTimerFinished, ForgeActiveTimer, ForgeTimerFinished};

pub struct CraftingStationPlugin;

impl Plugin for CraftingStationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_forge_timer_finished)
            .add_observer(on_anvil_timer_finished)
            .add_systems(
                Update,
                (
                    poll_forge_timers.run_if(any_with_component::<ForgeActiveTimer>),
                    poll_anvil_timers.run_if(any_with_component::<AnvilActiveTimer>),
                ),
            );
    }
}

fn poll_forge_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ForgeActiveTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.trigger(ForgeTimerFinished { entity });
        }
    }
}

fn poll_anvil_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnvilActiveTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.trigger(AnvilTimerFinished { entity });
        }
    }
}

fn on_forge_timer_finished(
    trigger: On<ForgeTimerFinished>,
    mut commands: Commands,
    mut crafting_events: MessageWriter<ForgeCraftingCompleteEvent>,
) {
    let entity = trigger.event().entity;
    crafting_events.write(ForgeCraftingCompleteEvent { entity });
    commands.entity(entity).remove::<ForgeActiveTimer>();
}

fn on_anvil_timer_finished(
    trigger: On<AnvilTimerFinished>,
    mut commands: Commands,
    mut crafting_events: MessageWriter<AnvilCraftingCompleteEvent>,
) {
    let entity = trigger.event().entity;
    crafting_events.write(AnvilCraftingCompleteEvent { entity });
    commands.entity(entity).remove::<AnvilActiveTimer>();
}
