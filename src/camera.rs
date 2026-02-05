//! Camera configuration and plugin.

use bevy::prelude::*;

/// Configuration for the game camera.
#[derive(Resource, Clone, Debug)]
pub struct CameraConfig {
    /// Orthographic projection scale. Default: 0.5
    pub scale: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self { scale: 0.5 }
    }
}

/// Plugin that spawns and configures the game camera.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>()
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, config: Res<CameraConfig>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: config.scale,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
