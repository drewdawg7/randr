use bevy::prelude::*;

use game::plugins::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // Pixel-perfect rendering for sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "R&R".into(),
                        resolution: (1280., 720.).into(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .run();
}
