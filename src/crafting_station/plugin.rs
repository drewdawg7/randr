use bevy::prelude::*;
use tracing::instrument;

use crate::game::{AnvilCraftingCompleteEvent, ForgeCraftingCompleteEvent};

use super::anvil::handle_try_start_anvil_crafting;
use super::events::{
    AnvilCraftingStarted, ForgeCraftingStarted, TryStartAnvilCrafting, TryStartForgeCrafting,
};
use super::forge::handle_try_start_forge_crafting;
use super::{AnvilActiveTimer, AnvilTimerFinished, ForgeActiveTimer, ForgeTimerFinished};

pub struct CraftingStationPlugin;

impl Plugin for CraftingStationPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TryStartForgeCrafting>()
            .add_message::<ForgeCraftingStarted>()
            .add_message::<TryStartAnvilCrafting>()
            .add_message::<AnvilCraftingStarted>()
            .add_observer(on_forge_timer_finished)
            .add_observer(on_anvil_timer_finished)
            .add_systems(
                Update,
                (
                    handle_try_start_forge_crafting.run_if(on_message::<TryStartForgeCrafting>),
                    handle_try_start_anvil_crafting.run_if(on_message::<TryStartAnvilCrafting>),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    poll_forge_timers.run_if(any_with_component::<ForgeActiveTimer>),
                    poll_anvil_timers.run_if(any_with_component::<AnvilActiveTimer>),
                ),
            );
    }
}

fn poll_forge_timers(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
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
    time: Res<Time<Fixed>>,
    mut query: Query<(Entity, &mut AnvilActiveTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.trigger(AnvilTimerFinished { entity });
        }
    }
}

#[instrument(level = "debug", skip_all, fields(entity = ?trigger.event().entity))]
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
