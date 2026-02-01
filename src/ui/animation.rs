//! Unified sprite animation system.
//!
//! Provides a single animation component and system for all animated sprites.
//! Synchronized animations use a global `AnimationClock` to stay in phase.

use bevy::prelude::*;

/// Global animation clock for synchronized looping animations.
///
/// All synchronized animations with the same frame_duration will be in phase,
/// regardless of when they were spawned.
#[derive(Resource, Default)]
pub struct AnimationClock {
    pub elapsed: f32,
}

/// Configuration for a sprite animation.
#[derive(Debug, Clone)]
pub struct AnimationConfig {
    /// First frame index of the animation
    pub first_frame: usize,
    /// Last frame index of the animation (inclusive)
    pub last_frame: usize,
    /// Duration per frame in seconds
    pub frame_duration: f32,
    /// Whether the animation loops (default true). If false, stops on last frame.
    pub looping: bool,
    /// If true, animation phase is derived from the global AnimationClock
    /// so all animations with the same config stay in sync.
    pub synchronized: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            first_frame: 0,
            last_frame: 3,
            frame_duration: 0.15,
            looping: true,
            synchronized: true,
        }
    }
}

/// Component for animated sprites.
///
/// Add this to an entity with an `ImageNode` to animate it through
/// a range of atlas frames.
#[derive(Component)]
pub struct SpriteAnimation {
    /// Timer for frame advancement (used by non-synchronized animations)
    pub timer: Timer,
    /// Current frame index within the animation
    pub current_frame: usize,
    /// First frame index
    pub first_frame: usize,
    /// Last frame index (inclusive)
    pub last_frame: usize,
    /// Whether the animation loops
    pub looping: bool,
    /// Duration per frame in seconds (stored for clock-based calculation)
    pub frame_duration: f32,
    /// Whether this animation is synchronized to the global clock
    pub synchronized: bool,
}

impl SpriteAnimation {
    /// Create a new sprite animation from a configuration.
    pub fn new(config: &AnimationConfig) -> Self {
        Self {
            timer: Timer::from_seconds(config.frame_duration, TimerMode::Repeating),
            current_frame: config.first_frame,
            first_frame: config.first_frame,
            last_frame: config.last_frame,
            looping: config.looping,
            frame_duration: config.frame_duration,
            synchronized: config.synchronized,
        }
    }
}

/// System to tick the global animation clock.
pub fn tick_animation_clock(time: Res<Time>, mut clock: ResMut<AnimationClock>) {
    clock.elapsed += time.delta_secs();
}

/// System to animate UI sprites (ImageNode) with `SpriteAnimation` component.
pub fn animate_sprites(
    time: Res<Time>,
    clock: Res<AnimationClock>,
    mut query: Query<(&mut SpriteAnimation, &mut ImageNode)>,
) {
    for (mut animation, mut image) in &mut query {
        advance_animation(&time, &clock, &mut animation);
        if let Some(ref mut atlas) = image.texture_atlas {
            if atlas.index != animation.current_frame {
                atlas.index = animation.current_frame;
            }
        }
    }
}

/// System to animate world-space sprites (Sprite) with `SpriteAnimation` component.
pub fn animate_world_sprites(
    time: Res<Time>,
    clock: Res<AnimationClock>,
    mut query: Query<(&mut SpriteAnimation, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        advance_animation(&time, &clock, &mut animation);
        if let Some(ref mut atlas) = sprite.texture_atlas {
            if atlas.index != animation.current_frame {
                atlas.index = animation.current_frame;
            }
        }
    }
}

fn advance_animation(time: &Time, clock: &AnimationClock, animation: &mut SpriteAnimation) {
    if animation.synchronized && animation.looping {
        let frame_count = animation.last_frame - animation.first_frame + 1;
        let total_frames_elapsed = (clock.elapsed / animation.frame_duration) as usize;
        animation.current_frame = animation.first_frame + (total_frames_elapsed % frame_count);
    } else {
        animation.timer.tick(time.delta());
        for _ in 0..animation.timer.times_finished_this_tick() {
            if animation.current_frame >= animation.last_frame {
                if animation.looping {
                    animation.current_frame = animation.first_frame;
                } else {
                    break;
                }
            } else {
                animation.current_frame += 1;
            }
        }
    }
}
