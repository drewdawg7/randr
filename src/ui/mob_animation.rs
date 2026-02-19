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
                    revert_hurt_animation,
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

const MOB_FRAME_SIZE: UVec2 = UVec2::splat(32);

const MOB_SHEET_DEFS: [(MobId, &str, &str, Option<&str>, Option<&str>); 7] = [
    (MobId::Goblin,        "goblin",         "a_1", Some("a_4"), Some("a_6")),
    (MobId::Slime,         "slime",          "a_1", Some("a_4"), None),
    (MobId::Merchant,      "merchant",       "a_1", None,        None),
    (MobId::DwarfDefender, "dwarf_defender", "a_1", Some("a_4"), Some("a_7")),
    (MobId::DwarfWarrior,  "dwarf_warrior",  "a_1", Some("a_4"), Some("a_6")),
    (MobId::DwarfMiner,    "dwarf_miner",    "a_1", Some("a_4"), Some("a_6")),
    (MobId::DwarfKing,     "dwarf_king",     "a_1", Some("a_4"), Some("a_7")),
];

fn load_mob_sprite_sheets(
    asset_server: Res<AssetServer>,
    mut ase_sheets: ResMut<AseMobSheets>,
) {
    for (mob_id, file, idle_tag, hurt_tag, death_tag) in MOB_SHEET_DEFS {
        ase_sheets.insert(mob_id, AseMobSheet {
            aseprite: asset_server.load(format!("sprites/mobs/{file}.aseprite")),
            idle_tag,
            hurt_tag,
            death_tag,
            frame_size: MOB_FRAME_SIZE,
        });
    }
}

#[derive(Component)]
pub struct PlayingHurtAnimation;

fn trigger_hurt_animation(
    mut commands: Commands,
    mut events: MessageReader<DamageEntity>,
    mut mobs: Query<(&MobMarker, &mut AseAnimation), Without<PlayingHurtAnimation>>,
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
            .with_repeat(AnimationRepeat::Count(1))
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
