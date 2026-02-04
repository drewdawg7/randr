use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::crafting_station::{
    AnvilCraftingStarted, AnvilTimerFinished, CraftingStationType, ForgeCraftingStarted,
    ForgeTimerFinished,
};
use crate::ui::animation::{AnimationConfig, SpriteAnimation};

const CRAFTING_ANIMATION_FRAME_DURATION: f32 = 0.1;

pub fn handle_forge_crafting_started(
    mut events: MessageReader<ForgeCraftingStarted>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) else {
        return;
    };

    let (Some(first), Some(last)) = (
        sheet.get("forge_1_active1"),
        sheet.get("forge_1_active3"),
    ) else {
        return;
    };

    let config = AnimationConfig {
        first_frame: first,
        last_frame: last,
        frame_duration: CRAFTING_ANIMATION_FRAME_DURATION,
        looping: true,
        synchronized: false,
    };

    for event in events.read() {
        commands
            .entity(event.entity)
            .insert(SpriteAnimation::new(&config));
    }
}

pub fn handle_anvil_crafting_started(
    mut events: MessageReader<AnvilCraftingStarted>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) else {
        return;
    };

    let (Some(first), Some(last)) = (
        sheet.get("anvil_active1"),
        sheet.get("anvil_active_6"),
    ) else {
        return;
    };

    let config = AnimationConfig {
        first_frame: first,
        last_frame: last,
        frame_duration: CRAFTING_ANIMATION_FRAME_DURATION,
        looping: true,
        synchronized: false,
    };

    for event in events.read() {
        commands
            .entity(event.entity)
            .insert(SpriteAnimation::new(&config));
    }
}

pub fn on_forge_timer_finished(
    trigger: On<ForgeTimerFinished>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mut query: Query<&mut Sprite>,
) {
    let entity = trigger.event().entity;

    let Some(idle_idx) = game_sprites
        .get(SpriteSheetKey::CraftingStations)
        .and_then(|sheet| sheet.get(CraftingStationType::Forge.sprite_name()))
    else {
        return;
    };

    if let Ok(mut sprite) = query.get_mut(entity) {
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = idle_idx;
        }
    }

    commands.entity(entity).remove::<SpriteAnimation>();
}

pub fn on_anvil_timer_finished(
    trigger: On<AnvilTimerFinished>,
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    mut query: Query<&mut Sprite>,
) {
    let entity = trigger.event().entity;

    let Some(idle_idx) = game_sprites
        .get(SpriteSheetKey::CraftingStations)
        .and_then(|sheet| sheet.get(CraftingStationType::Anvil.sprite_name()))
    else {
        return;
    };

    if let Ok(mut sprite) = query.get_mut(entity) {
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.index = idle_idx;
        }
    }

    commands.entity(entity).remove::<SpriteAnimation>();
}
