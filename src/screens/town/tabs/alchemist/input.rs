//! Alchemist tab input handling.

use bevy::prelude::*;

use crate::game::BrewPotionEvent;
use crate::input::{GameAction, NavigationDirection};

use super::state::{AlchemistMode, AlchemistModeKind, AlchemistSelections};
use super::ALCHEMIST_MENU_OPTIONS;

/// Handle input for the Alchemist tab.
pub fn handle_alchemist_input(
    mut alchemist_mode: ResMut<AlchemistMode>,
    mut alchemist_selections: ResMut<AlchemistSelections>,
    mut action_events: EventReader<GameAction>,
    mut brew_events: EventWriter<BrewPotionEvent>,
) {
    for action in action_events.read() {
        match alchemist_mode.mode {
            AlchemistModeKind::Menu => match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    if alchemist_selections.menu > 0 {
                        alchemist_selections.menu -= 1;
                    } else {
                        alchemist_selections.menu = ALCHEMIST_MENU_OPTIONS.len() - 1;
                    }
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_selections.menu =
                        (alchemist_selections.menu + 1) % ALCHEMIST_MENU_OPTIONS.len();
                }
                GameAction::Select => {
                    // Only one option currently: Brew
                    if alchemist_selections.menu == 0 {
                        alchemist_mode.mode = AlchemistModeKind::Brew;
                        alchemist_selections.recipe.reset();
                    }
                }
                _ => {}
            },
            AlchemistModeKind::Brew => match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    alchemist_selections.recipe.move_up();
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_selections.recipe.move_down();
                }
                GameAction::Select => {
                    // Emit brewing event - game logic handled by CraftingPlugin
                    if let Some(&recipe_id) = alchemist_mode
                        .available_recipes
                        .get(alchemist_selections.recipe.selected)
                    {
                        brew_events.send(BrewPotionEvent { recipe_id });
                    }
                }
                GameAction::Back => {
                    alchemist_mode.mode = AlchemistModeKind::Menu;
                    alchemist_selections.menu = 0;
                }
                _ => {}
            },
        }
    }
}
