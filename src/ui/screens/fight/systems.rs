use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::combat::ActiveCombatResource;
use crate::stats::{HasStats, StatSheet};
use crate::ui::{
    update_health_bar, HealthBarText, MobSpriteSheets, SpriteAnimation, SpriteHealthBar,
};

use super::components::{
    EnemyHealthBar, EnemyNameLabel, FightScreenRoot, NeedsFightBackground, NeedsFightPopup,
    NeedsMobSprite, PlayerHealthBar,
};

/// Query for the player health bar entity, excluding enemy health bar.
type PlayerHealthBarQuery<'w, 's> =
    Query<'w, 's, Entity, (With<PlayerHealthBar>, Without<EnemyHealthBar>)>;

/// Query for the enemy health bar entity, excluding player health bar.
type EnemyHealthBarQuery<'w, 's> =
    Query<'w, 's, Entity, (With<EnemyHealthBar>, Without<PlayerHealthBar>)>;

/// Resource holding the selected fight background name for the current fight.
#[derive(Resource, Default)]
pub struct SelectedFightBackground(pub Option<String>);

pub fn update_combat_visuals(
    stats: Res<StatSheet>,
    combat_res: Res<ActiveCombatResource>,
    game_sprites: Res<GameSprites>,
    player_health_bar: PlayerHealthBarQuery,
    enemy_health_bar: EnemyHealthBarQuery,
    children: Query<&Children>,
    mut sprite_query: Query<&mut ImageNode, With<SpriteHealthBar>>,
    mut text_query: Query<&mut Text, With<HealthBarText>>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    if let Ok(bar_entity) = player_health_bar.get_single() {
        update_health_bar(
            bar_entity,
            stats.hp(),
            stats.max_hp(),
            &children,
            &mut sprite_query,
            &mut text_query,
            sheet,
        );
    }

    if let Some(combat) = combat_res.get() {
        if let Ok(bar_entity) = enemy_health_bar.get_single() {
            let enemy_info = combat.enemy_info();
            update_health_bar(
                bar_entity,
                enemy_info.health,
                enemy_info.max_health,
                &children,
                &mut sprite_query,
                &mut text_query,
                sheet,
            );
        }
    }
}

pub fn cleanup_fight_screen(
    mut commands: Commands,
    fight_root: Query<Entity, With<FightScreenRoot>>,
) {
    if let Ok(entity) = fight_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System to populate the fight background when the asset is ready.
pub fn populate_fight_background(
    mut commands: Commands,
    query: Query<Entity, With<NeedsFightBackground>>,
    selected_bg: Res<SelectedFightBackground>,
    game_sprites: Res<GameSprites>,
) {
    let Some(bg_name) = &selected_bg.0 else {
        return;
    };
    let Some(sheet) = game_sprites.get(SpriteSheetKey::FightBackgrounds) else {
        return;
    };
    let Some(bg) = sheet.image_node(bg_name) else {
        return;
    };

    for entity in &query {
        commands
            .entity(entity)
            .remove::<NeedsFightBackground>()
            .remove::<BackgroundColor>()
            .insert(bg.clone());
    }
}

/// System to populate the mob sprite when the asset is ready.
pub fn populate_mob_sprite(
    mut commands: Commands,
    query: Query<Entity, With<NeedsMobSprite>>,
    mob_sheets: Res<MobSpriteSheets>,
    combat_res: Res<ActiveCombatResource>,
) {
    let Some(combat) = combat_res.get() else {
        return;
    };

    let Some(sheet) = mob_sheets.get(combat.mob.mob_id) else {
        return;
    };

    for entity in &query {
        commands
            .entity(entity)
            .remove::<NeedsMobSprite>()
            .insert((
                ImageNode::from_atlas_image(
                    sheet.texture.clone(),
                    TextureAtlas {
                        layout: sheet.layout.clone(),
                        index: sheet.animation.first_frame,
                    },
                ),
                SpriteAnimation::new(&sheet.animation.clone().into()),
            ));
    }
}

/// System to populate the fight popup background when the sprite loads.
pub fn populate_fight_popup(
    mut commands: Commands,
    query: Query<Entity, With<NeedsFightPopup>>,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::FightPopup) else {
        return;
    };
    let Some(popup) = sheet.image_node_sliced("Popup", 8.0) else {
        return;
    };

    for entity in &query {
        commands
            .entity(entity)
            .remove::<NeedsFightPopup>()
            .insert(popup.clone());
    }
}

/// Updates the enemy name label when combat is initialized.
pub fn update_enemy_name(
    combat_res: Res<ActiveCombatResource>,
    mut name_query: Query<&mut Text, With<EnemyNameLabel>>,
) {
    let Some(combat) = combat_res.get() else {
        return;
    };

    let enemy_name = combat.enemy_info().name;
    for mut text in name_query.iter_mut() {
        **text = enemy_name.clone();
    }
}
