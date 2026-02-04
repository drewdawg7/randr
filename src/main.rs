use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use game::plugins::GamePlugin;

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let filename = format!("game_{}.log", timestamp);
    let file_appender = rolling::never("logs", filename);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::new("warn,game=debug");

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE))
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
                .json(),
        )
        .init();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // Pixel-perfect rendering for sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "R&R".into(),
                        resolution: (1280u32, 720u32).into(),
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(GamePlugin)
        .run();
}
