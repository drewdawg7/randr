use bevy::prelude::*;
use tracing::instrument;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::game::{AnvilCraftingCompleteEvent, ForgeCraftingCompleteEvent};
use crate::ui::SpriteAnimation;

use super::{
    AnvilActiveTimer, AnvilTimerFinished, CraftingStationType, ForgeActiveTimer, ForgeTimerFinished,
};

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

#[instrument(level = "debug", skip_all, fields(entity = ?trigger.event().entity))]
fn on_forge_timer_finished(
    trigger: On<ForgeTimerFinished>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mut crafting_events: MessageWriter<ForgeCraftingCompleteEvent>,
    mut query: Query<&mut ImageNode>,
) {
    let entity = trigger.event().entity;

    crafting_events.write(ForgeCraftingCompleteEvent { entity });

    if let Some(idle_idx) = game_sprites
        .get(SpriteSheetKey::CraftingStations)
        .and_then(|sheet| sheet.get(CraftingStationType::Forge.sprite_name()))
    {
        if let Ok(mut image) = query.get_mut(entity) {
            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = idle_idx;
            }
        }
    }

    commands.entity(entity).remove::<ForgeActiveTimer>();
    commands.entity(entity).remove::<SpriteAnimation>();
}

fn on_anvil_timer_finished(
    trigger: On<AnvilTimerFinished>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mut crafting_events: MessageWriter<AnvilCraftingCompleteEvent>,
    mut query: Query<&mut ImageNode>,
) {
    let entity = trigger.event().entity;

    crafting_events.write(AnvilCraftingCompleteEvent { entity });

    if let Some(idle_idx) = game_sprites
        .get(SpriteSheetKey::CraftingStations)
        .and_then(|sheet| sheet.get(CraftingStationType::Anvil.sprite_name()))
    {
        if let Ok(mut image) = query.get_mut(entity) {
            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = idle_idx;
            }
        }
    }

    commands.entity(entity).remove::<AnvilActiveTimer>();
    commands.entity(entity).remove::<SpriteAnimation>();
}
