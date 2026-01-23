//! Unified sprite animation system.
//!
//! Provides a single animation component and system for all animated sprites.

use bevy::prelude::*;

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
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            first_frame: 0,
            last_frame: 3,
            frame_duration: 0.15,
            looping: true,
        }
    }
}

/// Component for animated sprites.
///
/// Add this to an entity with an `ImageNode` to animate it through
/// a range of atlas frames.
#[derive(Component)]
pub struct SpriteAnimation {
    /// Timer for frame advancement
    pub timer: Timer,
    /// Current frame index within the animation
    pub current_frame: usize,
    /// First frame index
    pub first_frame: usize,
    /// Last frame index (inclusive)
    pub last_frame: usize,
    /// Whether the animation loops
    pub looping: bool,
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
        }
    }
}

/// System to animate all sprites with `SpriteAnimation` component.
pub fn animate_sprites(time: Res<Time>, mut query: Query<(&mut SpriteAnimation, &mut ImageNode)>) {
    for (mut animation, mut image) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if animation.current_frame >= animation.last_frame {
                if animation.looping {
                    animation.current_frame = animation.first_frame;
                }
            } else {
                animation.current_frame += 1;
            }

            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = animation.current_frame;
            }
        }
    }
}
