use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, spawn_fps_display)
            .add_systems(Update, update_fps_display);
    }
}

#[derive(Component)]
struct FpsText;

fn spawn_fps_display(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
            ZIndex(2000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                FpsText,
            ));
        });
}

fn update_fps_display(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **text = format!("FPS: {:.0}", value);
            }
        }
    }
}
