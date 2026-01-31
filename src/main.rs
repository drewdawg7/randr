use bevy::prelude::*;
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use game::plugins::GamePlugin;

fn main() {
    let file_appender = rolling::daily("logs", "game.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::new("warn,game=debug");

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
        .init();

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
