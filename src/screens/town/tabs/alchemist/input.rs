//! Alchemist tab input handling.

use bevy::prelude::*;

use crate::game::BrewPotionEvent;
use crate::input::{GameAction, NavigationDirection};

use super::state::{AlchemistMode, AlchemistModeKind, AlchemistSelections};

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
                    alchemist_selections.menu.move_up();
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_selections.menu.move_down();
                }
                GameAction::Select => {
                    // Only one option currently: Brew
                    if alchemist_selections.menu.selected == 0 {
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
                    alchemist_selections.menu.reset();
                }
                _ => {}
            },
        }
    }
}
