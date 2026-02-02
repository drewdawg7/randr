use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::{AppState, PreviousState, StateTransitionRequest};
use crate::ui::column_node;

pub struct KeybindsPlugin;

impl Plugin for KeybindsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Keybinds), spawn_keybinds_screen)
            .add_systems(OnExit(AppState::Keybinds), despawn_keybinds_screen)
            .add_systems(
                Update,
                handle_close_action.run_if(in_state(AppState::Keybinds)),
            );
    }
}

#[derive(Component)]
struct KeybindsScreenRoot;

struct KeybindCategory {
    name: &'static str,
    bindings: Vec<(&'static str, &'static str)>,
}

fn spawn_keybinds_screen(mut commands: Commands) {
    let categories = vec![
        KeybindCategory {
            name: "Navigation",
            bindings: vec![
                ("Arrow Keys", "Navigate menus and lists"),
                ("Enter", "Select / Confirm"),
                ("Backspace", "Back / Cancel"),
                ("Tab", "Next tab"),
                ("Shift+Tab", "Previous tab"),
            ],
        },
        KeybindCategory {
            name: "Actions",
            bindings: vec![("Space", "Mine / Attack")],
        },
        KeybindCategory {
            name: "Menus & Modals",
            bindings: vec![
                ("I", "Open Inventory"),
                ("P", "Open Profile"),
                ("?", "Open Keybinds (this screen)"),
                ("Escape", "Close modal"),
            ],
        },
    ];

    commands
        .spawn((
            KeybindsScreenRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(40.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    max_width: Val::Px(800.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Keybinds & Controls"),
                        TextFont {
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.9, 0.3)),
                        Node {
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        },
                    ));

                    parent
                        .spawn(column_node(30.0))
                        .with_children(|parent| {
                            for category in categories {
                                spawn_category(parent, category);
                            }
                        });

                    parent.spawn((
                        Text::new("Press Escape to close"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        Node {
                            margin: UiRect::top(Val::Px(30.0)),
                            ..default()
                        },
                    ));
                });
        });
}

fn spawn_category(parent: &mut ChildSpawnerCommands, category: KeybindCategory) {
    parent
        .spawn(column_node(10.0))
        .with_children(|parent| {
            parent.spawn((
                Text::new(category.name),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.8, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            for (key, description) in category.bindings {
                spawn_keybind_row(parent, key, description);
            }
        });
}

fn spawn_keybind_row(parent: &mut ChildSpawnerCommands, key: &str, description: &str) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.0),
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    padding: UiRect::new(Val::Px(12.0), Val::Px(12.0), Val::Px(6.0), Val::Px(6.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    min_width: Val::Px(120.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(key),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });

            parent.spawn((
                Text::new(description),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));
        });
}

fn handle_close_action(
    mut action_reader: MessageReader<GameAction>,
    mut state_requests: MessageWriter<StateTransitionRequest>,
    previous_state: Res<PreviousState>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            let target: StateTransitionRequest = previous_state
                .state
                .map_or(StateTransitionRequest::Menu, Into::into);
            state_requests.write(target);
        }
    }
}

fn despawn_keybinds_screen(
    mut commands: Commands,
    keybinds_root: Query<Entity, With<KeybindsScreenRoot>>,
) {
    if let Ok(entity) = keybinds_root.single() {
        commands.entity(entity).despawn();
    }
}
