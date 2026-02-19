use std::collections::HashMap;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::combat::events::DamageEntity;
use crate::mob::components::MobMarker;
use crate::mob::MobId;
use crate::states::AppState;

pub struct MobAnimationPlugin;

impl Plugin for MobAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AseMobSheets>()
            .add_systems(PreStartup, load_mob_sprite_sheets)
            .add_systems(
                Update,
                (
                    trigger_hurt_animation.run_if(on_message::<DamageEntity>),
                    revert_hurt_animation
                        .run_if(any_with_component::<PlayingHurtAnimation>),
                    trigger_death_animation.run_if(any_with_component::<DyingMob>),
                    despawn_after_death_animation
                        .run_if(any_with_component::<DyingMob>),
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

pub struct AseMobSheet {
    pub aseprite: Handle<Aseprite>,
    pub idle_tag: &'static str,
    pub hurt_tag: Option<&'static str>,
    pub death_tag: Option<&'static str>,
    pub frame_size: UVec2,
}

#[derive(Resource, Default)]
pub struct AseMobSheets {
    sheets: HashMap<MobId, AseMobSheet>,
}

impl AseMobSheets {
    pub fn get(&self, mob_id: MobId) -> Option<&AseMobSheet> {
        self.sheets.get(&mob_id)
    }

    fn insert(&mut self, mob_id: MobId, sheet: AseMobSheet) {
        self.sheets.insert(mob_id, sheet);
    }
}

fn load_mob_sprite_sheets(
    asset_server: Res<AssetServer>,
    mut ase_sheets: ResMut<AseMobSheets>,
) {
    ase_sheets.insert(MobId::Goblin, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/goblin.aseprite"),
        idle_tag: "a_1",
        hurt_tag: Some("a_4"),
        death_tag: Some("a_6"),
        frame_size: UVec2::splat(32),
    });

    ase_sheets.insert(MobId::Slime, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/slime.aseprite"),
        idle_tag: "a_1",
        hurt_tag: Some("a_4"),
        death_tag: None,
        frame_size: UVec2::splat(32),
    });

    ase_sheets.insert(MobId::Merchant, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/merchant.aseprite"),
        idle_tag: "idle",
        hurt_tag: Some("damage"),
        death_tag: Some("death"),
        frame_size: UVec2::splat(32),
    });

    ase_sheets.insert(MobId::DwarfDefender, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/dwarf_defender.aseprite"),
        idle_tag: "idle",
        hurt_tag: Some("hurt"),
        death_tag: Some("death"),
        frame_size: UVec2::splat(32),
    });

    ase_sheets.insert(MobId::DwarfWarrior, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/dwarf_warrior.aseprite"),
        idle_tag: "idle",
        hurt_tag: Some("hurt"),
        death_tag: Some("death"),
        frame_size: UVec2::splat(32),
    });

    ase_sheets.insert(MobId::DwarfMiner, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/dwarf_miner.aseprite"),
        idle_tag: "idle",
        hurt_tag: Some("hurt"),
        death_tag: Some("death"),
        frame_size: UVec2::splat(32),
    });

    ase_sheets.insert(MobId::DwarfKing, AseMobSheet {
        aseprite: asset_server.load("sprites/mobs/dwarf_king.aseprite"),
        idle_tag: "idle",
        hurt_tag: Some("hurt"),
        death_tag: Some("death"),
        frame_size: UVec2::splat(32),
    });
}

#[derive(Component)]
pub struct PlayingHurtAnimation;

#[derive(Component)]
pub struct DyingMob;

fn trigger_hurt_animation(
    mut commands: Commands,
    mut events: MessageReader<DamageEntity>,
    mut mobs: Query<
        (&MobMarker, &mut AseAnimation),
        (Without<PlayingHurtAnimation>, Without<DyingMob>),
    >,
    ase_sheets: Res<AseMobSheets>,
) {
    for event in events.read() {
        let Ok((marker, mut ase_anim)) = mobs.get_mut(event.target) else {
            continue;
        };
        let Some(sheet) = ase_sheets.get(marker.0) else {
            continue;
        };
        let Some(hurt_tag) = sheet.hurt_tag else {
            continue;
        };

        ase_anim.animation = Animation::tag(hurt_tag)
            .with_repeat(AnimationRepeat::Count(0))
            .with_then(sheet.idle_tag, AnimationRepeat::Loop);
        commands.entity(event.target).insert(PlayingHurtAnimation);
    }
}

fn revert_hurt_animation(
    mut commands: Commands,
    query: Query<(Entity, &AseAnimation, &MobMarker), With<PlayingHurtAnimation>>,
    ase_sheets: Res<AseMobSheets>,
) {
    for (entity, ase_anim, mob_marker) in &query {
        let Some(sheet) = ase_sheets.get(mob_marker.0) else {
            continue;
        };
        if ase_anim.animation.tag.as_deref() == Some(sheet.idle_tag) {
            commands.entity(entity).remove::<PlayingHurtAnimation>();
        }
    }
}

fn trigger_death_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &MobMarker, &mut AseAnimation), Added<DyingMob>>,
    ase_sheets: Res<AseMobSheets>,
) {
    for (entity, marker, mut ase_anim) in &mut query {
        let Some(sheet) = ase_sheets.get(marker.0) else {
            commands.entity(entity).despawn();
            continue;
        };
        let Some(death_tag) = sheet.death_tag else {
            commands.entity(entity).despawn();
            continue;
        };
        commands.entity(entity).remove::<PlayingHurtAnimation>();
        ase_anim.animation = Animation::tag(death_tag)
            .with_speed(0.5)
            .with_repeat(AnimationRepeat::Count(0))
            .with_then(sheet.idle_tag, AnimationRepeat::Loop);
    }
}

fn despawn_after_death_animation(
    mut commands: Commands,
    query: Query<(Entity, &AseAnimation, &MobMarker), With<DyingMob>>,
    ase_sheets: Res<AseMobSheets>,
) {
    for (entity, ase_anim, mob_marker) in &query {
        let Some(sheet) = ase_sheets.get(mob_marker.0) else {
            commands.entity(entity).despawn();
            continue;
        };
        if ase_anim.animation.tag.as_deref() == Some(sheet.idle_tag) {
            commands.entity(entity).despawn();
        }
    }
}
