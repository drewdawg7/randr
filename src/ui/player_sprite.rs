use bevy::prelude::*;

use crate::combat::hitbox::Attacking;
use crate::input::HeldDirection;

use super::animation::{AnimationConfig, SpriteAnimation};
use super::sprite_marker::{SpriteData, SpriteMarker, SpriteMarkerAppExt};

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        use super::animation::{
            animate_sprites, animate_world_sprites, tick_animation_clock, AnimationClock,
        };

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
            .add_observer(revert_to_idle)
            .register_sprite_marker::<DungeonPlayerSprite>();
    }
}

#[derive(Resource, Default)]
pub struct PlayerSpriteSheet {
    pub texture: Option<Handle<Image>>,
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub animation: AnimationConfig,
    pub walk_animation: AnimationConfig,
    pub attack_animation: AnimationConfig,
    pub frame_size: UVec2,
}

impl PlayerSpriteSheet {
    pub fn is_loaded(&self) -> bool {
        self.texture.is_some() && self.layout.is_some()
    }
}

#[derive(Component)]
pub struct PlayerWalkTimer(pub Timer);

#[derive(Component)]
pub struct DungeonPlayerSprite;

impl SpriteMarker for DungeonPlayerSprite {
    type Resources = Res<'static, PlayerSpriteSheet>;

    fn resolve(&self, sheet: &Res<PlayerSpriteSheet>) -> Option<SpriteData> {
        if !sheet.is_loaded() {
            return None;
        }
        Some(SpriteData {
            texture: sheet.texture.as_ref()?.clone(),
            layout: sheet.layout.as_ref()?.clone(),
            animation: sheet.animation.clone(),
            flip_x: false,
        })
    }
}

fn load_player_sprite_sheet(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_sheet: ResMut<PlayerSpriteSheet>,
) {
    // MiniLightningWarrior: 13x8 grid of 32x32, idle is slices 0-3
    let texture: Handle<Image> = asset_server.load("sprites/player/lightning_warrior.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 13, 8, None, None);
    let layout_handle = layouts.add(layout);

    player_sheet.texture = Some(texture);
    player_sheet.layout = Some(layout_handle);
    player_sheet.frame_size = UVec2::splat(32);
    player_sheet.animation = AnimationConfig {
        first_frame: 0,
        last_frame: 3,
        frame_duration: 0.15,
        looping: true,
        synchronized: true,
    };
    player_sheet.walk_animation = AnimationConfig {
        first_frame: 13,
        last_frame: 18,
        frame_duration: 0.10,
        looping: true,
        synchronized: false,
    };
    player_sheet.attack_animation = AnimationConfig {
        first_frame: 39,
        last_frame: 47,
        frame_duration: 0.06,
        looping: false,
        synchronized: false,
    };

    info!("Loaded player sprite sheet: MiniLightningWarrior");
}

fn revert_to_idle(
    trigger: On<Remove, Attacking>,
    sheet: Res<PlayerSpriteSheet>,
    mut query: Query<&mut SpriteAnimation>,
) {
    if let Ok(mut anim) = query.get_mut(trigger.entity) {
        anim.apply_config(&sheet.animation);
    }
}

fn sync_player_animation(
    time: Res<Time>,
    sheet: Res<PlayerSpriteSheet>,
    held_direction: Res<HeldDirection>,
    mut query: Query<(&mut PlayerWalkTimer, &mut SpriteAnimation)>,
) {
    for (mut timer, mut anim) in &mut query {
        timer.0.tick(time.delta());

        let is_idle = anim.first_frame == sheet.animation.first_frame;
        let is_moving = held_direction.0.is_some();

        if is_moving {
            if is_idle {
                anim.apply_config(&sheet.walk_animation);
            }
            timer.0.reset();
        } else if timer.0.just_finished() && !is_idle {
            anim.apply_config(&sheet.animation);
        }
    }
}
