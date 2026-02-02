use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::prelude::*;

pub struct PhysicsDebugTogglePlugin;

impl Plugin for PhysicsDebugTogglePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsDebugPlugin::default())
            .add_systems(Startup, disable_physics_debug)
            .add_systems(Update, toggle_physics_debug);
    }
}

fn disable_physics_debug(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<PhysicsGizmos>();
    config.enabled = false;
}

fn toggle_physics_debug(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    if keyboard.just_pressed(KeyCode::BracketRight) {
        let (config, _) = config_store.config_mut::<PhysicsGizmos>();
        config.enabled = !config.enabled;
    }
}
