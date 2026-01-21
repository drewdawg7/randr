use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::AppState;
use crate::ui::widgets::PlayerStats;

use super::components::{ContentArea, TownUiRoot};

pub fn setup_town_ui(mut commands: Commands) {
    commands
        .spawn((
            TownUiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Player stats banner at top
            parent.spawn(PlayerStats);

            parent.spawn((
                ContentArea,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
            ));
        });
}

pub fn cleanup_town_ui(mut commands: Commands, query: Query<Entity, With<TownUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_back_action(
    mut action_events: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            next_state.set(AppState::Menu);
        }
    }
}
