use bevy::prelude::*;

use crate::combat::{PlayerCombatAction, PostCombatAction};
use crate::input::{GameAction, NavigationDirection};
use crate::ui::{nav_selection_text, update_menu_colors, MenuIndex};

use super::components::{ActionMenuItem, PostCombatMenuItem};
use super::state::FightScreenState;

pub fn handle_player_turn_input(
    mut action_reader: EventReader<GameAction>,
    mut fight_state: ResMut<FightScreenState>,
    mut combat_action: EventWriter<PlayerCombatAction>,
    mut action_items: Query<(&MenuIndex, &mut TextColor, &mut Text), With<ActionMenuItem>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                fight_state.action_up();
                update_action_visuals(&fight_state, &mut action_items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                fight_state.action_down();
                update_action_visuals(&fight_state, &mut action_items);
            }
            GameAction::Select => {
                let action = match fight_state.action_selection {
                    0 => PlayerCombatAction::Attack,
                    1 => PlayerCombatAction::Run,
                    _ => continue,
                };
                combat_action.send(action);
            }
            _ => {}
        }
    }
}

pub fn handle_post_combat_input(
    mut action_reader: EventReader<GameAction>,
    mut fight_state: ResMut<FightScreenState>,
    mut post_combat_action: EventWriter<PostCombatAction>,
    mut post_combat_items: Query<(&MenuIndex, &mut TextColor), With<PostCombatMenuItem>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                fight_state.post_combat_up();
                update_menu_colors::<PostCombatMenuItem>(
                    fight_state.post_combat_selection,
                    &mut post_combat_items,
                );
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                fight_state.post_combat_down();
                update_menu_colors::<PostCombatMenuItem>(
                    fight_state.post_combat_selection,
                    &mut post_combat_items,
                );
            }
            GameAction::Select => {
                let action = match fight_state.post_combat_selection {
                    0 => PostCombatAction::FightAgain,
                    1 => PostCombatAction::Continue,
                    _ => continue,
                };
                post_combat_action.send(action);
                fight_state.reset();
            }
            _ => {}
        }
    }
}

fn update_action_visuals(
    state: &FightScreenState,
    items: &mut Query<(&MenuIndex, &mut TextColor, &mut Text), With<ActionMenuItem>>,
) {
    let labels = ["Attack", "Run"];
    for (menu_index, mut color, mut text) in items.iter_mut() {
        let selected = menu_index.0 == state.action_selection;
        let prefix = if selected { ">" } else { " " };
        *color = TextColor(nav_selection_text(selected));
        **text = format!("{} {}", prefix, labels[menu_index.0]);
    }
}
