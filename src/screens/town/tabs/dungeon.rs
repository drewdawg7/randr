use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::states::RequestDungeonEvent;
use crate::ui::spawn_navigation_hint;

use super::super::shared::{
    spawn_menu, update_menu_selection, MenuOption, MenuOptionItem, MenuOptionText,
};
use super::super::{ContentArea, TabContent, TownTab};

/// Plugin for the Dungeon tab.
pub struct DungeonTabPlugin;

impl Plugin for DungeonTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DungeonTabState>()
            .add_systems(OnEnter(TownTab::Dungeon), spawn_dungeon_content)
            .add_systems(
                Update,
                (
                    handle_dungeon_input,
                    update_dungeon_selection.run_if(resource_changed::<DungeonTabState>),
                )
                    .run_if(in_state(TownTab::Dungeon)),
            );
    }
}

/// Spawns dungeon UI content when entering the Dungeon tab.
fn spawn_dungeon_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    dungeon_state: Res<DungeonTabState>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_dungeon_ui(&mut commands, content_entity, &dungeon_state);
}

/// Updates dungeon menu selection highlighting reactively.
fn update_dungeon_selection(
    dungeon_state: Res<DungeonTabState>,
    mut menu_query: Query<(&MenuOptionItem, &mut BackgroundColor, &Children)>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<MenuOptionText>>,
) {
    update_menu_selection(
        dungeon_state.selected_index,
        &mut menu_query,
        &mut text_query,
    );
}

/// Dungeon tab state - tracks menu selection.
#[derive(Resource, Default)]
pub struct DungeonTabState {
    pub selected_index: usize,
}

const DUNGEON_OPTIONS: &[MenuOption] = &[MenuOption {
    label: "Enter Dungeon",
    description: Some("Descend into the depths"),
}];

/// Handle input for the Dungeon tab.
fn handle_dungeon_input(
    mut dungeon_state: ResMut<DungeonTabState>,
    mut action_events: EventReader<GameAction>,
    mut dungeon_events: EventWriter<RequestDungeonEvent>,
) {
    for action in action_events.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                if dungeon_state.selected_index > 0 {
                    dungeon_state.selected_index -= 1;
                } else {
                    dungeon_state.selected_index = DUNGEON_OPTIONS.len() - 1;
                }
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                dungeon_state.selected_index =
                    (dungeon_state.selected_index + 1) % DUNGEON_OPTIONS.len();
            }
            GameAction::Select => match dungeon_state.selected_index {
                0 => {
                    dungeon_events.send(RequestDungeonEvent);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Spawn the dungeon UI.
pub fn spawn_dungeon_ui(
    commands: &mut Commands,
    content_entity: Entity,
    dungeon_state: &DungeonTabState,
) {
    commands.entity(content_entity).with_children(|parent| {
        parent
            .spawn((
                TabContent,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
            ))
            .with_children(|content| {
                // Menu options
                spawn_menu(
                    content,
                    DUNGEON_OPTIONS,
                    dungeon_state.selected_index,
                    Some("Dungeon"),
                );

                // Navigation hint
                spawn_navigation_hint(
                    content,
                    "[↑↓] Navigate  [Enter] Select  [←→] Switch Tab",
                );
            });
    });
}
