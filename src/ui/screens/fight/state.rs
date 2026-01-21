use bevy::prelude::*;

use crate::ui::SelectionState;

#[derive(Resource, Default)]
pub struct FightScreenState {
    pub action_selection: usize,
    pub post_combat_selection: usize,
}

impl FightScreenState {
    pub const ACTION_ITEMS: usize = 2;
    pub const POST_COMBAT_ITEMS: usize = 2;

    pub fn reset(&mut self) {
        self.action_selection = 0;
        self.post_combat_selection = 0;
    }
}

/// Selection state wrapper for action menu (Attack, Run).
pub struct ActionSelection<'a>(pub &'a mut FightScreenState);

impl SelectionState for ActionSelection<'_> {
    fn selected(&self) -> usize {
        self.0.action_selection
    }

    fn count(&self) -> usize {
        FightScreenState::ACTION_ITEMS
    }

    fn set_selected(&mut self, index: usize) {
        self.0.action_selection = index;
    }
}

/// Selection state wrapper for post-combat menu (Fight Again, Continue).
pub struct PostCombatSelection<'a>(pub &'a mut FightScreenState);

impl SelectionState for PostCombatSelection<'_> {
    fn selected(&self) -> usize {
        self.0.post_combat_selection
    }

    fn count(&self) -> usize {
        FightScreenState::POST_COMBAT_ITEMS
    }

    fn set_selected(&mut self, index: usize) {
        self.0.post_combat_selection = index;
    }
}
