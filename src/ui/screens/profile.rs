use bevy::prelude::*;

use crate::entities::Progression;
use crate::game::{PlayerGold, PlayerName};
use crate::input::GameAction;
use crate::stats::{HasStats, StatSheet};
use crate::states::AppState;
use crate::ui::widgets::StatRow;

/// Plugin that manages the profile/stats screen.
pub struct ProfilePlugin;

impl Plugin for ProfilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Profile), spawn_profile_screen)
            .add_systems(OnExit(AppState::Profile), despawn_profile_screen)
            .add_systems(
                Update,
                handle_back_action.run_if(in_state(AppState::Profile)),
            );
    }
}

/// Component marker for the profile screen UI root.
#[derive(Component)]
struct ProfileScreenRoot;

/// System to spawn the profile screen UI.
fn spawn_profile_screen(
    mut commands: Commands,
    name: Res<PlayerName>,
    gold: Res<PlayerGold>,
    stats: Res<StatSheet>,
    prog: Res<Progression>,
) {
    // Root container
    commands
        .spawn((
            ProfileScreenRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ))
        .with_children(|parent| {
            // Title - Character Name
            parent.spawn((
                Text::new(format!("{}'s Profile", name.0)),
                TextFont {
                    font_size: 56.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Stats container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    padding: UiRect::all(Val::Px(30.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        StatRow::builder("HP:", format!("{} / {}", stats.hp(), stats.max_hp()))
                            .label_width(150.0)
                            .font_size(28.0)
                            .column_gap(15.0)
                            .label_color(Color::srgb(0.8, 0.8, 0.8))
                            .value_color(Color::srgb(0.9, 0.2, 0.2))
                            .build(),
                    );

                    parent.spawn(
                        StatRow::builder("Gold:", format!("{}", gold.0))
                            .label_width(150.0)
                            .font_size(28.0)
                            .column_gap(15.0)
                            .label_color(Color::srgb(0.8, 0.8, 0.8))
                            .value_color(Color::srgb(1.0, 0.84, 0.0))
                            .build(),
                    );

                    parent.spawn(
                        StatRow::builder("Attack:", format!("{}", stats.attack()))
                            .label_width(150.0)
                            .font_size(28.0)
                            .column_gap(15.0)
                            .label_color(Color::srgb(0.8, 0.8, 0.8))
                            .value_color(Color::srgb(1.0, 0.4, 0.2))
                            .build(),
                    );

                    parent.spawn(
                        StatRow::builder("Defense:", format!("{}", stats.defense()))
                            .label_width(150.0)
                            .font_size(28.0)
                            .column_gap(15.0)
                            .label_color(Color::srgb(0.8, 0.8, 0.8))
                            .value_color(Color::srgb(0.4, 0.6, 1.0))
                            .build(),
                    );

                    parent.spawn(
                        StatRow::builder("Level:", format!("{}", prog.level))
                            .label_width(150.0)
                            .font_size(28.0)
                            .column_gap(15.0)
                            .label_color(Color::srgb(0.8, 0.8, 0.8))
                            .value_color(Color::srgb(0.6, 1.0, 0.6))
                            .build(),
                    );

                    // XP Bar
                    let xp_current = prog.xp;
                    let xp_needed = Progression::xp_to_next_level(prog.level);
                    let xp_percent = (xp_current as f32 / xp_needed as f32 * 100.0) as i32;
                    let xp_bar = create_text_progress_bar(xp_current, xp_needed, 20);

                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            margin: UiRect::top(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // XP Label
                            parent.spawn((
                                Text::new(format!("XP: {} / {} ({}%)", xp_current, xp_needed, xp_percent)),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.8, 0.5, 1.0)),
                            ));

                            // XP Bar
                            parent.spawn((
                                Text::new(xp_bar),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.4, 0.8)),
                            ));
                        });
                });

            // Instructions
            parent.spawn((
                Text::new("Press Backspace to return to Menu"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}

/// Creates a text-based progress bar like "[=====>    ] 50%"
fn create_text_progress_bar(current: i32, max: i32, width: usize) -> String {
    let filled_count = if max > 0 {
        ((current as f32 / max as f32) * width as f32) as usize
    } else {
        0
    };
    let filled_count = filled_count.min(width);

    let mut bar = String::from("[");

    // Filled portion with arrow
    if filled_count > 0 {
        for _ in 0..(filled_count - 1) {
            bar.push('=');
        }
        bar.push('>');
    }

    // Empty portion
    for _ in filled_count..width {
        bar.push(' ');
    }

    bar.push(']');
    bar
}

/// System to handle Back action to return to Menu.
fn handle_back_action(
    mut action_reader: MessageReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::Back {
            next_state.set(AppState::Menu);
        }
    }
}

/// System to despawn the profile screen UI.
fn despawn_profile_screen(
    mut commands: Commands,
    profile_root: Query<Entity, With<ProfileScreenRoot>>,
) {
    if let Ok(entity) = profile_root.get_single() {
        commands.entity(entity).despawn();
    }
}
