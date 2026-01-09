use bevy::prelude::*;

use crate::combat::{PlayerCombatAction, PostCombatAction};
use crate::input::{GameAction, NavigationDirection};

use super::components::{ActionMenuItem, PostCombatMenuItem};
use super::state::FightScreenState;

pub fn handle_player_turn_input(
    mut action_reader: EventReader<GameAction>,
    mut fight_state: ResMut<FightScreenState>,
    mut combat_action: EventWriter<PlayerCombatAction>,
    mut action_items: Query<(&ActionMenuItem, &mut TextColor, &mut Text)>,
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
    mut post_combat_items: Query<(&PostCombatMenuItem, &mut TextColor), Without<ActionMenuItem>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                fight_state.post_combat_up();
                update_post_combat_visuals(&fight_state, &mut post_combat_items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                fight_state.post_combat_down();
                update_post_combat_visuals(&fight_state, &mut post_combat_items);
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
    items: &mut Query<(&ActionMenuItem, &mut TextColor, &mut Text)>,
) {
    let labels = ["Attack", "Run"];
    for (item, mut color, mut text) in items.iter_mut() {
        let selected = item.index == state.action_selection;
        if selected {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0));
            **text = format!("> {}", labels[item.index]);
        } else {
            *color = TextColor(Color::srgb(0.5, 0.5, 0.5));
            **text = format!("  {}", labels[item.index]);
        }
    }
}

fn update_post_combat_visuals(
    state: &FightScreenState,
    items: &mut Query<(&PostCombatMenuItem, &mut TextColor), Without<ActionMenuItem>>,
) {
    for (item, mut color) in items.iter_mut() {
        if item.index == state.post_combat_selection {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0));
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7));
        }
    }
}
