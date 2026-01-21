use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FightScreenState {
    pub action_selection: usize,
    pub post_combat_selection: usize,
}

impl FightScreenState {
    const ACTION_ITEMS: usize = 2;
    const POST_COMBAT_ITEMS: usize = 2;

    pub fn action_up(&mut self) {
        if self.action_selection > 0 {
            self.action_selection -= 1;
        }
    }

    pub fn action_down(&mut self) {
        if self.action_selection + 1 < Self::ACTION_ITEMS {
            self.action_selection += 1;
        }
    }

    pub fn post_combat_up(&mut self) {
        if self.post_combat_selection > 0 {
            self.post_combat_selection -= 1;
        }
    }

    pub fn post_combat_down(&mut self) {
        if self.post_combat_selection + 1 < Self::POST_COMBAT_ITEMS {
            self.post_combat_selection += 1;
        }
    }

    pub fn reset(&mut self) {
        self.action_selection = 0;
        self.post_combat_selection = 0;
    }
}
