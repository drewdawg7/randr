use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::combat::hitbox::Attacking;
use crate::input::HeldDirection;

use super::animation::{
    animate_sprites, animate_world_sprites, tick_animation_clock, AnimationClock,
};

pub const PLAYER_IDLE_TAG: &str = "a_1";
pub const PLAYER_WALK_TAG: &str = "a_2";
pub const PLAYER_ATTACK_TAG: &str = "a_4";

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        use super::animation::SpriteAnimation;

        app.init_resource::<PlayerSpriteSheet>()
            .init_resource::<AnimationClock>()
            .add_systems(PreStartup, load_player_sprite_sheet)
            .add_systems(Update, tick_animation_clock)
            .add_systems(
                Update,
                (animate_sprites, animate_world_sprites)
                    .run_if(any_with_component::<SpriteAnimation>),
            )
            .add_systems(
                Update,
                sync_player_animation.run_if(any_with_component::<PlayerWalkTimer>),
            )
            .add_observer(revert_to_idle);
    }
}

#[derive(Resource, Default)]
pub struct PlayerSpriteSheet {
    pub aseprite: Option<Handle<Aseprite>>,
    pub frame_size: UVec2,
}

#[derive(Component)]
pub struct PlayerWalkTimer(pub Timer);

fn load_player_sprite_sheet(
    asset_server: Res<AssetServer>,
    mut player_sheet: ResMut<PlayerSpriteSheet>,
) {
    player_sheet.aseprite = Some(asset_server.load("sprites/player/lightning_warrior.aseprite"));
    player_sheet.frame_size = UVec2::splat(32);
    info!("Loaded player sprite sheet: MiniLightningWarrior (aseprite)");
}

fn revert_to_idle(
    trigger: On<Remove, Attacking>,
    mut query: Query<&mut AseAnimation>,
) {
    if let Ok(mut ase_anim) = query.get_mut(trigger.entity) {
        ase_anim.animation = Animation::tag(PLAYER_IDLE_TAG)
            .with_repeat(AnimationRepeat::Loop);
    }
}

fn sync_player_animation(
    time: Res<Time>,
    held_direction: Res<HeldDirection>,
    mut query: Query<(&mut PlayerWalkTimer, &mut AseAnimation)>,
) {
    for (mut timer, mut ase_anim) in &mut query {
        timer.0.tick(time.delta());

        let is_idle = ase_anim.animation.tag.as_deref() == Some(PLAYER_IDLE_TAG);
        let is_moving = held_direction.0.is_some();

        if is_moving {
            if is_idle {
                ase_anim.animation = Animation::tag(PLAYER_WALK_TAG)
                    .with_repeat(AnimationRepeat::Loop);
            }
            timer.0.reset();
        } else if timer.0.just_finished() && !is_idle {
            ase_anim.animation = Animation::tag(PLAYER_IDLE_TAG)
                .with_repeat(AnimationRepeat::Loop);
        }
    }
}
